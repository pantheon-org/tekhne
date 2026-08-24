---
name: plan-create
description: >
  Create `.context/plans/*.md` files with standard YAML frontmatter,
  phases/tasks/waves decomposition, and post-creation validation.
  Infers section conventions from existing plans in `.context/plans/`
  to match the local style. Integrates with plan-review for a
  create → review → iterate loop. Triggers: 'create a plan',
  'new plan', 'draft a plan', 'plan scaffold', 'write a plan'.
  Do NOT use for plans outside `.context/plans/`, ephemeral notes,
  or the main project SKILL.md.
---

# Plan Create -- Structured Plan Scaffolding

Create `.context/plans/` files that pass structural validation and match
local conventions. The workflow ensures every plan has valid frontmatter,
clear phases and tasks, and can be immediately reviewed by the
`plan-review` skill.

## Prerequisites

- A clear idea of the work to be planned (goal, scope, implementation steps)
- The `context-file` skill available if frontmatter repair is needed
- The `plan-review` skill available for post-creation validation (optional)
- Shell access to run validation scripts

## When to Use

- Drafting a new implementation plan with multiple phases
- Scoping work that needs review before implementation begins
- Creating a plan that will later be audited by `plan-review`

## When NOT to Use

- For the main project SKILL.md or agent skills -- use the skill template instead
- For one-off notes or scratch files -- use inline notes
- When the work is trivially small (1 step, no phases) -- just write a finding

## Workflow

### 1. Gather the plan specification

Ask the user about the plan. Collect at minimum:

- **Goal** -- what does success look like? What problem is being solved?
- **Scope** -- what's in and what's out? Are there known constraints?
- **Phases** -- what are the sequential stages? Each phase should:
  - Have a clear deliverable or exit criterion
  - Be independently shippable (could stop after this phase)
  - Be 2–5 tasks, not 20+
- **Tasks per phase** -- for each phase, list concrete work units:
  - Each task should be completable in a single session (hours, not weeks)
  - Use actionable language ("Add --format flag", not "Improve output")
  - Flag which tasks can run in parallel (waves)
- **Dependencies** -- does this plan depend on other plans, PRs, or external work?
- **Risks** -- what could go wrong? What's the biggest unknown?
- **Timeline** -- optional, if the user has a deadline or priority
- **Effort** -- a T-shirt-sized estimate. **Value** -- a benefit-of-action grade.
  **Themes** -- an ordered subject-area tag list. All three are required
  frontmatter fields; see
  [Grading Effort, Value, and Themes](references/grading-effort-value-themes.md)
  for the full rubric, allowed values, and worked examples. TYPICALLY these
  three fields are the ones a first-time plan author gets wrong -- read the
  reference before guessing.

### 2. Infer local conventions

Scan existing plans to understand what sections this repo uses:

```bash
grep -r '^## ' .context/plans/*.md | sed 's/.*## //' | sort | uniq -c | sort -rn
```

From the frequency table, identify:

- **Core sections** (present in >= 40% of plans): always include
- **Common sections** (present in 20-39%): include unless the plan is small
- **Rare sections** (< 20%): include only if relevant

Also check the naming convention: timestamped (`topic-YYYY-MM-DD.md`) or
topic-only (`short-description.md`). Follow whatever the majority of plans use.

### 3. Draft the plan

Construct the plan file following the structure in
[assets/templates/plan-scaffold.yaml](assets/templates/plan-scaffold.yaml).
The template is validated against
[assets/schemas/plan-scaffold.schema.json](assets/schemas/plan-scaffold.schema.json).

The plan always includes:

- YAML frontmatter with `title`, `type: PLAN`, `status: DRAFT`, `date` (today),
  `effort` (`S`/`M`/`L`/`TBD`), `value` (`HIGH`/`MEDIUM`/`LOW`), `themes` (ordered
  list from the theme vocabulary, primary-first)
- `## Goal` section -- one paragraph describing the desired end state
- `## Phases` section -- numbered phases with tasks and wave annotations
- `## Open Questions` section -- unresolved items for the reviewer

Additional sections based on what the user provided and what the local
conventions suggest (from step 2): `## Scope`, `## Risks`, `## Verification`.

### 4. Validate the plan

Run the frontmatter validation script (see the `context-index` skill's
`scripts/validate-context-frontmatter.sh`):

```bash
validate-context-frontmatter.sh .context/plans/<plan-file>.md
```

If validation fails, fix the frontmatter and re-run. Do not proceed until
validation passes -- a plan with invalid frontmatter is invisible to the index.

### 5. Review the plan (optional)

Offer to run the `plan-review` skill on the newly created plan:

> "I created the plan at `.context/plans/<file>`. Would you like me to run
> the `plan-review` skill on it now to catch any issues?"

If the user accepts, load the `plan-review` skill and follow its workflow.
This completes the create → review → iterate loop.

### 6. Confirm and conclude

Confirm the file was created with its path and a summary:

```
Created: .context/plans/<file>.md
Title:   <title>
Status:  draft
Phases:  <N> phases, <M> tasks total
Next:    Run plan-review on it, or mark status: READY once reviewed and approved
         (status becomes ACTIVE on its own once a worktree/branch for it exists)
```

## Verification

After creating the plan, run these checks:

1. **Frontmatter validation** -- run `validate-context-frontmatter.sh` on the
   file. If it fails, fix the frontmatter before presenting the result.

   ```bash
      validate-context-frontmatter.sh <file>
   ```

2. **Structure check** -- verify the plan has at minimum `## Goal` and
   `## Phases` sections. If the local convention requires others (from step 2),
   add those too.
3. **Phase completeness** -- verify each phase has 2-5 concrete tasks and
   an exit criterion. Phases with more than 8 tasks should be split.
4. **Wave annotation** -- verify tasks that can run in parallel are marked
   (e.g., "Wave A: frontend, Wave B: backend -- can run concurrently").
5. **Effort declared** -- frontmatter has `effort` set to `S`/`M`/`L`/`TBD`.
   If `TBD`, confirm the corresponding Open Question actually explains what's
   blocking the estimate.
6. **Value graded** -- frontmatter has `value` set to `HIGH`/`MEDIUM`/`LOW`,
   graded against the value rubric (see
   [Grading Effort, Value, and Themes](references/grading-effort-value-themes.md))
   rather than guessed.
7. **Themes tagged** -- frontmatter has a non-empty `themes` list, ordered
   primary-first, with every member drawn from the controlled theme vocabulary
   (same reference) -- no invented themes.

## Mindset

- A plan is a communication tool first, a todo list second. Write for the next
  person who reads this cold.
- Phases are about sequencing, not grouping. Phase 1 must finish before Phase 2
  starts. If two groups don't depend on each other, they're waves within a phase,
  not separate phases. PREFER an explicit wave label over leaving parallelism
  implicit.
- Tasks should be single-session-sized. TYPICALLY a phase holds 2-5 tasks. If a
  task takes "a few days", it's too large -- break it down. If it takes
  "5 minutes", it's too small -- combine it.
- BY DEFAULT a new plan is `status: DRAFT`. Promote to `READY` only after the
  plan is reviewed and approved -- AVOID skipping straight to `READY` UNLESS the
  work is genuinely trivial enough to skip review entirely (see When NOT to Use).
  `ACTIVE` is not a manual promotion at all -- it's entered automatically once a
  worktree/branch referencing the plan exists (see `planning-flow.md`).
- The YAML frontmatter is not optional. A plan without frontmatter is invisible
  to `.context/index.yaml` and to every agent that reads it.
- Sizing `effort` and grading `value` is RECOMMENDED to happen against the
  rubric in [Grading Effort, Value, and Themes](references/grading-effort-value-themes.md),
  not from gut feel -- a guessed grade misleads the reader more than a stated
  `TBD` with an Open Question.

## Anti-Patterns

**NEVER** -- Create a plan without YAML frontmatter

**SYMPTOM:** The file renders fine in markdown but never appears in the context
index. Future agents never discover the plan.

**CONSEQUENCE:** Effort goes into a plan no one reads. The plan is orphaned
until someone manually finds it and adds frontmatter.

**WHY:** The context index and pre-commit hooks both require frontmatter. A plan
without it is invisible machinery.

**BAD:** Starting the file with `# Plan: My Title` directly.
**GOOD:** Always open with `---\ntitle: "Plan: My Title"\ntype: PLAN\nstatus: DRAFT\ndate: YYYY-MM-DD\n---`.

**NEVER** -- Skip the convention inference step

**SYMPTOM:** The created plan uses a structure that doesn't match any other plan
in the repo -- no `## Open Questions`, no `## Scope`. It feels out of place.

**CONSEQUENCE:** Agents that parse plans expecting `## Steps` instead of
`## Phases` may skip sections. The plan is technically valid but practically
misaligned.

**WHY:** The local convention reflects what agents in this repo expect. A plan
that doesn't follow it is harder to review, harder to index, and harder to
discover.

**BAD:** Hardcoding "Goal → Steps → Open Questions" without scanning first.
**GOOD:** Running `grep -r '^## ' .context/plans/*.md` and using the actual
frequencies.

**NEVER** -- Create a phase with more than 8 tasks

**SYMPTOM:** "Phase 1: Everything" with 15 tasks and no sub-structure.
The phase cannot be shipped independently -- it's the whole plan.

**CONSEQUENCE:** No meaningful checkpoint exists. If the plan runs out of time,
there's no partial delivery. The cost of splitting later is higher than splitting
now.

**WHY:** A phase should be small enough to review, implement, and ship in a
sprint (1-2 weeks). 8+ tasks means the phase is underspecified.

**BAD:** A flat list of 15 tasks with no phase grouping.
**GOOD:** 3 phases, each with 3-5 tasks and an exit criterion.

**NEVER** -- Omit or guess the `effort` field

**SYMPTOM:** The plan has no `effort` in frontmatter, or has one picked
arbitrarily to satisfy validation rather than reflecting an actual estimate.

**CONSEQUENCE:** A reader scanning `.context/index.yaml` for quick wins vs.
big lifts can't distinguish them without opening every plan file. A fake
number is worse than a missing one -- it looks authoritative but isn't.

**WHY:** `effort` exists so plans are triageable at a glance, the same way
`status` lets a reader triage by lifecycle stage. `validate-context-frontmatter.sh`
requires it for any `type: PLAN` with `status: DRAFT`, `READY`, or `ACTIVE`.

**BAD:** Setting `effort: S` on a plan nobody has actually sized, just to pass
validation.
**GOOD:** `effort: TBD` with an Open Question stating exactly what decision
blocks sizing (see `migrate-off-tessl-eval-2026-06-29.md` for a real example).

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Common plan sections, their purpose, and file-naming conventions | [Plan Structure Reference](references/plan-structure.md) | Step 2, inferring which sections to include and how to name the file |
| Effort/value/themes rubric, allowed values, worked example | [Grading Effort, Value, and Themes](references/grading-effort-value-themes.md) | Step 1, whenever effort, value, or themes need grading rather than guessing |
| Repairing frontmatter on an existing file | `context-file` skill | When a plan's frontmatter is invalid and needs fixing, not creating from scratch |
| Post-creation multi-perspective audit | `plan-review` skill | Step 5, after a plan is drafted and ready for review |
| Regenerating the index after creation | `context-index` skill | After a plan is created or its frontmatter changes, to keep `.context/index.yaml` in sync |
