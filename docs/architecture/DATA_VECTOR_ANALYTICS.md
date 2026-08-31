# Data, Vector, Search & Analytics Architecture

## 1. Principle

**Canonical civic truth is relational/versioned; semantic representations are derived.**

PostgreSQL owns transactional state and immutable/versioned civic records. Search, embeddings, graph features and analytics are rebuildable views whose lineage points back to canonical entities.

## 2. PostgreSQL foundation

Required V1 capabilities:

- PostgreSQL current supported major;
- PostGIS for territories, administrative geometries and spatial queries;
- pgvector for semantic indexes;
- `pg_trgm` for fuzzy/lexical matching;
- built-in full-text search;
- native declarative partitioning for append-heavy tables;
- `jsonb` only for genuinely flexible metadata, not to avoid domain modeling.

### Core schemas

```text
identity_private
civic
policy
institution
moderation
ai
audit
search_projection
analytics_projection
```

Private identity and public civic data are separated structurally, not only by a boolean field.

## 3. Data lifecycle classes

| Class | Examples | Default retention posture |
|---|---|---|
| canonical public | needs, proposal versions, official responses | durable/versioned according to governance/legal policy |
| canonical private | identity assertions, drafts | minimum required + explicit retention |
| sensitive case | moderation/appeals, abuse evidence | restricted/time-bounded where possible |
| audit | policy/admin/action receipts | append-only/tamper-evident, retention policy explicit |
| derived search | text indexes, vectors | rebuildable; may be deleted/recomputed |
| analytics | pseudonymized interaction events | minimized and purpose-limited |
| ML eval | golden datasets, synthetic/adjudicated labels | versioned, privacy-reviewed |

## 4. Vectorization model

### Vectorizable entities

1. `NeedVersion` — original citizen statement + normalized public summary.
2. `NeedCluster` — canonical cluster representation.
3. `ProposalVersion` — sections independently chunked with version identity.
4. `EvidenceDocument` — public document chunks after extraction/safety validation.
5. `Claim` and `Argument` — structured semantic units.
6. `CivicTopic` — public taxonomy descriptions.
7. `CompetenceNode` — jurisdiction/authority descriptions and legal-source references.
8. `InstitutionPublicDocument` — public responses, plans, budgets, resolutions where licensed/allowed.
9. `HelpKnowledge` — public civic explainers.

### Never vectorize by default

- raw government ID scans;
- biometric templates/selfies;
- payment methods;
- secrets/tokens;
- private moderation evidence;
- private user messages;
- hidden political-profile inference;
- unnecessary sensitive attributes.

If a sensitive use case truly requires an embedding, it needs a dedicated ADR/DPIA/security policy, an isolated index and explicit deletion/rebuild semantics.

## 5. Embedding schema

Recommended canonical metadata:

```sql
entity_id UUID NOT NULL,
entity_type TEXT NOT NULL,
source_version BIGINT NOT NULL,
content_hash BYTEA NOT NULL,
chunk_ordinal INT NOT NULL,
locale TEXT,
embedding_model_id TEXT NOT NULL,
embedding_model_version TEXT NOT NULL,
embedding_dimensions INT NOT NULL,
vector_kind TEXT NOT NULL, -- dense | sparse | half | binary-derived
policy_classification TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL,
valid_until TIMESTAMPTZ,
PRIMARY KEY(entity_id, entity_type, source_version, chunk_ordinal, embedding_model_id, embedding_model_version)
```

The vector may be stored beside this metadata in pgvector or in a derived vector store later.

## 6. Model/version migrations

Do not rewrite embeddings in place.

Migration procedure:

1. register new embedding model/version;
2. generate shadow embeddings asynchronously;
3. run golden retrieval eval against current production representation;
4. build new index concurrently;
5. compare latency, recall, bias/exposure and cost;
6. atomically switch `active_embedding_index_version`;
7. retain old version during rollback window;
8. delete old derived vectors under retention policy.

This makes model upgrades observable/reversible.

## 7. Hybrid retrieval

### Query path

```text
Authorization + territory/process filters
          ↓
     query normalization
          ↓
   ┌──────┼─────────┐
   ↓      ↓         ↓
 lexical  dense    sparse/metadata
 FTS      HNSW     optional
   └──────┼─────────┘
          ↓
Reciprocal Rank Fusion
          ↓
explicit graph/competence boosts
          ↓
small Top-N reranker when justified
          ↓
civic policy/visibility filters
          ↓
results + reason/provenance
```

### Why hybrid

Pure vector similarity is not enough for civic/public-administration data. Exact legal terms, institution names, article numbers, geographic names and recent process IDs need lexical strength. Semantic similarity helps paraphrases such as 'no puedo pagar el alquiler' ↔ housing affordability.

## 8. pgvector V1

Use pgvector because:
- transactionally close to source records;
- mature HNSW/IVFFlat support;
- exact search available as truth/evaluation baseline;
- supports vector/half/sparse representations;
- metadata/jurisdiction filtering can stay relational;
- drastically lower operational complexity than a second DB.

### Index strategy

- HNSW for most dense semantic retrieval.
- use `halfvec` only after quality/size benchmark.
- use partitioning/partial indexes for high-cardinality, well-defined scopes such as locale/entity class where it improves selectivity.
- filter on territory/policy class before or during ANN query as supported by the plan.
- use iterative scans/tuned `ef_search` when filtered recall suffers.

### Evaluation

For a sampled/golden query set:

`ANN recall@K = overlap(ANN topK, exact topK) / K`

Track recall, NDCG/MRR/precision/recall for the product task, latency p50/p95/p99, memory and index-build time.

## 9. Qdrant decision gate

Evaluate Qdrant only when measured V1/V2 workloads require:
- independent horizontal vector scaling;
- complex payload-filtered ANN where Postgres consistently misses latency/recall targets;
- named dense + sparse vectors at large scale;
- vector storage/index pressure materially harms transactional Postgres;
- operational ownership for another stateful system is justified.

Migration must keep Postgres as authority and treat Qdrant as a rebuildable projector target.

## 10. Graph intelligence

V1 graph is a typed relational projection (edge tables/materialized read models). Use recursive CTEs and adjacency tables for modest-depth civic relations.

Later evaluate a graph engine only if measured workloads require high-depth traversals/community algorithms beyond PSQL SLOs.

Candidate derived graph features:
- shared territory/topic;
- need→proposal coverage;
- claim→evidence support;
- source diversity;
- institutional competence path;
- actor contribution context;
- duplicate cluster neighborhood;
- related process/outcome history.

No graph centrality score becomes civic authority by default.

## 11. Analytics architecture

### V1

- canonical domain events/outbox in Postgres;
- privacy-minimized product events;
- OpenTelemetry traces/metrics/logs;
- materialized/read-model metrics for QCLC and operational dashboards.

Do not add an analytics warehouse merely for dashboard aesthetics.

### Scale gate → ClickHouse

Introduce ClickHouse when:
- high-cardinality impression/candidate/recommender telemetry becomes expensive in OLTP;
- cohorts/funnels need sub-second analytics across tens/hundreds of millions of events;
- real-time abuse/recommender observability benefits from columnar scans.

Pipeline:

```text
client/domain/recommender events
        ↓
outbox / durable stream
        ↓
ClickHouse raw versioned events
        ↓
materialized views
        ↓
product / SRE / recommender / civic-quality dashboards
```

Postgres remains transaction authority.

## 12. Event schema

Minimum analytics envelope:

```text
event_id
schema_version
event_name
occurred_at
received_at
correlation_id
session_id (pseudonymous where appropriate)
actor_ref (pseudonymous/omitted by privacy class)
territory_ref
process_ref
object_type/object_ref
client/app version
experiment exposures
privacy_class
payload
```

Never dump arbitrary civic text into analytics payloads by default.

## 13. Offline data/evaluation lake

Use object storage for immutable Parquet snapshots:
- recommender candidates/features/decisions under privacy policy;
- retrieval golden corpora;
- model evaluation datasets;
- anonymized/pseudonymized product metrics;
- backup/export artifacts.

Only introduce Iceberg/Delta-like table formats if multiple engines/versioned large datasets create a real requirement.

## 14. Data quality

Every critical dataset defines:
- owner;
- schema version;
- freshness SLA;
- completeness checks;
- uniqueness/idempotency keys;
- privacy class;
- lineage/source;
- backfill method;
- deletion handling;
- replay semantics.

## 15. North-star data contract

QCLC must be reconstructible from immutable domain events plus canonical states. It must not depend on mutable analytics heuristics.

A completed loop records at minimum:
- originating Need/Cluster;
- competence route/version;
- proposal or explicit resolution;
- process/policy snapshot;
- institution/authority response;
- commitment/outcome where applicable;
- evidence/provenance;
- completion reason/timestamps.
