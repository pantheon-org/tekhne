---
name: frame-problem
description: "Sense-making before action. Classify problem using Cynefin triangulation (3 tests + decomposition) to route to the right skill chain. Use when: frame, what approach, how should I start, which skill, where to begin, unsure what to do. NOT for known tasks — just do them."
allowed-tools: AskUserQuestion
model: opus
argument-hint: <task or problem to frame>
cynefin-domain: confused
cynefin-verb: decompose
---

# Frame

Sense-make → triangulate → decompose if needed → route. The domain decides the agent pattern, not just the skill.

**Framing:** **$ARGUMENTS**

## Mindset

- Keep every classification as a hypothesis you test against the user's answers, never a verdict you announce.
- Check your own confidence with suspicion. You hold expert knowledge on nearly everything, so every problem looks analysable to you and drifts towards Complicated.
- Use what people can observe (history, repeatability, separability) and derive the domain yourself. Self-classification by constraint type is unreliable.
- Stop at the handoff. Framing never proposes a fix, a diagnosis or an implementation.

## AskUserQuestion Guard

Some agentic harnesses silently return blank answers from `AskUserQuestion` outside their planning mode, with no UI shown. Check the answers after EVERY call.

1. Stop if any answer is empty or blank, and do not proceed on assumptions.
2. Write the notice below, with the options as a numbered text list.
3. Do not continue until the user replies with a number.

```text
Questions didn't display (known harness bug outside planning mode).
Reply with the number of your choice:
1) [option one]  2) [option two]  3) [option three]
```

## Procedure

### Step 0. Auto-classify (skip when $ARGUMENTS is empty)

1. Read $ARGUMENTS for constraint language: "was working, now failing" signals Degraded, "we think X causes Y" signals a hypothesis, "we need a <thing>" signals a solution-shaped statement.
2. Check your confidence. Below 80%, go to Step 1.
3. Run the Adjacent Domain Challenge before you propose anything, at 80% or above. Use one or two sentences to argue for the nearest neighbouring domain, then say why the original holds or does not.
4. Test for Complex whenever you land on Complicated: would two experts disagree, is the combination genuinely novel, has this exact combination been tried here?
5. Use this shape for the proposal:

```text
🎯 Auto-classified: [Domain] (constraint: [type]) → Verb: [probe|analyze|execute|act|decompose]
→ Proposed route: [skill chain]
⚖️ Adjacent challenge: what if this is [nearest domain]? [argument, and why the original holds]
Confirm? [Yes / Re-classify manually]
```

### Step 1. Triangulate

Use a single `AskUserQuestion` call for three concrete questions the user can answer accurately. Load `references/triangulation-tests.md` for the exact wording and options.

- **T1** asks who has done this before (Keogh scale), and measures whether expertise exists.
- **T2** asks whether the same inputs give the same result, and measures predictability.
- **T3** asks whether the problem can be taken apart and reassembled, and measures entanglement.
- **Q-Scale** asks boulder or pebble. Avoid asking it when the answer to T2 is Chaotic.
- **Q-Complicated sub** applies only to T1=Complicated with T2=ordered: Evolving → `investigate`, Degraded → `troubleshoot`, Both → `investigate` with a `troubleshoot` sub-task.

Write 2-3 clarifying sub-questions first when $ARGUMENTS is vague or broad, and put them in the same call.

Read the answers by majority. Classify with high confidence when all three agree. Use the majority when two of three agree, and report the dissenting test as a liminal signal. Start Step 1.5 when all three disagree or T3 answers composite. Do not let T1 win alone against T2 and T3.

### Step 1.5. Decompose (tests disagree or the problem is composite)

Snowden's rule: if you cannot agree on it, break it down until you can.

1. Define 2-4 sub-problems from $ARGUMENTS.
2. Apply the three tests to each sub-problem from context. Do not re-ask the user.
3. Use this domain map shape:

```text
🧩 Composite problem, sub-parts in different domains:
├── [sub-problem 1]: [Domain] → [verb] → [skill]
├── [sub-problem 2]: [Domain] → [verb] → [skill]
└── Proposed sequence: [order]
```

4. Keep dependency order first: a sub-problem that blocks the others (a failing service) is stabilised before anything else. Start with the Complex parts among independent ones, because probes take longest to return signal.
5. Call `AskUserQuestion` with "Adjust / Confirm / Re-frame" before routing.

### Step 2. Classify and route

| Domain | Verb | Scale | Route | OpenSpec |
|---|---|---|---|---|
| Clear | execute | Pebble | Just code it | No |
| Clear | execute | Boulder | `openspec-develop` directly | Yes |
| Complicated, Evolving | analyze | Any | `investigate` → `openspec-plan` | Boulder only |
| Complicated, Degraded | analyze | Any | `troubleshoot` → stabilise → re-frame | No |
| Complicated, Both | analyze | Any | `investigate` + `troubleshoot` sub-task | Boulder only |
| Complex, no hypothesis | probe | Any | `brainstorm` → `probe` → `openspec-plan` | Yes |
| Complex, has hypothesis | probe | Any | `probe` → sense → `openspec-plan` | Yes |
| Liminal Complicated↔Complex | probe+analyze | Any | `probe` first to resolve the boundary → re-frame | No |
| Chaotic | act | none | `experiment` → stabilise → `frame-problem` | No |
| Confused | decompose | Any | Step 1.5 if not done, else ask for more context | none |
| Composite | per part | Mixed | Follow the domain map from Step 1.5 | Per part |

Use `🎯 [Domain] → [Verb] → [skill chain] | OpenSpec: [yes/no] | Scale: [boulder/pebble]` for a single-domain result. Use the full domain map for a composite result.

### Step 3. Handoff

1. Use the handoff template that matches the route (see References) and fill in the thinking trail, decisions and accumulated context.
2. Call `AskUserQuestion` with "Start chain / Re-frame / Skip framing".
3. Call the first skill with $ARGUMENTS on confirm, or with the first sub-problem for a composite result.

## When to Use

- The problem statement is vague, contradictory or solution-shaped ("we need a microservice").
- The team disagrees on what the problem is or what success looks like.
- A previous attempt failed despite correct execution, so the framing was probably wrong.
- You cannot tell whether to investigate, probe, troubleshoot or experiment.
- The scope is unclear: pebble or boulder, one domain or several.

## When Not to Use

- The task is fully specified and unambiguous. Execute it.
- You already know the Cynefin domain and the next skill.
- The problem is a simple bug with a clear reproduction path. Use `troubleshoot` directly.
- The user has explicitly asked to skip framing and go to a named skill.
- Time is critical and the domain is Chaotic. Start with action and frame later.

## Anti-Patterns

- **NEVER frame the problem as a solution.** "We need a microservice" is not a problem statement. **Why:** a solution-framed problem forecloses better options; restate it as a need, such as "reduce deployment coupling".
- **NEVER skip domain classification.** **Why:** Clear problems need best practice and Complex problems need probes; applying one to the other fails in production.
- **NEVER accept the first framing at face value.** **Why:** the first statement is usually a symptom, and the real constraint sits one "why?" deeper.
- **NEVER classify as Complicated just because you have knowledge.** **Why:** fluent expertise bypasses the probe path that Complex problems need, which yields confident but wrong solutions.
- **NEVER skip the Adjacent Domain Challenge when auto-classifying.** **Why:** a verdict never tested at the nearest boundary sends liminal problems to the wrong verb (analyse instead of probe), the usual gotcha at that boundary.
- **NEVER ask the user to name the constraint type.** **Why:** people misclassify their own situation, so ask the three tests and derive the domain yourself.
- **NEVER propose a fix while framing.** **Why:** a solution offered before the domain is settled anchors the chain on one answer and skips the safe-to-fail experiments.

### NEVER let T1 (expertise) alone override a T2/T3 disagreement

**BAD:**

```text
T1: "our team has done this before" -> Complicated
T2: "results vary heavily by customer segment" -> Complex
T3: "entangled, changing one part changes the whole" -> Complex
Verdict: Complicated (T1 wins because "we have expertise")
```

**GOOD:**

```text
T1: Complicated, T2: Complex, T3: Complex -> 2 of 3 agree -> Complex
Route: probe, with T1 noted as a liminal signal (some governing constraints exist)
```

**Why:** T1 measures whether expertise exists, not whether the outcome is predictable. A majority across all three tests decides, never T1 alone.

## Usage Examples

```bash
# "The dashboard is slow" -> ask what is slow, for whom, under what load
# Reframed: "Queries exceed 10s for accounts with >1000 records" (Complicated, Degraded)
# Route: troubleshoot

# "Add AI recommendations to the product page" -> unknown user behaviour, emergent
# Route: Complex, no hypothesis -> brainstorm -> probe

# "Migrate the monolith and fix the checkout bug" -> composite, T3 = some parts yes, some no
# checkout bug: Complicated/Degraded -> troubleshoot FIRST, because it blocks the rest
# monolith migration: Complex -> brainstorm -> probe -> openspec-plan
```

See `references/worked-examples.md` for full dialogues.

## References

- [Triangulation tests](references/triangulation-tests.md): exact question wording, answer mapping and misclassification traps
- [Worked examples](references/worked-examples.md): end-to-end framing dialogues, including a composite case
- [Frame → Brainstorm](references/frame-problem-to-brainstorm-llm.md): handoff template, Complex with no hypothesis → brainstorm
- [Frame → Probe](references/frame-problem-to-probe-llm.md): handoff template, Complex with a hypothesis → probe
- [Frame → Probe (Liminal)](references/frame-problem-to-probe-liminal-llm.md): handoff template, Liminal Complicated↔Complex → probe
- [Frame → Investigate](references/frame-problem-to-investigate-llm.md): handoff template, Complicated and evolving → investigate
- [Frame → Troubleshoot](references/frame-problem-to-troubleshoot-llm.md): handoff template, Complicated and degraded → troubleshoot
- [Frame → Experiment](references/frame-problem-to-experiment-llm.md): handoff template, Chaotic → experiment
