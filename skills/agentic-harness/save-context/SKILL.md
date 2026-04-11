---
name: save-context
description: Save session to CONTEXT-llm.md with conversation summary. Use when saving work, checkpointing progress, preserving session state. Triggers include "save context", "save session", "checkpoint", "save my progress".
argument-hint: "[stream-name] [description]"
allowed-tools: bash, read, write, edit, askuserquestion
model: haiku
context: main
user-invocable: true
---

# Save Context

Save current session state to `.context/session/CONTEXT-{stream}-llm.md` with LLM-optimized format.

**Target**: 1200-1500 tokens MAX | **Speed**: 3-5 seconds

## AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known Claude Code bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "Questions didn't display (known Claude Code bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Performance Rules

1. **Use `rtk` for ALL shell commands**
2. **Parallel tool calls** — ALL independent calls in one message
3. **Minimize round-trips** — gather all data phase 1, reason phase 2, write phase 3

## Workflow

### Phase 1: Gather Data (parallel)

```
Bash: rtk ls .context/session/ + rtk ls -t .context/session/CONTEXT-*llm.md
```

**Stream resolution**: First word of `$ARGUMENTS` = stream name (`^[a-zA-Z0-9_-]{1,50}$`), rest = description. Empty → reuse prior `/load-context` stream or AskUserQuestion.

### Phase 2: Analyze & Synthesize (single pass)

From conversation (last 15-20 messages):
1. **Next** — infer 3 tasks from conversation (IMPORTANT: use heading "Next" — Claude Code compaction grep-matches `next`/`todo`/`pending`/`remaining` keywords for survival priority)
2. **Session** — progression, decisions, thinking, unexpected (780 tokens max)
3. **Hot Files** — max 10 discussed/edited
4. **Focus & Goal** — 1-2 sentence focus + goal

### Phase 3: Write & Report

Write CONTEXT file using template, then upsert `.context/session/INDEX.md` via `scripts/upsert-index.sh`.

**Stream naming**: `"default" → .context/session/CONTEXT-llm.md`, `"{name}" → .context/session/CONTEXT-{name}-llm.md`

### Phase 3b: Auto-archive to `.context/session/done/`

If status is `done` or `parked` → move file to `.context/session/done/` subfolder:
```
Bash: mkdir -p .context/session/done && mv .context/session/CONTEXT-{stream}-llm.md .context/session/done/
```
Report: `"Archived to .context/session/done/ (status: {status})"`

See `reference.md` for CONTEXT file template, quality self-check, status mapping, done/ archival rules, and INDEX.md upsert logic.

## Philosophy

- Context saves are **checkpoints, not archives** — they capture the current state of intent and decisions, not a full transcript.
- **Meaningful milestones over time intervals** — save when something worth remembering happens, not on a schedule.
- **Traceability over brevity** — a save that omits the "why" behind a decision is incomplete.
- **Reversible by design** — saves accumulate; nothing is overwritten without explicit archival.

## When to Use

- **Before ending a multi-hour session** — preserve decisions and next steps before the conversation closes.
- **Before a context window reset or compaction** — Claude Code compacts long conversations; save first to survive the reset.
- **After completing a discrete milestone** — a working feature, a passing test suite, a completed PR review.
- **When switching streams or workspaces** — checkpoint the current stream before pivoting to a different task.
- **When you've made non-obvious architectural decisions** — lock in the reasoning so future sessions don't relitigate it.

## When Not to Use

- **For trivial changes with obvious git history** — a single-line fix with a clear commit message needs no context file.
- **At the very start of a session** — there is nothing meaningful to summarise yet; save at checkpoints, not at startup.
- **As a substitute for proper documentation** — context files are ephemeral session aids, not design docs or ADRs.
- **When no meaningful work has occurred** — an empty or near-empty context file creates noise and misleads future sessions.
- **More than once per hour on short tasks** — over-saving dilutes the signal; one file per meaningful checkpoint is enough.

## Anti-Patterns

- **NEVER overwrite an existing context file without reading it first** — Silent overwrites lose prior session decisions. **Why:** The INDEX.md may reference decisions that inform current architecture; discarding them breaks traceability.
- **NEVER save with a generic or missing stream name when multiple streams are active** — Ambiguous names cause the wrong file to be loaded next session. **Why:** `CONTEXT-llm.md` collides across unrelated workstreams and forces manual disambiguation.
- **NEVER skip the INDEX.md upsert step** — Omitting it leaves the index out of sync. **Why:** `load-context` relies on `INDEX.md` to discover the latest file; a stale index means the wrong snapshot is loaded.
- **NEVER archive to `done/` prematurely** — Moving an active context to `done/` hides it from future loads. **Why:** Auto-archive is only correct when status is explicitly `done` or `parked`; applying it to in-progress work severs continuity.
- **NEVER exceed the 1500-token budget** — Bloated context files slow down load and waste model capacity. **Why:** The target is 1200-1500 tokens; beyond that the signal-to-noise ratio degrades and compaction may truncate critical sections.

## Usage Examples

**Saving context before ending a long session:**

```bash
# Invocation (default stream)
/save-context

# Files written to .context/session/
# .context/session/CONTEXT-llm.md        ← session snapshot
# .context/session/INDEX.md              ← updated index entry
```

**Saving context under a named stream:**

```bash
# Invocation with explicit stream name
/save-context auth-refactor "completed JWT middleware extraction"

# Files written to .context/session/
# .context/session/CONTEXT-auth-refactor-llm.md   ← named snapshot
# .context/session/INDEX.md                        ← updated index entry
```

**Archiving a completed stream:**

```bash
# When status resolves to "done" the skill auto-archives
/save-context auth-refactor "all tests green, PR merged" --status done

# Result
# .context/session/done/CONTEXT-auth-refactor-llm.md  ← archived
# .context/session/INDEX.md                            ← updated with archived status
```

## References

- [Reference](reference.md) — CONTEXT file template, status mapping, INDEX.md upsert logic, auto-archive rules, and token budget constraints
