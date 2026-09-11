# CP01 Progress Evidence — 2026-09-11

Checkpoint: `CP01 — Product Constitution`

Status: **IN_PROGRESS / STRUCTURAL INVARIANTS ADVANCED**

This packet records what is executable/decided and what remains policy/governance authority. It does not claim CP01 passed.

## Verified implementation baseline

Current verified PR code head:

`308bed10ebf51476983c7e65ce7986c9e137167d`

GitHub Actions PR run:

- run number: `75`
- run ID: `34601567381`
- result: **all six jobs successful**

Verified jobs:
1. Lockfile integrity.
2. Contract and truth sanity.
3. Rust fmt/Clippy/tests/release build.
4. Web typecheck/production build.
5. Supply-chain and secret checks.
6. Container build smoke.

## CP01 structural decisions now encoded

### Civic action semantics

- support, signature, assessment, prioritization, advisory ballot and regulated ballot are not interchangeable actions;
- legal effect is explicit;
- active process policies are immutable/frozen by version/hash;
- regulated public elections/referenda remain outside the ordinary V1 engine.

Canonical sources:
- `PRODUCT_CONSTITUTION.md`
- `docs/architecture/PROCESS_INTEGRITY_PROFILES.md`
- `contracts/policy/civic-process-policy.schema.json`

### Actor / capability / authority separation

The authorization model explicitly separates:
- actor identity/type assertions;
- capability assignment;
- organization scope;
- territory scope;
- competence scope;
- process/resource scope;
- ownership scope;
- assurance/step-up state;
- policy/version provenance.

`actor_type != capability != institutional/legal authority`

Canonical sources:
- `docs/architecture/ACTOR_CAPABILITY_MODEL.md`
- `contracts/policy/capability-assignment.schema.json`
- `contracts/policy/authorization-decision.schema.json`

### Executable authorization baseline

`crates/policy` now contains a project-native `PolicyEngine` port and compiler-tested `StaticPolicyEngine` baseline.

Verified invariants include:
- deny by default;
- exact action + subject + scope match;
- invalid empty scoped grant does not match;
- cross-territory institutional authority is denied;
- owner-only editing requires real ownership;
- high-impact capability can require step-up auth;
- high-impact capability can require a second approver;
- authorization decision includes reason and policy provenance.

This baseline is intentionally the control implementation for CP02 Cedar/OPA benchmarking. It is not yet the final authorization-engine decision.

## Process integrity profiles

Engineering integrity profiles are now separated from legal effect:

- `IP0`: public signal/discussion;
- `IP1`: qualified advisory participation;
- `IP2`: high-impact administrative/public-resource process;
- `IP3`: regulated public election/referendum — excluded from ordinary V1 architecture.

The first real product vertical slice can be developed under IP0 without pretending that its support counts are certified resident/person ballots.

## Public-algorithm integrity decisions relevant to CP01

- social engagement/popularity is not civic authority;
- reputation does not automatically multiply formal civic vote weight;
- paid/commercial partner status is not a civic ranking or authorization input;
- inferred ideology/protected attributes are not baseline recommendation features;
- recommendation visibility/legal decisions are distinct from ranking relevance;
- ranking policy is versioned and explainable from actual decision traces.

## P0 decisions still blocking full CP01 PASS

The following remain intentionally unresolved or deferred:

1. final operator/governance authority model;
2. exact pilot process profiles beyond safe IP0 baseline;
3. exact identity-assurance requirements for each enabled IP1/IP2 process;
4. Civic Priority methodology, or explicit disablement for the pilot;
5. Unserved Demand activation policy, or explicit disablement for the pilot;
6. outbound project software license;
7. any future change to the V1 rule that reputation does not multiply formal ballot weight.

Architecture-only CP02 choices such as Cedar/OPA or Temporal/Restate are separate from political P0 semantics and may be benchmarked while CP01 remains open.

## Safe next action

1. Keep PR #1 draft while foundation review continues.
2. Benchmark the project-native policy engine against Cedar and OPA under the same civic authorization corpus.
3. Define the first pilot/development process profile explicitly as IP0 unless a real institutional partner/process requires stronger semantics.
4. Close only those P0 decisions necessary to activate a real process; keep unused high-impact paths disabled.
5. Do not encode open civic-priority, identity or governance semantics as code constants.
