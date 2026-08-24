# Structural Inference for Plan Review

Detail behind Workflow step 3's structural check: how to infer the local plan
body convention instead of hardcoding one, and how to evaluate phase/task/wave
structure.

## Plan body structure -- infer the local standard

Beyond frontmatter, check the plan's body sections (`##` headings) against what
other plans in this repo actually use. Do NOT hardcode a template -- scan the
existing plans and infer what's conventional.

### How to infer

Run this to collect all H2 headings from every plan:

```bash
grep -r '^## ' .context/plans/*.md | sed 's/.*## //' | sort | uniq -c | sort -rn
```

This gives a frequency table like:

```
 7 ## Goal
 7 ## Steps
 5 ## Open Questions
 4 ## Scope
 4 ## Related
 4 ## Decisions
 3 ## Verification
 3 ## Risks
 3 ## Success Criteria
 3 ## Background
```

From this, identify the **core sections** (present in >= 40% of plans), the
**common sections** (present in 20-39%), and the **rare sections** (<20%).

Missing core sections are **advisory warnings** -- they don't invalidate the plan
(plans can be valid with different structures) but the reviewer should note them.
Missing common sections are **neutral observations**.

Rare or unique sections are also interesting -- they may indicate scope creep or
a plan that doesn't fit the local conventions. Note them as "unusual sections"
but do not flag them as problems.

### Why infer instead of hardcode

Hardcoding a template (Goal → Steps → Open Questions) penalises plans that
legitimately need a different structure. Inferring from actual plans reflects
what this repo's conventions actually are -- and those conventions evolve as
new plans are added. A plan that omits `## Goal` but has a `## Problem` and
`## Target State` is following the spirit of the convention, just not the
letter.

## Implementation architecture -- Phases, Tasks, and Waves

A well-structured plan divides the work into **Phases**, **Tasks**, and
**Waves** -- and the review should check for this structure.

| Term | Meaning | Example |
|------|---------|---------|
| **Phase** | Sequential stage -- Phase N must finish before Phase N+1 starts | Phase 1: Foundation, Phase 2: Integration |
| **Task** | Smallest unit of work within a phase -- independently assignable | Task 1.1: Create schema, Task 1.2: Write migration |
| **Wave** | Group of tasks within a phase that can run in **parallel** | Wave A (can run with Wave B): frontend + backend |

Common bad patterns to flag:

- **Flat step list** -- 15 numbered steps with no grouping, no parallelisation.
  Every step waits for the previous one, even when they don't depend on each other.
- **Missing dependency annotation** -- steps are listed sequentially but some
  could run in parallel. The plan doesn't say which.
- **Oversized phases** -- "Phase 1: Everything" with 20+ tasks and no sub-structure.
  A phase should be small enough that it can be reviewed and shipped independently.
- **Missing task boundaries** -- steps describe vague outcomes ("improve things")
  instead of concrete, completable units ("add --format flag to evaluate cmd").

### What to check

Look at the plan's `## Steps` / `## Implementation` / `## Phases` section:

1. **Are steps grouped into phases?** The plan should have 2-5 sequential phases
   (numbered or labelled). If there are >10 flat steps with no grouping, flag it.

2. **Are tasks the right size?** Each task should be completable in a single
   session (hours, not weeks). Tasks with vague descriptions ("finalise",
   "improve", "handle remaining") are a red flag.

3. **Are parallel waves identified?** Within a phase, are there tasks explicitly
   marked as parallelisable? Look for language like "can run in parallel",
   "independent of", "Wave A / Wave B". If all tasks are strictly sequential
   without justification, flag it.

4. **Are dependencies explicit?** Does the plan say why Phase 1 must come before
   Phase 2? Are cross-phase dependencies called out? If dependencies are
   implicit, that's a risk.

5. **Are there concrete exit criteria per phase?** Each phase should define what
   "done" means -- a deliverable, a passing test, a merged PR. Phases that end
   with "iterate" or "review" without a concrete gate are risky.
