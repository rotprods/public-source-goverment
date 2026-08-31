# HANDOFF

## Current state

Repository bootstrap is being compiled on `bootstrap/empezarproyecto-v1` from an almost-empty `main`.

Canonical direction:
- product name OPEN;
- Rust-first core;
- PostgreSQL/PostGIS/pgvector authority;
- hybrid/vector/graph civic discovery;
- explainable recommendation pipeline;
- civic utility instead of engagement optimization;
- institution workbench and public accountability are first-class;
- regulated public ballot is a future specialized boundary.

## Research applied

- MoviMurcia: retain evidence-first state, machine-readable manifests, security gates; avoid scope illusion/doc explosion and verifier-without-build failure modes.
- X/xAI: pipeline stages, parallel candidate sources, Rust service composition, visibility separation, diversity/reranking and model/policy config.
- Twitter legacy algorithm: Product Mixer, SimClusters, graph candidate sources, embedding representation and real-time user-action stream architecture. Code is AGPL-3.0; architectural study only by default.
- Reddit: community/thread/moderation patterns; hot/Wilson algorithms as discovery/quality primitives, not civic-importance truth.
- pgvector/Qdrant: hybrid semantic retrieval architecture.

## Immediate next work

1. Finish `/empezarproyecto` repo configs/skills/build skeleton.
2. Commit research and recommendation/vector architecture docs.
3. Add CI/security/supply-chain gates.
4. Open bootstrap PR for review.
5. CP01: close Product Constitution P0 decisions.
6. CP02: run workflow-engine, policy-engine and vector-store benchmark spikes.

## Blockers

- public product naming;
- operator/governance authority;
- final weighted participation/reputation effect;
- social/civic priority formula;
- unserved-demand timing/routing;
- identity assurance levels by process.

## Rule for next agent

Do not implement blocked policy semantics as magic constants. The architecture may provide ports/types/config, but runtime behavior waits for approved policy versions.
