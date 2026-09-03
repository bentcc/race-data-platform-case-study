# Data Model

## 1. Design objective

The persistence model is designed for **traceability under change**. Public source pages and APIs can change, parsers can be corrected, and the same domain entity can be observed multiple times before an event occurs. A purely overwrite-oriented schema would lose information needed to explain those changes.

## 2. Logical layers

The exact production schema is private. Conceptually, the store has five layers.

### A. Domain identity

Stable identifiers represent events and entities independently from a particular HTTP response. These identities provide join points for observations collected at different times.

### B. Raw evidence and source observations

An observation records that a source was fetched at a particular time and links to retained evidence where applicable. The purpose is not to keep arbitrary blobs forever; it is to retain enough information to audit and replay important transformations.

Conceptual shape:

```text
source_observation
- observation_id
- subject_id
- source_family
- observed_at
- payload_fingerprint
- retained_payload_ref
- collection_outcome
```

### C. Normalization runs and immutable content

Normalization is modeled as an interpretation step. A run can create immutable normalized content identified by content identity/fingerprint rather than blindly updating the previous row.

```text
normalization_run
- run_id
- observation_id
- parser/schema version
- started_at
- outcome

normalized_content
- content_id
- subject_id
- content_fingerprint
- normalized fields ...
```

### D. Admission/current projection

Not every observed or normalized candidate should become the current accepted representation. Admission/assessment records capture that decision, while read models expose the current convenient view.

This separation supports cases such as:

- repeated identical observations;
- partial/malformed source responses;
- later corrections;
- parser improvements;
- evidence replay.

### E. Operational state

Planner jobs, claims, health facts, backup manifests and maintenance ownership are operational records, not domain facts. Keeping that distinction explicit reduces accidental coupling.

## 3. Append-oriented history

The system favors append-oriented records for observations and accepted content history. "Current" state is a projection, not the only surviving truth.

Benefits:

- auditability;
- easier reconciliation after parser changes;
- deterministic replay from retained evidence;
- less destructive correction logic;
- clearer incident investigation.

Cost:

- more tables and stronger identity rules;
- read models are required for simple consumers;
- migrations need explicit compatibility thinking.

For this project, those costs are justified because the workload values correctness and recoverability over minimal schema size.

## 4. SQLite schema evolution

Schema changes are applied as ordered migrations. The operational approach emphasizes:

- additive tables/columns where practical;
- explicit migration versioning;
- foreign-key validation;
- bounded maintenance windows for writer-sensitive changes;
- migration/readback verification;
- separation of schema migration from large historical backfills.

A key lesson is that **DDL completion is not equivalent to data migration completion**. Large historical transformations are safer when they are separately resumable and observable.

## 5. Transaction boundaries

Writes that must agree are grouped in explicit transactions. Writer-sensitive operations use deliberate locking behavior rather than depending on default implicit transaction timing.

SQLite is configured for the single-node workload with WAL and a busy timeout. That does not remove the need for transaction discipline; it simply gives concurrent readers and short-lived writers a workable operating envelope.

## 6. What is intentionally omitted

The public case study excludes:

- exact table and column names for source-specific domain content;
- source URLs and request parameters;
- retained payload directory structure where it exposes production naming;
- real identifiers, timestamps or collected records;
- migration SQL from the private repository.

The goal is to expose the engineering model without publishing the production data contract.
