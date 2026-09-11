# Public Social Algorithms → Civic Recommendation Research

## Research rule

We are not building 'Twitter for politics'. We are extracting scalable, well-tested architectural primitives from social platforms and replacing their objective function and policy semantics with civic ones.

## 1. xAI `x-algorithm` — strongest directly reusable architecture pattern

Current public X architecture decomposes a feed request into:

1. query hydration;
2. candidate sources in parallel;
3. candidate hydration;
4. pre-scoring filters;
5. scoring;
6. Top-K selection;
7. post-selection visibility/dedup filters;
8. side effects/impression logging.

It also separates the labeling/visibility path from ranking. This separation is essential for civic infrastructure: **illegal/ineligible/blocked content is not merely 'low score'.**

### Civic adaptation

| X concept | Civic implementation |
|---|---|
| viewer action history | explicit subscriptions, prior civic actions, process/territory context; minimize implicit political profiling |
| Thunder/in-network | subscribed actors/topics/territories/processes |
| Phoenix/SimClusters | semantic/graph civic candidate sources |
| visibility filtering | policy/legal/authorization/moderation decision engine |
| predicted engagement actions | civic component features, not final objective |
| RankingScorer weights | versioned CivicRankingPolicy |
| author diversity penalties | actor/org/territory/source diversity constraints |
| unexplored/new-author boost | controlled civic exploration/freshness |
| impression side effects | privacy-minimized exposure logs for explanation/evaluation |
| Under the Hood | 'Why am I seeing this?' + public ranking methodology |

### Do not inherit

- maximizing likes/replies/shares/dwell;
- implicit ideology as a personalization feature;
- paid political insertion;
- opaque value weights without public-policy governance;
- engagement loops as product success.

## 2. Legacy `twitter/the-algorithm` — useful systems patterns, restrictive source-license boundary

Architectural concepts worth independently reimplementing:

### Unified user actions
A normalized event stream simplifies downstream feature generation. Civic equivalent: `CivicInteractionEvent` schemas versioned across client/server.

### User Signal Service
Centralized signal retrieval is useful, but civic signal profiles must be purpose-limited and should prefer explicit subscriptions/territory/process context over inferred political preferences.

### SimClusters
Community/sparse embedding concepts can help discover relevant topics, evidence and proposal neighborhoods. Do not equate latent communities with political ideology or expose them as political labels.

### TwHIN / representation scoring
Heterogeneous graph embeddings are useful research for entity similarity: actor↔topic, need↔proposal, evidence↔claim. For V1 we use explicit graph + text embeddings; learned graph embeddings require a later eval gate.

### RealGraph / interaction graph
A civic interaction graph may improve discovery, but social affinity must never determine formal civic authority or vote power.

### TweepCred/PageRank
Reputation/PageRank is useful for spam/source-quality research, not for democratic weight. We may compute reliability/quality signals scoped to role/domain and with appeals, but not a global political influence score.

### Product Mixer
The strongest pattern is composability: candidate sources, filters, scorers and selectors are independent pipeline components. Our Rust recommender follows this architecture.

### License boundary
The inspected legacy repository is AGPLv3. Unless the product's outbound-license strategy explicitly chooses a compatible path, implement patterns independently rather than copying code.

## 3. Reddit — community and sorting primitives

### Product patterns
- topic/community spaces are legible to humans;
- threaded discussion;
- explicit local rules/moderation;
- multiple sort modes instead of one algorithmic truth;
- visible provenance and history on edits/moderation are naturally understandable.

### Mathematical primitives

#### Wilson lower bound
Useful for ranking ratings/quality estimates when sample sizes differ. Candidate uses:
- confidence-adjusted usefulness rating of evidence;
- expert-review quality summaries;
- moderation/helpfulness quality.

Never use it as the sole civic priority measure.

#### Hot/time decay
Useful for **discovery freshness**, not public importance. A new local issue can surface without permanently outranking a large enduring need.

#### Controversy/balance
May help offer an optional 'active disagreement' view, but controversy must not receive a default positive boost because polarization itself is not civic value.

### Reddit pattern to reject
A global karma-style scalar should not determine civic capabilities or political authority. If reputation exists, scope it by domain × role × territory × time and attach reason codes/appeals.

## 4. Civic Product Mixer design

```text
Request
  ↓
Context hydration
  ├─ actor/session
  ├─ selected territory
  ├─ subscriptions
  ├─ process memberships/eligibility
  ├─ explicit interests
  └─ seen/exposure state
  ↓
Parallel candidate sources
  ├─ subscribed/network
  ├─ territory agenda
  ├─ civic priority
  ├─ semantic similarity
  ├─ graph relationships
  ├─ institutional deadlines/responses
  ├─ unserved demand
  ├─ deliberative need
  └─ controlled exploration
  ↓
Hydration
  ↓
Visibility/policy prefilters
  ↓
Feature vector
  ↓
Civic score policy
  ↓
Diversity / slate optimization
  ↓
Top K
  ↓
Postfilters / canonical cluster collapse
  ↓
Explanation artifact
  ↓
Response + privacy-minimized exposure telemetry
```

## 5. Feature policy

### Green signals
- explicit territory relevance;
- explicit topic subscriptions;
- process urgency/deadline;
- unresolved public need;
- evidence quality/coverage;
- need for missing evidence/expertise;
- freshness;
- authoritative official update relevant to followed objects;
- semantic relation to explicit current query;
- source/actor diversity;
- accessibility/language fit.

### Yellow signals — research/evaluation only
- interaction affinity;
- graph centrality;
- contribution reliability;
- inferred topic preference;
- engagement predictions.

Yellow features need purpose, bias/privacy eval and cannot affect formal rights.

### Red signals
- inferred party/ideology for persuasion;
- race/religion/health/sexuality/political-opinion sensitive traits for recommendation unless a narrowly lawful accessibility/public-interest purpose is approved;
- follower count as civic authority;
- ad spend or partner status;
- anger/outrage;
- political conversion probability.

## 6. Multi-objective score

Start with a transparent constrained score rather than a heavy neural ranker:

```text
base =
  wr * territorial_relevance
+ wi * explicit_interest_relevance
+ ws * semantic_relevance
+ wp * civic_priority
+ wu * unresolved_importance
+ we * evidence_quality
+ wd * deliberative_need
+ wg * institutional_urgency
+ wf * freshness
+ wx * exploration

penalties =
  manipulation_risk
+ fatigue
+ duplicate_probability
+ overexposure

score = base - penalties
```

Then use constrained reranking for slate diversity.

Every coefficient is policy/version data. Do not hard-code it across application modules.

## 7. Future learned ranker gate

Only adopt ML/LTR ranking after:
- enough unbiased training/evaluation data exists;
- QCLC and countermetrics are stable;
- feature purpose/privacy mapping exists;
- offline replay demonstrates benefit over transparent baseline;
- explanation fidelity remains acceptable;
- bias/territorial/actor exposure audits pass;
- rollback to deterministic baseline is instant.

Candidate research models:
- LambdaMART/XGBoost ranker for explainable feature importance;
- multi-task prediction models as input features;
- transformer/slate models only at much later scale.

## 8. Ranking transparency API

Every served candidate should be able to produce a privacy-safe record like:

```json
{
  "item_id": "...",
  "ranking_policy": "civic-feed/1.4.0",
  "candidate_sources": ["territory_agenda", "semantic_related"],
  "top_reasons": [
    "Está relacionado con Vivienda en tu municipio",
    "Es una necesidad sin propuesta técnica activa",
    "El periodo de participación termina en 3 días"
  ],
  "diversity_adjustment": "source_balance",
  "visibility_policy": "public/default/3.1.0"
}
```

The explanation must be generated from actual decision features, not a model hallucinating a plausible story after the fact.

## 9. North-star alignment

The recommender is successful only if it improves QCLC and companion quality metrics without worsening abuse, polarization proxies, concentration, accessibility or institutional-response quality.
