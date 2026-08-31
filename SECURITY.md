# SECURITY.md — Civic Security Baseline

Status: `BOOTSTRAP / NO PRODUCTION SECURITY CLAIM`.

## Security objectives

1. Protect private identity from civic-content compromise.
2. Prevent unauthorized civic actions and policy changes.
3. Detect Sybil, brigading, astroturfing and credential abuse without political profiling.
4. Preserve process/rule/version integrity.
5. Make administrative actions attributable and appealable.
6. Preserve evidence/provenance while respecting lawful deletion and retention requirements.
7. Secure the software supply chain and deployment path.

## Data zones

- `IDENTITY_PRIVATE`: authentication, identity assertions, residency/credential proofs. Separate schema/service boundary; never included in public embeddings.
- `CIVIC_PUBLIC`: public needs, proposals, evidence, arguments, official responses.
- `CIVIC_SENSITIVE`: non-public drafts, moderation cases, appeals, private process participation where applicable.
- `ANALYTICS`: pseudonymized/minimized events; no unnecessary political-profile features.
- `SECURITY_AUDIT`: append-only security/admin receipts with restrictive access.

## Authentication and authorization

- Passkeys/WebAuthn preferred; OIDC adapters supported.
- Identity verification is assertion-based; avoid storing raw DNI/biometric material unless legally unavoidable.
- Authorization is server-side ABAC/ReBAC over actor, capability, territory, process, organization and policy version.
- Evaluate Cedar vs OPA behind a `PolicyEngine` port before lock.
- High-impact admin/policy actions require step-up auth and, where appropriate, dual control.

## Application controls

Target OWASP ASVS 5.0 Level 2 baseline; selected Level 3 controls for identity, voting/process integrity, administration and evidence.

Required:
- CSRF/XSS/SSRF/injection protections;
- strict CSP and secure headers;
- rate limits + abuse controls;
- object-level authorization tests;
- secure file/evidence ingestion, MIME validation and malware scanning;
- idempotency keys for mutation endpoints;
- immutable process policy snapshots;
- audit receipts for authz-sensitive mutations;
- encryption in transit and at rest;
- KMS/envelope encryption for high-sensitivity fields;
- retention/deletion/export workflows;
- backup restore drills.

## Threat classes

- Sybil identities and credential farms.
- Coordinated brigading / astroturfing.
- Party/government capture of moderation or ranking.
- Insider/admin abuse.
- Institutional impersonation.
- Evidence forgery and malicious document upload.
- Ranking manipulation and recommender feedback loops.
- Data poisoning / prompt injection into AI Librarian.
- Model-induced omission/bias in summaries.
- Doxxing, threats and harassment.
- Supply-chain compromise.
- Stale sovereign/on-prem installations.
- Vote replay/double participation where a process requires uniqueness.
- Cross-territory authorization leakage.

## Supply chain

- pinned lockfiles;
- `cargo audit`, `cargo deny`, `pnpm audit`/approved SCA;
- CodeQL/Semgrep where applicable;
- gitleaks;
- Syft SBOM + Grype/Trivy scanning;
- signed OCI images/releases with Cosign or equivalent;
- SLSA provenance target: L2 initially, L3 for critical release paths;
- OpenSSF Scorecard in CI;
- Renovate with review gates.

## AI security

- Model gateway with task/data policy classification.
- No raw private identity to generative models.
- Retrieved external content is untrusted data, never instructions.
- Persist model/provider/version, prompt version, input refs, output, citations, eval and review state.
- Red-team prompt injection, retrieval poisoning, fabricated citations and partisan summarization drift.

## Civic integrity boundary

P0/P1/P2 civic processes may use platform decision methods subject to Product Constitution. A legally regulated public ballot (P3) is an integration boundary requiring dedicated legal/security assurance, not a generic feature of this codebase.

## Production gates

No production launch until: threat model, DPIA/legal mapping where applicable, authz matrix tests, dependency/secret scans, backup/restore evidence, incident runbook, abuse-response runbook, observability, penetration testing plan and release provenance exist.
