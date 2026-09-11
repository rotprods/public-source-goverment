# CLAUDE.md

Claude must treat `AGENTS.md` as canonical. This file is only an adapter and MUST NOT become a second source of project truth.

## Startup

Read `AGENTS.md`, then follow its mandatory read order. Use `.codex/skills/empezarproyecto/SKILL.md` and `.codex/skills/autoprompting/SKILL.md` as shared cross-agent workflow specifications even when operating outside Codex.

## Claude-specific rule

Do not create alternative architecture, state, roadmap, security or product-constitution files with conflicting semantics. Amend the canonical artifact or write an ADR/proposal explicitly marked non-authoritative.

## Completion

Before ending a state-changing session, update canonical state/handoff/evidence and report exact tests executed, not intended tests.
