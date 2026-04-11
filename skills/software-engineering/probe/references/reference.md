# probe Reference

## Probe Types

| Type | What is probed | Typical steps | Containment |
|------|---------------|---------------|-------------|
| architecture | Structural feasibility — can this design hold? | Scaffold minimal implementation, run against real constraints | Worktree isolation, no prod writes |
| library | Fit of an external library or tool | Install, write minimal integration test, observe behavior | Sandboxed env, pinned version |
| prompt | LLM behavior under specific conditions | Run prompt variants, collect outputs, compare against criteria | Stateless, no persistent writes |
| integration | Compatibility between two systems | Send minimal real requests, observe response shape and error surface | Read-only or staging environment |
| design | User or team response to a design decision | Prototype minimal artifact, present to stakeholder, sense reaction | No code commitment |

All probe types: safe-to-fail, minimal footprint, reversible by default. If a step cannot be reversed, it must be explicitly approved at Phase 1 entry gate.

---

## Observability Format

Every probe emits this format at Phase 1 plan presentation and Phase 2 exit gate:

```
🔬 Probe → [constraints] → [steps] → [patterns] → [result] → GATE
```

Examples:

```
🔬 Probe → [worktree, no prod writes] → [scaffold adapter, run unit tests] → [error surface, type compatibility] → confirmed → GATE
```

```
🔬 Probe → [staging env, read-only] → [send 3 API requests, inspect response] → [rate limit hit at step 2] → surprise → GATE
```

Fields:
- `constraints`: enabling constraints from Phase 1.2 (comma-separated, abbreviated)
- `steps`: probe steps executed (comma-separated, abbreviated)
- `patterns`: what emerged (observations, not conclusions)
- `result`: `confirmed` | `refuted` | `partial` | `surprise`
- `GATE`: signals human checkpoint — always present, always last

---

## Input Quality Table

Five elements required for a valid probe. If any are missing or weak, the agent asks or proposes — never skips.

| Element | Strong input | Weak/missing input | Agent fallback |
|---------|-------------|-------------------|----------------|
| Hypothesis | Specific, falsifiable statement: "X will behave like Y under condition Z" | Vague goal: "I want to see if this works" | AskUserQuestion: "State one specific hypothesis — what do you believe is true, and under what condition?" |
| Enabling constraints | Named: scope, reversibility boundary, time box | Absent or implied | Agent proposes based on context: "I suggest constraining to [worktree / staging / 30 min]. Confirm or adjust." |
| Confirm/refute criteria | Observable signals defined before execution | "We'll know it works when it works" | Agent proposes: "Confirmed if [X]. Refuted if [Y]. Surprise if [Z]. Revise or proceed?" |
| Prior probe results | Carried from `probe-to-probe-llm.md` handoff context | First cycle — none expected | Skip gracefully: no prior context needed on first invocation |
| Scope boundary | Explicit system/component boundary stated | Unbounded ("the whole system") | Agent proposes minimum viable scope: "I suggest scoping to [component]. Expand if needed." |

Fallback rule: agent asks or proposes — user validates. Never skip an element silently. Never assume a hypothesis the user did not state.

---

## Self-Transition Protocol

When result is `partial` and a refined hypothesis is identifiable:

1. Write full probe result to a thinking artifact file (e.g., `thinking/probes/{project}/{date}-{slug}-llm.md`). On collision, append `-2`, `-3`, etc.
2. Fill `probe-to-probe-llm.md` with:
   - Refined hypothesis (sharpened, not repeated)
   - Carried enabling constraints (unchanged unless probe discovered new ones)
   - What was confirmed (do not re-test)
   - What was refuted (do not retry)
   - New angle to probe
3. Re-invoke `probe` with the filled handoff as `$ARGUMENTS`
4. Phase 1 qualification is faster — skip rediscovery of confirmed facts

Accumulated context cap: 800 tokens total across a chain. At cap, compress prior cycles to 200 tokens (keep: confirmed facts, active constraints, rejected paths). References to thinking files do not count toward cap.

---

## Knowledge File Convention

Output path: `thinking/probes/{project}/{date}-{slug}-llm.md` (under your configured workspace root)

`{project}` = current project folder name. On collision (file exists), append sequence: `-2`, `-3`, etc.

Required sections in knowledge file:
- Hypothesis (as stated at Phase 1)
- Enabling constraints (as approved at entry gate)
- Steps taken (ordered list)
- Observations (raw — what was seen)
- Sensed patterns (interpreted — what the observations suggest)
- Result: `confirmed` | `refuted` | `partial` | `surprise`
- Eliminated causes (for refuted: what was ruled out — do not re-probe)

Naming: `{YYYY-MM-DD}-{hypothesis-slug}-llm.md`. Slug is lowercase hyphenated, derived from hypothesis subject.
