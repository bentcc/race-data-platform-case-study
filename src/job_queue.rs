//! A durable SQLite queue with atomic claims and generation-fenced settlement.

use std::io;
use std::time::Duration;

use rusqlite::{params, Connection, OptionalExtension};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct JobQueue {
    connection: Connection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    pub job_key: String,
    pub lease_until: i64,
    generation: i64,
}

impl JobQueue {
    /// Use a dedicated database; times throughout this API are Unix seconds.
    pub fn new(connection: Connection) -> Result<Self> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch(
            "PRAGMA journal_mode = WAL;
             CREATE TABLE IF NOT EXISTS jobs (
                 job_key TEXT PRIMARY KEY NOT NULL CHECK (length(job_key) > 0),
                 due_at INTEGER NOT NULL,
                 generation INTEGER NOT NULL DEFAULT 0,
                 lease_until INTEGER,
                 settled INTEGER NOT NULL DEFAULT 0 CHECK (settled IN (0, 1))
             );",
        )?;
        Ok(Self { connection })
    }

    /// Returns false for an existing key, preserving its original schedule and state.
    pub fn enqueue(&self, job_key: &str, due_at: i64) -> Result<bool> {
        Ok(self.connection.execute(
            "INSERT INTO jobs (job_key, due_at) VALUES (?1, ?2)
             ON CONFLICT (job_key) DO NOTHING",
            params![job_key, due_at],
        )? == 1)
    }

    /// Claims the earliest due job; an expired lease can be claimed again.
    pub fn claim(&self, now: i64, lease_seconds: i64) -> Result<Option<Claim>> {
        let lease_until = now
            .checked_add(lease_seconds)
            .filter(|_| lease_seconds > 0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid lease duration"))?;

        // Selection and ownership change share one atomic SQLite statement.
        Ok(self
            .connection
            .query_row(
                "UPDATE jobs SET generation = generation + 1, lease_until = ?1
                 WHERE job_key = (
                     SELECT job_key FROM jobs
                     WHERE settled = 0 AND due_at <= ?2
                       AND (lease_until IS NULL OR lease_until <= ?2)
                     ORDER BY due_at, job_key LIMIT 1
                 )
                 RETURNING job_key, generation, lease_until",
                params![lease_until, now],
                |row| {
                    Ok(Claim {
                        job_key: row.get(0)?,
                        generation: row.get(1)?,
                        lease_until: row.get(2)?,
                    })
                },
            )
            .optional()?)
    }

    /// Returns true for successful or repeated settlement, false for stale/expired claims.
    pub fn settle(&self, claim: &Claim, now: i64) -> Result<bool> {
        Ok(self.connection.execute(
            "UPDATE jobs SET settled = 1, lease_until = NULL
             WHERE job_key = ?1 AND generation = ?2
               AND (settled = 1 OR lease_until > ?3)",
            params![claim.job_key, claim.generation, now],
        )? == 1)
    }
}
