# ROADMAP — From Bootstrap to 100k+ Civic Users

This roadmap is evidence-gated. Calendar estimates assume a focused senior product/engineering pod and timely policy/stakeholder decisions.

## Phase 0 — Foundation / CP00–CP02
**Weeks 0–3**

Outcomes:
- recoverable Git/agent/project operating system;
- Product Constitution decision backlog;
- final architecture ADRs after benchmark spikes;
- threat model and privacy/data classes;
- baseline schema/OpenAPI/events;
- naming remains independent from software architecture.

Critical spikes:
- Postgres workflow ledger vs Temporal Rust vs Restate Rust;
- Cedar vs OPA authorization policy;
- pgvector hybrid retrieval baseline vs optional Qdrant benchmark fixture;
- ID strategy (UUIDv7 etc.);
- local/managed/sovereign deployment cost model.

Exit: CP02.

## Phase 1 — Executable platform foundation / CP03–CP04
**Weeks 3–6**

Build:
- Rust domain/application/persistence foundation;
- Postgres/PostGIS/pgvector migrations;
- auth/passkey/OIDC adapter baseline;
- Actor/Territory/Topic/CompetenceRegistry;
- institution/admin shell;
- outbox/projector worker;
- OpenTelemetry and CI/security gates.

Exit: clean checkout builds and runs end to end with integration DB.

## Phase 2 — Need golden path / CP05
**Weeks 6–9**

Vertical slice:

`plain-language concern → routing suggestions → topic/competence explanation → hybrid duplicate search → canonical Need → support → public read model → audit receipt`

Includes golden retrieval/routing evaluation and abuse cases.

Exit: real browser + Rust API + Postgres + vector/search + telemetry evidence.

## Phase 3 — Evidence, deliberation, technical proposals / CP06–CP07
**Weeks 9–12**

Build:
- evidence ingestion/object storage/hash/provenance;
- claims/arguments;
- structured deliberation;
- proposal authoring/versioning;
- Need↔Proposal coverage;
- evidence/source diversity indicators;
- moderation foundations.

Exit: immutable evaluated proposal version with traceable evidence.

## Phase 4 — Civic Process Engine / CP08
**Weeks 12–15**

Build generic process templates for allowed V1 profiles:
- support/prioritization;
- advisory consultation;
- expert assessment;
- selected participatory-budgeting primitives;
- internal organization decisions if approved.

Implement:
- legal-effect display;
- eligibility snapshot;
- identity-assurance policy;
- frozen policy bundle;
- participation receipt;
- tally/result reproducibility;
- appeals/administrative corrections where relevant.

No P3 regulated public-ballot implementation.

## Phase 5 — Civic recommender + semantic intelligence / CP09
**Weeks 14–17 (overlapping)**

Build:
- candidate-source mixer;
- hybrid lexical/vector/graph retrieval;
- visibility policy stage;
- transparent scoring policy;
- diversity/slate reranking;
- `Why am I seeing this?`;
- offline replay/golden eval;
- exposure analytics.

Start deterministic. Learned ranker remains gated.

## Phase 6 — Institution/accountability / CP10
**Weeks 15–18**

Institution Workbench:
- inbox/triage;
- competence/admissibility;
- assignment;
- request for information;
- official response;
- commitment/milestone/outcome;
- public accountability view;
- institutional SLO/reporting.

Exit: closed-loop demo using a real territory fixture.

## Phase 7 — AI Librarian / CP11
**Weeks 16–19**

Add AI behind `ai-gateway`:
- routing suggestion;
- duplicate/cluster assistance;
- cited summarization;
- argument mapping;
- evidence retrieval/reranking;
- proposal/institutional copilot;
- moderation triage.

Required: prompt-injection/retrieval-poisoning/citation evals and provenance.

## Phase 8 — Analytics/public audit / CP12
**Weeks 18–20**

Build:
- QCLC dashboards;
- process/recommender quality metrics;
- public audit/provenance explorer;
- operator dashboards;
- data-right workflows and audit exports.

## Phase 9 — Hardening + pilot / CP13–CP15
**Weeks 20–24**

- ASVS mapping/testing;
- penetration-test preparation/execution;
- load/capacity testing;
- accessibility audit;
- backup/restore and outage drills;
- signed SBOM/provenance/release;
- operator/institution training;
- pilot runbook and rollback.

Target: production-quality territorial pilot.

---

# Scale roadmap

## Stage A — 0 → 1,000 verified active users
Goal: prove loops, not growth vanity.

Must prove:
- users can express real problems without admin jargon;
- duplicate clustering works;
- institutions can process volume;
- first public closed loops exist;
- QCLC instrumentation trustworthy.

## Stage B — 1,000 → 10,000 / CP16
Focus:
- multiple topics/territories;
- recommender quality;
- moderation/abuse operations;
- expert/institution acquisition;
- Postgres index/partition tuning;
- capacity/error budgets.

Architecture remains simple unless measurements justify extraction.

## Stage C — 10,000 → 100,000 / CP17
Potential additions based on evidence:
- ClickHouse real-time product/recommender analytics;
- event-stream layer if fan-out/retention requires it;
- extracted recommender/projector workers;
- vector-backend benchmark and optional Qdrant migration;
- read replicas and partitioned high-volume tables;
- stronger anomaly/Sybil detection;
- cell-based territory scaling design.

Growth target is an objective, not a contractual guarantee.

## Stage D — 100,000 → 1M+
Possible architecture:
- territory/cell partitioning;
- independent search/vector/recommender scaling;
- multi-region HA/DR;
- public APIs/federation;
- sovereign regional deployments;
- multilingual EU expansion;
- advanced LTR/slate ranking after governance/eval maturity.

## Stage E — Regulated civic processes / CP19
Only after independent legal/security mandate:
- higher-assurance identity;
- end-to-end verifiable ballot integration;
- external certification/evaluation;
- independent observers/auditors;
- separate cryptographic threat model and operational ceremonies.

This is not an incremental toggle on the ordinary engagement engine.
