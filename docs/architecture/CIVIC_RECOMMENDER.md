# Civic Recommender — Professional Architecture v1

## North-star alignment

The recommender exists to increase **Qualified Civic Loop Completion (QCLC)** and improve discovery quality. It must never optimize for political persuasion, outrage or raw time-spent.

## 1. Architecture

```text
Feed/Search Request
      ↓
Context Hydrator
      ↓
Candidate Source Fan-out
      ↓
Candidate Hydration
      ↓
Visibility/Authz/Policy Filters
      ↓
Feature Extraction
      ↓
Civic Scoring Policy
      ↓
Constrained Slate Reranker
      ↓
Top-K Selector
      ↓
Post-selection Filters
      ↓
Explanation Builder
      ↓
Response
      ↓
Exposure/Evaluation Side Effects
```

The stage separation intentionally mirrors proven public recommendation architectures while keeping our semantics independent.

## 2. Rust component contract

Each stage implements a small typed port:

```rust
trait CandidateSource { async fn candidates(&self, ctx: &RequestContext) -> Result<Vec<Candidate>>; }
trait CandidateFilter { async fn filter(&self, ctx: &RequestContext, items: Vec<Candidate>) -> Result<Vec<CandidateDecision>>; }
trait FeatureHydrator { async fn hydrate(&self, ctx: &RequestContext, item: Candidate) -> Result<FeatureVector>; }
trait Scorer { fn score(&self, policy: &RankingPolicy, features: &FeatureVector) -> ScoreBreakdown; }
trait SlateReranker { fn rerank(&self, policy: &RankingPolicy, items: Vec<ScoredCandidate>) -> Vec<ScoredCandidate>; }
```

Do not create a network microservice for every trait. Initially all live in the same Rust process/crate boundary.

## 3. Candidate sources

- `SubscribedSource`: actors/topics/territories/processes explicitly followed.
- `TerritoryAgendaSource`: public items relevant to selected/home territory.
- `PrioritySource`: high civic-priority Needs under approved policy.
- `SemanticSource`: hybrid vector/lexical related items.
- `GraphSource`: typed related entities/proposals/evidence/processes.
- `InstitutionalSource`: official responses, deadlines, commitments, milestones.
- `UnservedDemandSource`: unresolved demand without adequate response.
- `DeliberativeNeedSource`: items missing evidence/perspective/expertise.
- `FreshSource`: new items relevant to user context before they accumulate interaction signals.
- `ExploreSource`: controlled randomized exploration with hard exposure budget.

Every candidate stores one or more source reasons; source provenance survives merging/dedup.

## 4. Visibility is independent from relevance

Visibility engine returns a structured action:

```text
ALLOW
ALLOW_WITH_CONTEXT/INTERSTITIAL
DOWNRANK_BY_POLICY (only where policy allows)
DROP
```

Reasons may include actor blocks/mutes, process scope, access control, moderation state, legal restrictions, duplicate canonicalization and age/closed-process state.

Security/legal removal must never be encoded as an arbitrary large negative ranking weight.

## 5. Feature vector

Feature groups:

### Relevance
- territorial distance/scope match;
- explicit subscription match;
- semantic similarity to query/current object;
- topic/competence match;
- language/accessibility fit.

### Civic utility
- approved civic-priority score;
- unresolved-demand status;
- evidence coverage/quality;
- institutional deadline/urgency;
- missing-expertise or missing-perspective demand;
- outcome/commitment update relevance.

### Quality/confidence
- source provenance coverage;
- duplicate probability;
- evidence/source diversity;
- current process/version confidence;
- manipulation/anomaly risk.

### Discovery
- freshness;
- controlled exploration;
- exposure fatigue;
- author/org/topic repetition.

No hidden ideology or protected-class inference is a ranking feature.

## 6. V1 transparent score

Use a configuration/policy object, not constants in code:

```yaml
policy: civic-feed/1.0.0
weights:
  territorial_relevance: 1.0
  explicit_interest: 0.8
  semantic_relevance: 0.6
  civic_priority: 0.8
  unresolved_importance: 0.7
  evidence_quality: 0.4
  deliberative_need: 0.5
  institutional_urgency: 0.7
  freshness: 0.3
  exploration: 0.15
penalties:
  manipulation_risk: 1.0
  fatigue: 0.4
  duplicate_probability: 1.0
```

Numbers above are schema examples, **not approved production weights**.

## 7. Slate optimization

A feed is not just independently ranked items. Enforce slate-level constraints:

- max repeated author/org within N positions;
- topic concentration limit;
- territory-level relevance floor;
- minimum source-class diversity;
- official-update urgency reservations where needed;
- controlled new-author/exploration allocation;
- conversation/cluster collapse.

V1 algorithm: constrained greedy reranking with Maximum Marginal Relevance-style novelty penalty. Keep deterministic/replayable under policy version and random seed.

## 8. User-facing modes

Do not force one algorithmic truth.

- **Para ti / Recomendado** — explained civic recommender.
- **Siguiendo** — explicit subscriptions, mostly chronological.
- **Prioridad cívica** — approved territory priority methodology.
- **Nuevo** — chronological.
- **Sin respuesta** — unserved-demand view.
- **Instituciones** — official responses/commitments/milestones.

## 9. Explanation fidelity

Explanation uses the actual scoring/filter/rerank trace.

Persist privacy-safe `RankingDecision`:

```text
request_id
item_id
ranking_policy_version
candidate_sources[]
visibility_policy_version
feature_snapshot_hash
score_components[]
rerank_adjustments[]
position
reason_codes[]
experiment_ids[]
```

Human copy is generated deterministically from `reason_codes`, not by an LLM.

## 10. Offline evaluation

Golden datasets:
- local/territory relevance;
- related-needs relevance;
- duplicate suppression;
- evidence/proposal relevance;
- official-update relevance;
- diversity/concentration;
- adversarial manipulation.

Metrics:
- Recall@K, NDCG@K, MRR for relevance tasks;
- QCLC proxy outcomes;
- source/org/topic concentration;
- exposure Gini/Herfindahl where useful;
- new-source coverage;
- manipulation resistance;
- explanation correctness 100%.

Every candidate-policy change runs replay/counterfactual eval before rollout.

## 11. Online experimentation

Required:
- explicit hypothesis;
- cohort/territory/process scope;
- QCLC-linked primary metric;
- guardrails: abuse reports, concentration, moderation burden, response quality, accessibility, latency;
- exposure logging;
- stop conditions;
- instant rollback to previous policy.

No experiment silently changes eligibility, legal effect or formal civic rights.

## 12. Future ML ranking

A learned ranker may be introduced when data supports it. Preferred progression:

1. deterministic weighted baseline;
2. LambdaMART/XGBoost-style interpretable LTR;
3. multi-task neural predictions as auxiliary features;
4. only later: sequence/slate transformer rankers.

Any learned model must be dominated by policy constraints and remain replaceable by deterministic baseline.

## 13. Abuse resistance

Attack tests:
- coordinated likes/supports;
- newly verified account swarm;
- party/organization brigading;
- bot-generated evidence spam;
- duplicate rephrasing to evade clustering;
- mass-report suppression;
- institutional account privilege abuse;
- exposure monopoly by high-volume actor;
- adversarial semantic text that targets embeddings;
- model/policy version mismatch.

Manipulation signals inform safety/quality workflows; they do not create secret political blacklists.
