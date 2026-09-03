# Reliability and Operational Safety

## 1. Reliability model

The platform assumes ordinary failures will happen:

- remote requests time out or return incomplete content;
- source formats change;
- the process restarts mid-job;
- SQLite writers contend briefly;
- backup uploads fail after local work has completed;
- a parser bug is discovered after data has already been collected;
- an operator needs to repair historical state.

The system is designed to make those failures observable and recoverable rather than impossible.

## 2. Failure containment

### Preserve evidence before aggressive interpretation

Where appropriate, source payloads are retained so that a normalization defect can be corrected without requiring the exact remote response to still exist.

### Keep source outcome separate from domain meaning

"HTTP request completed" is not the same as "usable domain truth was obtained". Collection results and admission/normalization decisions are represented separately.

### Bound writes

Critical multi-row state changes use explicit transactions. Writer-sensitive maintenance operations use an explicit maintenance ownership model.

### Prefer replay/reconciliation over silent mutation

When historical inconsistencies are found, the preferred recovery shape is inventory -> dry-run/reconcile -> apply -> verify, rather than direct ad-hoc database edits.

## 3. SQLite concurrency

The workload is deliberately single-node, but it still has concurrent concerns: daemon work, read-only dashboard traffic and occasional maintenance operations.

The implementation uses SQLite WAL, busy-timeout handling, bounded transactions and explicit writer coordination. For operations requiring stronger exclusion, `BEGIN IMMEDIATE`-style writer acquisition is used so failure occurs at a predictable boundary rather than halfway through a logical change.

This is not a substitute for a distributed database. It is an intentional fit for the actual deployment model.

## 4. Maintenance windows

Some operations should not race with routine writers: schema convergence, selected repair operations, vacuuming, or recovery cutovers. The platform therefore models maintenance ownership explicitly rather than relying on "I think the daemon is stopped" as the only safety control.

Useful maintenance facts include:

- owner identity;
- acquisition time;
- stale/timeout policy;
- post-operation verification;
- explicit release or controlled stale-clear procedure.

## 5. Health and observability

The operator surface focuses on actionable state rather than decorative metrics. Examples of useful questions are:

- Is collection progressing?
- Are planner jobs accumulating or stuck?
- Is retained evidence being written?
- Is backup current and verifiable?
- Are there stale claims or maintenance ownership?
- Did a recent release change observable behavior?

The dashboard is read-only to avoid turning an observation surface into an unreviewed mutation API.

## 6. Testing strategy

The private project includes tests around areas that commonly fail only during operations, including:

- job claim lifecycle and stale recovery;
- schema/migration compatibility;
- backup selection and manifest behavior;
- interrupted/resumable upload paths;
- restore/recovery cases;
- release workflow contract checks;
- normalization and replay invariants.

The CI gate also runs formatting, linting with warnings denied, and the locked workspace test suite.

## 7. Known limits

This architecture does not claim:

- multi-region availability;
- horizontal writer scaling;
- exactly-once execution across distributed nodes;
- zero-loss behavior under arbitrary host/disk failure.

Those would require a different system design. The current objective is a robust and auditable single-node service with verified off-host recovery.
