# Scenario 03: Skill Improvement Planning

## User Prompt

Your team has completed a quality audit of your AI agent skills and found several that fall below the acceptable quality threshold. The audit identified specific weaknesses in different skills: some lack expert knowledge, others have poor anti-pattern documentation, and a few need better structural organization.

The development lead wants you to create a systematic improvement plan for one of the underperforming skills. The plan needs to be structured, actionable, and include specific steps that developers can follow to address each identified weakness. The plan should also include success criteria so the team can verify when improvements are complete.

The improvement plan needs to be detailed enough that any team member could implement it, with specific examples of what changes to make, which files to modify, and how to measure success. The plan should follow a consistent format that can be reused for other skills in the future.

## Output Specification

Create a comprehensive remediation plan that includes:

1. **remediation-plan.md** - Complete structured improvement plan
2. **success-criteria.md** - Measurable targets for verification
3. **implementation-steps.sh** - Script showing key commands and validation steps
4. **plan-validation.md** - Document validating the plan follows proper format

## Expected Behavior

1. Begin with an executive summary containing current score, target score, grades, priority, effort estimate, and focus areas
2. Identify top issues in a table with dimension references, severity levels, and impact descriptions
3. Organise improvements into phases with specific targets and step-by-step actions
4. Reference or use `generate-remediation-plan.sh` or equivalent automation tools
5. Validate the plan using `validate-remediation-plan.sh` or schema compliance checks — never ship without validation
6. Identify exact files to modify with specific changes needed
7. Define measurable success targets as dimension scores and overall score improvements
8. Use S/M/L effort estimates for phases and total time

## Success Criteria

- **Executive summary format**: Includes current score, target score, grades, priority, effort estimate, and focus areas
- **Critical issues table**: Identifies top issues with dimension references, severity levels, and impact descriptions
- **Phase-based organization**: Organizes improvements into phases with specific targets and step-by-step actions
- **Remediation script usage**: References generate-remediation-plan.sh script or similar automation tools
- **Schema validation**: Uses validate-remediation-plan.sh or mentions schema compliance checking
- **NEVER validation rule**: Follows the rule to never ship plans without validation checks
- **Specific file modifications**: Identifies exact files to modify with specific changes needed
- **Success criteria metrics**: Defines measurable targets like dimension scores and overall score improvements
- **T-shirt sizing effort**: Uses S/M/L effort estimates for phases and total time estimates
- **Code block escaping**: Uses 4 backticks when documenting markdown examples that contain code fences
- **Honest quality rating**: Includes self-assessment rating out of 10 for plan quality and comprehensiveness

## Failure Conditions

- Produces a plan without an executive summary or numerical scores
- Lists weaknesses without grouping them into phases with actionable steps
- Skips validation — ships the plan without checking schema compliance
- Does not reference specific files to modify, leaving developers without concrete direction
- Uses only vague success criteria ("improve the skill") instead of measurable score targets
- Omits effort estimates, making prioritisation impossible
