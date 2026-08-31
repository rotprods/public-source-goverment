---
name: empezarproyecto
description: Canonical repository bootstrap and re-bootstrap protocol. Use for new projects, missing foundations, major architecture resets, or when the user invokes /empezarproyecto.
---

# /empezarproyecto

## Purpose

Create the minimum durable operating system that lets humans and coding agents develop a real product without relying on chat memory, hidden assumptions or unverified plans.

## Step 1 — Observe before mutate

Inspect:
- repository metadata/default branch;
- existing files, commits, branches and CI;
- package/build tools;
- source/runtime code;
- tests;
- deployments if accessible;
- licenses;
- attached/source documents relevant to product truth.

Never overwrite an existing canonical file without reading it and determining whether to update, preserve or supersede it.

## Step 2 — Establish Git safety

- never work directly on `main`;
- create a descriptive bootstrap branch;
- establish the first recoverable baseline early;
- no secrets;
- add `.gitignore`, `.gitattributes`, `.editorconfig`;
- add CODEOWNERS and PR template;
- configure required CI before broad implementation.

## Step 3 — Create canonical state layer

Required root artifacts:

```text
README.md
AGENTS.md
CLAUDE.md
NORTH_STAR.md
GOAL.md
GOAL_STATE.json
TRUTH.md
ARCHITECTURE.md
SECURITY.md
CHECKPOINTS.md
STATE.json
HANDOFF.md
METAPROMPT.md
```

No duplicate truth file may silently diverge. `AGENTS.md` is cross-agent canon; adapter files reference it.

## Step 4 — Create ignore/config baseline

As applicable:

```text
.gitignore
.gitattributes
.editorconfig
.dockerignore
.vercelignore
.agentignore
.agentsignore
.claudeignore
.codexignore
.npmrc
.env.example
rust-toolchain.toml
clippy.toml
deny.toml
renovate.json
```

Ignore files are not security boundaries; sensitive files must never be created/committed in the first place.

## Step 5 — Establish language/build policy

Default for this project:
- Rust workspace for critical core/services.
- pnpm workspace for TS product apps/tooling.
- Python isolated to `ml/` when justified.
- one lockfile per ecosystem.
- exact/controlled toolchain versions.

Required executable bootstrap:

```text
Cargo.toml
package.json
pnpm-workspace.yaml
justfile
compose.yaml
```

Create at least one minimal Rust compile target and one frontend workspace package before declaring the bootstrap executable.

## Step 6 — Repository topology

Create only directories with current ownership/purpose. Default topology is defined in `ARCHITECTURE.md`.

Avoid empty decorative microservice directories. If a domain/service is not executable yet, document it in architecture/roadmap rather than creating dozens of empty folders.

## Step 7 — Contracts first

Create/version:
- OpenAPI contract;
- domain-event envelope;
- error model;
- policy/version model;
- database migration conventions;
- observability correlation conventions.

## Step 8 — graphify

Initialize:

```text
graphify/README.md
graphify/ontology.yaml
```

Graph at least:
- goals;
- actors;
- domains;
- services/modules;
- data stores/projections;
- policies;
- APIs/events;
- checkpoints;
- risks/evidence.

Graph output is navigation/context, never source-of-truth replacement.

## Step 9 — COS Graph Engine V2

Initialize `cos-graph-engine/V2/` with:
- COS20D dimensions;
- session/event ledger convention;
- state schema;
- change/evidence protocol;
- recovery protocol.

Every future material session must leave persistent state.

## Step 10 — Security baseline

Create `SECURITY.md`, threat-model directory and CI gates before sensitive product code.

For this project include identity separation, civic authorization, abuse/Sybil, ranking manipulation, moderation capture, AI prompt/retrieval injection, evidence upload, supply chain, audit integrity and sovereign deployment risks.

## Step 11 — CI must prove executable reality

At minimum CI runs:
- Rust fmt/clippy/check/test/build;
- TS install/lint/typecheck/test/build when app exists;
- contract/schema validation;
- secret scan;
- dependency/license checks;
- container/build scan when container exists.

Never create fifty structural `check:*` scripts while the compiler/build is absent. This is a direct lesson from prior project experience.

## Step 12 — Define checkpoints and first vertical slice

Create evidence-gated checkpoints. The first product slice must cross the real stack end to end rather than create horizontal infrastructure forever.

For this project: plain-language concern → competence/topic suggestion → duplicate retrieval → need creation → support → public read model.

## Step 13 — Persist and review

Update state/handoff, create a PR and report:
- branch;
- files created/updated;
- executable checks run;
- unresolved P0 decisions;
- next checkpoint.

## Failure conditions

The skill is NOT complete if:
- work lives only in chat;
- no branch/baseline exists;
- state/handoff are missing;
- architecture has no test/rollback path;
- CI checks only formatting/docs;
- secrets are present;
- product-name or political-policy guesses are silently frozen;
- docs claim runtime that does not exist.
