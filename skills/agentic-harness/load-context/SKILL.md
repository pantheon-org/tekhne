---
name: load-context
description: Resume session from CONTEXT-llm.md. Use when resuming work, loading saved context, continuing a previous session. Triggers include "load context", "resume session", "continue where I left off".
argument-hint: "[stream-name] [--full]"
allowed-tools: [Bash, Read, AskUserQuestion]
model: haiku
context: main
user-invocable: true
---

# Load Context

Load session state from `.context/session/CONTEXT-{stream}-llm.md` and optionally expand full resources.

**Speed**: < 3 seconds (default), 5-8 seconds (--full)

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known Claude Code bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known Claude Code bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Performance Rules

1. **Use `rtk` for ALL shell commands**
2. **Parallel tool calls** — ALL independent calls in one message
3. **Minimize round-trips**
4. **No unnecessary synthesis** — present parsed data directly

## Workflow

### Phase 1: Detect & Read (parallel)

```
Bash: rtk ls -t .context/session/CONTEXT-*llm.md .context/session/done/CONTEXT-*llm.md 2>/dev/null || true
Read: .context/session/CONTEXT-{stream}-llm.md (if stream known from $ARGUMENTS)
```

If not found in `.context/session/`, check `.context/session/done/` subfolder. If found there, note `📦 (from done/)` in report.

If multiple streams and no selection → AskUserQuestion with options (mark done/ files with 📦).

**Filename**: `"default" → .context/session/CONTEXT-llm.md`, `"{name}" → .context/session/CONTEXT-{name}-llm.md`

### Phase 2: Expand Resources (if --full)

Parallel Read: OpenSpec project/proposal/tasks.md, top 3 hot files, manifest.yaml.
**DO NOT restore tasks** — informational only.

**Thinking Artifacts** (if `## Thinking Artifacts` section exists in CONTEXT file):
- Default mode: display artifact paths in resume report (no content read)
- `--full` mode: Read referenced thinking artifacts and include brief summaries in report

### Phase 3: Format Resume Report

Parse key-value header + markdown sections → human-friendly report.

See `reference.md` for section mapping, report structure, error messages, and formatting rules.

## Philosophy

- **Resume, don't restart** — loading context is about continuing with intent intact, not re-reading a history log.
- **Trust the INDEX.md** — the index is the authoritative pointer to current context; do not browse the directory manually.
- **Verify before acting** — a loaded context must be confirmed against the current task before influencing decisions.
- **Done means done** — contexts in `done/` are retired; loading them requires explicit user confirmation.

## When to Use

- At the start of a new session when `.context/session/INDEX.md` or `CONTEXT-*.md` files exist from prior work.
- When resuming interrupted work and the agent needs prior decisions, open tasks, and rationale.
- When the user says "load context", "resume session", "continue where I left off", or similar.
- When switching between named work streams and a specific `CONTEXT-{name}-llm.md` snapshot exists.
- When `--full` expansion is needed to bring in hot files, proposals, or thinking artifacts alongside the base context.

## When Not to Use

- When starting a genuinely new session with no prior `.context/session/` files — there is nothing to load.
- When the user has explicitly asked to start fresh or discard prior context.
- When only a quick one-off question is being answered and persistent session state is irrelevant.
- When the context files are clearly stale (weeks old and unrelated to the current task) — load only after confirming with the user.
- When the target `CONTEXT-*.md` file is already loaded in the current session and re-loading would cause duplication.

## Anti-Patterns

- **NEVER load context from `done/` without checking dates** — Stale context from completed sessions misleads current decisions. **Why:** The `done/` folder contains retired sessions; always check `INDEX.md` first to confirm the most recent active context before falling back to `done/`.
- **NEVER skip verifying the session_id after load** — Loading the wrong context silently diverges from the user's intent. **Why:** Multiple streams may coexist in `.context/session/`; the wrong file causes invisible drift and incorrect assumptions.
- **NEVER proceed with assumptions when `AskUserQuestion` returns empty** — Blank answers mean the UI never rendered. **Why:** A known Claude Code bug silently swallows questions outside Plan Mode; always fall back to a numbered text list and wait for a reply.
- **NEVER restore tasks from the context file as actionable work items** — Context files are informational snapshots, not live task lists. **Why:** The `--full` expansion is read-only; acting on tasks from a snapshot without user confirmation can duplicate or conflict with current work.
- **NEVER omit the `📦 (from done/)` marker when reporting a done/-sourced file** — Users need to know the context came from an archived session. **Why:** Presenting done/ context without a marker makes it indistinguishable from an active session, causing confusion about what work is current.

## Usage Examples

**Loading context at the start of a resumed session:**
```bash
# Skill reads .context/session/INDEX.md to find the latest snapshot
# Loads .context/session/CONTEXT-2026-04-10-143022.md
# Agent now has prior decisions, open tasks, and rationale
```

**Loading a named work stream:**
```bash
# User: "load context --stream api-redesign"
# Skill loads .context/session/CONTEXT-api-redesign-llm.md
# Resume report shows stream name, last updated timestamp, and open items
```

**Expanding full resources with --full:**
```bash
# User: "load context --full"
# Skill loads .context/session/CONTEXT-llm.md (base)
# Also reads top 3 hot files and thinking artifacts referenced in ## Thinking Artifacts
# Report includes brief summaries of each artifact; tasks remain read-only
```

## References

- [Reference](reference.md) — detailed section mapping, report structure, error messages, and formatting rules
