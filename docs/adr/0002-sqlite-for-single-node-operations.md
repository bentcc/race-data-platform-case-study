# ADR 0002: Use SQLite for the Single-Node Operational Store

- Status: Accepted
- Scope: Persistence and operations

## Context

The workload needs durable scheduling state, transactional updates, structured queries and operational read models. It does not currently require horizontally scaled independent writers or multi-region failover.

## Decision

Use SQLite as the primary operational database, with WAL, busy-timeout handling, explicit transaction boundaries and explicit maintenance/writer coordination.

## Why this is appropriate

- the service is intentionally single-node;
- deployment and backup remain simple;
- transactions are strong enough for planner/claim/state transitions;
- operational data can be inspected and recovered without running another database service;
- read-heavy dashboard access works well with WAL when writes remain bounded.

## Constraints

SQLite is not being treated as a distributed coordination system. If the workload evolves toward multiple independent writer nodes or substantially higher concurrent write throughput, the persistence architecture should be revisited.

## Rejected alternative

**Adopt a client/server database immediately.** It would add operational surface area without solving a current requirement. The project prefers complexity justified by workload rather than by résumé signaling.
