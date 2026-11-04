# Saving the report and linking it from the plan

Used in step 8 of SKILL.md, after the report is compiled and before it is presented. The review is recorded on disk, so
a tool can tell later which plans were reviewed, how fully, and whether the plan changed since.

## Where the report goes

```text
reviews/<plan-file-name>-review-<dd-mm-yyyy>.yaml
```

This is the `reviews/` folder beside the plan's `plans/` folder, in the repository that holds the plan (step 8 of
SKILL.md gives the full path). `<plan-file-name>` is the plan's file name without `.md`. One report per
review: a second review of the same plan adds a new file (a second one on the same day adds `-2`), it never overwrites
the first. The orchestrating agent writes it after collating the reviewers. A reviewer never writes it.

## What the report records

Three fields beside `title` and `sections`, all required and checked by `scripts/validate-review-report.sh`:

| Field | Value |
| --- | --- |
| `plan` | Path of the reviewed plan, relative to the report's folder, for example `../plans/<plan-file>.md` |
| `lenses_run` | The lenses that returned a review: `technical`, `strategic`, `risk`. A non-empty list, no repeats |
| `plan_hash` | `shasum -a 256 <plan file>`, taken when the brief is composed (step 2), before any edit |

If a reviewer fails twice and is omitted (see "When a Reviewer Fails" in SKILL.md), `lenses_run` lists only the lenses
that ran. That is what makes the review partial. Never list a lens that did not return a review.

## Linking the plan

After the report validates, add its path to the plan's `reviewed-by` frontmatter list, relative to the plan's folder
(for example `../reviews/<report file>`). Add it only if it is not already there, so running the step twice changes
nothing. Create the key if the plan has none. Never write `reviewed-by: []`.

Step 10 edits the plan after the review (editorial fixes, a `## Decisions` section). Those edits change the plan's
hash, so a tool reading `plan_hash` will call the review stale. Say so in the presentation, and offer a fresh review.

## How a tool reads the state

- No `reviewed-by`, or an empty one: none.
- The union of `lenses_run` over the listed reports covers all three lenses: full. Fewer: partial.
- The plan's current hash differs from `plan_hash` in the newest report: stale.

## Requirements

`scripts/validate-review-report.sh` needs python3 with PyYAML. Without it the script says so and fails. On a machine
where the default python3 lacks PyYAML, put one that has it first on `PATH`, for example `PATH=/usr/bin:$PATH`.
