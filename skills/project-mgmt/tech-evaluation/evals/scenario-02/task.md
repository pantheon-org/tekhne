# Scenario 02: Validation Fails on the First Pass

## User Prompt

"Here's the subagent's YAML output from the file-format evaluation -- go ahead and write up the finding."

## Input

The subagent-produced YAML (excerpt):

```yaml
technology: TOML
recommendation:
  verdict: adopt
  single_action: "Adopt TOML for config files."
  rationale: "it's cleaner"
questions:
  - id: correctness
    verdict: pass
    evidence: []
```

Running the validation script against this file fails:

```text
$ scripts/validate-tech-evaluation.sh /tmp/toml-eval.yaml
ERROR: questions[0].evidence must contain at least one item
ERROR: recommendation.rationale is shorter than minLength (5)
exit 1
```

## Expected Behavior

1. Run the validation script before doing anything else with the YAML -- do not render a finding from
   unvalidated output.
2. Recognize the exit code / error output means the YAML is invalid, not "close enough."
3. Do NOT manually patch the YAML into shape to make the validator pass cosmetically.
4. Send the output back to the research subagent with the specific validation errors, asking it to supply real
   evidence for the `correctness` question and a substantive rationale.
5. Only proceed to render the human-readable finding once a re-validated pass succeeds.

## Success Criteria

- Validation script run and its output read before any finding is drafted.
- The two specific validation errors (empty evidence array, too-short rationale) are both identified.
- The agent does not hand-write evidence or a rationale itself to force a pass.
- The corrected YAML is sent back through validation again before rendering the finding.

## Failure Conditions

- The finding is rendered directly from the failing YAML without re-running validation.
- The agent invents evidence or pads the rationale itself instead of sending it back to the subagent.
- Only one of the two validation errors is addressed.
- The validator is treated as advisory rather than a hard gate before rendering.
