---
name: wave-executor
description: "Execute a wave-based plan by spawning parallel agents per wave, enforcing merge gates, and building self-contained per-agent prompts from wave document rows. Use when asked to: run the plan, execute the plan, start the waves, launch agents, execute wave N, run wave execution. Reads a wave document produced by wave-execution-planner."
allowed-tools: Read, Bash, Agent
---

# Wave Executor

Execute a wave document by spawning parallel agents per wave, pausing at merge gates, and verifying each wave before advancing.

## When to Use

- User says "run the plan", "execute the plan", "start wave N", "launch the agents"
- A wave document exists at `.context/plans/<slug>.md`
- The plan was produced by `wave-execution-planner` (or follows the same format)

## When Not to Use

- No wave document exists — run `wave-execution-planner` (Mode A) first
- The user wants to update wave status only — use `wave-execution-planner` (Mode B) instead
- The plan has only 1–2 sequential tasks with no parallelism — just run them directly

## Workflow

### 1. Locate the plan

Ask for the plan slug if not provided. Load `.context/plans/<slug>.md`.

Read the full document. Identify all waves in order and their current status. Skip waves already marked `— DONE`.

### 2. For each pending wave — execute the loop

Repeat steps 2a–2g for every wave that is not `— DONE`, in order.

#### 2a. Check the gate

Read the `> Gate:` blockquote at the top of the wave. If it requires a prior wave to be verified, confirm that wave's `Verification:` checklist is fully ticked (`[x]`). If not, stop and report the unmet gate to the user.

#### 2b. Extract phases

Parse the wave table or inline task list. For each row / item capture:

- **branch** — the branch name. In phase tables: the phase identifier (use as branch if no explicit branch). In inline tasks: the value after `— branch`.
- **focus** — the task description (Focus column, or phase file content if linked).
- **model** — the `Model` column value, or `— model:` suffix on inline tasks. Default: `sonnet` if absent.
- **writes** — if a `Writes` column or `— writes:` annotation is present, capture it. Otherwise derive from the phase file, or leave unspecified and let the agent determine scope from the focus.

#### 2c. Build per-agent prompts

Read `references/per-agent-template.md`.

For each phase, fill the template slots with the values from step 2b. Append the model-tier addition from `references/model-tier-guide.md` matching the phase's `model` value.

**Do not paraphrase the focus / task description. Copy it verbatim.**

If the phase links to a phase file, read that file and include its full content under "Your task" rather than only the Focus cell value.

#### 2d. Spawn agents in parallel

Send a **single response** containing one `Agent` tool call per unblocked phase.

Each call must set:
- `subagent_type`: `"general-purpose"`
- `model`: the exact tier string from step 2b (`"haiku"` / `"sonnet"` / `"opus"`)
- `isolation`: `"worktree"` for all phases that commit to a branch (omit for phases that commit directly to main)
- `description`: `"Wave N: <branch>"`
- `prompt`: the filled per-agent prompt from step 2c

#### 2e. Wait for all agents to complete

Do not proceed until every agent in the wave has returned.

#### 2f. STOP — merge gate

After all agents complete, report to the user:

> "Wave N agents done. Please merge the following branches to `main` before I continue:
> [list branches]
>
> Then run the Verification checklist for this wave (copied below) and reply **'verified'** when all checks pass."

Copy the wave's `Verification:` checklist verbatim into your message so the user can run each item.

Wait for the user to reply before continuing.

#### 2g. Update wave status and advance

After the user confirms verification:

1. Run `wave-execution-planner` Mode B on the wave document to tick the verification checkboxes and mark the wave `— DONE`.
2. Report which wave is now unblocked.
3. Continue the loop with the next wave.

### 3. Report completion

After all waves are done:
- Summarise what was executed (waves, branches, agents).
- Confirm the plan's `**Status**` field is set to `Complete`.
- List any decisions or blockers flagged by agents during execution.

## Anti-Patterns

### NEVER paraphrase the task description

**WHY**: Information loss — the verbatim focus or phase file contains constraints, tool names, and scope boundaries that summaries drop.

**BAD** Summarise "Triage Mem0, Zep, MemoryOS, Letta via `tessl__triage-tool`" as "triage memory tools".
**GOOD** Copy the full text verbatim into the per-agent prompt.

### NEVER omit the `model` parameter on Agent calls

**WHY**: Omitting it silently defaults to the orchestrator's model. For `haiku`-tier mechanical tasks, this wastes significant cost with no quality benefit.

**BAD** `Agent(subagent_type="general-purpose", prompt=...)` with no `model`.
**GOOD** `Agent(model="haiku", ...)` as specified in the wave document.

### NEVER start the next wave before the gate passes

**WHY**: Consolidation agents read files written by prior waves. Unmerged branches mean stale reads and incorrect output that is hard to detect.

**BAD** Proceed to Wave N+1 as soon as Wave N agents return their results.
**GOOD** STOP, ask the user to merge and verify, confirm gate before advancing.

### NEVER spawn wave agents sequentially

**WHY**: Defeats the parallelism that wave planning was designed to achieve.

**BAD** Spawn one agent, wait for it, then spawn the next.
**GOOD** Send a single response with all parallel agents in one message.

### NEVER spawn agents for phases blocked by unmet dependencies

**WHY**: A phase that needs a prior branch merged will read stale state and produce incorrect output.

**BAD** Spawn `feat/synthesis` before `feat/rubric` is merged (it's in the `Blocked on` note).
**GOOD** Check each phase's gate / blocked-on annotation; skip and report blocked phases.

## References

- [Per-agent prompt template](references/per-agent-template.md) — fill for every spawned agent
- [Model tier guide](references/model-tier-guide.md) — when to use haiku / sonnet / opus
- Wave format spec — defined in the `wave-execution-planner` skill (`wave-execution-planner/references/wave-format.md`)
- Status tracking protocol — defined in the `wave-execution-planner` skill (`wave-execution-planner/references/status-tracking.md`); used in step 2g
