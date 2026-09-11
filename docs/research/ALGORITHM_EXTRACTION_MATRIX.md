# Algorithm Extraction Matrix — X / Twitter / Reddit → Civic Infrastructure

Status legend:
- `IMPLEMENT`: safe project-native primitive to build now.
- `ADAPT`: useful architecture/algorithm, but objective/data/policy must change.
- `RESEARCH`: promising later-scale capability; evaluate before runtime use.
- `REJECT`: conflicts with civic invariants or creates unacceptable risk.

## xAI `x-algorithm` (current Rust implementation)

| Upstream pattern | Civic mapping | Status | Notes |
|---|---|---:|---|
| `CandidatePipeline` stage composition | typed Rust candidate-source/filter/scorer/selector pipeline | IMPLEMENT | strongest architecture transfer; already reflected in `CIVIC_RECOMMENDER.md` |
| query hydration | actor/session/territory/subscriptions/process context | ADAPT | prefer explicit civic context; avoid unnecessary demographic/political inference |
| Thunder in-network source | followed topics/territories/actors/processes | ADAPT | explicit subscriptions rather than social follower graph only |
| Phoenix retrieval | semantic candidate source | ADAPT | retrieval objective becomes civic relevance, not engagement likelihood |
| SimClusters | sparse community/topic representations | RESEARCH | may help discovery; never expose latent cluster as political identity |
| candidate hydration | batched canonical civic features | IMPLEMENT | avoid N+1; provenance/version each feature family |
| duplicate/seen filters | cluster collapse/exposure fatigue | IMPLEMENT | especially useful to avoid repeated civic issues dominating feed |
| visibility filtering | authorization/legal/moderation stage | IMPLEMENT | hard separation from relevance scoring is non-negotiable |
| weighted predicted actions | transparent civic component score | ADAPT | do not import engagement objective/weights |
| author cold-start/new-author boost | exploration/new-source allocation | ADAPT | bounded exploration policy and exposure budget |
| repeated-author decay | actor/org/topic concentration controls | IMPLEMENT | extend to organization and territory diversity |
| Top-K selector | deterministic selection after civic score/rerank | IMPLEMENT | preserve decision trace |
| VM reranker | optional later slate/cross-encoder reranker | RESEARCH | only after transparent baseline/eval maturity |
| side effects/impression cache | privacy-minimized exposure/eval telemetry | ADAPT | necessary for fatigue, audits and counterfactual replay |
| feature switches | versioned ranking/policy configs | IMPLEMENT | active civic process may freeze materially relevant versions |
| demographic/inferred-gender/app signals | civic personalization | REJECT | not legitimate default input to civic recommendation |

## Legacy `twitter/the-algorithm`

| Upstream component | Civic use | Status | Notes |
|---|---|---:|---|
| Product Mixer | reusable conceptual framework for candidate pipelines | ADAPT | project-native Rust implementation; upstream code is AGPL-3.0 |
| Unified User Actions | versioned civic interaction event envelope | IMPLEMENT | purpose-limit and minimize telemetry |
| User Signal Service | explicit civic-signal/context read model | ADAPT | avoid building a hidden political-profile warehouse |
| SimClusters | topic/community sparse embeddings | RESEARCH | evaluate on need/topic/evidence discovery |
| TwHIN heterogeneous embeddings | Need/Proposal/Evidence/Topic graph embeddings | RESEARCH | V1 uses explicit graph + text embeddings first |
| RealGraph | interaction-affinity graph | RESEARCH | may improve discovery, never authority/ballot weight |
| TweepCred/PageRank | source/reliability/anti-spam signal | RESEARCH | scope by role/domain/time; never global civic power |
| GraphJet traversal candidate sources | fast interaction/related-entity graph retrieval | RESEARCH | introduce only after graph workload proves PostgreSQL projection insufficient |
| graph-feature service | typed pair/entity graph features | ADAPT | useful for explicit civic relationships/competence paths |
| light + heavy ranker cascade | cheap retrieval/rank → expensive rerank | RESEARCH | valuable when candidate volume demands two-stage ranking |
| real-time aggregation framework | rolling feature/materialized aggregate jobs | ADAPT | begin with Postgres/projectors; scale later |

## Reddit archived ranking/community code

The inspected archive is CPAL 1.0; direct code copying is not the default. Mathematical primitives can be independently implemented with provenance.

| Reddit primitive | Civic use | Status | Notes |
|---|---|---:|---|
| threaded discussions | structured deliberation/argument trees | ADAPT | claims/evidence/arguments should be first-class, not only comments |
| community/subreddit boundaries | civic topic/territory/process spaces | ADAPT | jurisdiction and authority add stronger semantics |
| local moderation rules | process/topic moderation policy | ADAPT | policies versioned/appealable; institutional actors cannot bypass them |
| `new` sort | chronological civic mode | IMPLEMENT | one of several explicit user modes |
| Wilson lower bound confidence | confidence-adjusted usefulness/quality ratings | IMPLEMENT | use for evidence/helpfulness quality; never civic priority or vote power |
| `hot` time-decay formula | freshness/discovery primitive | ADAPT | useful as one candidate source, not public-importance truth |
| `controversy` sort | optional disagreement/discussion discovery | RESEARCH | never default positive boost for polarization |
| karma | contribution/reliability context | ADAPT/RESTRICT | scope by role/domain/territory/time; no formal voting power |
| global popularity rank | civic priority | REJECT | virality is not public need/competence/impact |

## Algorithms we should implement project-native

### Wave A — now / deterministic baseline

1. Reciprocal Rank Fusion for lexical + dense + sparse + graph candidate merging.
2. Explicit versioned weighted civic score with feature/penalty breakdown.
3. Visibility/policy decision as a separate stage/type.
4. Duplicate/canonical-cluster collapse.
5. Actor/org/topic diversity constraints.
6. Wilson lower-bound confidence for optional quality/helpfulness signals.
7. Exposure fatigue / already-seen filtering.
8. Deterministic reason-code explanation builder.

### Wave B — after retrieval/ranking golden datasets

1. Maximum Marginal Relevance / constrained slate reranker.
2. Learning-to-rank baseline such as LambdaMART only if it beats deterministic policy under civic countermetrics.
3. Sparse topic/community representations.
4. Graph relation features and bounded centrality/reliability signals.
5. Cross-encoder reranking for small Top-N semantic search sets.

### Wave C — scale only

1. Transformer/sequence retrieval/ranking.
2. Learned slate reranking.
3. Heterogeneous graph embeddings.
4. Dedicated vector/graph infrastructure.
5. Real-time large-scale feature service.

## Rejected optimization family

Do not implement as positive ranking objectives:
- outrage/anger;
- political persuasion or party conversion;
- raw time spent/dwell;
- follower count as public authority;
- paid political promotion;
- partner/customer status;
- hidden inferred ideology;
- protected/sensitive personal attributes without a narrowly approved lawful purpose.

## Evaluation rule

A public algorithm becomes a project algorithm only after it has:
1. a civic purpose;
2. a data/privacy classification;
3. an explicit policy version;
4. task-specific golden evaluation;
5. manipulation/bias/concentration analysis;
6. explanation strategy;
7. rollback path;
8. license/provenance record where implementation material is borrowed.
