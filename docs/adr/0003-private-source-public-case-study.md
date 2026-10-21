# ADR 0003: Publish Architecture, Keep Production Source Private

- Status: Accepted
- Scope: Portfolio publication

## Context

The private project contains useful engineering work, but publishing it wholesale would expose source-specific contracts, operational procedures, infrastructure details and potentially sensitive history that are irrelevant to evaluating engineering ability.

## Decision

Create a separate public repository containing curated architecture, design decisions, sanitized visuals, and independently written generic Rust reference implementations with synthetic data. License the public material under MIT. Do not fork or rewrite the private repository into a public one.

## Consequences

### Positive

- recruiters and engineers can review the reasoning quickly;
- no production credentials, endpoints or data are required;
- the case study can focus on transferable engineering concepts;
- public Git history starts clean.

### Negative

- reviewers can inspect and run reference code, but cannot inspect the private collector implementation directly;
- claims must remain conservative and distinguish tested public behavior from documented private-system behavior;
- diagrams and screenshots require manual sanitization.

## Publication rule

Statements about the private platform should be defensible from its implemented behavior. Statements about public components should be supported by their source and tests. Avoid adding architecture merely because it sounds more impressive.
