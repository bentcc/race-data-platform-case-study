# Release Process and Provenance

## 1. Goal

The release process is designed to answer a basic operational question:

> Which exact source/dependency/build state produced the executable that was accepted for deployment?

A tag or filename alone is not enough to answer that reliably.

## 2. Continuous verification

The private repository uses CI to run a locked verification gate including:

```text
format check
    -> lint / warnings denied
    -> locked test suite
    -> workflow contract checks
```

Dependency resolution is locked so CI and release steps do not silently select newer dependency versions.

## 3. Candidate artifact

A release candidate contains the executable plus machine-readable build provenance. The provenance binds information such as:

- package/version identity;
- exact source commit;
- compiler/toolchain identity;
- target platform;
- dependency lockfile fingerprint;
- executable fingerprint;
- build-runner context.

The archive itself is checksummed and its expected entries/modes are verified before publication.

## 4. Provenance is not reproducible-build theater

The project does **not** claim that independent builds are byte-for-byte reproducible unless that property has actually been established.

The narrower and defensible claim is:

- the candidate artifact has a known hash;
- its embedded provenance names the exact build inputs/context;
- the same candidate is what gets acceptance-tested;
- deployment verifies that accepted artifact rather than rebuilding under a tag.

That is a stronger operational story than treating a tag-triggered rebuild as automatically identical.

## 5. Human-approved deployment

CI creates and verifies artifacts; it does not silently mutate the live service. Deployment is a separate human-approved operation that verifies the candidate on the target host before promotion.

This boundary is deliberate because target compatibility, existing persistent state and operational timing are live-system concerns.

## 6. Immutable version discipline

Once a versioned release identity is accepted, changing its contents would destroy provenance. Corrections therefore require a new version and a new acceptance record rather than replacing old artifacts in place.

## 7. Deployment and persistent-state compatibility

Before promotion, the operator must consider:

- executable/OS/ABI compatibility;
- version and help/smoke behavior;
- artifact checksum/provenance;
- current schema compatibility;
- backup/restore readiness if the change crosses a persistent-state boundary.

This is why release engineering is part of the system design rather than a final `cargo build --release` command.
