---
name: blast-radius
description:
  "Find what a change could break somewhere else before it ships, beyond the diff, and prove the one fact it's safe because of by running real code instead of writing it up. Use for 'blast radius of
  X', 'what could this break', or reviewing a small diff you don't trust. DO NOT use for explaining how existing code works (use how instead) or investigating why it was built that way (use why
  instead). Triggers: blast radius of X, what could this break, is this change safe, review this diff, what am I missing in this change."
---

# Blast radius

> Ported from the `blast-radius` skill in [cursor/plugins pstack](https://github.com/cursor/plugins/tree/main/pstack/skills) — Cursor, a separate AI-coding-editor product, not to be confused with a
> text/DB cursor — for use with this harness's `Agent` tool. Two adaptations from the original:
>
> - Step 6's "run it as an `arena`" (ask several _different vendor_ models the same question) doesn't map onto this harness's `Agent` tool, which only spawns Claude-family models. The equivalent here
>   is spawning several independent `general-purpose` agents, each drawing a distinct `model`/`effort` combination from the same four-combination panel `how`'s critique mode uses — see Step 6 below.
> - "Write it through `unslop`" (a skill from that other editor for stripping AI-slop prose) has no port here — if your project enforces an equivalent prose-quality or AI-slop-detection hook, writing
>   the blast-radius writeup will get that scoring for free; otherwise apply the same discipline manually (no narrative comments, no hedge-padding, no filler).
>
> If your environment has code-graph MCP tools (e.g. `code-review-graph`'s `get_impact_radius_tool`/`get_affected_flows_tool`/`detect_changes_tool`, or `tokensave`'s `tokensave_callers`/
> `tokensave_impact`/`tokensave_context`), they do the mechanical "find the callers" part of Step 3 far more cheaply than grep — use them there instead of manual search when the target is in an
> indexed repo. They don't replace Steps 2, 4, and 5, which are the actual point of this skill.
>
> **Third adaptation: prefer a code-graph tool over raw grep, when one is connected.** The original has Step 3 read library source and grep by hand. If a code-graph MCP indexes the target repo,
> route through its context/search tools first, the same preference `how` and `why` apply. Raw `Read`/`grep` stay the fallback for anything the code-graph tool genuinely doesn't index: a pinned
> dependency's code under `node_modules`/`vendor`/`site-packages`, a file kind outside its indexed languages, or an environment with no such tool connected at all.

Find what a change breaks somewhere else, before it ships. Use for "blast radius of X", "what could this break", or reviewing a small diff you don't trust yet.

Companion to `how` and `why`. `how` tells you what the code does. `why` tells you why it's shaped that way. Blast radius tells you what it breaks somewhere else.

Listing the callers is not the job. `tokensave_callers`/`tokensave_impact`, or `code-review-graph`'s impact-radius tools, can find those in a second. The job is the breakage a caller list won't show
you.

## Prerequisites

- A concrete diff or change to review — a hypothetical "what if we changed X" has no diff to read and no code to run a proof against
- A code-graph MCP (e.g. `tokensave`, `code-review-graph`) speeds up the caller/impact sweep in Step 3 when connected, but Glob/Grep/Read work as a fallback
- Ability to actually run code in this environment (a script or test) — without it, step 4 on the how-sure-are-you ladder is unreachable and every fact stays capped at "unproven"

## When to Use

- "Blast radius of X", "what could this break" — finding what a change could break beyond the diff, before it ships.
- Reviewing a small diff you don't trust yet, or want a second, evidence-backed opinion on before merging.

## When NOT to Use

- The question is about what existing code does or how it works ("how does X work") — use `how` instead.
- The question is about why code was built a certain way ("why is this here") — use `why` instead.
- There's no concrete diff or change to review — a hypothetical with nothing to read has no proof to run.

## Verification

Before replying, run the same check step 5 already calls for: confirm the one safety fact is either proven, with the actual script/test output pasted in, or explicitly marked unproven. Run this check
the same way you'd run tests before shipping code — never submit a writeup where that status is ambiguous or implied rather than stated.

## Mindset

- A convincing-sounding writeup and a correct one are indistinguishable until something is actually run — proof, not prose, is the product
- "Unproven" is an honest, acceptable answer; a confident-sounding guess dressed up as settled is not
- A search that finds nothing is still a citable result — report it, don't let it become silence that reads as "not checked"
- Don't manufacture risks to look thorough; an empty "Risks" section next to a well-proven safety fact is a legitimate, complete answer

## Don't trust your own writeup

A blast-radius writeup that sounds right is worthless. It reads as convincing whether or not it's true, and that is the trap you are walking into. So don't hand back the writeup. Find the one or two
facts the whole thing depends on and prove them by running code. Words are where you start, not what you ship.

### How sure are you

For each fact the change's safety depends on, get it as far down this list as is cheap, and say where it stopped.

1. You said so. Worthless on its own.
2. You pointed at the line. A real `file:line`, or the library's own source.
3. You showed the bad case can't happen. You walked the failure step by step and it doesn't reach.
4. You ran it. A script or test that calls the real code and fails loud if you're wrong.
5. You reproduced it in the running app.

Any safety fact you can't get to step 4, say so out loud. Don't write it up as settled. Step 4 is usually one small script that imports the same library the app ships and calls the exact function
you're worried about.

## Steps

1. Read the change. The diff, the symbols it adds, changes, and deletes, and what it now does differently, including the part the diff doesn't spell out — pull surrounding context for that via
   `tokensave_context` rather than opening files cold. Use the `why` skill's Step 2 (code anchor: `tokensave_blame`/`tokensave_log`, PR/MR context) to pull the history if you need more than the diff
   shows.
2. Find the one fact it's safe because of. Most changes that look scary are safe because of a single fact, like "this call only drops already-dead cache entries and does nothing else". Find that fact.
   If it holds, most of the scary cases die at once. Spend your time here, not on a long list of maybes.
3. Look where a caller list stops. Check the source of the library you call — `tokensave_context`/`tokensave_search` first if it's inside this repo's own graph (a local patch, a vendored copy
   tokensave indexes), raw `Read` only for source outside that graph (a pinned dependency under `node_modules`/`vendor`/`site-packages`) — and check its pinned version and any local patch either way.
   Work out when things run: microtasks, unmount and teardown, framework-specific lifecycle quirks. Follow what a symbol search misses: the JSON an API returns, a DB column, a wire format, another
   language reading the same bytes, a feature flag, code three hops downstream. `tokensave_callers`/`tokensave_impact`, or `get_impact_radius_tool`/`get_affected_flows_tool` (code-review-graph),
   handle the direct-caller sweep; this step is about what those tools structurally cannot see.
4. Be honest about each risk. Give it a real chance of happening and a real cost if it does. Keep the risks you confirmed; list the ones you checked and cleared separately. Same rules as `why`. Cite a
   real `file:line`, a search that finds nothing is still an answer, and never make up a caller or an API.
5. Prove the one fact. Write a script or test that runs the real code, run it, and paste what happened. If you can't prove it cheaply, mark it unproven. Don't round up.
6. For a big or wide change, don't rely on a single pass. Spawn 2-4 independent `general-purpose` agents on the same question, each with a distinct `model`/`effort` combination drawn from
   `sonnet`/`high`, `opus`/`xhigh`, `sonnet`/`max`, `fable`/`high` (the same panel `how`'s critique mode uses) so they aren't all reasoning the same way — for 2 agents use the first two, for 3 the
   first three, for 4 use all of them. Then merge the answers yourself as the lead — keep a risk only if it survives your own read of the evidence, not just because one agent flagged it.

## Troubleshooting

- **Can't find a single fact the change is safe because of:** that itself is a finding — say so plainly rather than forcing a reassuring narrative onto a change that may genuinely be risky.
- **The proof script fails, or fails differently than expected:** that's a real result, not a bug to hide or retry silently — report exactly what happened; it may be the actual answer to "what could
  this break."
- **Proving the fact would take disproportionate effort:** mark it unproven and say what it would take to prove, rather than spending the rest of the budget forcing a proof or rounding up to "probably
  fine."
- **The change is too large for one pass to reason about confidently:** use Step 6's multi-agent panel rather than guessing alone or listing every conceivable risk to cover for the uncertainty.
- **`tokensave_context`/`tokensave_callers` don't cover a dependency (it's vendored, in `node_modules`):** that's the documented fallback case for raw `Read`, not a sign something's broken.

## What to hand back

- **What it does.** What changed, including the part that isn't obvious.
- **The one fact it's safe because of.** State it, say which step you got it to, and show the proof. If you couldn't prove it, write unproven.
- **Risks.** Only the real ones. Each names how it breaks, the `file:line`, how likely and how bad, and how to check. Paste the proof for the ones that matter.
- **Cleared.** What you checked and why it's fine.
- **Before you merge.** The cheapest test or repro that catches the real bug, including the script you wrote.

Cite real code, and strip anything private before it goes anywhere public.

## Common Failure Modes to Avoid

**NEVER** hand back the writeup as the final answer without proving the one fact the change's safety depends on.

**WHY:** a blast-radius writeup that sounds right is worthless — it reads as convincing whether or not it's true. Proof, not prose, is what this skill exists to produce.

**NEVER** treat a caller list as the deliverable.

**WHY:** `tokensave_callers`/`tokensave_impact`, or `code-review-graph`'s impact tools, can produce a caller list in a second. The job is the breakage those tools structurally cannot see — a caller
list on its own answers a question nobody asked.

**NEVER** write an unproven safety fact up as settled.

**WHY:** any fact that didn't reach step 4 ("you ran it") or step 5 ("you reproduced it live") on the how-sure-are-you ladder is still a guess with a plausible shape. Marking it unproven honestly is
more useful than a confident-sounding sentence that turns out wrong.

**NEVER** invent a caller, an API, or a search result that wasn't actually found.

**WHY:** a fabricated citation is worse than an honest "not found" — it misleads the reader into false confidence about coverage that doesn't exist.

**NEVER** round an unproven claim up to "probably fine" because proving it looked expensive.

**WHY:** step 4 is usually one small script that imports the same library the app ships and calls the exact function in question — cheap enough that skipping it to round up rarely saves effort, it
just defers the risk to whoever ships the change.

**Reply:** the writeup above, with the one safety fact either proven or marked unproven.
