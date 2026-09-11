# LICENSE POLICY

Status: `OPEN DECISION`.

The repository is public, but **public source does not by itself determine the project's outbound software license**. Do not add an MIT/Apache/AGPL project license until governance/product ownership explicitly approves it.

## Third-party research boundaries

- `xai-org/x-algorithm`: Apache License 2.0. Code reuse may be possible subject to NOTICE/attribution/compatibility review, but prefer clean project-native implementations of the architectural patterns we need.
- `twitter/the-algorithm`: GNU AGPLv3. Treat as architectural/research reference by default. Do not copy AGPL code into a differently licensed product core without explicit legal/license architecture decision.
- `reddit-archive/reddit`: the inspected archive and ranking source `r2/r2/lib/db/_sorts.pyx` are under Common Public Attribution License 1.0 (CPAL). Treat code as research/reference by default. Independently reimplement mathematical techniques such as Wilson confidence or time decay from their mathematical definitions rather than copying Reddit source text.

## Dependency policy

Dependencies must pass `cargo-deny`/SCA license checks. New reciprocal/copyleft dependencies require an ADR and compatibility review before entering production runtime.

## Source provenance

For every borrowed implementation rather than general idea, record:
- upstream repository;
- file/commit;
- upstream license;
- modifications;
- attribution/NOTICE requirements;
- approved outbound-license compatibility.

For independently implemented public algorithms, record the algorithm/paper/source lineage and tests while keeping project-native source code.
