# ADR-0004 — Civic recommendation is not engagement optimization

Status: Accepted architectural invariant; production coefficients remain policy-blocked

## Context

X/Twitter and Reddit expose useful public patterns for candidate generation, retrieval, ranking, filtering, diversity and feedback. Their historical objectives, however, are social/content engagement objectives. Directly transplanting those objective functions into civic infrastructure would make virality and behavioral prediction masquerade as public value.

## Decision

Adopt the **pipeline decomposition**, not the social objective:

```text
context hydration
→ parallel candidate sources
→ candidate hydration
→ visibility/policy filters
→ civic feature scoring
→ constrained diversity reranking
→ selection
→ post-selection filters
→ explanation
→ exposure/evaluation telemetry
```

Safety, legality, authorization and process eligibility are separate decisions from relevance scoring.

The V1 scorer accepts only public/approved civic feature families such as:
- territorial relevance;
- explicit interest/subscription relevance;
- semantic relevance;
- approved civic priority;
- unresolved importance;
- evidence quality/coverage;
- deliberative need;
- institutional urgency;
- freshness/exploration;
- fatigue/duplicate/manipulation-risk penalties.

The core ranking feature contract intentionally contains no inferred ideology, protected-class profile, party-conversion probability, outrage score, ad spend, partner status or follower-count-as-authority feature.

## Objective

Ranking policy is evaluated against Qualified Civic Loop Completion and companion quality/countermetrics, not raw dwell/likes/shares/comments.

Interaction predictions may later be auxiliary features only if a privacy/purpose/evaluation review shows that they improve a civic objective without becoming the objective.

## Explainability

Every served recommendation must be reconstructible from the actual decision trace: candidate sources, visibility policy, component scores, rerank adjustments and reason codes. Human-facing explanations are generated from these facts, not invented post hoc by a generative model.

## Production-weight gate

No numeric ranking weight in documentation is production authority. Production ranking coefficients require a versioned `RankingPolicy`, offline replay/evaluation, concentration/manipulation review, approval and rollback path.

## Source/license boundary

- `xai-org/x-algorithm` is Apache-2.0 and may inform implementation subject to attribution/license review.
- `twitter/the-algorithm` is AGPL-3.0; architectural study only by default.
- archived Reddit code inspected uses CPAL 1.0; mathematical ideas may be independently reimplemented, but code copying requires license review.
