# PRODUCT CONSTITUTION — Baseline v0.1

Status: `DRAFT FOR CP01 APPROVAL`.

This document defines product invariants that software must not silently reinterpret. It is not legal advice and does not itself grant legal effect to any civic process.

## Article 1 — Product purpose

The platform exists to convert public needs into understandable, evidence-backed, technically assessable and institutionally traceable action while preserving pluralism, accountability and representative-democratic institutions.

It does not claim to replace elected institutions or automatically make platform popularity legally binding.

## Article 2 — Equal public rules

Citizens, associations, professionals, parties, public institutions and other authorized actor classes participate under published rules appropriate to their role.

Special institutional capabilities must derive from competence/authority, not customer status or algorithmic privilege.

`commercial_partner != civic_privilege`

## Article 3 — Civic actions are semantically distinct

The product never uses one ambiguous 'vote' primitive for every action.

Minimum action classes:
- `SUPPORT` — expresses support for a need/object;
- `SIGNATURE` — formal/qualified signature where process rules define it;
- `ASSESSMENT` — expert/structured evaluation;
- `PRIORITIZATION` — chooses relative priority under a method;
- `ADVISORY_BALLOT` — advisory choice among options;
- `FORMAL_PROCESS_INPUT` — input recognized by an external administrative/legal process;
- `REGULATED_BALLOT` — out-of-V1 high-assurance integration category.

Counts across different classes are not interchangeable.

## Article 4 — Legal effect must be explicit

Every CivicProcess publishes one legal-effect class before participation begins:

- `INFORMATIVE`
- `SIGNAL`
- `ADVISORY`
- `ADMINISTRATIVE_INPUT`
- `FORMAL_CONSULTATION`
- `INTERNAL_BINDING`
- `REGULATED_PUBLIC_PROCESS`

UI must explain in plain language what participation can and cannot cause.

## Article 5 — Rules freeze when a process starts

A process snapshot binds:
- process template version;
- eligibility policy;
- identity assurance level;
- decision method;
- threshold/quorum policy;
- ranking/discovery policy where it affects process visibility materially;
- moderation/publication policy;
- competence registry version;
- relevant feature flags.

Runtime deployments may change, but an active process does not silently change its rules.

## Article 6 — Reputation and civic rights are separate

Reputation/contribution history may support anti-abuse, expertise discovery, moderation queues or capability qualification under approved policies.

It does not automatically multiply a person's formal ballot weight.

Any weighted-participation mechanism must identify:
- purpose;
- scope;
- weighting formula;
- justification;
- appeal path;
- legal effect;
- simulation/adversarial analysis;
- approving authority.

Until approved, formal civic ballots default to equal eligible-person treatment within the process rules.

## Article 7 — Identity is layered

These are independent states:

`authenticated`
`unique-person assertion`
`age assertion`
`residency assertion`
`professional credential`
`role/office credential`
`publisher capability`
`public identity disclosure`

The platform minimizes private identity and prefers signed assertions/credentials over retaining raw ID/biometric artifacts.

## Article 8 — Citizen language and administrative truth are separate layers

Users enter through plain-language civic topics/problems. Internally, a versioned competence registry maps objects toward territorial/legal/institutional responsibility.

AI may suggest routing. Durable routing stores source/model/rule/human provenance and remains correctable.

## Article 9 — Need canonicalization and duplicates

The product should reduce fragmentation without erasing distinct experiences.

Automatic duplicate detection only suggests/assists. Merging/clustering decisions are explainable, reversible or appealable under policy, and preserve original submissions.

## Article 10 — Evidence and deliberation

The platform distinguishes:
- source document;
- evidence artifact;
- factual claim;
- argument;
- opinion/value judgment;
- technical assessment.

Citations/provenance survive summaries and proposal versions.

Minority positions may not be hidden merely to produce cleaner consensus summaries.

## Article 11 — Technical proposals are versioned

A proposal/MAT-equivalent is one versioned artifact with layers for fast comprehension, thesis/evidence and technical implementation detail.

The exact version evaluated or participated on is immutable after the relevant process begins. Amendments create new versions.

## Article 12 — Unserved demand

The system may identify needs with insufficient technical response and activate a structured unserved-demand workflow.

Activation timing, publisher qualification, capacity limits, escalation and closure are **BLOCKED policy semantics until approved**. They are versioned policy data, never hard-coded assumptions.

## Article 13 — Civic priority is not engagement popularity

A territory may publish a Civic Priority view. Its formula must be public/versioned and should model meaningful civic demand rather than raw social virality.

The exact formula remains a CP01/CP02 policy decision. No hidden engagement score is permitted to masquerade as public importance.

## Article 14 — Recommendation objective

The recommended feed/search/discovery system optimizes civic usefulness/QCLC subject to safety, diversity and transparency constraints.

Forbidden objectives/signals:
- ideological persuasion/party conversion;
- outrage as a positive objective;
- paid political boost in V1;
- follower count as civic authority;
- customer/partner status;
- protected/sensitive attributes without a separately approved lawful purpose.

Every recommendation exposes a truthful reason derived from actual ranking decisions.

## Article 15 — AI role

AI is a librarian/copilot. Allowed tasks include retrieval, duplicate suggestion, clustering, routing suggestion, summarization with citations, argument mapping, translation/accessibility, drafting assistance and triage.

AI may not autonomously:
- grant/deny civic rights;
- declare institutional legal competence as unquestionable truth;
- decide political winners;
- fabricate consensus;
- secretly personalize political persuasion;
- change policy or vote weight.

Every persistent AI artifact has provenance/evaluation metadata.

## Article 16 — Institutional accountability

Authorized institutions receive tools to:
- validate competence/admissibility;
- request information;
- publish official responses;
- create commitments;
- assign owners/milestones/budget references where applicable;
- publish outcomes.

Historical official responses/commitments are attributable and cannot be silently rewritten.

## Article 17 — Moderation and appeals

Moderation policy is public/versioned. High-impact enforcement is attributable, reason-coded and appealable.

Party/institution accounts have the same content rules plus stronger authenticity/disclosure requirements, not softer enforcement.

Mass reporting is not proof of violation.

## Article 18 — Public audit

The platform exposes enough public provenance to reconstruct:
- what object/process existed;
- which version/rules applied;
- who had authority to respond;
- what decision/result occurred;
- what commitment/outcome followed;
- which public sources/evidence supported the record.

Private identity/security data remains protected.

## Article 19 — Regulated ballots are separate

P3 regulated public elections/referenda require specialized legal mandate, cryptographic/security assurance, independent evaluation and operational governance. The V1 ordinary civic-process engine is not marketed as an election system.

## Article 20 — Amendment process

Changing an Article requires:
- explicit proposal;
- impact analysis;
- ADR/policy diff;
- approving authority;
- effective version/date;
- migration/active-process compatibility plan;
- public changelog where applicable.
