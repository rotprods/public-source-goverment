# P0 Decision Register

P0 decisions can change civic legitimacy, identity assurance, public authority, legal effect or irreversible architecture. They are not silently resolved by implementation convenience.

| ID | Decision | Current safe default | Status | Closure evidence |
|---|---|---|---|---|
| P0-01 | Public product name/brand | repository name is working-only; UI says Civic Infrastructure | OPEN | trademark/domain/public-brand review |
| P0-02 | Operator/governance authority | repository owner controls bootstrap engineering; no claim of civic institutional authority | OPEN | governance charter + accountable operator identity |
| P0-03 | Reputation/weighted participation | reputation may not multiply formal civic ballot weight | PROVISIONAL_CONSTITUTION | approved use-case/policy + simulation + legal review if changed |
| P0-04 | Civic Priority formula | no production formula; architecture accepts a versioned policy | OPEN | methodology, golden cases, adversarial/exposure analysis, approval |
| P0-05 | Unserved-demand activation | no hard-coded threshold/timer | OPEN | policy with timing, capacity, publisher qualification, escalation/closure |
| P0-06 | Identity assurance by process | layered assertions; exact requirements resolved per process profile | OPEN | assurance matrix + threat/legal/privacy review |
| P0-07 | Durable workflow backend | outbox/worker baseline; `WorkflowEngine` port | SPIKE_REQUIRED | Postgres worker vs Temporal Rust vs Restate benchmark |
| P0-08 | Authorization policy engine | explicit Rust application authorization port | SPIKE_REQUIRED | Cedar vs OPA/other benchmark + auditability review |
| P0-09 | Outbound software license | repository public; no outbound license inferred | OPEN | governance/legal decision + third-party compatibility matrix |
| P0-10 | Regulated public ballots | excluded from V1 ordinary process engine | CLOSED_FOR_V1 | separate legal/security mandate required to reopen |

## Rules

- `OPEN`/`SPIKE_REQUIRED` decisions may have ports, types, schemas, fixtures and tests, but not politically meaningful magic constants.
- Active civic processes freeze approved policy versions; later P0 closure never retroactively mutates their semantics.
- Any proposal to change P0-03 or P0-10 receives an adversarial governance/security review before code.
- P0 closure updates `PRODUCT_CONSTITUTION.md`, relevant ADR/policy schema, `STATE.json` and release notes.

## CP01 exit

CP01 passes when all P0 decisions required for the pilot's actual process profiles are CLOSED/APPROVED. Decisions irrelevant to the pilot may remain explicitly deferred if their code paths are disabled and cannot accidentally activate.
