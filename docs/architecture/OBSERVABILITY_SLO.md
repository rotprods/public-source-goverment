# Observability & SLO Architecture

## Correlation model

One user-visible action should be traceable across:

`HTTP request → authn/authz → domain command/query → SQL transaction → domain event/outbox → workflow/projector → search/ranking/AI call → response/notification`

Required IDs:
- `request_id`;
- `correlation_id`;
- `causation_id` for events;
- `workflow_id` where applicable;
- `actor_ref` only where allowed by telemetry privacy class;
- `process/territory/object refs` when operationally needed.

## Telemetry standards

OpenTelemetry is the instrumentation contract. Application code emits structured traces, metrics and logs independent of the final backend.

### Metrics

Golden signals:
- request rate/error/latency;
- DB pool/connections/query latency/locks;
- outbox backlog/age/retries;
- projector lag/rebuild duration;
- workflow timers/retries/stuck executions;
- hybrid search latency/candidate counts/ANN recall evals;
- recommender source fan-out/filter/drop reasons/rank latency;
- AI request latency/error/cost/fallback/eval outcome;
- evidence upload/scan failure;
- authz denial/anomaly rates;
- moderation/appeal queues;
- institution response SLA distribution.

### Civic/product quality

- QCLC cohort rate;
- Need→route/proposal/response/outcome times;
- duplicate merge/suggestion precision/recall;
- evidence diversity;
- institutional response/closure rate;
- recommender concentration/diversity;
- explanation coverage/fidelity;
- appeal reversal rate.

## Initial SLO candidates

These are targets to validate in CP13, not marketing guarantees:

| Surface | SLO candidate |
|---|---|
| public API availability | 99.9% monthly |
| public read p95 | <300 ms excluding AI generation |
| standard mutation p95 | <500 ms excluding external identity/provider latency |
| hybrid search p95 | <600 ms under pilot corpus/load |
| projection lag p95 | <5 s |
| high-impact audit receipt coverage | 100% |
| recommender explanation metadata | 100% |
| authz regression unauthorized access | 0 |
| backup restore | demonstrated within documented RTO/RPO |

## Error budgets

SLOs have error budgets. Repeated budget exhaustion blocks feature rollout and prioritizes reliability work.

## Logs and political/privacy data

Logs must not casually contain:
- raw identity documents;
- biometric material;
- access tokens/secrets;
- full civic text unless explicitly required and redacted/classified;
- hidden political-profile labels;
- private moderation/evidence payloads.

Use structured identifiers and hashes/references instead.

## Dashboards

Minimum operator views:
1. platform health;
2. DB/outbox/projectors;
3. search/vector quality/latency;
4. recommender pipeline quality and concentration;
5. identity/authz/security anomalies;
6. institutional workflow queues;
7. AI provider/model health and evaluation;
8. QCLC/product funnel;
9. moderation/appeal operations.

## Alerting

Alert on user/public harm, not every metric wiggle. Examples:
- authz anomaly spike;
- outbox/projection backlog beyond SLO;
- database saturation;
- backup failure;
- ranking policy version mismatch;
- model output/citation failure spike;
- institution workflow timer stuck;
- high 5xx rate;
- storage/evidence integrity failure.

## SRE readiness gate

Production requires:
- dashboards;
- alerts with runbook links;
- on-call ownership/escalation;
- incident severity model;
- rollback instructions;
- RTO/RPO;
- backup/restore evidence;
- post-incident review template.
