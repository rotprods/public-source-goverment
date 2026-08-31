# ARCHITECTURE — Target State v1

## 1. Architectural thesis

Build a **Rust-first, policy-driven civic operating system** that borrows the strongest engineering patterns from large social networks without inheriting their engagement objective.

The core is a modular monolith until evidence justifies extraction. Data authority remains transactional; recommendation, vector, graph, search and analytics systems are projections. Politically meaningful behavior is policy/version driven and explainable.

## 2. What we carry forward from MoviMurcia

Adopt:
- evidence-first repository operation;
- explicit source-of-truth hierarchy;
- machine-readable goal/state/route/contract manifests;
- security gates before sensitive runtime work;
- agent instructions, handoffs, evidence ledgers and checkpoint discipline;
- snapshots/contracts for parity and regression;
- imported designs treated as evidence, not shipped truth.

Refactor:
- do not allow documentation/evidence count to dwarf runtime truth without a visible code-to-doc ratio;
- one canonical `AGENTS.md`, with tool adapters only — no divergent AGENTS/AGENTS2/CODEX/CLAUDE truth;
- establish a committed Git baseline immediately;
- no temporary Node oracle that silently becomes production architecture;
- every structural verifier is subordinate to real compile/typecheck/build/tests;
- generated plans must expire or be tied to a checkpoint/decision.

## 3. Repository topology

```text
/
├── apps/
│   ├── web/                  # citizen/public PWA — Next.js/TypeScript
│   ├── institution/          # institutional workbench — Next.js/TypeScript
│   └── admin/                # policy/operator console — Next.js/TypeScript
├── services/
│   ├── api/                  # Rust Axum public/application API
│   ├── worker/               # Rust outbox/projector/background jobs
│   ├── recommender/          # Rust candidate pipeline/ranking service when extracted
│   └── ai-gateway/           # Rust policy/provenance gateway; model providers behind ports
├── crates/
│   ├── domain/               # aggregates, value objects, invariants
│   ├── application/          # commands/queries/use cases
│   ├── persistence/          # SQLx repositories, transactions, outbox
│   ├── policy/               # authorization/civic policy interfaces
│   ├── events/               # domain-event contracts
│   ├── search/               # hybrid retrieval orchestration
│   ├── ranking/              # civic recommender pipeline
│   ├── security/             # cryptographic/security primitives and redaction
│   └── telemetry/            # OTel shared instrumentation
├── contracts/
│   ├── openapi/
│   ├── events/
│   ├── policy/
│   └── schemas/
├── data/
│   ├── migrations/
│   ├── seeds/
│   ├── evals/
│   └── legal-source-registry/
├── ml/
│   ├── evals/                # Python allowed
│   ├── training/             # Python allowed
│   └── notebooks/            # non-authoritative research only
├── infra/
│   ├── docker/
│   ├── k8s/
│   ├── terraform/
│   ├── observability/
│   └── policies/
├── graphify/
├── cos-graph-engine/V2/
├── docs/
│   ├── adr/
│   ├── architecture/
│   ├── research/
│   ├── threat-models/
│   ├── runbooks/
│   └── evidence/
└── .codex/skills/
```

## 4. Runtime planes

### 4.1 Experience plane

Three primary surfaces:

- Citizen/Public: home/civic agenda, territory, topics, needs, evidence/deliberation, proposals, processes, institutional tracking, audit explorer.
- Institution: inbox, competence/admissibility, assignment, evidence review, official response, commitments, milestones/outcomes.
- Admin/Policy: actor/credential administration, taxonomy/competence versions, process templates, moderation/appeals, ranking policy, feature flags, release/policy diff.

PWA first. Native mobile only after interaction/product model is proven.

### 4.2 Domain plane

Bounded contexts:

1. Identity & Assertions
2. Actors & Organizations
3. Territory & Jurisdiction
4. Civic Topics
5. Competence Registry
6. Needs & Clusters
7. Evidence / Claims / Arguments
8. Technical Proposals & Versions
9. Civic Processes
10. Participation / Eligibility / Receipts
11. Civic Priority
12. Unserved Demand
13. Institutions / Official Responses
14. Commitments / Milestones / Outcomes
15. Moderation / Appeals
16. Notifications / Subscriptions
17. Policy / Feature Config
18. Audit / Provenance
19. AI Artifacts
20. Discovery / Recommendation

These are modules/crates first, not twenty services.

### 4.3 Data authority plane

PostgreSQL is system of record.

Required extensions/features:
- PostGIS: territory, administrative geometry, spatial eligibility/routing.
- pgvector: dense/half/sparse semantic indexes.
- PostgreSQL full-text search + `pg_trgm`: lexical/exact/fuzzy retrieval.
- native partitioning for high-volume event/embedding tables.
- row-level security only as defense-in-depth where it simplifies guarantees; application authz remains explicit.

Write transaction:

`validate → authorize → BEGIN → domain mutation → event → outbox → audit receipt metadata → COMMIT`

No projection participates in the authoritative transaction.

### 4.4 Event/projection plane

V1: transactional outbox + Rust worker with idempotent consumers, leases and replay.

Scale gate: introduce NATS JetStream or Redpanda/Kafka only when multiple consumers, throughput, retention or independent scaling justify it.

Projectors build:
- public read models;
- search indexes;
- vector indexes;
- graph adjacency/read models;
- civic-priority snapshots;
- notification queues;
- analytics events;
- public audit views.

### 4.5 Durable workflow plane

All long-running civic/institutional processes use a `WorkflowEngine` port.

Do not hard-code a preview dependency before benchmarking. Sprint CP02 compares:
- native Postgres workflow ledger + Rust workers;
- Temporal Rust SDK (public preview as of 2026-08);
- Restate Rust SDK.

Selection criteria: deterministic/replay semantics, operational maturity, versioning, timers, signals, observability, self-hosting/sovereignty, upgrade compatibility and team complexity.

### 4.6 Search/vector/knowledge plane

Every semantic object has an immutable source record and zero or more derived representations.

Vectorizable object classes:
- need text and canonical cluster summaries;
- proposal sections/versions;
- evidence/document chunks;
- claims/arguments;
- civic topic/node descriptions;
- competence/legal-source descriptions;
- institutional public documents;
- public knowledge/help content.

Do **not** vectorize by default:
- raw DNI/identity artifacts;
- biometrics;
- payment data;
- security secrets;
- private moderation evidence;
- private political-profile features.

Embedding record minimum:

```text
entity_id
entity_type
source_version
content_hash
embedding_model_id
embedding_model_version
dimensions
vector_kind: dense|sparse|multi
locale
created_at
policy_classification
```

Never update an embedding in place when source/model changes; create a new derived version and switch an index pointer after evaluation.

#### V1 hybrid retrieval

1. authorization/territory/policy prefilter;
2. lexical candidate set: FTS/trigram;
3. dense semantic candidates: pgvector HNSW;
4. optional sparse semantic candidates;
5. merge with Reciprocal Rank Fusion;
6. graph/competence boosts;
7. cross-encoder/reranker only on small top-N when justified;
8. civic-policy filters;
9. explanation artifact.

Use exact nearest-neighbor queries in golden evals to measure HNSW recall.

#### Qdrant gate

Do not add a second vector database on day one. Evaluate Qdrant when:
- vector working set/index memory becomes a material PostgreSQL constraint;
- complex named dense+sparse vectors and filtered ANN dominate latency;
- independent vector scaling materially improves SLO/cost;
- migration/rebuild path has been proven.

### 4.7 Graph plane

The product is graph-shaped but the graph is initially a Postgres projection, not a second source of truth.

Typed nodes include actor, organization, territory, topic, competence, need, cluster, claim, evidence, argument, proposal/version, process, participation, institution, response, commitment, outcome, policy and AI artifact.

Typed/hyper edges carry provenance and time. Examples:
- `NEED CONCERNS TOPIC`
- `NEED ROUTED_TO COMPETENCE`
- `NEED ADDRESSED_BY PROPOSAL_VERSION`
- `CLAIM CITES EVIDENCE`
- `ARGUMENT SUPPORTS/OPPOSES CLAIM`
- `PROCESS EVALUATES PROPOSAL_VERSION`
- `INSTITUTION RESPONDS_TO PROCESS`
- `COMMITMENT IMPLEMENTS PROPOSAL`
- `OUTCOME EVIDENCED_BY SOURCE`

At scale, evaluate a dedicated graph store only for measured workloads that cannot meet SLOs using Postgres projections/columnar analytics.

## 5. Civic recommender architecture

We adopt the **pipeline decomposition** seen in X/Twitter — query hydration → parallel candidate sources → candidate hydration → pre-filters → scoring → selection → post-filters → side effects — but replace engagement optimization with civic utility.

### 5.1 Candidate sources

Parallel sources:

1. Territory/network: subscribed/followed territories, topics, actors and processes.
2. Civic priority: unresolved high-priority needs and unserved demand.
3. Semantic retrieval: similarity to explicit interests/current query and followed civic objects.
4. Graph retrieval: related needs/proposals/evidence and social proof that is safe to use.
5. Freshness: new relevant items with insufficient historical engagement.
6. Institutional: official responses, deadlines, commitments and milestones relevant to the user.
7. Deliberative need: items lacking evidence, perspectives or expert contribution.
8. Exploration: controlled exposure to relevant underrepresented/new sources.

Candidate source output must include `source`, `reason`, `policy_scope`, raw score and evidence refs.

### 5.2 Hard pre-filters

Before scoring:
- authorization/visibility;
- territorial/process eligibility when needed;
- blocks/mutes/subscriptions;
- duplicate/canonical-cluster suppression;
- content-policy state;
- legal restrictions;
- stale/closed process rules;
- already-served fatigue when appropriate;
- self-content treatment;
- language/accessibility constraints.

Safety/legality is not a negative ranking weight; it is a separate visibility decision.

### 5.3 Civic multi-objective scoring

No single secret engagement score. Versioned component vector:

```text
territorial_relevance
explicit_interest_relevance
semantic_relevance
civic_priority
unresolved_importance
evidence_quality
deliberative_need
institutional_urgency
freshness
source_diversity
perspective_diversity
novelty
confidence
manipulation_risk
fatigue
```

Final policy may be a constrained weighted score, learned-to-rank model, or Pareto/blended policy — but it MUST preserve explanations and offline counterfactual evaluation.

Forbidden optimization signals:
- inferred ideology for persuasion;
- outrage/anger as positive objective;
- political conversion;
- follower count as civic authority;
- paid political boost;
- raw dwell time as a north-star proxy.

### 5.4 Diversity/reranking

Adapt X repeated-author penalty into stronger civic constraints:
- author/source diversity;
- organization diversity;
- territory balance;
- topic fatigue control;
- duplicate-conversation collapse;
- controlled viewpoint/perspective exposure where classifications are transparent and do not infer hidden ideology.

Use Maximum Marginal Relevance or constrained greedy slate optimization initially. Research DPP/learned slate rerankers later.

### 5.5 Reddit patterns we can use

Useful:
- community/topic segmentation;
- threaded deliberation;
- explicit moderation rules per community/process;
- multiple sortable views (`new`, `top`, `hot`, evidence-first etc.);
- Wilson lower-bound/confidence ranking for quality signals with low sample sizes;
- time-decay for discovery;
- karma-like contribution reputation **only as capability/reliability metadata, never formal vote power**.

Do not directly transplant Reddit's `hot` objective into civic priority. Its time-decayed popularity formula is useful as a discovery primitive, not as public-importance truth.

## 6. Analytics/data intelligence plane

### Operational analytics V1

PostgreSQL read models + OpenTelemetry metrics answer product/operator questions without introducing an analytics platform prematurely.

### Scale analytics

When event volume/concurrency justifies it:

`domain/client events → outbox/stream → ClickHouse`

ClickHouse owns high-cardinality analytical queries, not civic transactional state.

Datasets:
- product interaction events;
- recommender impressions/candidate decisions/explanations;
- moderation and abuse aggregates;
- workflow timing;
- QCLC funnels;
- retrieval/ranking eval outcomes;
- operational logs/traces if ClickStack is selected.

Object storage stores immutable Parquet snapshots for offline evaluation/replay. Consider Iceberg only if multi-engine lakehouse use becomes real.

### Experimentation

Experiments must have:
- hypothesis;
- affected cohort/process scope;
- policy/legal review classification;
- primary civic metric + countermetrics;
- stop conditions;
- exposure logs;
- reproducible config version;
- no experiments on formal civic rights without explicit authority.

## 7. AI/ML plane

Rust `ai-gateway` performs policy, redaction, provider/model routing, provenance and output validation.

Model tasks:
- semantic routing suggestion;
- duplicate/cluster candidate generation;
- evidence retrieval/reranking;
- claim/argument extraction;
- summarization with citations;
- translation/accessibility;
- proposal drafting copilot;
- institutional triage assistance;
- abuse/moderation triage.

Python is allowed for training/evals/model experimentation. Production model inference may be local/self-hosted or provider-backed behind the gateway.

Every AI artifact records input refs, source versions, model/provider, prompt/template version, output, citations, confidence/evals and review state.

## 8. Frontend stack

- Next.js + React + TypeScript.
- pnpm workspace.
- generated API client/types from OpenAPI; never hand-maintain duplicate contracts.
- TanStack Query for server state.
- React Hook Form + Zod/Valibot at the UI boundary.
- Storybook for design-system states.
- Playwright for E2E and visual acceptance.
- accessibility: WCAG 2.2 AA target, keyboard/screen reader/high contrast/reduced motion.

Avoid infinite-scroll dark patterns as the primary civic home. Offer chronological, subscribed, civic-priority and explainable recommendation modes.

## 9. Rust backend stack

- stable Rust pinned by `rust-toolchain.toml`;
- Tokio async runtime;
- Axum + Tower for HTTP/middleware;
- SQLx for compile-checked SQL/migrations;
- Serde;
- tracing + OpenTelemetry;
- UUID v7/ULID-style sortable IDs after ADR benchmark;
- `thiserror`/typed errors;
- `proptest` and fuzz targets for invariants;
- `cargo-deny`, `cargo-audit`, Clippy and rustfmt.

Use tonic/gRPC only for measured internal-service benefits; HTTP/JSON contracts remain the public boundary.

## 10. Infrastructure

### Dev
Docker Compose: Postgres/PostGIS/pgvector, Valkey, MinIO, optional workflow engine, OTel Collector and local app services.

### Staging/production
Container-first. Support:
- EU managed cluster;
- sovereign regional cluster;
- on-prem deployment profile where contractually required.

Kubernetes is a scale/HA deployment target, not a local-dev requirement. IaC via Terraform/OpenTofu + Helm/Kustomize after CP03.

### Caching
Valkey/Redis for ephemeral caches, distributed rate limits and invalidation metadata. Never authoritative civic state.

## 11. Observability/SLOs

OpenTelemetry everywhere. Correlation ID links user request → command → DB transaction → event → workflow → projection → AI call.

Initial SLO candidates:
- public read APIs p95 < 300 ms excluding model generation;
- mutation APIs p95 < 500 ms excluding external verification;
- search p95 < 600 ms V1;
- availability 99.9% pilot target;
- event projection lag p95 < 5 s;
- 100% high-impact mutations produce audit receipt;
- 100% feed items include explanation metadata;
- zero unauthorized object access in authz regression suite.

## 12. Release engineering

- trunk-friendly PR workflow; no direct main.
- Conventional Commits or equivalent machine-readable commit taxonomy.
- required CI: Rust fmt/clippy/test/build; TS lint/typecheck/test/build; schema/OpenAPI diff; migrations; Playwright smoke; secret/SCA/SAST; container scan; SBOM.
- signed release tags/images.
- SLSA provenance.
- deployment manifests bind app/schema/policy/model versions.
- database expand/contract migrations; rollback/canary documented.

## 13. Federation and interoperability

Do not federate V1 prematurely. Preserve ports for:
- ActivityPub-compatible publication/interactions where useful;
- AT Protocol-inspired signed user/public data repositories and schema lexicons;
- W3C PROV-style provenance export;
- Akoma Ntoso/public-document linkage;
- EUDI/eIDAS identity assertions.

Critical lesson from AT Protocol: speech/data ownership and reach/ranking can be separate layers. Civic records should remain addressable/auditable even when ranking policies change.

## 14. Scale evolution

### 0–10k users
Postgres + pgvector + PostGIS, Rust modular monolith, outbox worker, Valkey, S3/MinIO. No Kafka, Qdrant, Neo4j or ClickHouse unless measured.

### 10k–100k
Read replicas, partition event/vector tables, ClickHouse analytics, stronger stream/event layer if needed, extracted recommender/AI workers, vector recall/latency benchmark, horizontal search/read projections.

### 100k–1M+
Cell/territory partitioning, independent recommender/search scaling, Qdrant/OpenSearch if benchmarked superior, Redpanda/Kafka where durable multi-consumer streams justify it, multi-region disaster recovery, sovereign cells/federation.

## 15. Architectural definition of done

The architecture is not 'final' because a diagram exists. It becomes locked when CP02 contains:
- accepted ADRs;
- measured benchmark spikes;
- threat model;
- schema and API baseline;
- policy/version model;
- retrieval/ranking golden evals;
- operational/deployment profile;
- cost model;
- rollback/recovery plan;
- owner and acceptance evidence.
