use race_data_platform_examples::{job_queue::JobQueue, scheduling::plan};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = JobQueue::new(Connection::open_in_memory()?)?;
    let jobs = plan("synthetic-event-001", 1_700_000_000, &[-300, -60, 0, -60])?;
    assert_eq!(jobs.len(), 3);
    for job in &jobs {
        assert!(queue.enqueue(&job.job_key, job.due_at)?);
        assert!(!queue.enqueue(&job.job_key, job.due_at)?);
        println!("{} due at {}", job.job_key, job.due_at);
    }
    let claim = queue.claim(1_699_999_700, 30)?.expect("first job is due");
    assert_eq!(claim.job_key, jobs[0].job_key);
    assert!(queue.settle(&claim, 1_699_999_701)?);
    Ok(())
}
