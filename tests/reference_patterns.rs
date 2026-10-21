use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Barrier};

use race_data_platform_examples::{
    backup::{verify_backup, ManifestEntry},
    job_queue::JobQueue,
    scheduling::plan,
};
use rusqlite::Connection;

struct TestRoot(PathBuf);

impl TestRoot {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "race-data-reference-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn queue_recovers_expired_claims_and_rejects_stale_settlement() {
    let queue = JobQueue::new(Connection::open_in_memory().unwrap()).unwrap();
    assert!(queue.enqueue("b", 100).unwrap());
    assert!(queue.enqueue("a", 100).unwrap());
    assert!(!queue.enqueue("a", 999).unwrap());
    assert!(queue.enqueue("future", 200).unwrap());
    assert!(queue.claim(99, 10).unwrap().is_none());

    let first = queue.claim(100, 10).unwrap().unwrap();
    let second = queue.claim(100, 10).unwrap().unwrap();
    assert_eq!(first.job_key, "a");
    assert_eq!(second.job_key, "b");
    assert!(queue.claim(109, 10).unwrap().is_none());
    assert!(!queue.settle(&first, 110).unwrap());

    let retry = queue.claim(110, 10).unwrap().unwrap();
    assert_eq!(retry.job_key, first.job_key);
    assert!(!queue.settle(&first, 111).unwrap());
    assert!(queue.settle(&retry, 111).unwrap());
    assert!(queue.settle(&retry, 999).unwrap());
    assert!(!queue.settle(&second, 111).unwrap());
}

#[test]
fn separate_connections_claim_distinct_jobs_and_settlement_survives_reopen() {
    let root = TestRoot::new();
    let database = root.0.join("queue.sqlite");
    let queue = JobQueue::new(Connection::open(&database).unwrap()).unwrap();
    queue.enqueue("a", 100).unwrap();
    queue.enqueue("b", 100).unwrap();
    drop(queue);

    let barrier = Arc::new(Barrier::new(2));
    let queues: Vec<_> = (0..2)
        .map(|_| JobQueue::new(Connection::open(&database).unwrap()).unwrap())
        .collect();
    let workers: Vec<_> = queues
        .into_iter()
        .map(|queue| {
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                queue.claim(100, 10).unwrap().unwrap()
            })
        })
        .collect();
    let claims: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_ne!(claims[0].job_key, claims[1].job_key);

    let queue = JobQueue::new(Connection::open(&database).unwrap()).unwrap();
    assert!(queue.claim(101, 10).unwrap().is_none());
    for claim in &claims {
        assert!(queue.settle(claim, 101).unwrap());
    }
    drop(queue);
    let queue = JobQueue::new(Connection::open(&database).unwrap()).unwrap();
    assert!(queue.claim(200, 10).unwrap().is_none());
}

#[test]
fn invalid_queue_inputs_do_not_claim_work() {
    let queue = JobQueue::new(Connection::open_in_memory().unwrap()).unwrap();
    assert!(queue.enqueue("", 100).is_err());
    queue.enqueue("a", 100).unwrap();
    assert!(queue.claim(100, 0).is_err());
    assert!(queue.claim(100, -1).is_err());
    assert!(queue.claim(i64::MAX, 1).is_err());
    assert_eq!(queue.claim(100, 10).unwrap().unwrap().job_key, "a");
}

#[test]
fn scheduling_is_order_independent_deduplicated_and_checked() {
    let jobs = plan("synthetic-event", 1_000, &[0, -60, 60, -60]).unwrap();
    assert_eq!(jobs, plan("synthetic-event", 1_000, &[60, -60, 0]).unwrap());
    assert_eq!(
        jobs.iter().map(|job| job.due_at).collect::<Vec<_>>(),
        vec![940, 1_000, 1_060]
    );
    assert_eq!(jobs[0].job_key, "synthetic-event:-60");
    assert_ne!(jobs, plan("other-event", 1_000, &[-60, 0, 60]).unwrap());
    assert!(plan("event", 1_000, &[]).unwrap().is_empty());
    assert!(plan("", 1_000, &[0]).is_err());
    assert!(plan("event", i64::MAX, &[1]).is_err());
    assert!(plan("event", i64::MIN, &[-1]).is_err());
}

#[test]
fn planned_jobs_enqueue_and_settle_without_duplicates() {
    let queue = JobQueue::new(Connection::open_in_memory().unwrap()).unwrap();
    let jobs = plan("synthetic-event", 1_000, &[-60, 0]).unwrap();
    for job in &jobs {
        assert!(queue.enqueue(&job.job_key, job.due_at).unwrap());
        assert!(!queue.enqueue(&job.job_key, job.due_at).unwrap());
    }
    let claim = queue.claim(940, 10).unwrap().unwrap();
    assert_eq!(claim.job_key, jobs[0].job_key);
    assert!(queue.settle(&claim, 941).unwrap());
    assert!(queue.claim(941, 10).unwrap().is_none());
    assert_eq!(
        queue.claim(1_000, 10).unwrap().unwrap().job_key,
        jobs[1].job_key
    );
}

#[test]
fn backup_verification_detects_same_size_tampering_size_changes_and_missing_files() {
    let root = TestRoot::new();
    let file = root.0.join("evidence.txt");
    fs::write(&file, b"abc").unwrap();
    let entry = ManifestEntry::capture(&root.0, Path::new("evidence.txt")).unwrap();
    assert_eq!(entry.size_bytes, 3);
    assert_eq!(
        entry.sha256,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let manifest = [entry];
    verify_backup(&root.0, &manifest).unwrap();

    fs::write(&file, b"abd").unwrap();
    assert_eq!(
        verify_backup(&root.0, &manifest).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    fs::write(&file, b"abcd").unwrap();
    assert!(verify_backup(&root.0, &manifest).is_err());
    fs::remove_file(&file).unwrap();
    assert_eq!(
        verify_backup(&root.0, &manifest).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
}

#[test]
fn backup_verification_checks_every_file_and_streams_multiple_buffers() {
    let source = TestRoot::new();
    let restored = TestRoot::new();
    fs::create_dir(source.0.join("nested")).unwrap();
    fs::create_dir(restored.0.join("nested")).unwrap();
    fs::write(source.0.join("a.txt"), b"synthetic evidence").unwrap();
    fs::write(source.0.join("nested/b.txt"), vec![42; 20_000]).unwrap();
    let paths = ["a.txt", "nested/b.txt"];
    let manifest: Vec<_> = paths
        .iter()
        .map(|path| ManifestEntry::capture(&source.0, Path::new(path)).unwrap())
        .collect();
    for path in paths {
        fs::copy(source.0.join(path), restored.0.join(path)).unwrap();
    }
    verify_backup(&restored.0, &manifest).unwrap();
    fs::write(restored.0.join("nested/b.txt"), vec![43; 20_000]).unwrap();
    assert!(verify_backup(&restored.0, &manifest).is_err());
}

#[test]
fn backup_verification_rejects_empty_manifests_directories_and_unsafe_paths() {
    let root = TestRoot::new();
    assert!(verify_backup(&root.0, &[]).is_err());
    for path in [Path::new(""), Path::new("../outside"), root.0.as_path()] {
        let entry = ManifestEntry {
            relative_path: path.to_owned(),
            size_bytes: 0,
            sha256: String::new(),
        };
        assert_eq!(
            verify_backup(&root.0, &[entry]).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }
    fs::create_dir(root.0.join("directory")).unwrap();
    assert!(ManifestEntry::capture(&root.0, Path::new("directory")).is_err());
}

#[cfg(unix)]
#[test]
fn backup_verification_rejects_symlinks_outside_the_root() {
    let root = TestRoot::new();
    let outside = TestRoot::new();
    fs::write(outside.0.join("evidence.txt"), b"outside evidence").unwrap();
    std::os::unix::fs::symlink(outside.0.join("evidence.txt"), root.0.join("escape")).unwrap();
    assert_eq!(
        ManifestEntry::capture(&root.0, Path::new("escape"))
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
}
