# Scenario 01: Choosing a Prompt-Templating Library

## User Prompt

"We need to pick a templating library for the Bedrock digest prompts -- properly evaluate mustache vs.
micromustache vs. a hand-rolled `{{var}}` replace function before we commit."

## Input

Context: the Lambda bundles as a single zip via an `archive_file` Terraform resource; there is no
`node_modules` install step at deploy time, so any dependency must either have zero further dependencies or be
pre-bundled. Alternatives on the table: `mustache` (full spec, includes sections/partials), `micromustache` (a
lighter subset, no partials), and a hand-rolled regex-based replace function.

## Expected Behavior

1. Fix the question set: use the five default dimensions, or explicitly justify a substitution.
2. Dispatch a research subagent with the exact package identifiers, the alternatives already on the table, the
   question set verbatim, and an instruction to cite evidence for every verdict.
3. Instruct the subagent to check the actual Terraform bundling config (not assume how Lambda zips "usually"
   work) before making any footprint claim.
4. Require the subagent's output to be YAML matching the tech-evaluation scaffold shape.
5. Run `scripts/validate-tech-evaluation.sh` against the output and check its exit code before trusting it.
6. If validation fails, send the YAML back to the subagent rather than manually patching it.
7. End with exactly one recommendation (`adopt` / `keep_current` / `reject` / `needs_more_research`), never a
   comparison table with no conclusion.

## Success Criteria

- Question set is the default five, or a substitution is explicitly justified.
- Subagent instructed to cite evidence per verdict, not accept "general knowledge."
- Subagent explicitly told to check the real Terraform bundling config for the footprint dimension.
- Validation script run against the output before it is trusted.
- Recommendation is exactly one of the four enum values with a non-hedging rationale.

## Failure Conditions

- The subagent is allowed to answer the footprint question from memory ("Lambda zips usually support
  node_modules") without checking the actual bundling config.
- Validation step skipped because the YAML "looks fine."
- Final answer is a comparison table with no single recommendation, or hedges with "it depends."
- `needs_more_research` used to avoid a decision the evidence already supports.
