# Public Source Government — Civic Infrastructure

> Working repository name only. Public product naming remains an explicit branding decision gate.

Open, auditable civic infrastructure for converting citizen needs into jurisdiction-correct, evidence-backed, technically assessable and institutionally traceable public action.

## North Star

**Close high-quality civic loops:** from a real need, through evidence and deliberation, to a versioned proposal, an attributable institutional response, a tracked commitment and a verifiable outcome — without optimizing for outrage, partisan persuasion or addictive engagement.

## Architecture stance

- **Rust-first core** for domain, policy, ranking, search orchestration, eventing and security-critical workloads.
- **TypeScript/Next.js** for citizen, institution and operator interfaces.
- **PostgreSQL + PostGIS + pgvector** as authoritative transactional/data-semantic foundation.
- **Append-only domain events + transactional outbox**; projections are rebuildable.
- **Hybrid retrieval**: lexical + dense/sparse vectors + graph + policy filtering + explainable reranking.
- **Civic recommender**, not an engagement recommender.
- **AI as librarian/copilot, never civic sovereign.**
- **Formal ballots are separate from support, ranking, reputation and expert assessment.**
- **Policy/version snapshots are frozen per civic process.**

## Start here

1. `AGENTS.md`
2. `NORTH_STAR.md`
3. `GOAL.md`
4. `TRUTH.md`
5. `ARCHITECTURE.md`
6. `SECURITY.md`
7. `CHECKPOINTS.md`
8. `STATE.json`
9. `.codex/skills/empezarproyecto/SKILL.md`
10. `METAPROMPT.md`

## Repository operating model

```text
apps/              citizen/institution/admin clients
services/          Rust runtime services
crates/            shared Rust domain/application/platform crates
contracts/         OpenAPI, events, policy and schema contracts
data/              migrations, seeds, vector/search schemas
ml/                evaluation/training research only; no hidden civic authority
infra/             containers, deployment, observability, IaC
docs/              ADRs, research, runbooks, threat models, evidence
graphify/          typed project/product/knowledge graph
cos-graph-engine/   COS 20D operating state and execution protocol
.codex/skills/      canonical agent skills
```

## Delivery rule

Documentation is evidence, not completion. A feature is only `DONE` when executable code, tests, security gates, observability and acceptance evidence exist.

## Current phase

`BOOTSTRAP / ARCHITECTURE LOCK` on branch `bootstrap/empezarproyecto-v1`.
