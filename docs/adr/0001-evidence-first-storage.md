# ADR 0001: Retain Source Evidence Before Normalized Projection

- Status: Accepted
- Scope: Data ingestion and recovery

## Context

Remote public data can change, parsers can contain defects, and later code may interpret an earlier response differently. If ingestion stores only the latest normalized row, debugging and historical correction depend on the remote source still returning equivalent content.

## Decision

For important collection paths, retain source evidence and link normalization/observation records to that evidence. Treat current normalized state as a projection rather than the only surviving record of what happened.

## Consequences

### Positive

- normalization can be replayed after parser fixes;
- incidents can be investigated against original evidence;
- repeated observations and revisions are explainable;
- destructive "update the truth" logic is reduced.

### Negative

- storage usage increases;
- retention policy becomes a real operational concern;
- lineage and identity rules are more complex;
- consumers need read models rather than directly selecting the newest arbitrary row.

## Rejected alternative

**Store only normalized current state.** Simpler initially, but too weak for the project's audit and recovery goals.
