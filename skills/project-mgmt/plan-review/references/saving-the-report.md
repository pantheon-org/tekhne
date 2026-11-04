# Saving the report and linking it from the plan

Used in step 8 of SKILL.md, after the report is compiled and before it is presented. The review is recorded on disk, so
a tool can tell later which plans were reviewed, how fully, and whether the plan changed since.

## Where the report goes

```text
reviews/<plan-file-name>-review-<dd-mm-yyyy>.yaml
```

This is the `reviews/` folder beside the plan's `plans/` folder, in the repository that holds the plan (step 8 of
SKILL.md gives the full path). `<plan-file-name>` is the plan's file name without `.md`. One report per review, and a
report is never overwritten. If the name for today is taken, use the next free suffix (`-2`, then `-3`). Create the file
so that it fails when it already exists (a shell `set -o noclobber`, or an exclusive open), and stop if it does: two
reviews raced for the same name. The orchestrating agent writes it after collating the reviewers. A reviewer never
writes it.

## What the report records

Three fields beside `title` and `sections`, all required and checked by `scripts/validate-review-report.sh`:

| Field | Value |
| --- | --- |
| `plan` | Path of the reviewed plan, relative to the report's folder, for example `../plans/<plan-file>.md` |
| `lenses_run` | The lenses that returned a review: `technical`, `strategic`, `risk`. A non-empty list, no repeats |
| `plan_hash` | SHA-256 of the plan's body, taken when the brief is composed (step 2), see below |

If a reviewer fails twice and is omitted (see "When a Reviewer Fails" in SKILL.md), `lenses_run` lists only the lenses
that ran. That is what makes the review partial. Never list a lens that did not return a review.

## The plan hash covers the body only

The hash is of the lines after the frontmatter, with LF line endings. The link you add in the next section changes the
frontmatter, and so would a later status change, so a hash of the whole file would make every reviewed plan stale at
once. Take it with:

```bash
LC_ALL=C sed 's/\r$//' PLAN.md | LC_ALL=C awk 'n>=2{print} /^---$/{n++}' | (shasum -a 256 2>/dev/null || sha256sum)
```

`shasum` is missing on some Linux images, so the command falls back to `sha256sum`, and the locale is fixed so `sed` and
`awk` agree across machines. The plans index check of the claude-plugins repository computes the same value in
TypeScript and tests it against this command.

Take it before any edit to the plan, and again just before writing the report. If the two differ, the body changed
while the reviewers ran: do not write the report, and say so. Editing the body, including the editorial fixes of
step 10, changes it. A plan has frontmatter (the frontmatter validator of step 3 rejects one without it), so the command
has no case for a plan without it.

## Linking the plan

After the report validates, add its path to the plan's `reviewed-by` frontmatter list, relative to the plan's folder
(for example `../reviews/<report file>`). Add it only if it is not already there, so running the step twice changes
nothing. Create the key if the plan has none. Never write `reviewed-by: []`. Only the frontmatter changes, so the hash
still matches.

## Findings and a second review

Step 10 edits the plan body after the review (editorial fixes, a `## Decisions` section). That makes the report stale.
Say so in the presentation. After such edits, run the review again: it makes a second report (`-2` on the same day, or
the new date) with the new hash, and the plan lists both. The state is judged on the reports that match the current
body, so `full` means three lenses on the text as it stands.

## What the state means

`full` means the three lenses ran on the current body. It does not mean the findings were resolved or the plan was
approved, which is the owner's decision. If the harness added text to the reviewer prompts (some plugins append routing
hints), say so in a comment at the top of the report, so a later reader knows the reviewers saw more than the brief.

## How a tool reads the state

- No `reviewed-by`, or an empty one: none.
- Reports are linked but none has the plan's current hash: stale.
- Among the reports with the current hash, the union of `lenses_run` covers all three lenses: full. Fewer: partial.
- The order of the list does not matter. A report of an older body never adds lenses.

## Requirements

`scripts/validate-review-report.sh` needs bun 1.3.10 or later and nothing else: no python and no PyYAML. Without bun, or
with an older one, it says what is needed and fails. It runs `scripts/validate-review-report.ts` with `bun --no-install`,
so it also works from an installed copy of the skill, which has no `node_modules`.
