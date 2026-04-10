# Scenario 05: Payment Processing Feature Specification

## User Prompt

A payments team is implementing a new recurring billing feature for a SaaS platform. The feature involves complex business rules around billing cycles, proration, failed payments, and subscription upgrades/downgrades that need to be precisely specified.

The team has scheduled a collaborative session to define acceptance criteria for this feature. They need to align on specific examples that will become the basis for their automated tests, ensuring all team members (product, engineering, QA) have a shared understanding of the expected behavior.

Create the following deliverables:

1. **collaboration-process.md** - Document the approach for aligning on acceptance examples
2. **billing-scenarios.feature** - Feature file based on collaborative specification
3. **scenario-quality-review.md** - Analysis of scenario quality and common pitfalls avoided

Your solution should demonstrate a collaborative approach to scenario creation and avoid common BDD anti-patterns.

## Input Files

The following file is provided. Extract it before beginning.

```
# Stakeholder Perspectives on Recurring Billing

## Product Manager View
- Customers should understand exactly what they'll be charged
- Failed payments need graceful handling with retry logic
- Upgrades should prorate automatically

## Engineering Perspective
- Need clear rules for edge cases like mid-month changes
- Failed payment retry attempts should have limits
- System should handle timezone differences

## QA Concerns
- Previous features had vague acceptance criteria
- Need specific examples of proration calculations
- Edge cases around cancelled subscriptions need coverage

## Business Analyst Notes
- Billing cycles: monthly, quarterly, annual
- Failed payments: retry 3 times over 7 days
- Prorations: calculated daily, rounded to nearest cent
```

## Expected Behavior

1. Document or demonstrate alignment between product, engineering, and QA perspectives before writing scenarios
2. Specify concrete, observable outcomes in Then steps rather than vague assertions like "should work correctly"
3. Specify user-visible or system-observable outcomes that can be verified in scenarios
4. Show consideration of different stakeholder perspectives (product, engineering, QA) in scenario creation
5. Use concrete examples and specific values rather than abstract or generic wording
6. Actively avoid documented BDD anti-patterns like implementation details in steps or scenario dependencies
7. Maintain shared business terminology that all stakeholders can understand

## Success Criteria

- **Three Amigos collaboration**: Documents or demonstrates alignment between product, engineering, and QA perspectives before writing scenarios
- **Avoids vague outcomes**: Then steps specify concrete, observable outcomes rather than vague assertions like "should work correctly"
- **Observable behavior specification**: Scenarios specify user-visible or system-observable outcomes that can be verified
- **Stakeholder alignment evidence**: Shows consideration of different stakeholder perspectives (product, engineering, QA) in scenario creation
- **Specific examples over abstractions**: Uses concrete examples and specific values rather than abstract or generic wording
- **Anti-pattern avoidance**: Actively avoids documented BDD anti-patterns like implementation details in steps or scenario dependencies
- **Business language consistency**: Maintains shared business terminology that all stakeholders can understand

## Failure Conditions

- No documentation of alignment between product, engineering, and QA perspectives
- Then steps use vague assertions like "payment should work" without concrete observable outcomes
- Scenarios describe internal system state rather than user-visible or verifiable outcomes
- Different stakeholder perspectives are not reflected in the scenario design
- Scenarios use abstract wording instead of specific values and concrete examples
- BDD anti-patterns (implementation details, scenario dependencies) are present without acknowledgment
- Business terminology in scenarios is inconsistent or uses technical jargon
