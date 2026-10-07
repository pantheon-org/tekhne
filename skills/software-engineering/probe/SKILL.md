---
name: probe
description: "Safe-to-fail experiment for Complex domain problems where cause-effect is only visible in retrospect. Two-phase: foreground qualify → background probe → sense result. Use when: probe, safe-to-fail, test hypothesis, experiment with hypothesis, Complex domain with hypothesis. NOT for brainstorming (use brainstorm) or known cause-effect (use investigate)."
allowed-tools: AskUserQuestion, Read, Glob, Grep, WebSearch, WebFetch, Write, Bash, Task
model: opus
argument-hint: <hypothesis to probe>
cynefin-domain: complex
cynefin-verb: probe
---

# Probe

Safe-to-fail experiment in Complex domain. Cause-effect only visible in retrospect — probe to sense patterns, not to prove.

**Probing:** **$ARGUMENTS**

Check for handoff context: if `$ARGUMENTS` references a `probe-to-probe-llm.md` file, load it before Phase 1 — carried context accelerates qualification.

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known harness bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known harness bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Phase 1: Qualify (foreground — MANDATORY)

**ENTRY GATE: Phase 2 does not start until Phase 1 is complete. No bypass path exists.**

### 1.1 Parse hypothesis

Extract from `$ARGUMENTS` or handoff context:
- Hypothesis statement (what you believe might be true)
- Enabling constraints already known (carry forward from prior cycles — do NOT rediscover)
- Confirm/refute criteria already defined (carry forward, update if refined)

If no hypothesis present: AskUserQuestion — ask user to state the hypothesis. Do not proceed without one.

### 1.2 Identify enabling constraints

Bounds without prescribing path:
- **Scope**: time, access, reversibility boundary
- **Immutable**: production systems, data integrity, user-facing state
- **Variable**: what can be freely changed within experiment

Carry forward from prior cycles unchanged unless explicitly updated.

### 1.3 Define confirm/refute criteria

Before running: define observable signals. For each criterion:
- Confirmed: observable evidence that supports the hypothesis
- Refuted: observable evidence that contradicts the hypothesis
- Surprise: unexpected result that suggests a different hypothesis

Criteria must be defined before Phase 2 executes. Gate on this.

### 1.4 Present probe plan

Output:

```text
🔬 Probe → [constraints] → [steps] → [expected patterns] → [confirm/refute criteria] → GATE
```

Probe type (see `references/reference.md`): architecture | library | prompt | integration | design

### 1.5 Entry gate

AskUserQuestion — one call:
- "Proceed with probe? [Yes / Revise hypothesis / Revise criteria / Abort]"

On confirm: Phase 2 executes. On anything else: loop back to 1.1–1.4.

---

## Phase 2: Execute (background)

**Configuration: `isolation: worktree` + `run_in_background: true`**

Runs only after Phase 1 entry gate passes.

### 2.1 Execute probe steps

Run the experiment as defined in Phase 1. Prefer minimal, reversible actions. Gate frequency is SPARSE — enabling constraints bound the agent, not human micromanagement.

### 2.2 Sense patterns

Observe results against confirm/refute criteria:
- What signal emerged?
- What was unexpected?
- What constraints were discovered during execution?

### 2.3 Persist Thinking Artifact ⚠️ MANDATORY

**MUST execute before exit gate. DO NOT skip. DO NOT wait for user to ask.**

Write probe result to a thinking artifact file in your configured workspace (e.g., `thinking/probes/{project}/{date}-{slug}-llm.md`).

`{project}` = current project folder name. Create the directory if missing.

**Collision handling**: If filename exists, append sequence: `{date}-{slug}-2-llm.md`, `{date}-{slug}-3-llm.md`, etc. First write gets clean name.

```text
thinking/probes/{project}/{date}-{slug}-llm.md
thinking/probes/{project}/{date}-{slug}-2-llm.md
```

**Guard**: If workspace root is not configured, warn user and skip artifact persistence.

Content: hypothesis + enabling constraints + steps taken + observations + sensed patterns + result classification.

### 2.4 Exit gate

Classify result: `confirmed` | `refuted` | `partial` | `surprise`

Produce B4-compatible handoff:

| Result | When | Transition | Template |
|--------|------|-----------|----------|
| confirmed | Hypothesis holds | Complex → Complicated | `references/probe-to-investigate-llm.md` |
| partial (enough signal) | Some evidence, ready for expert analysis | Complex → Complicated | `references/probe-to-investigate-llm.md` |
| partial (need another angle) | Some evidence, hypothesis needs sharpening | Complex → Complex (re-probe) | `references/probe-to-probe-llm.md` |
| refuted / surprise | Hypothesis failed or unexpected result | Complex → Complex (brainstorm) | `references/probe-to-brainstorm-llm.md` |

Handoff token budget: target 300 tokens inline, flex 200-500, hard cap 600. References to thinking files do not count toward cap.

Self-transition: if result is `partial` and hypothesis can be sharpened, re-invoke `probe` via `references/probe-to-probe-llm.md` with accumulated context. Prior cycles compressed to 200 tokens at 800-token accumulated cap.

---

## Philosophy

- Probe to sense patterns, not to prove: one successful probe is a weak signal, not a decision.
- Keep every probe safe to fail: time-boxed, reversible and isolated from production state.
- Define what confirms, refutes and surprises before running anything, so results cannot be bent to fit the hypothesis.
- Change the hypothesis after a refutation instead of repeating the same experiment.

## When to Use

- The problem space is in the **Cynefin Complex domain**: cause-and-effect relationships are only visible in retrospect, not in advance.
- You have a **falsifiable hypothesis** but no reliable way to reason your way to a conclusion — experimentation is the only path forward.
- The cost of a failed experiment is **low and bounded**: time-boxed, reversible, and isolated from production state.
- You are entering **unknown territory** — a new architecture, library, integration pattern, or service boundary where prior knowledge does not transfer.
- A previous probe returned `partial` or `surprise` and you need to **sharpen the hypothesis** before escalating to expert analysis.

## When Not to Use

- The problem has a known solution or a clear best practice — use **investigate** instead (Complicated domain, cause-effect knowable by experts).
- No hypothesis exists yet and the goal is creative divergence — use **brainstorm** instead (diverge before you converge).
- The action cannot be **reversed or rolled back**: if failure has permanent side effects, it is not a probe.
- The team needs a **guaranteed outcome** by a fixed deadline — probes produce signals, not commitments.
- The experiment would require **production traffic or real user data** to execute — probes must run in isolation.

## Anti-Patterns

### NEVER run a probe with production traffic

**WHY:** The value of a probe is learning at low cost; exposing real users to experimental code defeats this.

**BAD:** Route 10% of live customer requests to the experimental code path.
**GOOD:** Shadow the calls or run against staging with synthetic data.

### NEVER treat probe results as proof

**WHY:** Complex domains resist confirmation; one successful probe is a weak signal, not a decision.

**BAD:** "It worked once in isolation, so the approach scales."
**GOOD:** Record the result as `confirmed` and hand it to investigate for expert analysis.

### NEVER design an irreversible probe

**WHY:** Reversibility is the core property that makes probing safe; irreversible probes transfer all risk back to the team.

**BAD:** Run a destructive schema migration on shared data to see what happens.
**GOOD:** Use a staging branch or feature flag that can be rolled back.

### NEVER skip Phase 1 qualification

**WHY:** Without pre-defined criteria, confirmation bias dominates and results are interpreted to fit the hypothesis rather than test it.

**BAD:** Start executing steps before confirm/refute criteria exist.
**GOOD:** Pass the Phase 1 entry gate with constraints and criteria agreed first.

### NEVER carry a failed hypothesis forward unchanged

**WHY:** Re-running the same experiment expecting different results is wishful thinking, not probing.

**BAD:** Re-probe a refuted hypothesis with a slightly bigger sample.
**GOOD:** Revise the hypothesis, or route to brainstorm, before the next probe.

### NEVER proceed on empty AskUserQuestion answers

**WHY:** An empty answer means the question was never shown, so any assumption replaces the user's decision.

**BAD:** Treat blank answers as "Yes" and start Phase 2.
**GOOD:** Present the options as a numbered text list and wait for the reply.

### NEVER skip the thinking artifact

**WHY:** The artifact is the durable record that makes handoffs and later cycles possible; without it the learning is lost.

**BAD:** Go straight to the exit gate because the result seems obvious.
**GOOD:** Write the artifact in the configured workspace before classifying and handing off.

### NEVER start a probe without a hypothesis

**WHY:** A probe without a falsifiable statement has nothing to confirm or refute, so it produces noise.

**BAD:** Begin exploring because the topic "feels risky".
**GOOD:** Ask the user to state the hypothesis and do not proceed until one exists.

### NEVER route a result against the handoff table

**WHY:** The transition encodes what the evidence supports; misrouting sends the next phase down the wrong path.

**BAD:** Send a `refuted` or `surprise` result to investigate.
**GOOD:** Send `refuted` and `surprise` to brainstorm, `confirmed` and sufficient `partial` to investigate, and `partial` needing another angle to probe.

## Usage Examples

**Probing a new caching strategy:**

```bash
# Problem: unknown whether Redis or Memcached fits the read pattern
# Probe: run both on 1% of traffic for 24h, measure hit rate + latency
# Reversible: yes (feature flag)
```

**Probing a service extraction:**

```bash
# Problem: unclear if extracting auth into a microservice reduces coupling
# Probe: shadow the auth calls for 48h without routing real traffic
```

**Probing an LLM prompt strategy:**

```bash
# Problem: uncertain whether chain-of-thought prompting improves accuracy for this task
# Probe: run 50 test cases through both zero-shot and CoT variants
# Confirm: CoT accuracy >= zero-shot + 10% on held-out eval set
# Refute: no measurable difference, or CoT latency cost exceeds acceptable threshold
# Reversible: yes (eval harness only, no production change)
```

**Probing a database schema migration approach:**

```bash
# Hypothesis: adding a surrogate UUID primary key reduces join complexity in reporting queries
# Step 1: create a staging branch with the new schema
# Step 2: run the existing reporting query suite against staging
./scripts/run-query-benchmarks.sh --env staging --compare main
# Step 3: compare execution plans and latency — confirm/refute threshold: >=20% reduction
# Reversible: yes (staging only, main schema untouched)
```

---

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Probe types and observability | [references/reference.md](references/reference.md) | Classifying the probe type and recording observations |
| Probe to investigate | [references/probe-to-investigate-llm.md](references/probe-to-investigate-llm.md) | Result is `confirmed` or `partial` with enough signal |
| Probe to brainstorm | [references/probe-to-brainstorm-llm.md](references/probe-to-brainstorm-llm.md) | Result is `refuted` or `surprise` |
| Probe to probe | [references/probe-to-probe-llm.md](references/probe-to-probe-llm.md) | Result is `partial` and the hypothesis needs another angle |
