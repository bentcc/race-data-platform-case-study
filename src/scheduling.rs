//! Deterministic jobs derived from an event time and signed second offsets.

use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedJob {
    pub job_key: String,
    pub due_at: i64,
}

/// Negative offsets run before the event. Duplicate offsets produce one job.
/// An event-time change preserves job identity; rescheduling existing jobs is a separate policy.
pub fn plan(
    event_id: &str,
    event_at: i64,
    offsets: &[i64],
) -> Result<Vec<PlannedJob>, &'static str> {
    if event_id.is_empty() {
        return Err("event identity must not be empty");
    }
    offsets
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|offset| {
            let due_at = event_at.checked_add(offset).ok_or("timestamp overflow")?;
            Ok(PlannedJob {
                job_key: format!("{event_id}:{offset}"),
                due_at,
            })
        })
        .collect()
}
