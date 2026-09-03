# Backup and Recovery

## 1. Principle

A copied database file is not, by itself, a recovery strategy. The platform treats backup as a stateful workflow with selection, packaging, metadata, verification, off-host transfer, retention and restore semantics.

## 2. Backup unit

A logical backup export can include:

- SQLite state relevant to the selected scope;
- retained raw evidence;
- an export manifest;
- fingerprints/checksums and lineage metadata;
- upload/result metadata.

The exact production archive format and remote provider identifiers are intentionally omitted.

## 3. Backup lifecycle

```mermaid
flowchart LR
    A[Eligible local state] --> B[Stage export]
    B --> C[Build manifest]
    C --> D[Fingerprint / verify]
    D --> E[Archive]
    E --> F[Upload off-host]
    F --> G[Record remote result]
    G --> H[Retention / cleanup]
```

Failures are tracked by phase so that an interrupted upload does not have to be confused with a failed export.

## 4. Resume and reuse

Large backup operations should not repeat expensive local work unnecessarily after a transient remote failure. The implementation therefore includes concepts for resumable/reusable staged artifacts, while still validating that reused content belongs to the expected export lineage.

The important rule is **reuse only after identity verification**, not merely because a similarly named file exists.

## 5. Restore workflow

A restore is accepted only after validation. Conceptually:

```text
select backup
    -> validate archive/manifest
    -> materialize into controlled target
    -> verify retained paths + database state
    -> run integrity/parity checks
    -> explicitly promote/cut over
```

Recovery is normally rehearsed against a copied/drill root before a live cutover.

## 6. Why restore testing matters

Backups can fail silently in several ways:

- wrong files were selected;
- an archive is incomplete;
- remote metadata points to the wrong object;
- retained payload references no longer resolve;
- a newer schema cannot safely consume an older state shape;
- operators know how to create backups but not how to restore them.

A verified restore path turns backup from an assumption into an operational capability.

## 7. Schema-aware rollback

Application rollback is not always equivalent to replacing the executable with an older binary. If a release changes persistent schema or representation, rollback must respect the migration and restore contract.

That is why release and recovery processes are linked: a deployable artifact must be considered together with the persistent state it expects.

## 8. Public-repository boundary

This document intentionally excludes:

- remote storage account/folder identifiers;
- credential setup values;
- production filesystem paths;
- exact retention policy values;
- provider-specific API calls that expose operational details;
- real backup manifests or collected data.
