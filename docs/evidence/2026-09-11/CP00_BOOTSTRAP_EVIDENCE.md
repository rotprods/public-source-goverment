# CP00 Bootstrap Evidence — 2026-09-11

Checkpoint: `CP00 — Repository truth`

Status: **PASS ON BRANCH / PENDING REVIEW+MERGE**

Verified implementation head:

`4c32f5deb0e999e709265d8b63e61dd0bfda5dd6`

Verified GitHub Actions run:

- workflow: `ci`
- run: `34600057502` / run number 43
- branch: `bootstrap/empezarproyecto-v1`
- result: every job successful

## Executable evidence

### Lockfile integrity — PASS

- `cargo metadata --locked --no-deps --format-version 1`
- `pnpm install --lockfile-only --frozen-lockfile`

CI no longer mutates the branch to repair lockfiles. Drift is a failure.

### Contract and truth sanity — PASS

- `node scripts/check-contracts.mjs`
- parses JSON contracts and machine-readable state;
- asserts truthful bootstrap OpenAPI/runtime parity for `/healthz`.

### Rust compile and tests — PASS

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `cargo test --locked --workspace`
- `cargo build --locked --workspace --release`

The tested workspace includes:
- `civic-domain`;
- `civic-ranking`;
- `civic-search`;
- `civic-api`.

The initial project-native public-algorithm primitives are therefore compiler/test-backed rather than documentation-only:
- transparent policy-driven civic scorer;
- visibility decision separated from relevance score;
- Reciprocal Rank Fusion;
- Wilson lower-bound quality/confidence primitive;
- explicit domain distinction between support and advisory ballot semantics.

### Web typecheck and production build — PASS

- `pnpm install --frozen-lockfile`
- `pnpm -r typecheck`
- `pnpm --filter @civic/web build`

The current web surface is intentionally a truthful bootstrap shell, not a claim that the final product UI exists.

### Supply chain and secrets — PASS

- locked Cargo metadata;
- `cargo audit`;
- `cargo deny check`;
- frozen pnpm lockfile;
- `pnpm audit --audit-level high`;
- Gitleaks secret scan.

### Container builds — PASS

- PostgreSQL/PostGIS/pgvector development image;
- locked Rust API OCI image.

## CP00 canonical artifacts present

- `README.md`
- `AGENTS.md`
- `CLAUDE.md`
- `NORTH_STAR.md`
- `GOAL.md`
- `GOAL_STATE.json`
- `TRUTH.md`
- `ARCHITECTURE.md`
- `PRODUCT_CONSTITUTION.md`
- `SECURITY.md`
- `CHECKPOINTS.md`
- `STATE.json`
- `HANDOFF.md`
- `METAPROMPT.md`
- `LICENSE_POLICY.md`
- source/license registry;
- ADRs;
- threat model;
- privacy/data-governance model;
- graphify ontology;
- COS Graph Engine V2 20D schema;
- Docker/CI/toolchain/lockfiles;
- initial Rust/API/web/data executable foundation.

## Source/research provenance established

Architectural research is explicitly separated from project truth:

- MoviMurcia — evidence-first/project-operating-system contrast;
- `xai-org/x-algorithm` — Apache-2.0 public Rust recommender architecture research;
- `twitter/the-algorithm` — AGPL-3.0 architecture research by default;
- `reddit-archive/reddit` — CPAL-1.0 archive; mathematical techniques independently reimplemented;
- pgvector/Qdrant/ClickHouse/workflow/federation sources tracked as research or scale gates.

## CP00 residuals that do NOT block CP00

These intentionally move to later gates:

- public product naming;
- operator/governance authority;
- exact civic-priority policy;
- identity-assurance requirements per process;
- unserved-demand policy;
- outbound project license;
- Cedar-vs-OPA authorization-engine decision;
- Postgres-vs-Temporal-vs-Restate workflow-engine decision;
- final vector/search scale decisions;
- final product UI;
- production deployment/security certification.

They remain explicit P0/CP01/CP02 work and must not be guessed in runtime code.

## CP00 conclusion

The repository now has a recoverable, executable, security-aware and agent-operable baseline. CP00 is considered passed on the bootstrap branch. Merge approval is a repository-governance action, not proof needed for the technical checkpoint itself.
