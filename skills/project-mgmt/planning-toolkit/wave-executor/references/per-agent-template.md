# Per-Agent Prompt Template

Fill the `{slots}` from the wave document phase row before passing to the `Agent` tool.
Append the model-tier addition from `model-tier-guide.md`.
Do not include these instructions in the actual prompt.

---

## Template

```
You are working in the {repo} repository.

## Your task

{focus — copy verbatim from the wave document Focus cell or phase file. Do not summarise.}

## Writes scope

{If a Writes column or phase file specifies scope, state it here:}
You may only write to:
{writes}

{If no writes scope is specified, use:}
Limit your changes to files directly related to your task. Do not modify shared
index files, changelogs, or summary documents unless your task explicitly names them.

## Isolation

Other agents may be running in parallel in separate worktrees.
Only touch files within your writes scope.
Do not read files owned by another concurrent agent's worktree — this causes merge conflicts.

## When done

Verify:
1. Every file you created or modified exists and is non-empty.
2. Any script or command named in your task ran without error.
3. No file outside your writes scope was modified — check with: git diff --name-only

Report:
- Files written (exact paths).
- Any decision or assumption you had to make.
- Any blocker that prevented full completion.

{model_tier_addition}
```

---

## Model-tier additions

Replace `{model_tier_addition}` with the block matching the phase's `Model` value.

### haiku

```
Work mechanically. Follow each step in the task description in order.
Do not infer, expand, or improve beyond the stated scope.
If any step is ambiguous, take the most conservative interpretation and note it in your report.
```

### sonnet

```
Use the skill or script named in your task. Follow its output format exactly.
Verify that any file you produced matches the expected structure before reporting done.
If the skill requires a confirmation sub-step, take the default/conservative option and note it.
```

### opus

```
You may read any existing file in the repo to inform your analysis.
Cite every file you read by path. If evidence is weak, contradictory, or absent, say so
explicitly — do not hedge silently or fill gaps with plausible-sounding inference.
Depth and accuracy matter more than speed.
```

---

## Worked example

Wave document row (phase table, Wave 0):

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| `feat/triage-memory` | Triage Mem0, Zep, MemoryOS, Letta via `tessl__triage-tool` | 4 | Pending | sonnet |

Filled prompt:

```
You are working in the agentic-context repository.

## Your task

Triage Mem0, Zep, MemoryOS, Letta via `tessl__triage-tool`

## Writes scope

You may only write to:
references/*.md

Limit your changes to files directly related to your task. Do not modify shared
index files, changelogs, or summary documents unless your task explicitly names them.

## Isolation

Other agents may be running in parallel in separate worktrees.
Only touch files within your writes scope.
Do not read files owned by another concurrent agent's worktree — this causes merge conflicts.

## When done

Verify:
1. Every file you created or modified exists and is non-empty.
2. Any script or command named in your task ran without error.
3. No file outside your writes scope was modified — check with: git diff --name-only

Report:
- Files written (exact paths).
- Any decision or assumption you had to make.
- Any blocker that prevented full completion.

Use the skill or script named in your task. Follow its output format exactly.
Verify that any file you produced matches the expected structure before reporting done.
If the skill requires a confirmation sub-step, take the default/conservative option and note it.
```

Corresponding `Agent` call:

```
Agent(
  subagent_type="general-purpose",
  model="sonnet",
  isolation="worktree",
  description="Wave 0: feat/triage-memory",
  prompt=<above>
)
```
