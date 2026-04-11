# save-context Reference

## CONTEXT File Template

```markdown
# Session Context: {title}

saved: YYYY-MM-DDTHH:MM:SSZ
stream: {name}
status: {exploring|building|decided|parked|done}
focus: {1-2 sentences}
goal: {1 sentence}

---

## Project

type: {project type}

## Next

- {task 1}
- {task 2}
- {task 3}

## Session

progression:
  - {aggregated timeline steps}
decisions:
  - {key choice}: {rationale}
thinking:
  - {reasoning, trade-offs, insights}
unexpected:
  - {pivots, corrections, surprises}

## Hot Files

- {path}: {brief role}

## Refs

- {external references if any}
```

## Context Quality Self-Check

- **Save**: non-trivial work (>1 file, decisions made), mid-stream checkpoint, learning/insights
- **Ask user**: quick fix (1-2 files, obvious), exploration with no conclusions
- **Suggest skip**: greeting/chat only, single read/question, unresolved troubleshooting

If marginal: `"Session appears brief. Save context anyway?"` — wait for confirmation.

## Status Mapping (for INDEX.md)

| Raw Value | Display |
|-----------|---------|
| `exploring` | exploring |
| `decided` | decided |
| `building`, `in_progress` | building |
| `parked` | parked |
| `operational`, `verified` | operational |
| `done`, `completed`, `closed` | done |
| missing/empty/`n/a` | unknown |

## INDEX.md Upsert (Phase 3)

Run `scripts/upsert-index.sh` with 6 positional args:

```
Bash: ./scripts/upsert-index.sh <area> <project> <context> "<status>" "<focus>" <saved_date>
```

- **area**: derive from CWD relative to repo root (e.g., `repos`, `projects`, `code`)
- **project**: project folder name
- **context**: stream name
- **status**: label from Status Mapping table (e.g., `"building"`)
- **focus**: <=80 char summary
- **saved**: YYYY-MM-DD

Script handles: find `.context/session/INDEX.md`, match/replace or append row, skip if missing. Parked/Done/Archived sections preserved.

## Auto-Archive to `.context/session/done/` (Phase 3b)

When status is `done` or `parked`:
1. `mkdir -p .context/session/done/` in the project folder
2. `mv .context/session/CONTEXT-{stream}-llm.md .context/session/done/`
3. Confirm: `"Archived to .context/session/done/ (status: {status})"`

**Exceptions** — do NOT move:
- `.context/session/CONTEXT-llm.md` (default stream) — always stays in `.context/session/`
- `.context/session/CONTEXT-baseline-llm.md` — always stays in `.context/session/`
- If user explicitly says "keep here" or "don't move"

**Note**: upsert-index.sh runs BEFORE the move (it reads CWD, not file path).

## Stream Naming

- **Reserved**: `default` → `.context/session/CONTEXT-llm.md`, `baseline` → fork point from `/create-context`
- **Pattern**: `^[a-zA-Z0-9_-]{1,50}$`

## Token Budget

- Session section: 780 tokens max
- Total: 1200-1500 tokens MAX
- Hot Files: max 10 with brief roles
- Use YAML inline objects: `{done: 5, active: 2, pending: 3}`

## Related

- `/load-context [stream] [--full]` - Load saved stream
- `/list-contexts [--sync] [--archive <stream>]` - List/sync all contexts
- `/create-context` - Create baseline from .in/ folder
