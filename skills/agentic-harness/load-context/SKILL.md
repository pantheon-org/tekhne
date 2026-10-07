---
name: load-context
description: Resume session from CONTEXT-llm.md. Use when resuming work, loading saved context, continuing a previous session. Triggers include "load context", "resume session", "continue where I left off".
argument-hint: "[stream-name] [--full]"
allowed-tools: bash, read, askuserquestion
model: haiku
user-invocable: true
---

# Load Context

Load session state from `CONTEXT-{stream}-llm.md` in the session directory and optionally expand full resources.

```text
.context/session/
  INDEX.md                        index of streams
  CONTEXT-llm.md                  default stream
  CONTEXT-{name}-llm.md           named stream
  done/CONTEXT-{name}-llm.md      archived stream (status done or parked)
```

**Speed**: < 3 seconds (default), 5-8 seconds (--full)

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known harness bug: outside Plan Mode, AskUserQuestion can silently return empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "Questions didn't display (known AskUserQuestion bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Performance Rules

1. **Use `rtk` for shell commands when it is installed** (a harness hook may already rewrite them; bare commands are also fine)
2. **Parallel tool calls** — ALL independent calls in one message
3. **Minimize round-trips**
4. **No unnecessary synthesis** — present parsed data directly

## Workflow

### Phase 1: Detect & Read (parallel)

```
Bash: ls -t .context/session/CONTEXT-*llm.md .context/session/done/CONTEXT-*llm.md 2>/dev/null || true
Read: .context/session/CONTEXT-{stream}-llm.md (if stream known from $ARGUMENTS)
```

If not found in the session directory, check its `done/` subfolder. If found there, note `📦 (from done/)` in report.

If multiple streams and no selection → AskUserQuestion with options (mark done/ files with 📦).

**Filename**: `default` → `CONTEXT-llm.md`, `{name}` → `CONTEXT-{name}-llm.md`

### Phase 2: Expand Resources (if --full)

Parallel Read: OpenSpec project/proposal/tasks.md, top 3 hot files, manifest.yaml.
**DO NOT restore tasks** — informational only.

**Thinking Artifacts** (if `## Thinking Artifacts` section exists in CONTEXT file):
- Default mode: display artifact paths in resume report (no content read)
- `--full` mode: Read referenced thinking artifacts and include brief summaries in report

### Phase 3: Format Resume Report

Parse key-value header + markdown sections → human-friendly report.

See `references/reference.md` for section mapping, report structure, error messages, and formatting rules.

## Philosophy

- **Resume, don't restart** — loading context is about continuing with intent intact, not re-reading a history log.
- **Trust the INDEX.md** — the index is the authoritative pointer to current context; do not browse the directory manually.
- **Verify before acting** — a loaded context must be confirmed against the current task before influencing decisions.
- **Done means done** — contexts in `done/` are retired; loading them requires explicit user confirmation.

## When to Use

- At the start of a new session when `INDEX.md` or `CONTEXT-*.md` files exist in the session directory from prior work.
- When resuming interrupted work and the agent needs prior decisions, open tasks, and rationale.
- When the user says "load context", "resume session", "continue where I left off", or similar.
- When switching between named work streams and a specific `CONTEXT-{name}-llm.md` snapshot exists.
- When `--full` expansion is needed to bring in hot files, proposals, or thinking artifacts alongside the base context.

## When Not to Use

- When starting a genuinely new session with no prior files in the session directory — there is nothing to load.
- When the user has explicitly asked to start fresh or discard prior context.
- When only a quick one-off question is being answered and persistent session state is irrelevant.
- When the context files are clearly stale (weeks old and unrelated to the current task) — load only after confirming with the user.
- When the target `CONTEXT-*.md` file is already loaded in the current session and re-loading would cause duplication.

## Anti-Patterns

- **NEVER load context from `done/` without checking dates** — Stale context from completed sessions misleads current decisions. **WHY:** The `done/` folder contains retired sessions; always check `INDEX.md` first to confirm the most recent active context before falling back to `done/`.
- **NEVER skip verifying the session_id after load** — Loading the wrong context silently diverges from the user's intent. **WHY:** Multiple streams may coexist in the session directory; the wrong file causes invisible drift and incorrect assumptions.

  ```bash
  # BAD - reads the first match and never checks which stream it belongs to
  head -1 <(ls -t .context/session/CONTEXT-*llm.md) | xargs cat

  # GOOD - read the file, then confirm its `stream:` header matches the request
  grep '^stream:' .context/session/CONTEXT-feature-auth-llm.md   # expect: stream: feature-auth
  ```
- **NEVER proceed with assumptions when `AskUserQuestion` returns empty** — Blank answers mean the UI never rendered. **WHY:** A known harness bug silently swallows questions outside Plan Mode; always fall back to a numbered text list and wait for a reply.
- **NEVER restore tasks from the context file as actionable work items** — Context files are informational snapshots, not live task lists. **WHY:** The `--full` expansion is read-only; acting on tasks from a snapshot without user confirmation can duplicate or conflict with current work.

  ```text
  # BAD  - "The context lists 3 tasks, so I'll start on task 1."
  # GOOD - "The saved Next list shows 3 tasks. Which of these is still current?"
  ```
- **NEVER omit the `📦 (from done/)` marker when reporting a done/-sourced file** — Users need to know the context came from an archived session. **WHY:** Presenting done/ context without a marker makes it indistinguishable from an active session, causing confusion about what work is current.

## Usage Examples

**Resuming the default stream:**

```text
> load context

# 🔄 Session Resume: default
Stream: default | Saved: 2026-09-29T16:40Z | Status: building
Focus: Moving the retry logic into the client
Goal: Ship the retry change behind a flag
✅ NextTasks: 1. Add retry tests  2. Wire the flag  3. Update the README
💬 Session Context: Chose exponential backoff over fixed delay (fewer bursts).
📁 Hot Files: src/client.ts, src/retry.ts
🎯 Next Step: Add retry tests for the backoff path in src/retry.ts.
```

**Loading a named stream:**

```text
> load context api-redesign
# Reads .context/session/CONTEXT-api-redesign-llm.md; report title is "Session Resume: api-redesign"
```

**Expanding with `--full`:**

```text
> load context --full
# Base file first, then one parallel batch: tasks.md, top 3 hot files, manifest.yaml,
# and any files under "## Thinking Artifacts". Tasks are shown as information only.
```

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Section mapping, report layout, error messages | [Reference](references/reference.md) | Formatting the resume report or wording an error |
