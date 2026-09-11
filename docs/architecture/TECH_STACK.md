# Technology Stack — Proposed Production Baseline

Status meanings:
- `LOCKED_FOR_BOOTSTRAP`: safe to build foundation around now.
- `LOCK_AFTER_SPIKE`: port/abstraction exists; benchmark before dependency lock.
- `SCALE_GATE`: introduce only after measured trigger.
- `RESEARCH_ONLY`: not runtime dependency.

| Area | Choice | Status | Reason |
|---|---|---|---|
| Core language | Rust 1.98 | LOCKED_FOR_BOOTSTRAP | correctness, memory safety, performance, concurrency, strong types |
| Async runtime | Tokio | LOCKED_FOR_BOOTSTRAP | mature Rust async ecosystem |
| HTTP API | Axum + Tower | LOCKED_FOR_BOOTSTRAP | small composable Rust web stack |
| SQL access | SQLx | LOCKED_FOR_BOOTSTRAP | explicit SQL, transactions, compile/query checking path |
| Serialization | Serde | LOCKED_FOR_BOOTSTRAP | ecosystem standard |
| Telemetry | tracing + OpenTelemetry | LOCKED_FOR_BOOTSTRAP | request→event→workflow→AI correlation |
| Frontend | Next.js 16 + React 19 + TypeScript | LOCKED_FOR_BOOTSTRAP | product/web ecosystem and deployment leverage |
| JS package manager | pnpm 11 | LOCKED_FOR_BOOTSTRAP | deterministic efficient workspaces |
| Server state | TanStack Query | CP04 | avoid premature frontend dependency before real API slice |
| Forms | React Hook Form + schema validator | CP04 | complex accessible civic forms |
| Component docs | Storybook | CP04 | design-system state/a11y review |
| E2E | Playwright | CP04 | real browser acceptance |
| Transaction DB | PostgreSQL 18 | LOCKED_FOR_BOOTSTRAP | relational/versioned authority |
| Geography | PostGIS 3.6 | LOCKED_FOR_BOOTSTRAP | territory/jurisdiction geometry |
| Vector | pgvector 0.8.6 | LOCKED_FOR_BOOTSTRAP | semantic index next to relational filters |
| Lexical search | PostgreSQL FTS + pg_trgm | LOCKED_FOR_BOOTSTRAP | exact/fuzzy legal/civic terminology |
| Dedicated vector DB | Qdrant | SCALE_GATE | only when vector scale/filter benchmarks justify another stateful system |
| Search engine | OpenSearch/Typesense/etc. | SCALE_GATE | only if PSQL search misses feature/SLO needs |
| Cache/rate limit | Valkey | CP03 | ephemeral only; not civic authority |
| Object evidence | S3-compatible API | CP03 | immutable/hashable evidence and exports |
| Local object store | MinIO or equivalent pinned S3 implementation | CP03 | dev/on-prem profile; image/version lock required |
| Durable workflow | WorkflowEngine port | LOCKED_FOR_BOOTSTRAP | prevents vendor/preview lock |
| Workflow backend | Postgres worker vs Temporal Rust vs Restate Rust | LOCK_AFTER_SPIKE | benchmark maturity/ops/versioning |
| Event bus | Transactional outbox | LOCKED_FOR_BOOTSTRAP | correct/simple V1 |
| Distributed stream | NATS JetStream or Redpanda/Kafka | SCALE_GATE | only multi-consumer throughput/retention evidence |
| Analytics | Postgres projections initially | LOCKED_FOR_BOOTSTRAP | avoid premature warehouse |
| Real-time analytics | ClickHouse | SCALE_GATE | event/recommender telemetry at 10k–100k+ scale |
| Artifact/offline eval | S3 + Parquet | CP09 | replay/golden datasets |
| Lakehouse table format | Iceberg | SCALE_GATE | only multi-engine/lake requirements |
| AI runtime gateway | Rust service/module | CP11 | policy/redaction/provenance boundary |
| AI training/evals | Python | RESEARCH_ONLY/CP11 | mature ML tooling, isolated from authority core |
| Local model serving | vLLM/TGI/other via gateway | LOCK_AFTER_SPIKE | hardware/model dependent |
| AuthN | Passkeys/WebAuthn + OIDC adapters | CP04 | phishing-resistant + institutional identity providers |
| AuthZ | Rust application policy port | LOCKED_FOR_BOOTSTRAP | explicit server-side decisions |
| Policy engine | Cedar vs OPA | LOCK_AFTER_SPIKE | benchmark expressiveness/latency/audit/ops |
| Secrets/KMS | cloud KMS or Vault/OpenBao-class | CP13 | deployment-profile dependent |
| Local dev | Docker Compose | LOCKED_FOR_BOOTSTRAP | reproducible low-friction stack |
| Container runtime | OCI images | LOCKED_FOR_BOOTSTRAP | deployment portability |
| Orchestration | Kubernetes | SCALE/HA_GATE | production/sovereign HA profile, not dev architecture |
| IaC | OpenTofu/Terraform + Helm/Kustomize | CP13 | after topology locks |
| CI | GitHub Actions | LOCKED_FOR_BOOTSTRAP | repository-native |
| Dependency updates | Renovate | LOCKED_FOR_BOOTSTRAP | controlled reviewable updates |
| Rust supply chain | cargo-audit + cargo-deny | LOCKED_FOR_BOOTSTRAP | advisories/license/source controls |
| Secret scanning | gitleaks | LOCKED_FOR_BOOTSTRAP | baseline secret defense |
| SBOM | Syft | CP13 | release artifact |
| Image scan | Trivy/Grype | CP13 | release security |
| Signing | Cosign/Sigstore | CP13 | signed images/releases |
| Provenance | SLSA target L2→L3 | CP13–CP15 | build integrity |
| Static security | CodeQL/Semgrep | CP03/CP13 | code/security analysis |
| Accessibility | WCAG 2.2 AA + axe-core | CP04 onward | public/civic product requirement |
| Performance | k6/Vegeta + Criterion for Rust microbenchmarks | CP09/CP13 | SLO evidence |
| Fuzz/property | proptest + cargo-fuzz | CP06 onward | civic/parser/ranking invariants |
| API contract | OpenAPI 3.1 | CP03 onward | generated clients/contracts |
| Events | JSON Schema/AsyncAPI candidate | CP03–scale | explicit versioned event envelope |
| Provenance export | W3C PROV mapping | future/public audit | interoperability |
| Legislative docs | Akoma Ntoso mapping | future | legal/public-document interoperability |
| Federation | ActivityPub/AT Protocol research | future | no premature federation |

## Explicit non-selections for V1

- No Neo4j as source of truth.
- No Elasticsearch/OpenSearch by default.
- No Kafka by default.
- No Qdrant by default.
- No blockchain for civic audit.
- No Kubernetes for local development.
- No generic 'AI agent owns politics' layer.
- No microservice-per-domain.
- No Python backend for core civic state.
- No engagement-maximizing neural feed model in V1.

## Decision standard

A technology crosses from `SCALE_GATE`/`LOCK_AFTER_SPIKE` only with:
1. a written workload;
2. current baseline measurement;
3. candidate benchmark;
4. operational/security/cost analysis;
5. migration/rollback path;
6. ADR and checkpoint evidence.
