# CHECKPOINTS

A checkpoint passes only with linked evidence. Dates are planning aids, never proof of completion.

| CP | Gate | Evidence required |
|---|---|---|
| CP00 | Repository truth | Git baseline, canonical agent files, state, source registry, license inventory |
| CP01 | Product Constitution | process/legal-effect model, actor/capability matrix, pilot-critical P0 decisions, naming explicitly open/closed |
| CP02 | Architecture lock | ADRs, domain boundaries, data model, recommender/data/security design, benchmark spike results |
| CP03 | Dev foundation | reproducible Docker dev env, Rust workspace, pnpm workspace, CI, migrations, telemetry smoke tests |
| CP04 | Identity + territory | auth, actor/territory/competence contracts, object-level authz tests |
| CP05 | Need golden path | concern → routing → duplicate search → need → support; E2E and retrieval evals |
| CP06 | Evidence + deliberation | evidence ingestion, claims/arguments, moderation, provenance; abuse tests |
| CP07 | Technical proposals | versioning, evidence coverage, assessments, immutable evaluated version |
| CP08 | Civic Process Engine | support/prioritization/advisory decision methods, eligibility snapshot, frozen policies, receipts |
| CP09 | Discovery/recommender V1 | candidate sources, hybrid retrieval, civic scoring, diversity, explainability, offline eval dashboard |
| CP10 | Institution workbench | triage, competence/admissibility, official response, commitment, milestone/outcome tracking |
| CP11 | AI Librarian | routing/dedupe/summarization/argument/evidence pipelines + provenance + red-team evals |
| CP12 | Analytics + public audit | event analytics, QCLC metrics, audit explorer, data minimization tests |
| CP13 | Security hardening | ASVS mapping, SCA/SAST/DAST, penetration-test plan/results, backup/restore, incident drills |
| CP14 | Pilot acceptance | real territory fixture, institutional counterparty, accessibility, performance, operator training |
| CP15 | Production release | release manifest, SBOM, signed images/provenance, rollback, SLOs, production approval |
| CP16 | 10k scale | measured capacity, recommender eval stability, abuse controls, analytics separation |
| CP17 | 100k scale | partition/sharding decision evidence, ClickHouse/event-streaming gate, HA/DR validation |
| CP18 | Federation/sovereignty | interoperability/public API/federation decision and security model |
| CP19 | P3 ballot integration | only if separately authorized: legal mandate + certified/evaluated external ballot architecture |

## Live checkpoint state — 2026-09-11

| CP | Status | Evidence / blocker |
|---|---|---|
| CP00 | **PASS_ON_BRANCH_PENDING_REVIEW_MERGE** | `docs/evidence/2026-09-11/CP00_BOOTSTRAP_EVIDENCE.md`; CI run `34600057502` all six jobs green on verified code head `4c32f5d...` |
| CP01 | **IN_PROGRESS** | `PRODUCT_CONSTITUTION.md` + `docs/decisions/P0_DECISION_REGISTER.md`; pilot-critical P0 decisions remain open |
| CP02 | **PENDING** | requires workflow, authorization-policy and semantic/vector benchmark spikes plus ADR closure |
| CP03+ | **PENDING** | no completion claim; existing bootstrap code/infrastructure is only partial evidence toward later gates |

## Hard rule

`generated artifact != passed checkpoint`

Every CP records: entry criteria, exact commands/tests, output hashes/links, residual risks, approving authority and rollback path.

A later checkpoint never passes merely because some of its files already exist. For example, Docker, Rust and CI bootstrap artifacts contribute to CP03 but do not close CP03 until the required persistence/integration/telemetry evidence exists.
