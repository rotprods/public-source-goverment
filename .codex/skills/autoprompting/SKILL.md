---
name: autoprompting
description: Compile ambiguous or complex requests into a truth-grounded, graph-aware, security-reviewed execution plan and implementation prompt. Use before multi-domain product, architecture, refactor, migration or release work.
---

# /autoprompting

## Input
Any user request that spans multiple systems, has ambiguous scope, changes product/domain semantics, or could create irreversible complexity.

## Output
A compiled execution specification that another strong agent can execute without rediscovering the mission.

## Compiler stages

1. **Intent** — actual outcome, deliverables, constraints, acceptance.
2. **Live truth** — inspect repo/code/state/contracts/tests/deployments; resolve stale docs.
3. **Source classification** — SOURCE / DECISION / DEFAULT / RESEARCH / HYPOTHESIS / BLOCKED.
4. **Graphify** — scoped nodes/edges/dependencies/owners/evidence.
5. **COS20D** — impact map across identity, execution, data, dependency, time, authority, capability, policy/security, causality, evidence, state, projections, communication, knowledge/AI, observability, economics, failure, recovery, learning, governance.
6. **Adversarial review** — security, privacy, civic integrity, abuse, failure/recovery, cost/complexity.
7. **Architecture decision** — alternatives, tradeoffs, reversible choice, ADR trigger.
8. **Dependency-ordered execution plan** — contract → migration → implementation → tests → rollout → persistence.
9. **Verification matrix** — exact commands/tests/evals and pass conditions.
10. **Persistence** — state, handoff, graph, checkpoint, evidence, PR.

## Prompt shape

Compile into:

```text
SYSTEM ROLE
MISSION
CURRENT TRUTH
SOURCE/DECISION BOUNDARIES
NORTH STAR
CONSTITUTIONAL INVARIANTS
OBJECTIVE
NON-GOALS
SCOPE
ARCHITECTURAL CONTEXT
DATA/SECURITY CLASSIFICATION
GRAPH IMPACT
COS20D IMPACT
WORKSTREAMS
DEPENDENCIES
IMPLEMENTATION STEPS
TEST/EVAL MATRIX
ADVERSARIAL GAUNTLET
ROLLBACK/RECOVERY
DEFINITION OF DONE
PERSISTENCE REQUIREMENTS
OUTPUT CONTRACT
```

## Critical behavior

- Do not ask the user to repeat facts discoverable from the repository or tools.
- Do not collapse uncertainty into guessed truth.
- Do not let a subproblem replace the mission.
- Do not optimize for number of artifacts.
- Prefer a working vertical slice over horizontal scaffolding after foundations exist.
- When the user requests 'maximum', maximize correctness/coverage, not architecture entropy.
- Always include anti-overengineering review before commit.

## Project-specific extension

For recommender/search/AI work additionally compile:
- candidate sources;
- filters;
- feature/vector sources;
- score components;
- diversity constraints;
- explanation path;
- offline golden evals;
- countermetrics;
- manipulation/adversarial cases;
- privacy constraints;
- rollout/experiment policy.

For civic process work additionally compile:
- legal effect;
- competent authority;
- actor/eligibility model;
- identity assurance;
- decision method;
- policy snapshot;
- audit/appeal path;
- P0/P1/P2/P3 integrity profile.

The full master prompt lives at `METAPROMPT.md`; this skill is its invocation/compiler protocol.
