---
name: how
description:
  'Use for "how does X work", code walkthroughs before changing something, and placement / ownership / layering questions ("where should this live", "which package owns this", "is this the right
  layer"). Explains subsystem architecture, runtime flow, onboarding mental models. Can critique architecture. DO NOT use for design rationale or history (use why instead), or for proving a change is
  safe to ship (use blast-radius instead). Triggers: how does X work, walk me through X, where should this live, critique this architecture, whats wrong with this design.'
---

# How

> Ported from the `how` skill in [cursor/plugins pstack](https://github.com/cursor/plugins/tree/main/pstack/skills) — Cursor, a separate AI-coding-editor product, not to be confused with a text/DB
> cursor — for use with this harness's `Agent` tool. Agent-spawning mechanics below use this repo's actual subagent types (`Explore`, `general-purpose`) and Claude model/effort params
> (`sonnet`/`opus`/`haiku`/`fable`, `low`/`medium`/`high`/`xhigh`/`max`) in place of the original's model names specific to that other editor, and its `readonly` flag.
>
> **One more adaptation, not cosmetic:** the original has every explorer/synthesizer/critic use Glob/Grep/Read directly. If your environment has a code-graph MCP (e.g. `tokensave`,
> `code-review-graph`), prefer its structural query tools (`tokensave_context` and friends) over raw file search — they return relevant source directly and track symbols across renames, which
> Glob/Grep/Read cannot do. Every codebase-reading step below names the code-graph tool first and Glob/Grep/Read as the fallback when no such tool is connected. If your code-graph tool supports a
> per-worktree index, re-index the current worktree before relying on it — a stale index can silently omit symbols only changed there.

Explore the codebase to answer "how does X work?" questions. Produce clear architectural explanations at the level of a senior engineer onboarding onto a subsystem. Enough to build a working mental
model, not annotated source code.

Two modes:

1. **Explain** (default). Explore the codebase and produce a clear explanation
2. **Critique.** Explain first, then spawn multiple independent reviewers to identify architectural issues

## Prerequisites

- A question scoped to this codebase, not a general architecture question with nothing to explore
- For critique mode specifically: the explanation from Explain mode already produced, since critique never runs standalone
- No code-graph MCP is required — Glob/Grep/Read work, a code-graph tool (e.g. `tokensave`) is just cheaper when one is connected

## When to Use

- "How does X work?" — a subsystem, feature flow, or architectural overview.
- "Walk me through what happens when..." — a runtime trace through the codebase.
- Placement/ownership/layering questions: "where should this live", "which package owns this", "is this the right layer".
- Architecture critique: "what's wrong with this design", "critique this architecture" — always after an explanation, never standalone.

## When NOT to Use

- The question is about design rationale or history ("why was this built this way") — use `why` instead.
- The question is about whether a change is safe to ship ("what could this break") — use `blast-radius` instead.
- The question has nothing to explore in this codebase (a general architecture question with no concrete target).

## Verification

Before treating the explanation as done, run a spot check: pick one non-obvious claim and re-run `tokensave_context` on it to confirm the output still matches what the explanation says. Run this check
the same way you'd run tests before shipping code — confirm the actual output agrees with the claim, don't just trust the first pass.

## Mindset

- Write the explanation for a senior engineer who has never seen this subsystem — assume no prior context, but don't re-explain language or framework fundamentals
- `tokensave_context`'s returned source is the ground truth for a claim; a symbol's name, a stale comment, or an explorer's paraphrase is not
- In critique mode, an empty critique is a valid, honest outcome — don't manufacture a finding just to justify having spawned critics
- State your best-guess interpretation of an ambiguous question and proceed; let the user redirect rather than blocking on a clarifying question first

## Explain Mode

### Step 1. Understand the Question and Assess Complexity

Parse what the user is asking about:

- "How does the rate limiter work?", a subsystem
- "How do we handle billing for on-demand usage?", a feature flow
- "How is the auth service structured?", an architectural overview
- "Walk me through what happens when a user submits a form", a runtime trace

Identify the scope. If ambiguous, state your best-guess interpretation before exploring. Don't ask. Let the user redirect if you're off.

**Assess complexity to decide the approach:**

- **Simple** (a single module, a small utility, a narrow question like "how does function X work"): skip explorer agents; explore and explain in a single pass yourself, or one agent. Go to Step 2b.
- **Complex** (a subsystem spanning multiple files/services, a cross-cutting feature, a full architectural overview): spawn parallel explorer agents first, then hand off to a synthesizer. Go to Step
  2a.

When in doubt, lean simple. You can always spawn explorers if the explainer hits a wall.

### Step 2a. Explore (complex questions only)

Decompose the question into 2-4 parallel exploration angles, each a distinct slice of the subsystem so explorers don't duplicate work. Example split for "how does the rate limiter work?":

- Explorer 1: data model and state management
- Explorer 2: request path and enforcement
- Explorer 3: configuration and metrics infrastructure

The right decomposition depends on the question. Use your judgment. Narrow questions: 2 explorers is fine. Broad subsystems: up to 4.

Spawn all explorers in a single `Agent` message so they run concurrently:

- `subagent_type`: `Explore` (its tool grant is "everything except `Agent`/`Artifact`/`ExitPlanMode`/`Edit`/`Write`/`NotebookEdit`" — that includes MCP tools generally, `tokensave_*` among them;
  there's no separate "read-only MCP tools only" filter, so the no-Read/Grep/Glob constraint below is a prompt instruction the explorer follows, not a tool restriction the harness enforces)
- `model`: omit to inherit the session model unless the subsystem is unusually large or unfamiliar, in which case set a higher-effort model explicitly

Each explorer gets the same base prompt from `references/explorer-prompt.md` plus a specific exploration angle naming its slice. If a code-graph MCP is connected, each explorer's exploration tool
should be its context-query tool (e.g. `tokensave_context`, supplemented by `tokensave_search`/`tokensave_callees`/`tokensave_callers`/`tokensave_impact` for depth) rather than Glob, Grep, Read, or
list_directory — the source sections it returns are the relevant code. Without a code-graph MCP, fall back to Glob/Grep/Read. Each explorer should:

- Start broad: call `tokensave_context` with its exploration angle as the task, in plain English
- Follow the thread: from what comes back, trace the call chain (callers, callees, data flow, type definitions) using `tokensave_callees`/`tokensave_callers`/`tokensave_impact` as needed, and further
  `tokensave_context` calls for anything the first pass didn't cover — passing `seen_node_ids` from each response into the next call's `exclude_node_ids`
- Trust the returned source, don't guess from symbol names alone
- Stop when it can describe the full path from input to output (or trigger to effect) without hand-waving any step
- Note things that are surprising, non-obvious, or that a newcomer would get wrong

Each explorer returns structured findings: components found, flow traced, files/symbols consulted, anything non-obvious. Overlap between explorers is fine; the synthesizer reconciles.

Then proceed to Step 3.

### Step 2b. Direct Explain (simple questions)

For a narrow question, either explore and write the explanation yourself in one pass, or spawn a single agent:

- `subagent_type`: `Explore` for the exploration, or `general-purpose` if you want the agent to also draft the writeup
- `model`: omit to inherit the session model

Read `references/explainer-prompt.md` for the communication style and output format. Same structure as the synthesizer below, just no explorer findings as input — the agent does its own exploration
(via a code-graph MCP's context-query tool when one is connected, otherwise Glob/Grep/Read) and writes the explanation directly.

Proceed to Step 4.

### Step 3. Synthesize (complex questions only)

Once all explorers return, spawn a single agent to synthesize their findings into one coherent explanation:

- `subagent_type`: `general-purpose` (needs to reconcile/verify against the codebase, not just search)
- `model`: omit to inherit the session model, or escalate (e.g. `opus`, effort `high`) for a large or contentious subsystem

The synthesizer gets all explorers' findings and writes the human-facing explanation (output format below). Read `references/explainer-prompt.md` for the full prompt template. It reconciles
overlapping findings, resolves contradictions, and weaves the slices into a unified picture — verifying against the codebase via the code-graph MCP when one is connected, raw Read/Grep/Glob otherwise.

### Step 4. Present

Before presenting, verify: pick at least one non-obvious claim in the explanation and re-run the code-graph query (or re-read the source directly) to confirm it still says what the explanation says
it says. This is cheap and catches the case where an explorer's summary drifted from what the code actually returned. Then present the synthesizer's output to the user. You may lightly edit for clarity or add context
from the conversation, but don't substantially rewrite. The explanation is the product.

### Output Format

Follow this structure, adapted to the question. Not every section is needed for every question.

**Overview.** 1-2 paragraphs. What it is, what it does, why it exists. Enough to decide whether to keep reading.

**Key Concepts.** The important types, services, or abstractions. Brief definition of each. Not exhaustive, just the ones needed to understand the rest.

**How It Works.** The core of the explanation. Walk through the flow: what triggers it, what happens step by step, where data goes, the decision points. Prose, not pseudocode. Reference specific files
and functions so the reader can go look, but don't dump code blocks unless a snippet is genuinely necessary.

**Where Things Live.** A brief map of the relevant files/directories. Not every file, just the ones needed to start working in this area.

**Gotchas.** Non-obvious or surprising things that would trip someone up. Historical context that explains why something looks weird. Known sharp edges.

## Critique Mode

Triggered when the user asks for architectural issues, problems, or improvements, not just understanding.

### Step 1. Explain First

Run the full explain flow above (Steps 1-4). You must understand the architecture before critiquing it.

### Step 2. Spawn Critics

After the explanation is complete, spawn several independent architectural critics in a single message so they run concurrently and don't see each other's output. Vary the model and/or reasoning
effort across critics so they read the architecture from genuinely different angles rather than converging on the same take — for example: `sonnet`/`high`, `opus`/`xhigh`, `sonnet`/`max`,
`fable`/`high`. Three or four critics is usually enough; scale down for a small subsystem.

For each critic:

- `subagent_type`: `general-purpose` (needs to check the actual code, not just search)
- `model` / `effort`: one distinct combination per critic, per above

Read `references/critic-prompt.md` for the prompt template. Each critic gets:

1. The explanation from Step 1 (so they don't re-explore)
2. The relevant file paths (so they can pull the actual code via `tokensave_context`/`tokensave_node`, not raw Read)
3. The architectural critique rubric from `references/critique-rubric.md`

### Step 3. Lead Judgment

You're a pragmatic lead, not an aggregator.

Categorize findings:

- **Act on.** Architectural problems worth fixing now
- **Consider.** Real concerns, but the cost/benefit is unclear
- **Noted.** Valid observations, low priority
- **Dismissed.** Wrong, missing context, or style preference

Present the explanation first (from Step 1), then the critique verdict below it. The explanation should stand on its own; someone who just wants to understand the system shouldn't wade through
critique.

## Troubleshooting

- **The code-graph query returns nothing relevant to the question:** try a narrower search for the specific symbol name instead of a broad task description, or adjust the path scope — don't fall
  back to Glob/Grep/Read just because the first call came up empty.
- **Explorers contradict each other:** the synthesizer re-checks the disputed point directly against the source rather than picking whichever explorer sounded more confident.
- **A question that looked narrow turns out to span multiple files once you start exploring:** switch to the complex-question path (parallel explorers, Step 2a) mid-stream rather than forcing a
  single-pass explanation to cover ground it can't.
- **Critics converge on the same finding despite varied model/effort combinations:** that's a legitimate corroboration signal, not a sign something went wrong — note it as strongly confirmed rather
  than suspicious.
- **The subsystem is too large for even 4 explorer angles to cover:** narrow the question back to the user rather than spawning more agents than the 2-4 range calls for; ask which part matters most.

## Common Failure Modes to Avoid

**NEVER** conclude what a symbol does from its name alone.

**WHY:** `tokensave_context` returns the actual source; a name can be stale, misleading, or copied from an unrelated pattern elsewhere in the codebase. The returned code is the source of truth, not
the identifier.

**NEVER** dump a large code block into the explanation just because you have it.

**WHY:** the explanation should read as prose a senior engineer can follow without cross-referencing a wall of code. A snippet earns its place only when it clarifies something prose genuinely can't.

**NEVER** let critics see each other's findings before each submits their own, in critique mode.

**WHY:** seeing another critic's output invites anchoring toward the same conclusions, defeating the point of spawning several independent reviewers with deliberately varied model/effort combinations.

**NEVER** do line-level code review in critique mode.

**WHY:** critique mode is scoped to architectural problems — abstraction fit, data model, boundary discipline, evolution readiness. Line-level nits are a different job and dilute the critic's signal.

**NEVER** flag missing abstraction, or suggest a rewrite, without naming the concrete problem it would solve.

**WHY:** "this could use more abstraction" or "this should be rewritten" with no demonstrated problem is the single most common valueless finding in architecture review — it isn't actionable and
erodes trust in the rest of the critique.

**NEVER** flag a deliberate, well-reasoned tradeoff as an architectural issue just because an alternative exists.

**WHY:** architecture is built from tradeoffs. Penalizing a sound one for not being the only possible choice produces noise, not signal — an empty critique is a valid, honest outcome when the
architecture holds up.
