# Scenario 03: Triage Tool — Duplicate Detection

## User Prompt

You are working in the research repository.

Current state of REVIEWED.md (excerpt):

```markdown
## Summary table

| Date | Name | Type | Disposition | Notes |
|---|---|---|---|---|
| 2026-03-20 | versatly-clawvault | tool | pending | Session lifecycle CLI with memory graph and context injection |
| 2026-03-15 | jiang-llmlingua | paper | promoted | Prompt compression via token importance scoring |

## versatly-clawvault — ClawVault

- **Repo**: https://github.com/versatly/clawvault
- **Author**: Versatly
- **Version**: v2.6.1
- **Language**: TypeScript / Node.js
- **Tags**: session-lifecycle, injection, cli
- **Summary**: CLI tool for structured markdown context vault with session wake/sleep/checkpoint, knowledge graph, observation pipeline, and OpenClaw hooks.
- **Scope fit**: in scope — manages active context window via session lifecycle and structured injection
- **Disposition**: pending — awaiting user decision on promotion
```

A team member has asked you to add ClawVault to the repo:

- **Repo**: https://github.com/versatly/clawvault
- **Tool**: ClawVault (npm: `clawvault`)

## Expected Behavior

1. Check `REVIEWED.md` for the repo URL (`versatly/clawvault`) or slug (`versatly-clawvault`) before attempting to write any new file
2. Check `references/REFERENCE_INDEX.md` as part of the duplicate detection step
3. Identify that `versatly/clawvault` already exists in REVIEWED.md and reference the existing entry (`versatly-clawvault` section or slug)
4. Halt the workflow — do not create a new `references/<slug>.md` file
5. Do not append a new row to the REVIEWED.md summary table
6. Do not add a new detailed section to REVIEWED.md
7. Do not add a new row to `references/REFERENCE_INDEX.md`
8. Report the duplicate clearly to the user, naming the existing entry and providing its location
9. Stop after reporting the duplicate — do not continue with the triage workflow

## Success Criteria

- **REVIEWED.md is searched before any file is created**: The agent checks REVIEWED.md for the repo URL (versatly/clawvault) or slug (versatly-clawvault) before attempting to write any new file.
- **references/REFERENCE_INDEX.md is also checked**: The agent checks REFERENCE_INDEX.md in addition to REVIEWED.md as part of the duplicate detection step.
- **Duplicate match is identified and the existing entry is cited**: The agent explicitly identifies that versatly/clawvault already exists in REVIEWED.md and references the existing entry (versatly-clawvault section or slug).
- **No new references/<slug>.md file is created**: The agent does not create any new file under references/ for this tool.
- **No new row is added to the REVIEWED.md summary table**: The REVIEWED.md summary table is not modified. No new row is appended.
- **No new detailed section is added to REVIEWED.md**: The REVIEWED.md body is not modified. No new detailed section is written for this tool.
- **No new row is added to REFERENCE_INDEX.md**: references/REFERENCE_INDEX.md is not modified. No new entry is added for this tool.
- **The duplicate is reported clearly to the user**: The agent communicates that this tool has already been triaged, naming the existing entry (versatly-clawvault or the REVIEWED.md section).
- **Path or location of the existing entry is provided**: The agent tells the user where the existing entry is (e.g. REVIEWED.md section 'versatly-clawvault' or references/versatly-clawvault.md).
- **Workflow halts without requesting further action**: The agent stops after reporting the duplicate and does not continue with the triage workflow steps.

## Failure Conditions

- Agent proceeds to create files without checking for a duplicate first
- Agent checks only one of REVIEWED.md or REFERENCE_INDEX.md
- Agent fails to identify the existing entry or does not cite it by name
- Agent creates a new `references/<slug>.md` file despite the duplicate existing
- Agent appends a new row to the REVIEWED.md summary table
- Agent adds a new detailed section to REVIEWED.md
- Agent adds a new entry to REFERENCE_INDEX.md
- Agent reports the duplicate without naming the slug or specifying its location
- Agent continues the triage workflow after finding the duplicate
