---
name: design-debate
description: >
  Stress-test an unwritten idea or decision by spawning independent subagents in
  genuinely OPPOSING roles (advocate, skeptic, migration/risk) rather than
  complementary review lenses — grounded in real repo investigation, not
  assumption, and concluding in a synthesized verdict rather than a list of
  opinions. Use for "should we do X", "what are the pros and cons", "I need
  pushback", "collegial review", "second opinion on this decision" — BEFORE a
  plan exists. NOT for reviewing an already-written plan file (use plan-review)
  and NOT for refining a vague question through dialogue with the user (use
  socratic-method).
---

# Design Debate

Decide whether to do something — before writing a plan for it — by making independent
subagents genuinely disagree, not politely converge. The value isn't in getting three
opinions; it's in the friction between them, and in a final verdict that says which
argument actually held up.

## Prerequisites

- A concrete decision or idea to debate, not yet written as a plan file (if it already
  is, use `plan-review` instead)
- Enough repo/codebase access to gather real facts before the debate starts
- The `context-file` skill, to persist the verdict as a finding
- The `plan-create` skill, as the next step if the verdict is "proceed"

## When to Use

- "Should we split/merge/replace X?" — a decision with real tradeoffs on more than one side
- The user explicitly asks for pushback, a second opinion, or a collegial review
- A decision surfaced from real pain and it's unclear whether the fix is worth its cost

## When NOT to Use

- A plan file already exists for this work — use `plan-review`
- The question needs narrowing before it's even debatable — use `socratic-method` first
- There's only one reasonable answer — AVOID manufacturing debate where none is warranted

## Workflow

### 1. State the decision precisely

Write it as a yes/no or A-vs-B choice, not a vague topic.

### 2. Ground the debate in real facts BEFORE assigning positions

UNLESS you already have hard numbers, ALWAYS investigate first: current scale, who
consumes the thing in question, what a proposed change would actually touch.

```bash
wc -l path/to/thing/in/question
grep -rl "the/consumer/pattern" . | wc -l
```

This confirms the actual size and blast radius before anyone argues about it — an
ungrounded debate produces confident conclusions built on guessed facts.

### 3. Compose one identical brief for every reviewer

Every reviewer gets the same decision, the same grounding facts, and any candidate
designs already on the table — differences in conclusions must come from role, not
information.

### 4. Assign genuinely opposing roles

PREFER two roles (Advocate, Skeptic) as the minimum for real tension. ALWAYS add a
Migration/Risk role when the change touches multiple files or downstream consumers. See
[references/role-design.md](references/role-design.md) for the full role table and
sample instructions.

### 5. Model selection

BY DEFAULT, diverse models catch different blind spots — see `plan-review`'s model
routing guidance for environment-specific pairs. A single model is acceptable for
lower-stakes debates; the role framing alone still produces more tension than one pass.

### 6. Spawn all reviewers in parallel

```bash
# single message, N parallel Agent/Task calls — never sequential
# each call gets the identical brief plus its own role instruction
```

```text
# wrong: spawn advocate -> wait -> spawn skeptic -> wait
# right: spawn advocate + skeptic (+ migration_risk) in one message
```

This produces both reviews in the time of the slowest single one, not their sum.

Never spawn sequentially — that triples wall-clock time for no benefit, since the
reviews are fully independent.

### 7. Synthesize a verdict

The step most likely to be skipped under time pressure, and the one that matters most:

- Say which argument holds up under the others' scrutiny, and why.
- If the Skeptic's case rests on an existing mitigation, check whether the Advocate's
  case accounts for it.
- Render an actual recommendation — `proceed`, `do_not_proceed_for_now`, or
  `proceed_with_modification`. "It depends" is only acceptable when the brief genuinely
  lacked a fact needed to decide; if so, name exactly which fact.
- If the verdict is `do_not_proceed_for_now`, ALWAYS name a concrete revisit trigger —
  "revisit if X crosses N", never "revisit later".

Run the validation script to check the verdict conforms to schema:

```bash
./scripts/validate-debate-verdict.sh path/to/verdict.yaml
# expected: OK: 1 file(s) validated against debate-verdict schema
```

A missing `revisit_trigger` on a `do_not_proceed_for_now` verdict produces this instead:

```text
Debate verdict validation errors:
  path/to/verdict.yaml: verdict 'do_not_proceed_for_now' must set 'revisit_trigger' ...
```

This confirms the verdict is complete before it ships or gets persisted.

### 8. Always record the outcome — the verdict decides which artifact

ALWAYS persist the outcome and ALWAYS ask explicitly before doing so — never decide
silently, and never skip recording just because the main question got resolved. A
verdict discussed and then left only in chat is the same failure mode `session-reflection`
exists to prevent — RECOMMENDED is not the same as done.

Which artifact is a rule of thumb, not a free choice: **if it's an issue that won't be
fixed, it's a known-issue; otherwise, it's a finding. Record either way.**

- **Verdict is `proceed` or `proceed_with_modification`** — something is being acted on.
  Use `context-file` to write a **finding** capturing the decision, the grounding facts,
  and why the losing arguments didn't hold up.
- **Verdict is `do_not_proceed_for_now`** — this is, by definition, a real issue being
  consciously left unfixed. Use `context-file`'s `KNOWN_ISSUE` type
  (`severity: CRITICAL|HIGH|MEDIUM|LOW`) instead of a finding, with the revisit trigger
  from Step 7 as its eventual fix condition.
- The same rule applies to any individual role's finding, not just the overall verdict:
  if a role (commonly Migration/Risk, but not exclusively) surfaced a separate concrete
  gap that won't be fixed this session — even on an otherwise `proceed` verdict — that
  specific gap is its own known-issue, independent of how the main decision gets recorded.

```bash
./scripts/validate-context-frontmatter.sh <path-to-finding>
# confirms that the finding's frontmatter is valid before it's committed
```

If the verdict reveals a binding architectural decision, use `adr-capture` too.

### 9. Hand off

If the verdict is `proceed`, use `plan-create` next with the debate's grounding facts and
chosen design as input — do not re-derive them from scratch.

## Anti-Patterns

**NEVER** assign all reviewers a "balanced/neutral" framing.
**WHY:** Neutral reviewers converge toward consensus, which reads as validation but is
just multiple copies of the same unopposed take.
**GOOD:** Assign explicit, opposing stances (Step 4).

**NEVER** skip Step 2 and let reviewers debate from assumption.
**WHY:** An ungrounded debate produces confident conclusions built on guessed facts.
**GOOD:** Investigate real numbers/consumers first; put them in the brief.

**NEVER** present raw reviewer output as the final answer.
**WHY:** Opinions without a synthesized verdict push the actual decision back onto the
user — the work this skill exists to do.
**GOOD:** A verdict that names which argument won and what to do next (Step 7).

**NEVER** let a "not now" verdict omit a revisit trigger.
**WHY:** "Not now" with no trigger silently becomes "never" by default.
**GOOD:** Name a concrete, checkable condition (Step 7).

## Mindset

- The value is real disagreement, not the appearance of thoroughness
- Ground everything in the actual repo — an argument equally true of any codebase isn't
  a real argument about this decision
- A well-run debate can end with "no, and here's why the strongest case for yes didn't
  hold up" — that's success, not failure to reach consensus
- Losing arguments are still worth recording — future readers benefit from knowing what
  was considered and rejected

## Troubleshooting

| Situation | Response |
|-----------|----------|
| All reviewers agree | Not a bug — the facts favor one side. Note this in the synthesis; apply forced disagreement only when the brief genuinely supports more than one reading. |
| A reviewer's argument rests on an unverified claim | Verify it yourself before weighting it — an adversarial role doesn't exempt a reviewer from being wrong. |
| The decision needs facts the brief lacked | Say so in the verdict; name exactly what's missing rather than guessing. |
| User wants to act on "proceed" immediately | Hand off to `plan-create`, skip re-deriving facts (Step 9). |

## Templates

| Artifact | Template | Schema | Validation Script |
|----------|----------|--------|-------------------|
| Debate Verdict | [assets/templates/debate-verdict.yaml](assets/templates/debate-verdict.yaml) | [assets/schemas/debate-verdict.schema.json](assets/schemas/debate-verdict.schema.json) | [scripts/validate-debate-verdict.sh](scripts/validate-debate-verdict.sh) |

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Full role table, sample role instructions, why not to reuse plan-review's lenses | [Role Design](references/role-design.md) | Assigning roles (Step 4) |
| A real worked debate end to end | [Worked Example](references/worked-example.md) | Seeing grounding, roles, and synthesis together |
