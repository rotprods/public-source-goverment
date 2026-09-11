# ADR-0003 — Modular monolith + transactional outbox first

Status: Accepted for V1 baseline

## Context

The domain is broad: identity assertions, territory, competence, needs, evidence, proposals, civic processes, institutional workflows, moderation, recommendation and AI. That breadth does not imply that each bounded context deserves its own network service on day one.

Premature microservices would add distributed transactions, service discovery, duplicated authorization, version skew, more secrets, more CI/CD paths and a larger sovereign-deployment burden before product-market or scaling evidence exists.

## Decision

- Start with a Rust modular monolith composed of explicit domain/application/persistence modules/crates.
- Keep the write path transactional inside PostgreSQL.
- Persist domain event + transactional outbox row in the same transaction as authoritative state.
- Run background work through idempotent Rust workers/projectors.
- Treat feed/search/vector/graph/analytics/notifications as derived projections/effects.
- Extract services only when a measured boundary exists: independent scaling, failure isolation, security isolation, distinct operational ownership or deployment topology.

Authoritative mutation path:

```text
request
  → authentication
  → authorization/policy
  → validation
  → BEGIN
  → mutate aggregate/state
  → append domain event
  → append outbox record
  → COMMIT
  → asynchronous idempotent consumers
  → projections/effects/receipts
```

## Event rules

Every domain event carries:
- globally unique event ID;
- aggregate type/ID/version;
- event type/schema version;
- correlation and causation IDs;
- relevant policy-version references;
- occurred/recorded timestamps;
- privacy classification where needed.

Consumers use event ID/idempotency keys and must tolerate replay.

## Service extraction gate

A module can become a network service only after an ADR demonstrates at least one of:
- materially different horizontal-scaling profile;
- materially stronger trust/security boundary;
- failure isolation needed to meet an SLO;
- independent team/release ownership;
- sovereign/deployment placement requirement;
- benchmark showing network separation is worth the added complexity.

## Consequences

This keeps correctness and iteration speed high while preserving future extraction seams through ports/contracts. It also makes projection rebuild, local development and disaster recovery substantially easier during the early phases.
