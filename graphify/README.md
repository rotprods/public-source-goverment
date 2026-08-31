# graphify — Project/Product Knowledge Graph

This directory stores the typed graph used by humans and agents to navigate the product, code, policies, evidence and delivery state.

## It is not source of truth

The graph points to authoritative sources. It does not override code, database state, approved policies or ADRs.

## Node families

- `Goal`, `NorthStarMetric`, `Checkpoint`, `Risk`, `Decision`, `Evidence`;
- `ActorType`, `Capability`, `Territory`, `CivicTopic`, `CompetenceNode`;
- `Need`, `EvidenceDocument`, `Claim`, `Argument`, `Proposal`, `Process`, `Participation`, `InstitutionalResponse`, `Commitment`, `Outcome`;
- `App`, `Service`, `Crate`, `Database`, `Projection`, `API`, `Event`, `Policy`, `Workflow`, `Model`, `Dataset`, `Test`.

## Core relationships

- goals `REQUIRE` capabilities/checkpoints;
- modules `OWN` entities/contracts;
- APIs `EXECUTE` commands/queries;
- commands `MUTATE` aggregates;
- events `PROJECT_TO` read models/search/analytics;
- policies `AUTHORIZE` actions;
- tests `PROVE` invariants;
- evidence `SUPPORTS` claims/decisions/checkpoint status;
- civic needs `ROUTE_TO` competences and `ADDRESSED_BY` proposal versions;
- process `FREEZES` policy versions;
- institution `RESPONDS_TO` case/process;
- commitment `PRODUCES` outcome.

## Update protocol

After material architecture/domain changes:
1. update ontology/seed/generated graph;
2. attach source/evidence refs;
3. run graph queries for impacted dependencies;
4. persist change in COS/state/handoff.
