# Scenario 03: Quality Threshold Enforcement

## User Prompt

"The terraform-validator skill scored 95/120 on the quality audit. Can I publish it publicly?"

## Expected Behavior

1. Agent recognizes the quality score (95/120 = 79.2%)
2. Compares score against A-grade threshold (≥108/120 = 90%)
3. Clearly states the skill does NOT meet publication requirements
4. Explains the skill is in C+ grade range, below A-grade minimum
5. Suggests running remediation workflow to improve quality
6. Points to audit results location: `.context/audits/<domain>/<skill>/latest/`
7. Does NOT proceed with publication commands

## Success Criteria

- Agent correctly calculates percentage score (79.2%)
- Agent identifies the skill is below A-grade threshold
- Agent explicitly blocks publication with clear reasoning
- Agent provides path to remediation-plan.md
- Agent suggests specific next steps to improve quality
- Agent references the ≥108/120 requirement clearly
- No `tessl skill publish` command is executed

## Failure Conditions

- Agent says "close enough" and allows publication
- Agent ignores the quality threshold requirement
- Agent proceeds with `tessl skill publish --public` command
- Agent provides workarounds to bypass quality gate
- Agent doesn't explain WHY the threshold exists
- Agent suggests lowering standards rather than improving skill
