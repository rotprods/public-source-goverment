# MoviMurcia ↔ Civic Infrastructure — Architecture Contrast

## Executive conclusion

MoviMurcia demonstrates an unusually strong **evidence-first project operating system**, but also demonstrates why this repository must keep **runtime truth ahead of documentation volume**.

The civic platform should inherit MoviMurcia's traceability discipline and reject its observed scope-illusion failure mode.

## What MoviMurcia proves useful

### 1. Evidence-first source of truth
MoviMurcia explicitly separates implemented runtime from target architecture, imported screens and generated planning. That discipline is mandatory here.

Adopt:
- source-of-truth documents;
- machine-readable goal/state manifests;
- route/endpoint/component coverage artifacts;
- decision logs and contradiction registries;
- security gates before sensitive implementation;
- handoff/evidence ledgers;
- graphify navigation;
- acceptance matrices and snapshots.

### 2. Security is a project axis
MoviMurcia correctly treats identity, ticketing/payment, location, validation, provider integrations, DB writes and production deploy as gated surfaces. The civic platform broadens this to civic rights, political-opinion data, ranking, moderation and institutional authority.

### 3. Canonical design evidence is valuable
MoviMurcia preserves reference screens and requires browser evidence before claiming visual completion. Use the same pattern for Figma/reference UI: design evidence informs runtime but never equals runtime.

## What must be refactored

### 1. Documentation/runtime imbalance
The inspected MoviMurcia reality map reported thousands of files and a very large evidence/documentation corpus while the runtime remained partial. This creates an illusion that planning completion equals product completion.

Rule here:
- each checkpoint has a `runtime_evidence_ratio` requirement;
- architecture docs may be extensive at bootstrap, but subsequent waves prioritize vertical executable slices;
- generated documentation must identify which executable artifact/test it derives from;
- generated docs have freshness checks or are deleted/deprecated.

### 2. Too many instruction canons
MoviMurcia contains multiple agent instruction variants (`AGENTS.md`, `AGENTS 2.md`, `CLAUDE.md`, `CODEX.md`, etc.). This is dangerous for autonomous development.

Rule here:
- `AGENTS.md` is canonical;
- `CLAUDE.md` is an adapter;
- skills are shared workflow specs;
- no second agent file may redefine project truth.

### 3. Structural validators cannot become pseudo-compilers
MoviMurcia has many useful generated `check:*` scripts, but the deeper lesson is that a repository can become rich in structural checks before final runtime exists.

Rule here:

`compiler/typechecker + production build + executable tests > structural manifest checks`.

A generated route manifest is valuable only after the app actually builds and the route is exercised.

### 4. Bootstrap oracle must not fossilize
MoviMurcia used a dependency-light ESM API oracle to stabilize contracts before the target NestJS backend. That is useful as a migration pattern, but this greenfield repo does not need a temporary JS backend.

Rule here:
- contracts first;
- executable Rust API from the first bootstrap;
- stubs must use production domain boundaries and be obviously incomplete.

### 5. Establish Git baseline immediately
MoviMurcia's inspected project state explicitly flagged lack of a committed baseline as a recoverability/review risk. This repo therefore bootstraps on a branch immediately and will use PR review before main.

## Architecture mapping

| MoviMurcia pattern | Civic equivalent | V1 decision |
|---|---|---|
| mobility route manifest | civic process/route manifest | machine-readable, derived from executable contracts |
| provider adapters | institution/identity/model/search adapters | ports at real trust boundaries |
| ticketing state machine | civic process/unserved-demand workflows | durable workflow + frozen policies |
| PostGIS mobility geometry | territory/jurisdiction geometry | PostGIS authoritative projection |
| realtime movements | civic events/workflow transitions | outbox + projectors, stream bus at scale |
| auth/eligibility | identity assertions + process eligibility | separate layers; no generic verified boolean |
| security command gates | civic policy/authz gates | server-side ABAC/ReBAC + audit receipts |
| screen reference corpus | Figma/canonical UI evidence | implementation verified with Playwright |
| graphify | product/domain/dependency knowledge graph | canonical navigation context |
| handoff/progress docs | COS state/event/handoff | mandatory per material wave |

## New requirements unique to civic infrastructure

MoviMurcia does not solve these and therefore cannot be copied architecturally without extension:

- legal effect of civic processes;
- institutional competence and authority provenance;
- advisory vs binding semantics;
- policy snapshots per active process;
- political neutrality and actor symmetry;
- abuse by parties/governments/opposition/organized groups;
- ranking transparency and civic-utility objective;
- sensitive political-opinion processing;
- public audit/provenance;
- versioned technical proposals and evidence;
- future regulated-ballot boundary;
- federation/public-record portability.

## Quantitative engineering guardrails derived from the contrast

Track in CI/project state:

- executable source files vs generated docs/evidence;
- real route E2E coverage vs planned route count;
- real endpoint integration tests vs OpenAPI paths;
- real DB migrations vs schema-design documents;
- build success on clean checkout;
- stale generated-document count;
- unowned policy constants;
- unresolved architecture blockers;
- code paths with no authz test on protected objects.

## Final principle

MoviMurcia teaches us to preserve **evidence density** while preventing **documentation entropy**. The civic repository must be easier for an agent to prove than to describe.
