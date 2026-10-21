# Security and Publication Boundary

## 1. Purpose of this public repository

This repository contains an engineering case study and independently written Rust reference implementations using synthetic data. It demonstrates systems/backend engineering decisions without converting a private operational project into a public deployment manual.

## 2. Safe to publish

The following are intentionally suitable for public discussion:

- high-level component architecture;
- generic Rust/SQLite technology choices;
- append-oriented evidence and normalization model;
- event-relative scheduling concepts;
- lease/claim lifecycle concepts;
- backup/restore invariants;
- CI and release-provenance principles;
- sanitized diagrams;
- design trade-offs and known limitations;
- generic reference code, regression tests, and synthetic fixtures under the repository's MIT license.

## 3. Keep private

The following should remain in the private repository:

- `.env`, secret files, tokens, OAuth credentials or refresh tokens;
- VPS IPs, SSH details, usernames and exact filesystem layout;
- real domains, tunnel identifiers, access-policy audience/issuer values;
- source-specific endpoints, GraphQL contracts, request parameters and anti-abuse workarounds;
- complete scraping adapters and parsers;
- production database, retained raw payloads and backup archives;
- exact source-specific schema where it exposes operational/domain implementation;
- live runbooks containing commands against real infrastructure;
- private repository/release identifiers or account-specific controls;
- model/prediction logic, betting rules, paper-trading details or performance claims unless there is a separate reason to publish them.

## 4. Screenshots

A dashboard screenshot can be useful, but only after sanitization. Before publishing, remove or replace:

- domain/event names that identify live data unnecessarily;
- timestamps that reveal operational cadence if sensitive;
- hostnames, paths, IDs, backup object names and release hashes;
- user/account information;
- error messages containing URLs or filesystem paths.

A purpose-built demo screenshot with synthetic data is preferable to blurring a production screenshot.

## 5. Naming

Use an engineering-functional name such as **Race Data Collection & Operations Platform**.

A neutral description is:

> A Rust-based platform for collecting publicly available horse-racing data with evidence retention, event-relative scheduling, SQLite-backed operational state, verified backup/restore, and a read-only operator dashboard.

This is accurate without presenting the repository as a prediction or gambling product.

## 6. Before every public push

Check the repository for:

```text
secrets / tokens
private IPs and hostnames
real email addresses
cloud account identifiers
source endpoints
production paths
real collected data
private repository URLs
screenshots with operational metadata
Git history containing previously removed secrets
```

Important: deleting a secret from the working tree is not enough if it already exists in Git history. This public repository has separate history; keep additions limited to curated documents, independently written generic code, and synthetic fixtures.
