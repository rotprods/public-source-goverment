# ADR-0002 — PostgreSQL authority; pgvector-first semantic search

Status: Accepted for V1 baseline

## Context

The product needs transactional civic state, territorial geometry, exact/lexical search, semantic retrieval, auditable policy/version joins and a low-entropy local/sovereign deployment story.

Adding PostgreSQL + a separate vector database + a separate search engine + a graph database at bootstrap would create four stateful systems before a measured workload exists.

## Decision

1. PostgreSQL is the authoritative transactional database.
2. PostGIS owns territorial geometry.
3. PostgreSQL FTS + `pg_trgm` provide exact/fuzzy lexical retrieval.
4. pgvector is the initial dense/sparse semantic index.
5. Search/vector/graph/feed/analytics representations are rebuildable projections of canonical records.
6. Exact nearest-neighbor queries are the offline reference for ANN recall evaluation.
7. A dedicated vector database is a scale gate, not a V1 dependency.

## Qdrant extraction gate

Qdrant may be introduced only if a representative benchmark proves material benefit for the real workload, including filtered ANN recall/latency, independent vector scaling, memory/index pressure, named dense+sparse vector use, operational cost and rebuild/rollback behavior.

PostgreSQL remains authority after extraction; the external vector store is populated by an idempotent projector.

## Consequences

Positive:
- one transactional source of truth;
- simple backup/restore and local Docker topology;
- jurisdiction/policy metadata can be filtered relationally;
- low operational burden for pilot/10k scale;
- deterministic rebuild/migration path.

Tradeoffs:
- very large filtered vector workloads may eventually justify a dedicated vector store;
- search features beyond PostgreSQL's strengths may require a future OpenSearch/other projection;
- high-volume analytics must not overload OLTP and may later move to ClickHouse.

## Evidence required to supersede

A replacement ADR must include reproducible dataset/workload, p50/p95/p99 latency, task quality metrics, ANN recall against exact baseline, memory/index-build measurements, operations/security/cost analysis, migration/backfill and rollback plan.
