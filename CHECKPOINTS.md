# CHECKPOINTS

A checkpoint passes only with linked evidence. Dates are planning aids, never proof of completion.

| CP | Gate | Evidence required |
|---|---|---|
| CP00 | Repository truth | Git baseline, canonical agent files, state, source registry, license inventory |
| CP01 | Product Constitution | process/legal-effect model, actor/capability matrix, P0 decisions, naming explicitly open/closed |
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

## Hard rule

`generated artifact != passed checkpoint`

Every CP records: entry criteria, exact commands/tests, output hashes/links, residual risks, approving authority and rollback path.
