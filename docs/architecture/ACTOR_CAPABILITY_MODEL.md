# Actor, Capability & Authority Model

Status: `CP01 DRAFT — structural model accepted; concrete pilot assignments require approval`.

## 1. Principle

The platform does not implement a global `role -> power` shortcut.

Three independent concepts are evaluated for every protected action:

```text
WHO ARE YOU?        → Actor / identity assertions
WHAT MAY YOU DO?    → Capability assignment
WHERE/ON WHAT?      → Territory + organization + civic object/process scope
WHY IS IT ALLOWED?  → Policy version + competence/authority provenance
```

`actor_type != capability != legal/institutional authority`.

A public institution does not gain moderation power merely because it is an institution. A commercial partner does not gain civic priority. A professional credential does not create formal voting weight.

## 2. Actor types

Actor type is descriptive context. It is not the authorization decision.

| Actor type | Meaning | Examples of possible capabilities | Capabilities never implied automatically |
|---|---|---|---|
| `PERSON` | natural person using the civic product | create/support Need, submit evidence, participate when eligible | institutional response, policy admin, moderation |
| `CIVIL_SOCIETY_ORG` | association/NGO/community organization | publish organization profile, proposals/evidence under policy | extra ballot weight, official government authority |
| `PROFESSIONAL` | person with a verified professional assertion | scoped technical assessment | formal civic authority outside explicit process policy |
| `PUBLIC_INSTITUTION` | verified public body | receive/triage cases, publish official responses, commitments | platform policy admin, moderation bypass, ranking boost |
| `ELECTED_OR_APPOINTED_OFFICE` | office-holder/representative assertion | official communication within competence | hidden privilege in public recommendation/participation |
| `ACADEMIC_RESEARCH_ORG` | research/academic actor | evidence, public research, approved audit access | unrestricted private-data access |
| `INDEPENDENT_AUDITOR` | explicitly appointed/qualified audit actor | scoped read/audit/export | mutation of civic outcomes |
| `PLATFORM_OPERATOR` | technical/operational actor | service operation, security response under scoped policy | invisible civic-policy override |
| `MODERATOR` | moderation capability holder | content/case enforcement under policy | institutional/civic decision authority |
| `SYSTEM` | non-human service principal | tightly scoped technical actions | human civic participation |

An actor may hold multiple actor-type assertions over time. Assertions are versioned/revocable.

## 3. Capability families

### Citizen/public capabilities

- `need.create`
- `need.edit_own_draft`
- `need.publish`
- `need.support`
- `need.withdraw_support`
- `evidence.submit`
- `claim.create`
- `argument.create`
- `proposal.create`
- `proposal.publish_version`
- `process.participate`
- `process.view_receipt`
- `moderation.report`
- `appeal.create`
- `subscription.manage`

### Qualified/expert capabilities

- `assessment.submit`
- `assessment.publish`
- `expertise.profile_publish`
- `proposal.technical_review`

These require a process/policy-defined credential when used as a formally qualified assessment. A person may still express ordinary public opinion without professional qualification.

### Institutional capabilities

- `case.receive`
- `case.claim`
- `case.assign`
- `case.competence_decide`
- `case.admissibility_decide`
- `case.request_information`
- `response.publish_official`
- `commitment.create`
- `commitment.update_status`
- `outcome.publish`

Each institutional capability is scoped by organization, territory, competence registry and sometimes process.

### Moderation/appeal capabilities

- `moderation.case_view`
- `moderation.enforce`
- `moderation.restore`
- `appeal.review`
- `appeal.decide`

High-impact enforcement and appeal review should be separable duties for appropriate process profiles.

### Policy/governance capabilities

- `policy.draft`
- `policy.approve`
- `policy.activate`
- `process_template.manage`
- `competence_registry.manage`
- `ranking_policy.manage`
- `feature_flag.manage_civic`
- `release.approve_production`

No single ordinary operator session should silently both author and activate high-impact civic policies in production.

### Security/platform capabilities

- `security.incident_manage`
- `security.session_revoke`
- `security.secret_rotate`
- `deployment.execute`
- `backup.restore`
- `audit.security_read`

Security emergency powers are time-bounded/audited and do not imply civic-content authority.

## 4. Authorization decision model

Protected server actions evaluate an `AuthorizationContext` conceptually containing:

```text
principal / actor_id
identity assertions + assurance
actor type assertions
capability assignments
organization memberships
territory scope
competence scope
resource owner/visibility
process_id + frozen policy refs
requested action
request time
risk/session assurance
```

Decision output:

```text
ALLOW | DENY | REQUIRE_STEP_UP | REQUIRE_SECOND_APPROVER
reason_codes[]
policy_version
matched capability/scope
expires_at? / obligations[]
```

A boolean without reason/policy provenance is insufficient for high-impact actions.

## 5. Scope model

Capabilities can be scoped by one or more dimensions:

- global technical scope;
- organization;
- territory;
- competence node;
- topic;
- civic process;
- case/resource;
- actor ownership;
- time window;
- identity/assurance condition.

Example:

```text
Actor: municipal officer A
Capability: response.publish_official
Organization: Municipality M
Territory: M
Competence: local_roads
Process: optional specific consultation
```

This actor cannot use the same capability for municipality N or an unrelated regional-health competence.

## 6. Institution authority provenance

Publishing an `OfficialResponse` requires more than an authenticated institution account:

1. verified organization/office membership;
2. active capability assignment;
3. resource territory/competence match;
4. process/case authority where applicable;
5. required step-up authentication;
6. attributable authority reference in the resulting record;
7. policy version/reason stored with the audit receipt.

Official history is versioned/append-oriented. Corrections supersede; they do not silently erase the previous attributable record.

## 7. Capability assignment lifecycle

Every capability assignment has:
- assignment ID;
- actor/service principal;
- capability;
- scope;
- grant authority;
- grant policy version;
- `valid_from` / `valid_until`;
- optional credential dependency;
- revocation state/reason;
- audit event.

No indefinite wildcard capability should be used when a narrower scoped grant is practical.

## 8. Separation-of-duty candidates

Require dual control or independent review for high-risk operations such as:
- activating a new civic decision/eligibility policy;
- changing a live ranking policy with major public exposure impact;
- creating/revoking institution root authority;
- mass moderation/enforcement actions;
- backup restore into production;
- signing/releasing regulated-process integration code;
- break-glass access to identity-private datasets.

The exact dual-control policy is deployment/governance dependent, but the architecture must support it.

## 9. Pilot-safe capability posture

Engineering recommendation for the first pilot:
- default deny;
- ordinary `PERSON` abilities limited to public Needs/evidence/proposals and explicitly enabled process participation;
- institutional capabilities granted only through verified organization + territory/competence scope;
- no actor receives formal vote multipliers from reputation;
- no commercial/partner role appears in civic authorization or ranking;
- platform operators cannot publish institutional responses unless separately credentialed as that institution;
- civic policy activation remains a high-impact audited governance action.

This is an engineering safety posture, not a claim that the final operator/governance authority (P0-02) is resolved.

## 10. Required tests

Before CP04/production:
- generated authorization matrix covering allow/deny for every protected command;
- cross-territory denial tests;
- cross-organization denial tests;
- expired/revoked credential tests;
- step-up requirement tests;
- actor-type-without-capability denial tests;
- capability-with-wrong-scope denial tests;
- process-frozen-policy tests;
- dual-control tests for configured high-impact operations;
- property tests proving no capability escalation through role combinations.
