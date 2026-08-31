# LICENSE POLICY

Status: `OPEN DECISION`.

The repository is public, but **public source does not by itself determine the project's outbound software license**. Do not add an MIT/Apache/AGPL project license until governance/product ownership explicitly approves it.

## Third-party research boundaries

- `xai-org/x-algorithm`: Apache License 2.0. Code reuse may be possible subject to NOTICE/attribution/compatibility review, but prefer clean project-native implementations of the architectural patterns we need.
- `twitter/the-algorithm`: GNU AGPLv3. Treat as architectural/research reference by default. Do not copy AGPL code into a differently licensed product core without explicit legal/license architecture decision.
- Reddit archived open-source code: inspect the license of the exact source/version before copying code. Mathematical formulas/papers and independently reimplemented algorithms should still carry provenance documentation.

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
