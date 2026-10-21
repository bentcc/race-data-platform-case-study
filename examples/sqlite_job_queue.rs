use race_data_platform_examples::job_queue::JobQueue;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = JobQueue::new(Connection::open_in_memory()?)?;
    assert!(queue.enqueue("synthetic-event-001:collect", 100)?);
    assert!(!queue.enqueue("synthetic-event-001:collect", 100)?);

    let abandoned = queue.claim(100, 10)?.expect("due job");
    assert!(queue.claim(105, 10)?.is_none());
    let retry = queue.claim(110, 10)?.expect("expired lease is recoverable");
    assert!(!queue.settle(&abandoned, 111)?);
    assert!(queue.settle(&retry, 111)?);
    assert!(queue.settle(&retry, 112)?);
    assert!(queue.claim(120, 10)?.is_none());
    println!("Recovered an expired claim, rejected stale settlement, and settled idempotently.");
    Ok(())
}
