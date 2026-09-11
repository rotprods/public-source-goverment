# Civic Process Integrity Profiles

Status: `CP01 DRAFT — engineering security taxonomy, not a legal classification`.

## 1. Why profiles exist

One generic `vote()` endpoint would collapse very different acts into one unsafe abstraction. A low-stakes preference signal and a legally regulated ballot do not share the same requirements for identity, uniqueness, secrecy, audit, coercion resistance or certification.

The platform therefore classifies processes by **integrity/security profile** in addition to the explicit `legal_effect` in `PRODUCT_CONSTITUTION.md`.

Profile is an engineering control floor. Jurisdiction/process rules may require stronger controls.

## 2. Profiles

### IP0 — Public signal / discussion

Typical uses:
- support a Need;
- evidence/helpfulness rating;
- public prioritization signal with no formal legal effect;
- ordinary discussion/deliberation.

Default engineering posture:
- ordinary authenticated account may be sufficient for mutation;
- uniqueness of a human person is not necessarily claimed;
- public/pseudonymous participation may be allowed;
- abuse/rate limits and Sybil-aware analytics;
- action semantics shown as `SIGNAL`/informative;
- reversible support where policy permits.

Never present IP0 counts as certified resident/person counts unless a stronger assertion was actually required and frozen by policy.

### IP1 — Qualified advisory participation

Typical uses:
- resident advisory consultation;
- unique-person survey/ballot with advisory effect;
- structured expert assessment;
- local public consultation where product policy requires participant qualification.

Control floor:
- authenticated participant;
- process-specific eligibility snapshot;
- unique-person and/or residency assertion when the process claims those properties;
- explicit opening/closing window;
- immutable target/version;
- idempotent/unique active participation constraints;
- frozen process/policy/competence versions;
- receipt and reproducible result computation;
- correction/revocation rules explicit before start;
- audit and appeal/administrative-correction path.

IP1 does not imply legal binding effect.

### IP2 — High-impact administrative/public-resource process

Typical uses:
- participatory budgeting allocation;
- formal administrative-input workflow;
- process whose result can materially influence public resources or official prioritization;
- high-impact internal institutional decision supported by the platform.

Control floor includes IP1 plus:
- higher identity/eligibility assurance appropriate to the process;
- stronger separation of duties;
- step-up auth for officials/process operators;
- formally approved decision/threshold policy;
- deterministic/reproducible tally/allocation;
- signed or tamper-evident result/receipt artifacts where appropriate;
- independent audit capability;
- documented incident/freeze/recovery procedure;
- load/DoS/adversarial testing around close/deadline;
- explicit legal/governance approval before activation.

### IP3 — Regulated public election/referendum

Examples:
- binding public election;
- regulated referendum where election-grade secrecy, coercion resistance and legal certification apply.

**Excluded from the ordinary V1 process engine.**

Required before any implementation claim:
- explicit legal mandate;
- independent election/security architecture;
- end-to-end verifiability/secrecy/coercion threat model as required;
- certified/evaluated identity and voting components;
- external auditors/observers;
- operational ceremonies/key management;
- jurisdiction-specific accessibility/contingency requirements;
- independent approval that the platform/integration is fit for that regulated use.

IP3 is not reached by turning an IP1 feature flag on.

## 3. Profile vs legal effect

Integrity and legal effect are separate axes.

Example:
- an `ADVISORY` resident consultation may be IP1;
- an `ADMINISTRATIVE_INPUT` budget process may be IP2;
- an `INTERNAL_BINDING` organizational process could be IP1 or IP2 depending on impact;
- `REGULATED_PUBLIC_PROCESS` requires a dedicated IP3 architecture unless the authoritative legal/security assessment says otherwise.

The process record freezes both the legal-effect metadata and the applicable security/eligibility policy versions.

## 4. Participation semantics

A process declares the exact allowed actions:

| Action | Meaning | Typical profile |
|---|---|---|
| `SUPPORT` | non-ballot support signal | IP0/IP1 |
| `SIGNATURE` | qualified support/signature under explicit rules | IP1/IP2 |
| `ASSESSMENT` | structured expert/public evaluation | IP0/IP1/IP2 |
| `PRIORITIZATION` | ordered/allocated preference under a method | IP1/IP2 |
| `ADVISORY_BALLOT` | choice among options, advisory | IP1/IP2 |
| `FORMAL_PROCESS_INPUT` | input into recognized admin process | IP2 |
| `REGULATED_BALLOT` | regulated election/referendum participation | IP3 only |

No endpoint/UI label should use the generic word “vote” when a more precise action type exists.

## 5. Identity assurance dimensions

Do not express assurance as one scalar `verified` flag. A process may independently require:
- authenticated session;
- phishing-resistant auth/passkey;
- unique-person assertion;
- minimum-age assertion;
- residence in territory/version;
- professional qualification;
- organization/office membership;
- elevated official/operator assurance;
- public identity disclosure — independent from private eligibility proof.

The process template selects the minimum necessary assertions.

## 6. Pilot recommendation

For the first real territorial pilot:

**Enable:**
- IP0 Need support/evidence/deliberation;
- selected IP1 advisory consultation only after identity/eligibility path is implemented and reviewed;
- institutional responses/commitments as accountable administrative records.

**Disable by default:**
- IP2 public-resource allocation unless a specific institutional pilot partner and process policy are approved;
- all IP3 functionality.

This creates a useful civic product without prematurely claiming election-grade or binding authority.

## 7. Required process snapshot

At start, hash/freeze at minimum:
- process template ID/version;
- integrity profile;
- legal effect;
- competent authority/registry version;
- eligibility policy;
- identity-assurance requirements;
- participation action types;
- target objects and exact immutable versions;
- decision/tally/allocation method;
- threshold/quorum policy if any;
- relevant visibility/moderation policy;
- relevant ranking/discovery policy if process exposure can affect integrity materially;
- correction/revocation rules;
- start/end times/timezone;
- feature flags.

## 8. Result evidence

A completed process produces an evidence packet sufficient for its profile:
- frozen snapshot hash;
- eligible/participating aggregate counts at appropriate privacy level;
- exact target/version IDs;
- deterministic method/version;
- result artifact/hash;
- anomaly/incidence summary;
- corrections/appeals;
- authority approval/publication references;
- software/schema/release version used to compute the result.

Do not expose secret/private voter identity merely for public verifiability.

## 9. CP01 impact

This taxonomy closes the architecture question “are all votes the same?” with **no**. It does not close jurisdiction-specific identity, legal effect or operator-authority P0 decisions. Those remain process/pilot inputs and must be explicitly approved before enabling IP1/IP2 paths.
