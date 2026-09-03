# ADR 0003: Publish Architecture, Keep Production Source Private

- Status: Accepted
- Scope: Portfolio publication

## Context

The private project contains useful engineering work, but publishing it wholesale would expose source-specific contracts, operational procedures, infrastructure details and potentially sensitive history that are irrelevant to evaluating engineering ability.

## Decision

Create a separate public repository containing only curated architecture, design decisions and sanitized visuals. Do not fork or rewrite the private repository into a public one.

## Consequences

### Positive

- recruiters and engineers can review the reasoning quickly;
- no production credentials, endpoints or data are required;
- the case study can focus on transferable engineering concepts;
- public Git history starts clean.

### Negative

- reviewers cannot inspect implementation source directly;
- claims must remain conservative and clearly distinguish implemented private behavior from public documentation;
- diagrams and screenshots require manual sanitization.

## Publication rule

Every public statement should be defensible from implemented behavior in the private project. Avoid adding architecture merely because it sounds more impressive.
