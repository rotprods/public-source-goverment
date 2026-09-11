# HANDOFF — CP00 → CP01

## Authority

This handoff is continuity context, not higher authority than executable code/contracts/policies. Read `AGENTS.md` → `TRUTH.md` → `NORTH_STAR.md` → `GOAL.md` → `STATE.json` before acting.

## Current state

Branch: `bootstrap/empezarproyecto-v1`

Technical checkpoint:
- `CP00 — Repository truth`: **PASS ON BRANCH / pending review+merge**.
- `CP01 — Product Constitution`: **IN PROGRESS**.
- `CP02 — Architecture lock`: **PENDING measurable spikes**.

Verified code head: `4c32f5deb0e999e709265d8b63e61dd0bfda5dd6`

Verification run: GitHub Actions `34600057502` (run #43), all six jobs green:
- lockfile integrity;
- contract/truth sanity;
- Rust fmt/clippy/test/release build;
- web typecheck/production build;
- supply-chain/secrets;
- container build smoke.

Evidence packet: `docs/evidence/2026-09-11/CP00_BOOTSTRAP_EVIDENCE.md`.

## What now exists

### Operating system

- canonical `AGENTS.md` and lightweight `CLAUDE.md` adapter;
- `/empezarproyecto` canonical bootstrap skill;
- `/autoprompting` execution compiler + `METAPROMPT.md`;
- North Star, goals, checkpoints, truth hierarchy, state and handoff;
- graphify ontology and COS Graph Engine V2 20D operating schema;
- PR template/CODEOWNERS/reproducible toolchain/lockfiles/CI.

### Product/governance baseline

- North Star: **Qualified Civic Loop Completion (QCLC)**;
- Product Constitution draft;
- explicit P0 decision register;
- regulated public ballots excluded from ordinary V1 process engine;
- support ≠ signature ≠ assessment ≠ ballot;
- reputation ≠ automatic formal vote weight;
- legal effect and policy versions are explicit/frozen per process;
- AI is librarian/copilot, not civic sovereign.

### Architecture baseline

- Rust-first modular monolith;
- transactional PostgreSQL write path + domain event + outbox;
- PostGIS for territory/jurisdiction;
- pgvector-first semantic retrieval;
- hybrid lexical/vector/graph retrieval;
- rebuildable feed/search/vector/graph/analytics projections;
- Next.js/React/TypeScript product apps;
- OpenTelemetry contract;
- S3-compatible evidence storage and Valkey as non-authoritative infrastructure;
- workflow backend and policy engine deliberately behind ports until CP02 benchmarks.

### Executable foundation

Rust workspace includes:
- `civic-domain`;
- `civic-search`;
- `civic-ranking`;
- `civic-api`.

Compiler/test-backed primitives include:
- domain distinction between support and advisory ballot;
- Reciprocal Rank Fusion;
- Wilson lower-bound quality/confidence primitive;
- transparent policy-driven civic score with reconstructible component breakdown;
- visibility decision represented separately from relevance score;
- health API + truthful OpenAPI contract.

Data foundation includes:
- private identity zone;
- civic/territory/topic/competence/Need/Evidence/Claim/Proposal/Process/Participation models;
- institution response/commitment/outcome records;
- vector projection metadata;
- append-oriented domain events/outbox.

### Security/privacy baseline

- `SECURITY.md`;
- civic threat model covering object authz, Sybil/brigading, process-rule mutation, institutional impersonation, ranking capture, political profiling, hostile evidence, AI prompt injection, replay, stale projections, territory leakage, audit tampering, supply chain, DoS and recovery;
- purpose-limited data zones and vector/AI privacy rules;
- cargo-audit/cargo-deny/pnpm audit/Gitleaks in CI;
- dependency lock integrity and non-mutating CI.

## Research decisions already distilled

### MoviMurcia
Carry forward evidence-first operation, machine-readable truth, security gates, handoffs and traceability. Do **not** repeat documentation/runtime imbalance, multiple conflicting agent canons or structural validators masquerading as executable completion.

### xAI/X
Carry forward the staged Rust recommendation shape:

`context hydration → parallel candidate sources → candidate hydration → policy/visibility filters → scoring → selection → post-filters → side effects`

Replace engagement objective with civic utility/QCLC, constrain diversity and preserve truthful ranking explanations.

### legacy Twitter
Research Product Mixer, Unified User Actions, SimClusters, TwHIN, RealGraph, graph candidate sources and cascading rankers. Upstream code inspected as AGPL-3.0; independently implement architecture by default.

### Reddit
Use community/thread/moderation lessons and mathematical discovery/quality primitives such as Wilson confidence and bounded freshness. Inspected archive is CPAL-1.0; independently implement formulas. Do not use global karma/popularity as civic authority.

Exact reuse/adaptation matrix: `docs/research/ALGORITHM_EXTRACTION_MATRIX.md`.

## P0 decisions still open

Canonical register: `docs/decisions/P0_DECISION_REGISTER.md`.

Pilot-critical questions to close in CP01:
1. operator/governance authority model;
2. which civic process profiles the pilot actually supports;
3. identity-assurance matrix per process/action;
4. civic-priority methodology or explicit decision to keep it disabled for pilot;
5. unserved-demand activation policy or explicit deferral;
6. allowed role/reputation effects;
7. outbound repository license.

Public product naming can remain independently open if software uses neutral working naming.

## CP02 required spikes

Do not select by fashion. Benchmark:

1. **Workflow** — Postgres workflow ledger/workers vs Temporal Rust vs Restate Rust.
2. **Authorization** — project-native Rust policy port baseline vs Cedar vs OPA where appropriate.
3. **Semantic retrieval** — pgvector hybrid exact/ANN baseline; Qdrant only as a measured challenger.
4. **Identity/provider path** — passkey/OIDC assertion model and public-sector credential integration constraints.

Each spike must produce workload, benchmark/evaluation data, operational/security/cost tradeoff, ADR and rollback/migration path.

## Highest-value next execution

1. Open a review PR for the CP00 bootstrap instead of continuing to accumulate unreviewed foundation work.
2. Review/merge CP00 separately from CP01/CP02 policy choices where possible.
3. Run CP01 as a decision/evidence wave; do not encode unresolved civic semantics into code.
4. Run CP02 benchmark spikes.
5. Then start the first real vertical slice:

`plain-language concern → topic/competence suggestion → hybrid duplicate retrieval → canonical Need → support → domain event/outbox → public read model → audit/explanation`

That slice should be the transition from architecture foundation into actual product development.

## Hard stop conditions

Do not:
- claim production readiness;
- claim final product UI;
- turn partner/customer status into civic privilege;
- use inferred ideology/protected traits for political personalization;
- copy AGPL/CPAL source into an incompatible core;
- introduce Kafka/Qdrant/Neo4j/OpenSearch/Kubernetes because they look enterprise;
- implement regulated public elections in the ordinary V1 process engine;
- treat generated documentation as proof over compiler/build/tests.
