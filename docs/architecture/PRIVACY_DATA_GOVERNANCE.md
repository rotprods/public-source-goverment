# Privacy & Data Governance Architecture

Status: engineering baseline; legal bases/safeguards require jurisdiction-specific counsel/DPIA before production.

## Core rule

Civic participation can reveal highly sensitive beliefs or political opinions. The architecture therefore assumes **data sensitivity by default**, minimizes collection and separates public civic expression from private identity proof.

## Data zones

### `identity_private`
Contains only authentication/identity assertions needed to prove properties such as unique person, age/residency or professional/institutional role.

Rules:
- not public;
- not copied into recommendation embeddings;
- raw identity documents/biometrics avoided wherever a signed assertion can prove the required fact;
- strict least-privilege service/DB access;
- dedicated retention/deletion policy;
- every use tied to a process requirement.

### `civic_public`
Public needs, evidence, claims, arguments, proposal versions, process metadata, official responses, commitments and outcomes that are intentionally published.

Rules:
- version/provenance preserved;
- public status does not imply unrestricted downstream profiling;
- public search/vector representations remain purpose-limited and rebuildable;
- edits create version history according to product/public-record policy.

### `civic_sensitive`
Private drafts, appeals, moderation cases, abuse reports, restricted process participation and other non-public civic material.

Rules:
- no public indexing/embedding;
- restricted authorization and telemetry;
- retention minimized;
- exports/redactions controlled.

### `analytics`
Pseudonymous/minimized behavioral and system events needed for product quality, abuse detection, SLOs and recommender evaluation.

Rules:
- event allowlist, not arbitrary payload dumping;
- no raw identity documents/tokens/secrets;
- civic text omitted unless an approved analytics purpose requires it;
- no hidden ideology/party-affinity warehouse;
- separate retention from canonical civic records.

### `security_audit`
High-impact authentication, authorization, administration and policy-change receipts.

Rules:
- append-oriented;
- restrictive access;
- tamper-evidence/immutable-backup strategy;
- retention based on security/accountability need.

## Purpose registry

Every material data field/dataset records:

```text
data_asset
purpose
owner
privacy_class
source
lawful_basis_or_review_status
allowed_consumers
retention
public/private state
derived representations
model/analytics use allowed?
delete/export behavior
cross-border/provider constraints
```

No new recommender/AI feature is allowed simply because the data exists.

## Identity architecture

Do not store one generic `verified=true` flag.

Model independent assertions:
- authenticated session;
- unique-person assurance;
- age assertion;
- residency/territory assertion;
- professional credential;
- organization/office role;
- publisher capability;
- public-name disclosure.

A process requests the minimum assertion set it needs. UI may show public identity separately from private eligibility verification.

## Semantic/vector privacy

Embeddings can leak or preserve sensitive information even when they do not look human-readable.

Rules:
- only approved entity classes are vectorized;
- private identity data excluded by default;
- every embedding stores source/entity/model/version/policy classification;
- deletion/revocation triggers derived-index deletion/rebuild;
- embedding providers receive only the minimum content necessary under the provider/data policy;
- local/self-hosted models are available as a deployment profile where data-transfer constraints require it.

## AI provider gateway

All generative/embedding/reranking providers sit behind the AI gateway.

The gateway enforces:
- task allowlist;
- privacy/data classification;
- redaction/minimization;
- approved provider/model by class;
- regional/self-hosted routing where required;
- prompt/model version provenance;
- citation/output validation;
- retention/no-training provider settings where contractually supported;
- cost/rate limits.

Models never receive raw passwords/tokens/private identity documents as normal context.

## Analytics and recommender feature governance

Feature classes:

### Allowed baseline
- explicitly selected territory;
- explicit subscriptions/interests;
- current query/object context;
- public content relationships;
- process deadlines/status;
- exposure/fatigue metadata;
- approved civic-priority/evidence-quality signals.

### Restricted
- interaction-affinity models;
- reputation/reliability;
- graph centrality;
- inferred topic preference;
- abuse/anomaly scores.

Restricted features require purpose, bias/privacy eval, retention and appeal/contestability considerations where they affect people materially.

### Prohibited by default
- inferred ideology/party preference for persuasion;
- sensitive/protected traits for recommendation;
- commercial partner status as civic ranking input;
- ad-spend political amplification;
- cross-context identity enrichment unrelated to civic purpose.

## Data subject / user-control engineering

Architecture must support, subject to public-record/legal exceptions:
- account/profile data export;
- participation/history export;
- correction of private identity assertions through issuer/reverification path;
- deletion/closure requests;
- revocation/withdrawal where a process legally/product-wise permits it;
- explanation of recommendation/relevant automated assistance;
- notification/subscription controls;
- consent/preference records where consent is actually the chosen legal mechanism.

Do not promise deletability of public/institutional records where governance/law requires preservation; expose the exact policy instead.

## Retention

Retention is per dataset/purpose, never `keep everything forever`.

A production retention matrix must cover:
- authentication/session/security events;
- private identity assertions;
- public civic content/version history;
- private drafts;
- moderation/appeal evidence;
- recommender impressions/features;
- AI artifacts/prompts/provider logs;
- raw/derived embeddings;
- backups;
- institution cases/official records.

Deletion must propagate to rebuildable projections/vector/search/analytics according to policy while audit tombstones may preserve non-sensitive accountability metadata.

## DPIA / legal review triggers

Trigger dedicated privacy/legal review before:
- production identity verification;
- processing explicitly political-opinion data beyond user-directed public expression;
- sensitive-trait personalization;
- biometric identity proof;
- large-scale behavioral profiling;
- automated decisions with material civic effects;
- new cross-border AI/data processors;
- formal public-sector/regulated ballot integrations;
- federation that republishes user data beyond the original operator boundary.

## Security/privacy Definition of Done

A data feature is not DONE until its owner can answer:
1. Why is this data necessary?
2. Is there a less sensitive representation?
3. Who can read/write it?
4. Is it public by user intent or only operationally visible?
5. Where is it copied/embedded/analyzed?
6. How is it retained/deleted/exported?
7. Which models/providers see it?
8. What happens if the user revokes/deletes/corrects it?
9. How is misuse detected?
10. What evidence proves the policy is implemented?
