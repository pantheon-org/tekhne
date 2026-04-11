# Scenario 04: User Onboarding Experience

## User Prompt

Your team is developing a B2B SaaS platform for inventory management, and the current user onboarding process is causing significant customer churn. New users sign up but abandon the platform within the first week because they can't figure out how to set up their inventory systems effectively.

The customer success team reports that users get overwhelmed by the initial setup process - they need to configure their product catalog, set up warehouse locations, define user roles, and integrate with their existing systems. Currently, there's no guided flow, and users are dropped into a complex dashboard with no clear next steps.

The business team wants to reduce time-to-value from 2 weeks to 3 days, and increase activation rates from 35% to 65%. They've identified a step-by-step onboarding flow as the key to achieving these metrics.

The product team needs to design an onboarding experience that guides new users through essential setup tasks while being flexible enough to accommodate different business types and sizes.

Create comprehensive acceptance criteria for the user onboarding workflow. Your document should include:

- Well-structured user story following best practices
- Detailed acceptance criteria for the onboarding process
- Quality validation to ensure the story meets development standards
- Stakeholder alignment considerations

Save your work as `onboarding-workflow-criteria.md`.

The following files are provided as context. Extract them before beginning.

```text
=============== FILE: inputs/onboarding-research.md ===============
# User Onboarding Research

## Current Problems
- 65% of users never complete initial setup
- Average time-to-first-value: 14 days
- High support ticket volume for setup questions
- Users don't understand which steps are required vs optional

## User Research Findings
- Users want guided tour with progress indicators
- Need ability to skip non-essential steps
- Want to see value quickly (sample data helps)
- Different user types need different setup paths

## Business Requirements
- Must integrate with existing user management system
- Should work on mobile devices (responsive design)
- Need analytics to track completion rates by step
- Onboarding should be completable in under 30 minutes

## Success Metrics
- Target: 65% activation rate (up from 35%)
- Target: 3 days to first value (down from 14 days)
- Support ticket reduction: 40% fewer setup-related tickets
```

## Expected Behavior

1. Apply INVEST criteria (Independent, Negotiable, Valuable, Estimable, Small, Testable) to validate the user story
2. Use Given/When/Then format for multi-step onboarding workflow rather than rule-oriented format
3. Reference the need for alignment with product owner, QA, and development team before implementation
4. Ensure each onboarding step can be completed independently without unrelated dependencies
5. Write criteria with objective, binary pass/fail validation
6. Name scenarios after user actions and behavior rather than system functionality
7. Demonstrate clear user or business value in the story and criteria
8. Write requirements clear enough for the development team to estimate effort
9. Scope the story to be completable within one iteration
10. Allow for negotiable implementation details during development

## Success Criteria

- **INVEST validation applied**: Uses INVEST criteria (Independent, Negotiable, Valuable, Estimable, Small, Testable) to validate the user story
- **Given/When/Then sequences**: Uses Given/When/Then format for multi-step onboarding workflow rather than rule-oriented format
- **Stakeholder alignment mention**: References need for alignment with product owner, QA, and development team before implementation
- **Sequential step validation**: Each step can be completed independently without unrelated dependencies (Independent criterion)
- **Testable binary criteria**: Criteria have objective, binary pass/fail validation (Testable criterion)
- **User behavior scenarios**: Scenario names describe user actions and behavior rather than system functionality
- **Business value emphasis**: Clearly demonstrates user or business value in the story and criteria (Valuable criterion)
- **Estimable clarity**: Requirements are clear enough for development team to estimate effort (Estimable criterion)
- **Single iteration scope**: Story is scoped to be completable within one iteration (Small criterion)
- **Implementation flexibility**: Allows for negotiable implementation details during development (Negotiable criterion)

## Failure Conditions

- Skips INVEST validation entirely or applies it superficially without addressing each criterion
- Uses rule-oriented checklists instead of Given/When/Then for a sequential onboarding workflow
- Omits any reference to stakeholder alignment before implementation
- Onboarding steps have dependencies that prevent independent validation
- Criteria use subjective language that cannot be objectively tested as pass/fail
- Scenario names describe system features rather than user actions
- Business value is absent or vague in the story and criteria
- Requirements are too ambiguous for a development team to estimate effort
- Story scope spans multiple iterations without being broken down
- Implementation approach is prescribed rather than left to the team's discretion
