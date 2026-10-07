# Triangulation Tests

Exact wording for Step 1 of the skill. Ask every applicable question in one `AskUserQuestion` call. Never ask the user to pick a Cynefin domain or a constraint type: people misclassify their own situation, so ask what they can observe and derive the domain yourself.

## T1: "Who's done this before?" (Keogh scale)

| Option | Answer | Reads as |
|---|---|---|
| 1 | Everyone on the team knows how | Clear |
| 2 | Someone on our team, or we have access to expertise | Complicated |
| 3 | Someone outside our organisation has, but not us | Complex |
| 4 | Nobody has ever done this | Complex, near Chaotic |
| 5 | I cannot even tell what "this" is | Confused |

T1 measures whether expertise exists. It says nothing about whether the outcome is predictable, so it must never decide alone.

## T2: "Same inputs, same result?" (predictability)

| Option | Answer | Reads as |
|---|---|---|
| Yes | Yes, reliably | Ordered (Clear or Complicated) |
| Probably not | Path-dependent, sensitive to context | Unordered (Complex) |
| No | No relationship between action and outcome | Chaotic |

## T3: "Can you take it apart?" (disassembly)

| Option | Answer | Reads as |
|---|---|---|
| Yes | Independent pieces that reassemble identically | Complicated at most |
| No | Entangled: changing a part changes the whole | Complex at minimum |
| Some | Some parts yes, some parts no | Composite, so decompose (Step 1.5) |

## Q-Scale (skip when T2 is Chaotic)

- Boulder: multi-step, ambiguous or architectural.
- Pebble: a single file with an obvious implementation.
- Not sure: treat as a boulder until decomposition says otherwise.

## Q-Complicated sub (only when T1=2 and T2=ordered)

- Evolving: the system is improving or growing capacity. Route to `investigate`.
- Degraded: it was working and is now failing. Route to `troubleshoot`.
- Both: improving on one dimension and degrading on another. Route to `investigate` with a `troubleshoot` sub-task.

## Reading the answers

| Agreement | Action |
|---|---|
| All 3 agree | Classify directly with high confidence. |
| 2 of 3 agree | Classify by the majority and report the dissent: `🎯 [Domain] (2/3 tests agree)` then `⚠️ Liminal signal: T[N] suggests [adjacent domain]`. |
| T1=2 and T2 ordered, but T3 entangled | Treat as Liminal Complicated↔Complex rather than plain Complicated: expertise covers the parts, not the interactions. Route `probe` first, then re-frame. |
| All 3 disagree, or T3 is composite | The problem spans domains. Decompose in Step 1.5. |

## Misclassification traps

- **Expertise bias.** An engineer answers T1=2 and T2=ordered while T3 is entangled. The honest reading is Complex, not Complicated, because the expertise covers the parts and not the interactions.
- **Overwhelm.** A stressed team answers T2=chaotic when the situation is Complex with enabling constraints available. Slow down and decompose before routing to `experiment`.
- **Contradiction.** "Nobody has done this" together with "the result is predictable" cannot both be true. The parts sit in different domains, so decompose.
- **Solution-shaped answers.** If the user describes the problem as a missing technology ("we need Kafka"), restate it as a need before asking T1 to T3, or the tests measure the solution instead of the problem.
