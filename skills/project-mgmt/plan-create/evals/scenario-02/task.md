# Scenario 02: Infer Local Conventions Instead of Hardcoding Them

## User Prompt

"Draft a plan for migrating our Terraform state backend. Use whatever
structure makes sense."

## Expected Behavior

1. Do NOT hardcode a "Goal -> Steps -> Open Questions" template from memory.
2. Scan existing plans to infer actual conventions, e.g. by running:
   `grep -r '^## ' .context/plans/*.md | sed 's/.*## //' | sort | uniq -c | sort -rn`
3. From the resulting frequency table, classify sections as core (>=40% of
   plans), common (20-39%), or rare (<20%), and use that classification to
   decide which headings to include.
4. Check whether existing plans use timestamped (`topic-YYYY-MM-DD.md`) or
   topic-only (`short-description.md`) file names, and follow the majority
   convention rather than picking one arbitrarily.
5. Construct the plan against `assets/templates/plan-scaffold.yaml`.
6. Still include the frontmatter and the always-required `## Goal`,
   `## Phases`, `## Open Questions` sections regardless of what the scan
   returns for optional sections.

## Success Criteria

- The agent runs (or clearly simulates running) the grep-based convention
  scan before drafting section headings.
- The chosen file name follows whichever naming convention the scan
  indicates is dominant in this repo, with the reasoning stated.
- Optional sections included in the draft are justified by the frequency
  table, not asserted from a generic template.
- `## Goal`, `## Phases`, `## Open Questions` are present regardless of scan
  results, since these are always required.
- The plan is not hardcoded to a fixed section list without evidence of the
  inference step.

## Failure Conditions

- The agent drafts "Goal -> Steps -> Open Questions" (or any fixed skeleton)
  without running or describing the convention-inference scan first.
- The file name is chosen without checking the majority naming convention.
- The agent invents section names not seen in any real plan and presents
  them as "the convention" without evidence.
