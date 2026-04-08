# Scenario 02: Creating Missing Evaluation Scenarios

## User Prompt

"Create evaluation scenarios for the docker-containerization skill before publishing publicly."

## Expected Behavior

1. Agent locates the skill directory at `skills/infrastructure/docker-containerization/`
2. Creates `evaluation-scenarios/` directory if it doesn't exist
3. Analyzes the SKILL.md content to understand skill purpose and workflows
4. Generates 5-8 comprehensive scenario files (scenario-01.md through scenario-08.md)
5. Each scenario includes: user prompt, expected behavior, success criteria, failure conditions
6. Scenarios cover diverse use cases from the skill (basic, intermediate, edge cases)
7. Uses concrete examples with measurable outcomes

## Success Criteria

- Agent creates evaluation-scenarios/ directory
- Agent generates minimum 5 scenario files (scenario-01.md to scenario-05.md)
- Each scenario file has all four required sections (prompt, behavior, success, failure)
- Scenarios are specific to the skill domain (Docker containerization)
- Success criteria are measurable (files created, commands run, outputs verified)
- Failure conditions clearly indicate when skill was not applied correctly
- Scenarios cover breadth of skill capabilities (basic → advanced)

## Failure Conditions

- Agent creates fewer than 5 scenarios
- Agent uses generic scenarios not specific to Docker containerization
- Scenarios lack measurable success criteria (vague "agent does well")
- Missing any required section (prompt, behavior, success, failure)
- Scenarios test implementation details rather than skill application
- Agent copies scenarios from another skill without adaptation
