# Quality Audit: `skills/colyseus-multiplayer/SKILL.md`

Date: 2026-02-20
Auditor: Codex

## Findings

1. **High**: Matchmaking example contains multiple `const room` redeclarations in the same scope, so the snippet is not runnable as written.  
   File: `skills/colyseus-multiplayer/SKILL.md:389`, `skills/colyseus-multiplayer/SKILL.md:395`, `skills/colyseus-multiplayer/SKILL.md:398`, `skills/colyseus-multiplayer/SKILL.md:401`

2. **High**: Client reconnection snippet uses constructor options (`enableReconnect`, `reconnectAttempts`, `reconnectDelay`) that do not match current SDK guidance (reconnection options are configured on `room.reconnection.*` after join). This is likely to mislead users into ineffective config.  
   File: `skills/colyseus-multiplayer/SKILL.md:541`, `skills/colyseus-multiplayer/SKILL.md:542`, `skills/colyseus-multiplayer/SKILL.md:543`, `skills/colyseus-multiplayer/SKILL.md:544`  
   Source: <https://docs.colyseus.io/sdk>

3. **Medium**: Markdown lint failure (`MD036`) for using bold text as a heading; this violates repo markdown quality gates.  
   File: `skills/colyseus-multiplayer/SKILL.md:28`

4. **Medium**: Markdown lint failure (`MD040`) for a fenced code block without language tag.  
   File: `skills/colyseus-multiplayer/SKILL.md:683`

5. **Medium**: The document is a large monolithic tutorial in `SKILL.md` with no supporting `references/` split, which conflicts with repo authoring guidance to keep `SKILL.md` as a concise navigation hub and move deep detail out. This increases maintenance and review cost.  
   File: `skills/colyseus-multiplayer/SKILL.md:13`  
   Repo guideline reference: `AGENTS.md` (authoring rules)

6. **Low**: Some snippets rely on undeclared helpers/imports (`validate`, `z`) in-context, reducing copy/paste reliability.  
   File: `skills/colyseus-multiplayer/SKILL.md:327`, `skills/colyseus-multiplayer/SKILL.md:328`

## Open Questions / Assumptions

1. Should this skill optimize for copy/paste runnable snippets, or conceptual guidance only? That determines how strict snippet completeness should be.
2. Should I proceed with a remediation pass (lint fixes + reconnection section correction + snippet cleanup) in this file now?

## Checks Run

1. `bunx markdownlint-cli2 "skills/colyseus-multiplayer/SKILL.md"` (2 errors found).
