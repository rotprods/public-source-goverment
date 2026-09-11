# SOURCE REGISTRY

All external material is research input until converted into project-native decisions/ADRs. Refs identify the inspected state when available.

| ID | Source | Inspected ref | Use | License/status |
|---|---|---|---|---|
| R-MOVI | `rotprods/movimurcia` | `codex/bootstrap-agentic-foundation` @ `b809184bd25c64e5bf676c0cfa7d482aae76186d` | evidence-first project OS, route/contract manifests, security gates, lessons from runtime/docs imbalance | project source |
| R-XAI | `xai-org/x-algorithm` | `main`, README + `home-mixer/candidate_pipeline/phoenix_candidate_pipeline.rs` + `ranking_scorer.rs`, inspected 2026-09 | Rust request/candidate/filter/rank/visibility pipeline, scoring/exploration/diversity patterns | Apache-2.0 inspected |
| R-TW | `twitter/the-algorithm` | `main` public archive | Product Mixer, SimClusters/TwHIN/RealGraph/user-action streams/representation architecture | AGPL-3.0 inspected; architecture study by default |
| R-REDDIT | `reddit-archive/reddit` | `master`, `r2/r2/lib/db/_sorts.pyx` | Wilson confidence, hot/time decay, controversy, community/thread/moderation patterns | CPAL 1.0 inspected; independently reimplement math unless explicit compatibility review |
| R-PGV | `pgvector/pgvector` | 0.8.6-era docs/changelog | exact+ANN, HNSW/IVFFlat, half/sparse vectors, hybrid search, recall evaluation | PostgreSQL extension; inspect exact upstream license before copying code |
| R-QDR | Qdrant docs | current | dense+sparse named vectors, RRF/DBSF, late reranking patterns | service candidate only after benchmark gate |
| R-CH | ClickHouse current architecture docs | 2026 | real-time event analytics/columnar scale | SCALE_GATE |
| R-TEMP | Temporal Rust SDK/docs | 2026 public preview | durable workflow candidate | benchmark/maturity gate |
| R-RESTATE | Restate Rust SDK/docs | current | durable workflow candidate | benchmark/maturity gate |
| R-AP | ActivityPub W3C | standard | federation/publication research | future |
| R-ATP | AT Protocol | current architecture/docs | signed repos, DID/data portability, speech vs reach separation | future research |
| R-ASVS | OWASP ASVS 5.0 | current | application-security verification baseline | standard/guidance |
| R-SLSA | SLSA v1.2 | current | software provenance/release integrity | standard/guidance |

## Rules

1. Source-derived claims stay distinguishable from project decisions.
2. Architectural patterns may be independently implemented; source code reuse requires explicit license compatibility.
3. No research algorithm becomes civic policy without Product Constitution/ADR approval.
4. The public recommender's objective and weighting are project-specific even when its pipeline shape resembles social platforms.
5. Mathematical techniques such as RRF/Wilson/MMR are implemented project-native and documented with provenance; upstream source text is not copied merely because it is public.
6. Update this registry when a source commit/version materially changes an adopted conclusion.
