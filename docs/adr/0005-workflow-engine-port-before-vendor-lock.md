# ADR-0005 — WorkflowEngine port before workflow-vendor lock

Status: Accepted architecture; backend selection pending CP02 benchmark

## Context

Civic and institutional processes can run for hours, days or months and require timers, human signals, retries, resumability, versioned rules and deterministic recovery. They need durable workflow semantics. Selecting a workflow platform prematurely, however, can create deep operational and code lock-in.

## Decision

Define an application-level `WorkflowEngine` port and keep domain state authoritative in PostgreSQL.

Benchmark three V1 implementation strategies during CP02:

1. PostgreSQL workflow ledger + Rust workers;
2. Temporal Rust SDK;
3. Restate Rust SDK.

The benchmark must cover:
- durable timers;
- signals/human tasks;
- retries/backoff;
- replay/recovery;
- workflow version upgrades;
- deterministic/idempotent semantics;
- observability;
- self-hosting/sovereignty;
- operational maturity and SDK stability;
- failure behavior during DB/network/restart faults;
- throughput/latency/cost at pilot and 100k planning loads.

## Domain boundary

The workflow engine coordinates durable execution. It does not own civic truth, eligibility, participation, policy versions or institutional records. Those remain canonical domain/database state.

A workflow stores canonical object/version references and policy snapshot references; it never substitutes its internal history for the public civic record.

## Interim baseline

Until the benchmark closes, simple asynchronous work uses transactional outbox + idempotent Rust workers. Long-running process code must depend on the port rather than Temporal/Restate-specific types.

## Selection criteria

Prefer the simplest backend that meets durability/recovery/SLO requirements. A preview/immature SDK is not accepted solely because the server platform is mature.

## Rollback

Workflow instances must have externally persisted correlation/object/policy references and a documented export/recovery path so the platform can migrate orchestration technology without rewriting civic history.
