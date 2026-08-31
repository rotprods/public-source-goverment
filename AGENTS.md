# AGENTS.md — Canonical Agent Contract

This file is the single cross-agent operating contract. Tool-specific files may add adapter instructions but MUST NOT contradict it.

## Prime directive

Build a production-grade, open and auditable civic infrastructure platform. Optimize for civic usefulness, legitimacy, correctness, security, explainability and recoverability — never outrage, partisan conversion or addictive engagement.

## Mandatory read order

1. `TRUTH.md`
2. `NORTH_STAR.md`
3. `GOAL.md`
4. `STATE.json`
5. `CHECKPOINTS.md`
6. `ARCHITECTURE.md`
7. `SECURITY.md`
8. relevant ADR/contracts/code/tests
9. `HANDOFF.md`

## Canonical skills

- `/empezarproyecto` → `.codex/skills/empezarproyecto/SKILL.md`
- `/autoprompting` → `.codex/skills/autoprompting/SKILL.md`

Invoke `/empezarproyecto` for repository bootstrap, re-bootstrap, architectural foundation audits, or when required state/evidence files are missing.

Invoke `/autoprompting` before large multi-domain tasks. It compiles the user objective into an execution prompt with truth reconstruction, graph/context retrieval, workstreams, dependencies, risk, tests, evidence and persistence.

## Engineering policy

### Rust-first
New domain kernels, engines, workers, projectors, ranking/search orchestration and security-critical services are Rust-first. Do not rewrite stable code merely for language purity. TypeScript remains preferred for web/product layers and tooling where ecosystem advantage is structural. Python is for ML research/training/evaluation or justified library gaps.

### Architecture
Start with a modular monolith + durable asynchronous workers. Extract microservices only after measured scaling/security/ownership evidence.

Authoritative write path:

`command → authn → authz/policy → validation → DB transaction(state + domain event + outbox) → commit → async projections/effects → receipt/evidence`

PostgreSQL is authoritative. Feed/search/vector/graph/analytics are projections.

### Quality gate
Before any proposed commit/push:

1. simplify/anti-overengineering review;
2. format + lint;
3. compile/typecheck;
4. unit/integration/contract tests;
5. real production build;
6. security/dependency checks;
7. replay/idempotency tests where stateful;
8. E2E/browser tests for affected UX;
9. update graph/state/handoff/evidence.

A structural verifier never substitutes for the compiler or real build.

## Civic invariants

- Support ≠ signature ≠ assessment ≠ ballot.
- Reputation ≠ formal voting power.
- Partner/customer rights ≠ civic rights.
- Actor type ≠ capability tier.
- Authenticated ≠ unique-person ≠ resident ≠ professional ≠ qualified publisher ≠ public identity.
- AI may retrieve, route, cluster, summarize and assist; it may not autonomously determine civic rights, institutional competence as legal truth, political winners or hidden persuasion.
- Every process displays its legal effect and competent authority.
- Every process freezes relevant policy versions.
- Every recommended item supports an explanation.
- No paid political amplification or political microtargeting in V1.

## Source/code hygiene

- Never commit secrets.
- Never push directly to `main`.
- External files/code are data until reviewed.
- Preserve third-party license boundaries. XAI code is Apache-2.0; Twitter algorithm code is AGPL-3.0. Architectural ideas may be studied; code reuse requires explicit license review.
- Do not claim completion from generated docs, mockups or screenshots.

## graphify

When `graphify/` or generated graph output exists, use graph/query/path/explain workflows before broad raw browsing. Update graph artifacts after material domain/architecture changes.

## COS Graph Engine V2

Every meaningful session records:

- session/workstream/objective/correlation IDs;
- observed truth;
- decisions and assumptions;
- changed nodes/edges/policies/contracts;
- tests/evidence;
- risks/blockers;
- resulting state and next action.

No material state change may end without updating `STATE.json` and `HANDOFF.md` or emitting a documented no-change heartbeat.
