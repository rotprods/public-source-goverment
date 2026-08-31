# ADR-0001 — Rust-first core

Status: Accepted for bootstrap

## Decision
New domain kernels, command/query services, workers/projectors, recommendation/search orchestration and security-critical runtime code default to Rust. TypeScript is preferred for web/product UI and tooling where ecosystem leverage is material. Python is isolated to ML training/evaluation/research or justified library gaps.

## Why
- strong correctness/types for politically meaningful state;
- memory/concurrency safety;
- predictable performance for ranking/search/workers;
- aligns critical runtime in one ecosystem;
- reduces accidental complexity from transitional JS backends.

## Non-goal
Do not rewrite stable external/reference code for dogma. Language choice is revisited by workload and maintenance evidence.
