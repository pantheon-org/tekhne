---
name: challenge
description: "Challenge, push back, play devil's advocate on AI output. Use when: challenge this, are you sure, push back, prove it, what if you're wrong, devil's advocate, stress test, poke holes, second opinion, sanity check, too confident, really?, question this decision. Subcommands: anchor (committed too fast), verify (facts wrong?), framing (wrong problem?), deep (full devil's advocate in separate context)."
allowed-tools: read, glob, grep, agent, askuserquestion
model: opus
context: main
argument-hint: "[anchor|verify|framing|deep] <target>"
user-invocable: true
cynefin-domain: complicated
cynefin-verb: analyze
---

# Challenge

Apply structured provocation patterns to force reconsideration of current work.

**Target:** **$ARGUMENTS**

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known Claude Code bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known Claude Code bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Dispatch

Parse first word of $ARGUMENTS as subcommand:

| Subcommand | Error Type | Protocol |
|---|---|---|
| `anchor` | Premature commitment / anchoring bias | Read `references/protocols/anchor.md` → execute |
| `verify` | Factual errors / hallucination | Read `references/protocols/verify.md` → execute |
| `framing` | Wrong problem / framing errors | Read `references/protocols/framing.md` → execute |
| `deep` | High stakes — all 9 patterns in fresh context | Spawn devil-advocate sub-agent via Agent |

## No-Subcommand Fallback

If no subcommand detected:

AskUserQuestion: "What are you worried about with the current AI response?"
- A) Anchoring bias — AI committed too early to one approach
- B) Factual accuracy — claims may be wrong or hallucinated
- C) Wrong framing — solving the wrong problem
- D) High stakes — want all 9 patterns in fresh context (Devil's Advocate)

→ Dispatch to matching subcommand based on answer.

## Deep Subcommand

Spawn via Agent tool a devil's advocate sub-agent with:
- prompt: target description + relevant file paths to read
- The agent runs ALL 9 patterns (anchor: Gatekeeper, Reset, Alt Approaches, Pre-mortem · verify: Proof Demand, CoVe, Fact Check List · framing: Socratic, Steelman) comprehensively in fresh context
- DO NOT pass parent conversation reasoning — fresh context is the point

## Thinking Transparency (applies to all subcommands)

For every finding, make reasoning explicit:

1. **Observation**: What specifically in the target triggered this finding
2. **Technique family**: Which challenge family (anchor/verify/framing) and named pattern (e.g., Gatekeeper, CoVe, Steelman) — cite mechanism from `references/reference.md` pattern catalog
3. **Reasoning**: Why this observation matters — what cognitive bias or error it reveals
4. **Confidence**: How certain is this finding (High/Medium/Low) and what evidence supports that rating

## Output

All subcommands produce a **Challenge Report** (structured, not prose).
See `references/reference.md` for report format and pattern catalog.

## When to Use

- An AI assistant or team member has proposed a solution without exploring alternatives — use `anchor` to surface premature commitment.
- Output contains specific facts, citations, or confident numerical claims that have not been verified — use `verify` to stress-test accuracy.
- The stated problem feels mis-framed or the scope is suspiciously narrow — use `framing` to question whether the right problem is being solved.
- A proposal carries high organisational or technical risk (irreversible change, security boundary, large refactor) — use `deep` to run all nine patterns in a clean context.
- A decision is about to be committed to (PR merged, ticket closed, design doc signed off) and no one has played devil's advocate yet.

## When Not to Use

- The proposal is already in production and rollback is not possible — retrospective challenges generate friction without actionable outcomes; use a post-mortem instead.
- The work is exploratory or speculative and the author has explicitly labelled it a spike — early challenges can kill useful divergent thinking before it matures.
- A prior challenge session has already been run on the same target with the same subcommand and no new evidence has emerged.
- The requester is looking for encouragement or morale support, not critical analysis — misapplying challenge to emotional contexts causes harm.
- Time budget is under five minutes and the stakes are low — the protocol overhead exceeds the value; a quick gut-check comment is sufficient.

## Anti-Patterns

- **NEVER challenge without the anchor protocol first** — Unconstrained challenges devolve into unproductive debate. **Why:** The anchor establishes shared facts before disagreement; without it, participants argue from different baselines.
- **NEVER challenge the person, only the proposal** — Personal challenges trigger defensiveness and shut down learning. **Why:** The goal is to stress-test the idea, not the author; role-separate the proposal from the proposer explicitly.
- **NEVER issue a challenge without proposing an alternative** — Pure objections without alternatives leave teams stuck. **Why:** A challenge that can't be resolved produces friction, not insight; offer a "what if instead..." path.
- **NEVER run `deep` on a trivial or low-stakes target** — All nine patterns applied to a minor decision wastes cognitive bandwidth. **Why:** `deep` is calibrated for high-stakes irreversible choices; applying it broadly devalues the signal and desensitises the team.
- **NEVER suppress or soften findings to avoid conflict** — Diluted challenge output is worse than no challenge at all. **Why:** The entire value of this skill is honest, evidence-backed pushback; hedging findings defeats the purpose and misleads decision-makers.

## Usage Examples

**Challenging a proposed architecture decision:**
```bash
# Proposal: "Use a monorepo for all 40 services"
# Skill applies anchor: "What problem does this solve? What is the current pain?"
# Skill applies framing: "What alternatives were considered? What are the failure modes?"
# Skill applies verify: "Can we test this with 3 services first?"
```

**Challenging an implementation shortcut:**
```bash
# Proposal: "Just hardcode the config values for the demo"
# Skill challenges: "What is the rollback path? When does this become permanent?"
# Output: Structured challenge with anchor + alternative approach
```

**Running a full devil's advocate review before a design doc is signed off:**
```bash
# Target: architecture decision record for event-sourcing migration
# Invoke: /challenge deep path/to/adr.md
# Skill spawns a sub-agent with all 9 patterns (anchor x4, verify x3, framing x2)
# Output: Challenge Report with confidence ratings and ranked findings
```

## References

- [Reference](references/reference.md) — pattern catalog (9 patterns), when-to-use guide, and Challenge Report format
- [Anchor Protocol](references/protocols/anchor.md) — Gatekeeper, Reset, Alternative Approaches, Pre-mortem execution steps and output template
- [Framing Protocol](references/protocols/framing.md) — Socratic and Steelman execution steps and output template
- [Verify Protocol](references/protocols/verify.md) — Proof Demand, CoVe, Fact Check List execution steps and output template
