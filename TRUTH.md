# TRUTH — Canonical Source Hierarchy

## Authority order

1. Released executable contracts + production database/event evidence.
2. Approved Product Constitution and versioned runtime policies.
3. Database schema, OpenAPI/event contracts, tests and migrations on the active release branch.
4. ADRs and signed decision records.
5. `STATE.json`, `CHECKPOINTS.md`, `HANDOFF.md`.
6. Architecture/roadmap documents.
7. Design files and prototypes.
8. Research/reference implementations.
9. Chat history/model memory.

Lower layers never silently override higher layers.

## Evidence classes

- `SOURCE`: directly supported by source material or inspected implementation.
- `APPROVED_DECISION`: accepted by project authority and versioned.
- `ENGINEERING_DEFAULT`: safe reversible implementation assumption.
- `RESEARCH`: external pattern or benchmark.
- `HYPOTHESIS`: requires validation.
- `BLOCKED`: insufficient authority/evidence to implement.
- `DEPRECATED`: preserved for history, not current truth.

## Rules

- Every politically meaningful constant must point to `policy_version` + decision/evidence.
- Every ranking formula must have a public methodology version and offline evaluation set.
- Every civic process freezes eligibility, decision, ranking/moderation and competence-policy versions at start.
- AI output is a derived artifact, never source truth.
- Embeddings are derived indexes, never canonical records.
- Search/graph/feed/analytics stores are rebuildable projections.
- Reference repos are research inputs, not hidden dependencies.

## Current external research inputs

- MoviMurcia: evidence-first repo operations, machine-readable state, route/contract manifests, security gating; also a warning against documentation volume exceeding runtime truth.
- `xai-org/x-algorithm`: Apache-2.0 recommendation pipeline research.
- `twitter/the-algorithm`: AGPL-3.0 architectural research; do not copy code into permissive core without explicit license decision.
- Reddit archived ranking: mathematical ranking research only.
- ActivityPub / AT Protocol: federation/portability research, not V1 requirements.

## Naming

The repository name is not a product-name decision. Public naming remains OPEN until brand/legal/domain clearance.
