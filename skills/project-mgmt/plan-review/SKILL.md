---
name: plan-review
description: >
  Use when asked to review, audit or quality-check a plan in .context/plans/. Runs 3
  independent subagent reviewers (Technical, Strategic, Risk) on the same plan brief and
  consolidates their findings. Reads reviewer models from .claude/plan-review.json when
  present, otherwise asks the user first. Not for plans outside .context/plans/, one-off
  notes, or external documents.
---

# Plan Review -- 3-Agent Multi-Perspective Audit

Review `.context/plans/` files through 3 independent lenses: **Technical**,
**Strategic**, and **Risk**. Each reviewer is a separate subagent with a unique
prompt and perspective. The main agent collates results into a consolidated report.

## Prerequisites

- A `.context/plans/*.md` file exists to review (or a glob selecting multiple)
- The plan has standard YAML frontmatter (`title`, `type`, `status`, `date`, `tags`)
- Your environment supports spawning `general-purpose` and `Explore` subagents (the Agent tool)

## When to Use

- A `.context/plans/*.md` file needs an independent multi-perspective review before implementation
- A draft plan needs validation before marking it `READY`
- Multiple plans exist in the same domain and need prioritisation
- A stale plan needs a freshness check against current project state
- The user explicitly asks for a plan audit

## When NOT to Use

- For one-off notes, scratch files, or non-plan documents -- use `session-reflection` instead
- For plans outside `.context/plans/` -- the prompts assume `.context/` frontmatter conventions
- When the plan is trivially small (1 paragraph, no steps) -- 3 subagents are not justified
- When the user explicitly asks for a quick opinion, not a full audit

## Mindset

- Three independent reviewers catch what a single reviewer normalises. The value is in the divergence between their findings, not in consensus.
- Model diversity is the strongest lever for perspective diversity. PREFER routing each reviewer to a different model UNLESS the environment genuinely offers no routing (see [Model Routing Reference](references/model-routing.md)).
- A plan review is a service to the plan author, not a judgement. Findings should be actionable, not critical.
- Structural validation is a prerequisite, not the goal: the real value is the architecture and risk analysis from the 3 reviewers.
- If a review reveals a critical issue, improve the plan, do not reject it.
- Classify every critical/moderate finding as Editorial or Decision (Step 10) before calling the review
  finished. Most structural gaps are Editorial (one clearly correct fix); reserve Decision, and its
  interview, for genuine tradeoffs.

## Workflow

### 1. Identify the plan to review

The user specifies a plan path, or you suggest one from `.context/plans/`.
Read the plan in full. Note its frontmatter (`status`, `date`, `related`),
goal, steps, open questions, and risks.

### 2. Compose the plan brief

From the plan file, create a structured brief that all 3 reviewers will receive.
The brief must be self-contained -- each reviewer should not need to read the
original plan file. Include:

- **Title, status, date** -- from frontmatter
- **Goal** -- the plan's stated objective (verbatim if concise, summarised if long)
- **Steps** -- numbered list of implementation steps
- **Dependencies / related files** -- from `related:` frontmatter
- **Open questions** -- listed verbatim
- **Known risks** -- listed verbatim from the plan's risk section (if any)
- **Implementation status** -- what's already been done vs what's still pending
- **Context** -- any relevant background from related findings or docs

### 3. Validate plan structure against the standard trio

Every `.context/` plan should follow a **three-part standard**: a YAML template
defining required fields, a JSON schema for machine validation, and a validation
script. Check the plan against all three.

#### The trio

| Component | Location | What it defines |
|-----------|----------|-----------------|
| **YAML template** | `create-context-file` SKILL.md (frontmatter section) | Required fields: `title`, `type`, `status`, `date`, `tags` |
| **JSON schema** | `create-context-file/assets/schemas/context-frontmatter.schema.json` | Field types, allowed values (`enum`), patterns, required list |
| **Validation script** | `context-index/scripts/validate-context-frontmatter.sh` | CLI script that checks one or more files (its enums mirror the schema) |

#### What to check

Run the validation script on the plan file, with `sh`. Find it in the `context-index` skill with
`find -L <skills dir> -name validate-context-frontmatter.sh` (`-L`: installed skills are symlinks):

```bash
sh <path>/validate-context-frontmatter.sh .context/plans/<plan-file>.md
```

This checks:

- Frontmatter exists and opens/closes with `---`
- All required fields present (`title`, `type`, `status`, `date`, `tags`)
- `type` is one of the lowercase values `finding`, `plan`, `guide`, `follow-up`,
  `merge-request`, `ticket`, `decision`, `note`, `research`
- `status` is `active` or `done`
- `date` matches `YYYY-MM-DD` pattern
- A `tags` key exists (`tags: []` when empty). The schema forbids other fields, but the
  script does not check that, so note any extra field as a Structural Issue yourself

#### If validation passes

Include `**Structure: Valid**` in the plan brief. Proceed to step 4 (model config).

#### If validation fails

Capture each validation error and include it in the plan brief under a
`## Structural Issues` section so all 3 reviewers can reference them. Sample:

```
## Structural Issues
- Missing required field: 'status'
- 'date' does not match pattern YYYY-MM-DD, got '01-07-2026'
```

#### Plan body structure -- infer the local standard

Beyond frontmatter, check the plan's body sections (`##` headings) against what
other plans in this repo actually use. Do NOT hardcode a template -- scan the
existing plans and infer what's conventional. See
[Structural Inference Reference](references/structural-inference.md) for the
frequency-table method, the core/common/rare classification, and why this is
inferred rather than hardcoded.

##### What to report

Include in the plan brief under `## Structural Issues`:

```
### Body Structure (inferred from <N> existing plans)
Core sections (≥40% of plans):
  - Goal: present/missing
  - Steps/Phases: present/missing
  - Open Questions/Decisions: present/missing

Common sections (20-39%):
  - Scope/Out of scope: present/missing
  - Related findings: present/missing
  - Verification: present/missing
  - Risks/sequencing: present/missing

Note: Sections like Background, File-by-file change list, Academic References,
and Critical Review appear in <20% of plans and are optional.
```

Missing core sections are **advisory warnings**; missing common sections are
**neutral observations**; rare/unique sections may indicate scope creep --
note them as "unusual sections" but do not flag them as problems. See the
reference above for the full rationale.

#### Implementation architecture -- Phases, Tasks, and Waves

A well-structured plan divides the work into **Phases** (sequential stages),
**Tasks** (independently assignable units within a phase), and **Waves**
(groups of tasks within a phase that can run in parallel). Check for missing
phase dependencies/exit criteria, oversized or flat (ungrouped) phases, vague
task descriptions, and unidentified parallelisation opportunities -- see
[Structural Inference Reference](references/structural-inference.md) for the
full bad-pattern list and check criteria.

##### What to report

Include in the plan brief under `## Structural Issues`:

```
### Implementation Architecture
Phases: <N> phases identified -- <list phase names>
   - Phase dependencies: explicit/implicit/missing
   - Phase exit criteria: present for <N>/<N> phases

Task granularity: good / too vague / too large
   - <count> of <total> tasks have actionable descriptions

Parallelisation:
   - Waves identified: yes/no
   - Tasks flagged as parallel: <N> of <N>
   - Parallelism opportunity: <N> tasks could run concurrently but aren't grouped
```

### 4. Check for existing model config

Before asking the user anything, look for a saved choice: `.claude/plan-review.json` in the
git root (the working directory outside a git repo), then `~/.claude/plan-review.json` (keys
`technicalStrategic` and `risk`, see the [Model Routing Reference](references/model-routing.md)).
Check a file with `bash <this skill's directory>/scripts/validate-plan-review-config.sh <path>`,
from any working directory: exit 0 is valid, any other exit is invalid. A valid choice names the
models: pass them as each reviewer's `model`, skip steps 5 and 6, and state the models in the
final report. Both keys may name the same model, noting limited diversity. The first file found
decides: if it is invalid, say what the validator printed (and that a valid file further down
was skipped), go to step 5, never fall through to the next file and never guess. If nothing is
configured, go to step 5 and advise the user.

### 5. Ask the user which models to use

Reached only when step 4 found no valid saved choice. Do NOT proceed without asking.
Present the pricing/pairing table via the question box -- see the
[Model Routing Reference](references/model-routing.md) for the tables, the question
structure and the default "not sure" pairing. The goal is always to assign one model to
**Technical + Strategic** (`general` subagent type) and a **different model** to **Risk**
(`explore` subagent type) -- different models mean different blind spots. If the user
picks "Custom", ask them for each reviewer individually and suggest sensible defaults.

### 6. Offer to save the choice

Offer to save the chosen models to `.claude/plan-review.json` (template:
`assets/templates/plan-review.json`) so the question is not asked again. If step 4 found an
invalid file, offer to replace it, naming the file, and never overwrite it unasked. If they decline or
cannot choose, run all 3 but note in the output that model diversity was limited -- the
reviewer prompts alone provide the perspective separation.

### 7. Spawn 3 reviewers (in parallel)

Spawn all 3 subagents at once with the `Agent` tool. Each receives the **same plan brief**
prefixed with their role-specific instructions. Set each call's `model` from step 4 or 5
(`technicalStrategic` for Technical and Strategic, `risk` for Risk); omit it only when the
user declined to choose.

| Reviewer | subagent_type | model | Lens |
|----------|--------------|-------|------|
| Technical | `general-purpose` | `technicalStrategic` | Feasibility, implementation gaps, technical soundness |
| Strategic | `general-purpose` | `technicalStrategic` | Goal alignment, scope correctness, priority, sequencing |
| Risk | `Explore` | `risk` | Blind spots, edge cases, failure modes, dependencies |

The three prompt templates (Technical, Strategic, Risk), each a ready `Agent` call with a
`<PASTE PLAN BRIEF HERE>` slot, are in [Reviewer Prompts](references/reviewer-prompts.md).
Fill the same brief into all three.

### 8. Receive and collate results

Each subagent returns a structured review. Collect all 3 and compile a
**Consolidated Review Report** following the structure defined in
[assets/templates/review-report.yaml](assets/templates/review-report.yaml).
The template is validated against [assets/schemas/review-report.schema.json](assets/schemas/review-report.schema.json)
-- run `scripts/validate-review-report.sh` on the generated report to confirm
schema compliance.

Required sections: Model Configuration, Structural Validation, Implementation
Architecture, Scores, Critical Issues, Moderate Concerns, Strengths, Next Actions.

### 9. Present to the user

Present the consolidated report with a preamble that includes the models used:

> "I ran this plan through 3 independent reviewers using **`model A`** (Technical/Strategic) and **`model B`** (Risk). Here's the consolidated report:"

If the user wants to dig deeper on any finding, offer to spawn a follow-up
investigation subagent with the relevant context.

### 10. Classify findings and resolve -- this step is not optional

Step 9's presentation is a progress update, not sign-off. Before the review counts as
finished, sort every item in `critical_issues` and `moderate_concerns` into one of two
buckets:

- **Editorial** -- a contradiction, missing detail, wrong ownership, or unspecified
  mechanism with one clearly correct fix given the rest of the plan and this repo's
  existing conventions. No real tradeoff exists; a competent engineer looking at the
  same facts would land on the same fix. ALWAYS apply these directly to the plan file --
  don't make the user approve something that isn't actually a choice.
- **Decision** -- a genuine tradeoff with two or more valid answers (hard-fail vs.
  advisory rollout, gate-on-condition vs. run-unconditionally, spike-first vs.
  attempt-and-recover). No amount of re-reading the repo resolves these; only the plan
  owner's judgment does.

For decision-classified items, NEVER leave them sitting in `## Open Questions` for the
user to notice later -- run a short interview immediately: one question at a time, each
with concrete mutually-exclusive options plus room for free text, matching
`guided-interview`'s pattern. End with a one-message recap of every answer and get
explicit confirmation before writing anything into the plan.

Fold both outcomes back into the plan file, not just into chat:

- Editorial fixes land directly in the relevant section (Scope, Phases, Verification).
- Decisions land in a `## Decisions` section, each with the chosen option and why the
  alternative didn't win. If a decision carries a review-later condition (e.g. "flip to
  hard-fail after N clean runs"), state it as a concrete, checkable revisit trigger --
  never "revisit later" -- the same rule `design-debate` enforces on its own verdicts.
- A `## Decisions` heading is an ADR trigger. Check whether an existing ADR already covers
  the plan's topic; if not, run the `adr-creator` skill before treating the amendment as
  done.

### 11. (Optional) Persist a standalone finding

If the review surfaced something beyond this one plan -- a pattern likely to recur in
future reviews, a gap in a related skill -- capture it as a finding via the `create-context-file` skill,
separate from the decisions already folded into the plan in Step 10.

## Anti-Patterns

**NEVER** -- Skip the plan brief and just pass the file path

**WHY:** Subagents cannot read files unless they have the `general` type with file access. The plan brief ensures every reviewer works from the same complete information regardless of subagent capability -- skipping it produces shallow, inconsistent, self-contradicting reviews.

**BAD:** Sending "review .context/plans/foo.md" as the prompt.
**GOOD:** Extracting the plan content into a self-contained brief first.

**NEVER** -- Spawn reviewers sequentially

**WHY:** They are fully independent -- sequential spawning wastes 3x the time and lets context build up between calls. Always use parallel tool calls.

**BAD:** Spawn Technical, wait for result, spawn Strategic, wait, spawn Risk.
**GOOD:** Spawn all 3 in one message with 3 parallel `Agent` tool calls.

**NEVER** -- Proceed without asking about models first, unless a valid `.claude/plan-review.json` or `~/.claude/plan-review.json` already names them

**WHY:** Model diversity is the main lever for catching different blind spots. Asking first gives the user control over cost vs. depth; skipping it silently defaults to one model for all reviewers, and the user loses trust when they discover routing was available but unused. A saved config is the user having answered already, so asking again is the opposite failure.

**BAD:** "Reviewing the plan now with 3 reviewers..."
**GOOD:** (no saved config) "Before I start, which models should I use for the reviewers?"

**NEVER** -- Present raw subagent output without attribution

**WHY:** The user needs to know which perspective each finding comes from -- raw, unlabelled output mixes actionable findings with noise and loses the multi-reviewer value entirely.

**BAD:** Paste three blocks of text with no headers.
**GOOD:** Organise by reviewer with clear headings and attribution.

**NEVER** -- Modify the plan brief differently per reviewer

**WHY:** The whole point is that different reviewers reach different conclusions from the *same* information. A different brief per reviewer turns real disagreement into an artefact of different input, hiding true consensus and wasting time on false conflicts.

**BAD:** Giving the Strategic reviewer extra context about project history.
**GOOD:** Identical brief to all 3; the differences in output come from their lenses.

**NEVER** -- Use the same subagent type for all 3 reviewers

**WHY:** Model diversity is the main lever for catching different blind spots. Using the same subagent type for all 3 loses the independent-perspective benefit -- they converge on similar blind spots, so the review costs 3x but delivers the value of one pass.

**BAD:** All 3 use `general`.
**GOOD:** Mix `general` and `explore`.

**NEVER** -- Leave a decision-type finding sitting in Open Questions after the review

**WHY:** A report is a snapshot, not a commitment. Presenting findings (Step 9) and resolving them (Step 10) are different steps -- skipping from one to "the review is done" ships the plan with a real tradeoff unresolved, against whichever reading happened to be top of mind rather than a decision anyone actually made.

**BAD:** Present the consolidated report, thank the user, stop.
**GOOD:** Present the report, then interview the user on every decision-type finding
before treating the plan as amended (Step 10).

## Verification

After presenting the consolidated review report, run these checks before
signing off:

1. **Schema compliance** -- run the validation script to verify the report output:

   ```bash
   scripts/validate-review-report.sh <path-to-report>
   ```

   This checks the report conforms to the required structure and all sections
   are present. Re-run if validation fails.

2. **Model attribution check** -- confirm the report states which models were
   used for Technical/Strategic and for Risk, and whether routing was
   pre-configured or user-selected.

3. **Structural validation captured** -- confirm the plan's frontmatter validation
   results and body structure inference are included in the report. The structural
   validation must pass before the review is considered complete.

4. **Actionability review** -- verify the report ends with specific Recommended
   Next Actions that reference specific reviewer findings. Each action must be
   concrete and directly address a finding.

5. **Investigation offered** -- ask the user if they want to dig deeper on any
   finding. If they accept, run the investigation before concluding.

6. **Every critical/moderate finding is classified and resolved** -- confirm each item
   from `critical_issues` and `moderate_concerns` is either applied as an editorial fix
   in the plan file, or answered through the Step 10 interview and recorded in a
   `## Decisions` section. Run `grep -c '^- ' <finding-list>` against the plan's diff if
   unsure whether anything was dropped.

If any of these checks fail, correct the issue before presenting the report as
final.

## Error Handling

| Situation | Response |
|-----------|----------|
| Report validation fails | Check the report against `assets/schemas/review-report.schema.json` for missing or malformed sections; otherwise regenerate |
| A reviewer does not return results | Re-spawn that reviewer individually with the same brief; if a reviewer fails twice, fall back to the other two |
| User has no preference on models | Recommend the Claude Code default pair from the Model Routing Reference; stop if they confirm |
| Plan is trivially small | Skip if fewer than 2 H2 sections -- do a single pass instead |
| Review reveals a critical issue | Stop if critical and present immediately; ask if the user wants to fix before continuing |

## When a Reviewer Fails

If a subagent reviewer returns an error or times out, do not block the entire
review. Spawn a replacement using the same subagent type and brief. If the
replacement also fails, omit that perspective and proceed with the remaining
reviews. Note the failure in the final report so the user knows a perspective
is missing.

## Templates

This skill follows the repository convention of YAML template + JSON Schema + validation script
for all structured artifacts. See the project rules for the convention.

| Artifact | Template | Schema | Validation Script |
|----------|----------|--------|-------------------|
| Consolidated Review Report | [assets/templates/review-report.yaml](assets/templates/review-report.yaml) | [assets/schemas/review-report.schema.json](assets/schemas/review-report.schema.json) | [scripts/validate-review-report.sh](scripts/validate-review-report.sh) |
| Model Selection Question | [assets/templates/model-selection.yaml](assets/templates/model-selection.yaml) | [assets/schemas/model-selection.schema.json](assets/schemas/model-selection.schema.json) | [scripts/validate-model-selection.sh](scripts/validate-model-selection.sh) |
| Saved reviewer models (`.claude/plan-review.json`) | [assets/templates/plan-review.json](assets/templates/plan-review.json) | [assets/schemas/plan-review-config.schema.json](assets/schemas/plan-review-config.schema.json) | [scripts/validate-plan-review-config.sh](scripts/validate-plan-review-config.sh) |

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Saved model config and pairings for model diversity | [Model Routing Reference](references/model-routing.md) | Step 4/5, configuring or advising on per-reviewer model routing |
| The three reviewer prompt templates | [Reviewer Prompts](references/reviewer-prompts.md) | Step 7, building each reviewer's `Agent` call |
| Persisting review findings, and writing a plan's `## Decisions` section | `create-context-file` skill | Step 10/11, folding decisions or standalone findings back into durable files |
| The one-question-at-a-time interview pattern | `guided-interview` skill | Step 10, resolving a Decision-classified finding with the user |
| Naming a concrete, checkable revisit trigger for a time-boxed decision | `design-debate` skill | Step 10, whenever a decision carries a "review later" condition |
| Single-agent session-end reflection | `session-reflection` skill | Complementary use, not a substitute for a 3-reviewer plan audit |
| Capturing a `## Decisions` section as an ADR | `adr-creator` skill | Step 10, immediately after any plan gains a `## Decisions` heading |
