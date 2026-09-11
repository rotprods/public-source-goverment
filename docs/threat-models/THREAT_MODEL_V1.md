# Threat Model v1 — Civic Infrastructure

Status: `BOOTSTRAP BASELINE / MUST EVOLVE WITH CPs`.

This is an engineering threat model, not a claim of legal compliance or production security.

## 1. Assets

Critical assets:
- private identity assertions and authentication credentials;
- civic participation rights/eligibility state;
- process policy snapshots and legal-effect metadata;
- Needs, Evidence, Claims, Proposal Versions and official records;
- participation receipts/results;
- institutional authority/response/commitment records;
- moderation/appeal state;
- ranking/recommender policies and decision traces;
- audit/domain-event/outbox integrity;
- object-store evidence and hashes;
- backups/signing keys/release provenance;
- AI prompts/retrieval corpora/artifact provenance.

## 2. Trust boundaries

1. Anonymous Internet → public edge.
2. Authenticated user device → application API.
3. Identity provider/eID assertion → identity-private zone.
4. Application → PostgreSQL authority.
5. Application/outbox → workers/projectors.
6. Canonical DB → search/vector/analytics projections.
7. User/document upload → evidence ingestion pipeline.
8. Canonical/retrieved data → AI gateway/model provider.
9. Institution staff → institutional workbench.
10. Operator/admin → policy/security administration.
11. CI/CD → artifact registry/deployment.
12. Production → backup/restore/DR stores.
13. One territory/tenant/authority scope → another.

## 3. Threat actors

- opportunistic attacker;
- credential thief/phisher;
- bot/Sybil farm;
- coordinated political organization/brigade;
- malicious or compromised public institution account;
- malicious platform administrator/insider;
- disgruntled contractor/operator;
- evidence spammer/doxxer;
- supply-chain attacker;
- hostile third-party model/provider;
- prompt-injection/data-poisoning attacker;
- scraper attempting sensitive political-profile reconstruction;
- denial-of-service actor;
- compromised sovereign/on-prem installation.

## 4. High-priority abuse cases

### TM-01 — Object-level authorization bypass
**Attack:** authenticated actor reads/modifies another actor's private draft, identity assertion, moderation case or restricted institution case by changing an ID.

Controls:
- deny-by-default server-side authorization;
- typed actor/territory/process capability context;
- object-level integration/property tests;
- audit high-impact reads/writes where proportionate;
- no trust in client role flags.

### TM-02 — Sybil/brigading manipulation
**Attack:** coordinated accounts inflate support/ratings/reports/exposure.

Controls:
- process-specific identity/uniqueness assurance;
- action semantics separated from formal ballots;
- rate/velocity/anomaly features;
- cluster/duplicate controls;
- confidence-adjusted quality scores where appropriate;
- mass-report never equals proof;
- reversible enforcement/appeals;
- offline adversarial simulation.

### TM-03 — Policy or process-rule mutation after participation begins
**Attack/failure:** operator deploy or DB edit changes eligibility, threshold, decision method or ranking policy mid-process.

Controls:
- immutable policy/version records;
- canonical `policy_snapshot_sha256` frozen at process start;
- active-process mutation rejection;
- signed/attributable admin actions;
- public policy/version audit trail;
- release tests with old policy fixtures.

### TM-04 — Institutional impersonation/capture
**Attack:** actor falsely publishes an official response or compromised institution account rewrites history.

Controls:
- institutional credentials/organization membership assertions;
- capability scoped to territory/competence;
- step-up authentication for official publication;
- append/version official records rather than destructive overwrite;
- public authority attribution;
- optional dual control for high-impact actions.

### TM-05 — Ranking manipulation / exposure capture
**Attack:** high-volume actor, coordinated support, duplicate rephrasing or feature gaming monopolizes civic discovery.

Controls:
- visibility separate from ranking;
- canonical duplicate clusters;
- actor/org/topic diversity constraints;
- bounded exploration;
- manipulation-risk penalty under approved policy;
- candidate/source/feature/rerank decision trace;
- exposure concentration alerts;
- deterministic rollback to prior ranking policy.

### TM-06 — Hidden political profiling
**Attack/failure:** analytics/recommender reconstructs ideology or protected/sensitive traits and uses them for persuasion.

Controls:
- explicit feature allowlist in ranking contracts;
- no ideology/party-conversion/protected-trait feature fields;
- purpose-limited analytics schema;
- private identity never copied into public embeddings;
- feature lineage/audit;
- privacy/legal review for any new sensitive feature purpose.

### TM-07 — Evidence upload attack
**Attack:** malicious PDF/image/archive exploits parser, contains malware, doxxing or hidden prompt injection.

Controls:
- isolated upload/quarantine path;
- MIME/content signature verification;
- size/page/decompression limits;
- malware scanning;
- sandboxed extraction;
- hash + immutable source object;
- active-content stripping where applicable;
- human/report/moderation controls;
- retrieved document text always treated as untrusted data.

### TM-08 — AI prompt injection / retrieval poisoning
**Attack:** evidence instructs model/agent to ignore system policy, expose secrets, fabricate source conclusions or suppress content.

Controls:
- AI gateway separates instructions from retrieved untrusted content;
- tool/data least privilege;
- no raw secrets/private identity in model context;
- citation/provenance requirement;
- output schema validation;
- task-specific red-team evals;
- model output never directly changes eligibility/policy/results;
- review/approval for high-impact derived artifacts.

### TM-09 — Event replay/double mutation
**Attack/failure:** retries duplicate support/participation/official actions.

Controls:
- idempotency keys;
- unique active-participation constraints;
- monotonic aggregate versions;
- event IDs;
- idempotent consumers;
- transaction + outbox atomicity;
- replay/property tests.

### TM-10 — Projection corruption/staleness
**Attack/failure:** feed/search/vector/analytics contains deleted, blocked or stale-policy state.

Controls:
- projections explicitly non-authoritative;
- source version/content hash on derived representations;
- deletion/revocation events;
- lag/freshness metrics;
- rebuild tooling;
- periodic canonical-vs-projection reconciliation.

### TM-11 — Cross-territory/competence leakage
**Attack/failure:** admin/institution capability for municipality A operates on municipality B, or process eligibility uses wrong boundary version.

Controls:
- territory and competence are typed authorization inputs;
- PostGIS geometry/version references;
- process freezes registry/version;
- matrix tests for cross-scope denial;
- no global admin convenience role for ordinary institutional work.

### TM-12 — Audit tampering
**Attack:** privileged actor removes evidence of policy/admin/action history.

Controls:
- append-only domain/audit records;
- restricted DB role separation;
- hash-chain/signature research for high-value receipts where justified;
- immutable/off-site backups;
- release/deployment provenance;
- alert on destructive DDL/admin operations.

### TM-13 — Supply-chain compromise
**Attack:** malicious dependency/action/container injects code or steals credentials.

Controls:
- lockfiles;
- minimal dependency policy;
- cargo-audit/cargo-deny/pnpm audit;
- secret scanning;
- pin/review high-risk CI actions over time;
- SBOM + container scan before production;
- signed OCI/release artifacts;
- SLSA provenance target;
- restricted CI token permissions and environment separation.

### TM-14 — Denial of service / expensive query abuse
**Attack:** vector search, geospatial, evidence extraction or AI calls exhaust CPU/RAM/cost.

Controls:
- edge/application rate limits;
- bounded query parameters/candidate counts/upload size;
- timeouts/circuit breakers;
- workload queues and per-class concurrency;
- cache where safe;
- cost budgets for AI;
- DB statement/lock monitoring;
- load/adversarial testing.

### TM-15 — Backup/restore produces inconsistent civic truth
**Failure:** DB/object store/policies restore from different points and public process state diverges.

Controls:
- documented RPO/RTO;
- coordinated backup manifests;
- object hashes;
- event/outbox reconciliation;
- periodic full restore drill;
- projection rebuild from canonical state;
- process-policy snapshot verification after restore.

## 5. Security invariants

- No client-only authorization.
- No generic `is_verified` authorization shortcut.
- No mutation without actor/context/policy decision.
- No active process without immutable/frozen policy snapshot.
- No high-impact administrative mutation without attributable audit receipt.
- No model output directly grants rights or changes formal results.
- No projection is treated as authority when canonical state disagrees.
- No production secret in Git/repository artifacts.
- No third-party code copied across incompatible license boundaries.

## 6. Verification plan

Before pilot production:
- authz matrix/property tests;
- race/replay/idempotency tests;
- hostile upload/parser tests;
- prompt-injection/retrieval-poisoning eval;
- Sybil/brigading/mass-report simulations;
- ranking concentration/manipulation replay;
- dependency/secret/SAST/container scans;
- backup/restore drill;
- policy-snapshot mutation test;
- cross-territory isolation test;
- external penetration-test plan and remediation gate.

## 7. Residual risks requiring governance/legal closure

- lawful basis and safeguards for sensitive civic/political-opinion processing;
- identity assurance level by process;
- moderation/operator governance and appeal independence;
- publication/retention duties for public records;
- regulated process integrations;
- jurisdiction-specific public-sector security/compliance profile.
