# Eval Scenario Format

Every skill keeps its evals in `evals/`, with one folder per scenario and two
summary files at the `evals/` root. This is the layout `pantheon-skill-auditor`
scores for D9 (Eval Validation).

```text
evals/
  instructions.json        every instruction extracted from SKILL.md
  summary.json             coverage of those instructions by the scenarios
  scenario-1/
    task.md                the prompt and the repository state
    criteria.json          weighted checklist, sums to exactly 100
    capability.txt         one sentence naming the capability under test
  scenario-2/
    ...
```

## scenario-N/task.md

```markdown
# Scenario N: Title

## User Prompt

"Exact trigger phrase the user would type."

## Repository State

Files, flags or session state the agent starts from.
```

Describe the situation only. Do not list the steps the agent should take or the
outcome you expect: that belongs in `criteria.json`, and putting it in `task.md`
leaks the answer to the agent being tested.

## scenario-N/criteria.json

```json
{
  "context": "What this scenario tests and why.",
  "type": "weighted_checklist",
  "checklist": [
    {
      "name": "Short label",
      "description": "A yes/no check that is traceable to an instruction",
      "max_score": 10
    }
  ]
}
```

- `max_score` values must sum to exactly 100. The auditor does not count a
  scenario whose checklist sums to anything else.
- Use 10 or more items, each a binary check, so two reviewers score the same
  transcript the same way.
- Include failure checks (what a bad response does), not only success checks.
  Mark them with `failure_check`, described next.

### Failure checks

A checklist item can be marked as a failure check: a behaviour the response
must not show.

```json
{
  "name": "Does not delete the user's file",
  "description": "The response never removes or overwrites the original file",
  "failure_check": true
}
```

- `failure_check` is optional. Leave it out for an ordinary scored item, and
  `"failure_check": false` means the same.
- A failure check has **no `max_score`**. It is pass or fail: it passes when
  the bad behaviour is absent.
- It is **outside the 100 sum**, so it never changes a score. If one carries a
  `max_score` anyway the auditor ignores it, and the artifact validator
  reports it.
- D9 reports, without deducting any points, when a scenario has no failure
  check: "N of M scenario(s) have no failure check".
- An older `pantheon-skill-auditor` build ignores the field and reads the item
  as worth 0, so a file that uses it is still accepted and sums as before.

The schema is `assets/schemas/criteria.schema.json`. `scripts/validate-skill-artifacts.sh`
checks the `failure_check` rules for every `criteria.json` under `skills/`.

## scenario-N/capability.txt

One sentence stating the capability under test, for example "Load the default
stream and render the resume report directly from the parsed file."

## instructions.json and summary.json

`instructions.json` lists every instruction in `SKILL.md`, each with the
original text, why it was given (`reminder`, `new knowledge`, `preference` or
`anti-pattern`) and the dimension it supports. `summary.json` records how many
of those the scenarios cover:

```json
{
  "instructions_coverage": {
    "coverage_percentage": 100,
    "total_instructions": 10,
    "covered_instructions": 10,
    "scenario_count": 6
  }
}
```

Coverage of 80% or more earns the full coverage points. The D1 and D3 scores
also read `instructions.json`.

## Quantity

At least 3 valid scenarios (all three files present, checklist summing to 100)
earns the scenario points. Aim for 5 or 6, covering:

- the primary happy path
- edge cases and partial inputs
- failure and anti-pattern detection
- at least one scenario where the skill should refuse, warn or route elsewhere

## Naming

Folders are `scenario-1`, `scenario-2` and so on. The auditor only requires the
`scenario-` prefix, so keep numbering consecutive.

## Legacy and non-standard formats

| Format | Status |
| --- | --- |
| `evals/scenario-NN.md` (flat files) | Legacy. Scores nothing for scenarios, and the auditor warns to migrate to `scenario-N/` folders. An earlier repo-wide migration (#80) standardised on this; the auditor has since moved on. |
| `evals/*.yaml` | Not scored. |
| `evals.md` (single file) | Not scored; does not scale past three or four scenarios. |

To migrate a flat file, split each `scenario-NN.md` into `task.md` (the prompt
and state), `criteria.json` (turn Success Criteria and Failure Conditions into
checks that sum to 100) and `capability.txt`, then add `instructions.json` and
`summary.json`.
