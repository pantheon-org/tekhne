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

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known harness bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known harness bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Workflow

```text
0.Load → 1.Search → 2.Qualify → 3.Diagnose → 4.Investigate → 5.Persist → 6.Learn
```

### 0. Load Learnings

If a project-level learnings file exists (e.g., `learnings.yaml` in the project root or a known location), read it and apply known patterns before searching.

### 1. Search (do first)

80% of bugs solved online.

- WebSearch: `[error] [stack] [framework]` on SO, GitHub, Docs, Reddit

```text
"Type 'string | undefined' is not assignable to type 'string'" typescript
```

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

```bash
git bisect start
git bisect bad HEAD
git bisect good <last-known-good-sha>
```

1. **Root cause drill**: 5 Whys, Fishbone 6 M's

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

### NEVER jump to a fix without reproducing the issue

**WHY:** Reproduction confirms you are solving the actual problem, not a symptom; untested fixes introduce regressions.

**BAD:** "Roll back the SDK upgrade" on the strength of a support ticket.
**GOOD:** Reproduce the failure first, then change one variable and observe.

### NEVER ignore the source code location step

**WHY:** Vague "somewhere in the auth module" diagnoses cannot be verified or code-reviewed.

**BAD:** "The problem is in the user service."
**GOOD:** "Root cause at `src/api/auth.ts:142`: token expiry not caught."

### NEVER treat correlation as causation

**WHY:** Fixing the correlated symptom leaves the root cause intact.

**BAD:** Blame the deploy because errors started the same afternoon.
**GOOD:** Isolate with a swapped variable or `git bisect` before naming a cause.

### NEVER skip the search step

**WHY:** About 80% of bugs have documented answers; skipping search wastes diagnostic cycles.

**BAD:** Ask five qualifying questions before running any search.
**GOOD:** WebSearch the error, stack and framework first, then qualify.

### NEVER persist a thinking artifact before root cause is confirmed

**WHY:** Premature writes capture incomplete hypotheses as conclusions, and misleading notes repeat the misdiagnosis in future sessions.

**BAD:** Write the artifact mid-investigation because someone wants a write-up.
**GOOD:** Write it once the root cause is confirmed and the fix is verified.

### NEVER proceed on empty AskUserQuestion answers

**WHY:** An empty answer means the question was never shown, so any assumption replaces the user's reply.

**BAD:** Treat blank answers as "no recent changes" and continue.
**GOOD:** Present a numbered text list and wait for the user's choice.

### NEVER enter OODA before diagnosis is exhausted

**WHY:** Pattern matching, isolation and root cause drilling resolve many bugs cheaply; OODA costs the user a round trip per loop.

**BAD:** Ask the user to run a new command as the first step.
**GOOD:** Run mental models, isolation and the root cause drill first, and hand to OODA only when they are inconclusive.

### NEVER run the user's commands for them

**WHY:** The workflow is human-executed; the user holds the access and must see what runs in their environment.

**BAD:** Claim to have restarted the service or run the migration.
**GOOD:** Give the command and ask the user to run it and paste the output.

### NEVER mix Complicated and Complex strategies

**WHY:** Complex problems cannot be diagnosed in advance, so forcing a root-cause hunt produces confident but wrong answers.

**BAD:** Declare a cause for a problem whose three competing theories have all failed before.
**GOOD:** Reclassify as Complex, note the Cynefin transition and route to a probe.

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

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Isolation and root cause | [references/reference.md](references/reference.md) | Wolf Fence, Swap One Variable, Minimal Repro, 5 Whys, Fishbone, OODA loop |
| Diagnose protocol | [references/protocols/diagnose.md](references/protocols/diagnose.md) | Mental model matching, isolation sequence, OODA handoff conditions |
| Multi-source search | [references/protocols/search-multi-source.md](references/protocols/search-multi-source.md) | Query construction, source priority and the no-result fallback |
| Handoff to framing | [references/troubleshoot-to-frame-problem-llm.md](references/troubleshoot-to-frame-problem-llm.md) | Passing an unresolved problem to frame-problem |
