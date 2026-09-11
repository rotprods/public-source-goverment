# Contributing

## Branching

- No direct pushes to `main`.
- Use short-lived `feature/`, `fix/`, `security/`, `docs/`, or approved bootstrap branches.
- Every material change lands through a PR with executable evidence.

## Before coding

Read `AGENTS.md` and the canonical read order. For complex work run the `/autoprompting` compiler. For foundation/re-bootstrap use `/empezarproyecto`.

## Definition of Done

At minimum for affected surfaces:
- formatting/lint;
- compiler/typecheck;
- tests;
- production build;
- security gates;
- E2E/accessibility for UI;
- replay/idempotency for stateful/evented changes;
- retrieval/ranking eval for search/recommender;
- state/handoff/graph persistence.

## Architecture changes

Create an ADR for:
- new database/stateful infrastructure;
- new service boundary;
- policy/authorization engine change;
- workflow engine selection;
- public API/event compatibility change;
- civic/legal-effect semantic change;
- outbound-license change;
- new sensitive-data purpose;
- major ranking objective/feature change.

## Security

Do not submit vulnerabilities in public issues. Follow the security reporting route established by repository ownership once the project publishes one. Until then contact the repository owner privately.

## Third-party code

Do not paste external code because a repository is public. Record source/license/commit and obtain compatibility approval. See `LICENSE_POLICY.md`.

## State continuity

A PR that materially changes the project should update `STATE.json`/`HANDOFF.md` and any affected checkpoint/ADR/policy/version artifacts.
