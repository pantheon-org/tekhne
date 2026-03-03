# User Onboarding Experience

## Problem Description

Your team is developing a B2B SaaS platform for inventory management, and the current user onboarding process is causing significant customer churn. New users sign up but abandon the platform within the first week because they can't figure out how to set up their inventory systems effectively.

The customer success team reports that users get overwhelmed by the initial setup process - they need to configure their product catalog, set up warehouse locations, define user roles, and integrate with their existing systems. Currently, there's no guided flow, and users are dropped into a complex dashboard with no clear next steps.

The business team wants to reduce time-to-value from 2 weeks to 3 days, and increase activation rates from 35% to 65%. They've identified a step-by-step onboarding flow as the key to achieving these metrics.

The product team needs to design an onboarding experience that guides new users through essential setup tasks while being flexible enough to accommodate different business types and sizes.

## Output Specification

Create comprehensive acceptance criteria for the user onboarding workflow. Your document should include:

- Well-structured user story following best practices
- Detailed acceptance criteria for the onboarding process
- Quality validation to ensure the story meets development standards
- Stakeholder alignment considerations

Save your work as `onboarding-workflow-criteria.md`.

## Input Files

The following files are provided as context. Extract them before beginning.

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

