---
name: troubleshoot
description: "Use when user reports an error, bug, or something not working. Search-first troubleshooting with diagnostic phase. Triggers: debug, error, broken, not working, failing, crash, exception."
allowed-tools: WebSearch, WebFetch, AskUserQuestion, Read, Glob, Grep
model: opus
context: main
argument-hint: <error or symptom description>
cynefin-domain: complicated
cynefin-verb: analyze
---

# Troubleshoot

Search-first diagnostic workflow. Human executes commands.

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known Claude Code bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known Claude Code bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Workflow

`0.Load → 1.Search → 2.Qualify → 3.Diagnose → 4.Investigate → 5.Persist → 6.Learn`

### 0. Load Learnings

If a project-level learnings file exists (e.g., `learnings.yaml` in the project root or a known location), read it and apply known patterns before searching.

### 1. Search (do first)

80% of bugs solved online.

- WebSearch: `[error] [stack] [framework]` on SO, GitHub, Docs, Reddit
- Solution found → skip to 5.Learn

### 2. Qualify (2-3 questions)

AskUserQuestion:
- Stack? (language, framework, runtime)
- Environment? (local, container, cloud)
- Changed recently? (deploy, config, dependency)

### 3. Diagnose

See `references/protocols/diagnose.md` for details.

1. **Mental models**: Check learnings file → WebSearch pattern → reason with 5 Whys/Fishbone
2. **Isolation**: Wolf Fence (binary search), swap one variable, minimal repro
3. **Root cause drill**: 5 Whys, Fishbone 6 M's

Pattern matches → suggest fix, skip OODA

### 4. Investigate (OODA)

Only if diagnosis inconclusive.

- Observe: User runs command, pastes output
- Orient: Analyze, update hypothesis
- Decide: Next command or confirm cause
- Act: Suggest fix (user executes)

Exit when root cause confirmed and fix verified.

### 5. Persist Thinking Artifact ⚠️ MANDATORY

**MUST execute after root cause confirmed and fix verified. DO NOT skip. DO NOT wait for user to ask.**

If a persistent thinking store is configured (e.g., via an environment variable pointing to a notes directory), write the diagnostic session there. Skip silently if no store is configured.

**Content** (required sections):
- Symptoms (as reported)
- Hypotheses tested (ordered list with result: confirmed/eliminated)
- OODA loops (if Phase 4 was entered)
- Root cause (as confirmed)
- Resolution (fix applied)
- Cynefin transition (if problem re-classified during diagnosis)

This is the **active thinking trail** — distinct from any learnings file which captures distilled, reusable conclusions.

### 6. Learn

After resolution, AskUserQuestion: "Save this learning?"
- Global → append to a shared learnings store
- Project → append with `scope: project:<name>`
- Skip

## Philosophy

- **Reproduce before you theorize** — a bug that cannot be reproduced is a hypothesis, not a confirmed problem.
- **Source location is the exit condition** — every investigation ends with a specific file and line, not a module or service name.
- **Distinguish symptom from cause** — the first observable failure is rarely the root cause; follow the chain.
- **Domain determines method** — Complicated problems have known solutions; Complex problems need probes; do not mix the strategies.

## When to Use

- When a user reports an error, exception, or crash with no clear cause
- When a system exhibits unexpected behavior that cannot be explained by recent changes
- When a bug is intermittent or flaky and cannot be reproduced consistently
- When log output or stack traces point to a symptom but not a root cause
- When a fix has been applied but the problem recurs or manifests differently

## When Not to Use

- When the bug is already reproduced and the fix is obvious — apply the fix directly
- When the request is a feature request framed as a bug report — redirect to planning
- When the issue is a known, documented limitation with no workaround — communicate clearly
- When the environment is completely unavailable (no logs, no reproduction path, no user access) — gather access first
- When a postmortem or incident review is needed instead of a live diagnostic — use a dedicated retrospective workflow

## Anti-Patterns

- **NEVER jump to a fix without reproducing the issue** — Untested fixes introduce regressions. **Why:** Reproduction confirms you're solving the actual problem, not a symptom.
- **NEVER ignore the source code location step** — All investigations must end with a specific file and line. **Why:** Vague "somewhere in the auth module" diagnoses cannot be verified or code-reviewed.
- **NEVER treat correlation as causation** — Two events occurring together don't mean one causes the other. **Why:** Fixing the correlated symptom leaves the root cause intact.
- **NEVER skip the search step** — Searching external sources first avoids reinventing known solutions. **Why:** 80% of bugs have documented answers; skipping search wastes diagnostic cycles.
- **NEVER persist a thinking artifact before root cause is confirmed** — Premature writes capture incomplete hypotheses as conclusions. **Why:** Misleading learnings files cause the same misdiagnosis in future sessions.

## Usage Examples

**Investigating a silent API failure:**
```bash
# Skill prompts: reproduce → isolate → check logs → locate source
# Output: "Root cause at src/api/auth.ts:142 — token expiry not caught"
```

**Triaging a flaky test:**
```bash
# Skill prompts: classify domain (Clear/Complicated/Complex)
# Routes to probe skill if Complex domain detected
```

**Diagnosing a performance regression after a deploy:**
```bash
# Skill prompts: qualify stack/environment/recent changes → Wolf Fence bisect
# Output: "Regression introduced in commit abc123 — N+1 query in UserService.list()"
```

## ✅ Completion Checklist

Before responding to user, verify:
- [ ] Artifact written (or skipped if no store configured)

## Refs

- `references/protocols/diagnose.md` - Mental models, bisect strategies
- `references/protocols/search-multi-source.md` - Multi-source search workflow
- `references/reference.md` - Isolation techniques and root cause methods

## References

- [Reference](references/reference.md) — isolation techniques (Wolf Fence, Swap One Variable, Minimal Repro), root cause drilling (5 Whys, Fishbone), OODA loop
- [Diagnose Protocol](references/protocols/diagnose.md) — mental model matching, isolation sequence, root cause drill, OODA handoff conditions
- [Search Multi-Source Protocol](references/protocols/search-multi-source.md) — query construction, source priority (SO, GitHub Issues, Docs, Reddit), no-result fallback
