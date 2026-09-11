# Testing & Evaluation Architecture

## Quality hierarchy

1. real compiler/typechecker;
2. production build;
3. executable unit/integration/contract tests;
4. property/fuzz/security tests;
5. E2E/browser/accessibility tests;
6. load/recovery/replay/evaluation evidence;
7. structural/generated manifest checks.

Structural checks are useful but never outrank executable truth.

## Rust

- unit tests for pure domain invariants;
- integration tests against real PostgreSQL/PostGIS/pgvector containers;
- `proptest` for process/eligibility/tally/ranking invariants;
- `cargo-fuzz` for parsers, evidence ingestion and hostile boundary data;
- `criterion` only for performance-sensitive kernels where regression matters;
- Clippy with warnings denied.

Critical properties:
- support can never be interpreted as ballot;
- one active unique participation under rules where uniqueness is required;
- process policy snapshot cannot mutate after start;
- event aggregate versions are monotonic;
- outbox publish/retry is idempotent;
- projection rebuild equals canonical projection state;
- unauthorized territory/object access never succeeds;
- proposal version evaluated by a process cannot change;
- ranking explanation corresponds to actual decision trace.

## Database

Every migration tests:
- apply from previous released version;
- fresh apply;
- forward compatibility during expand/contract window;
- rollback strategy or explicit irreversible classification;
- representative query plans;
- constraints under concurrent mutations.

Use migration checksums and release manifests.

## API/contracts

- OpenAPI schema validation;
- generated client compile tests;
- compatibility diff in CI;
- contract tests for error/authorization semantics;
- idempotency tests for commands;
- versioned event schema tests.

## Frontend

- TypeScript strict typecheck;
- component behavior tests where value exceeds maintenance cost;
- Storybook states for design-system components;
- Playwright E2E for canonical user journeys;
- visual regression for approved high-value screens;
- axe/accessibility checks;
- keyboard-only flows;
- responsive/mobile/browser matrix.

## Search/vector evaluation

Golden sets include adjudicated query→relevant-object labels for:
- duplicate Needs;
- topic classification;
- competence-routing suggestions;
- evidence retrieval;
- related proposals;
- global/public search.

Metrics:
- Precision/Recall/F1;
- Recall@K;
- MRR;
- NDCG@K;
- exact-vs-ANN recall;
- p50/p95/p99 latency;
- index build time/memory/cost.

Any embedding-model migration shadows production and must outperform or satisfy non-regression gates before switch.

## Recommender evaluation

Offline:
- relevance NDCG/Recall;
- QCLC proxy lift;
- actor/org/topic/territory concentration;
- exposure inequality;
- new-source discovery;
- manipulation resistance;
- explanation fidelity;
- counterfactual replay against previous ranking policy.

Online experiments require civic primary metric plus guardrails, exposure logs and rollback.

## AI evals

Task-specific sets for:
- routing;
- duplicate suggestions;
- cited summaries;
- argument extraction;
- evidence retrieval/rerank;
- institutional triage;
- moderation triage.

Adversarial sets:
- prompt injection inside documents;
- fabricated citations;
- political framing bias;
- minority-position omission;
- multilingual ambiguity;
- legal/competence overclaiming;
- poisoned/repeated evidence.

## Security testing

- authorization matrix/property tests;
- secret/SCA/SAST scans;
- hostile upload tests;
- CSRF/XSS/SSRF/injection suite;
- rate-limit/abuse tests;
- session/passkey/OIDC tests;
- Sybil/brigading simulations;
- mass-report robustness;
- admin misuse exercises;
- recovery after compromised/stale projection.

## Resilience

- kill workers mid-job and prove idempotent recovery;
- replay event log from checkpoint;
- corrupt/drop projection and rebuild;
- database backup/restore drill;
- object evidence restore/hash verification;
- dependency/provider/model outage drills;
- workflow timer recovery after downtime.

## Performance gates

Load profiles:
- read-heavy public browsing;
- burst support/participation events;
- institutional response workflows;
- feed candidate generation;
- hybrid search;
- evidence upload/download;
- background vectorization/reindexing.

No performance target is declared met without a recorded workload and environment.
