# Payment Processing Feature Specification

## Problem/Feature Description

A payments team is implementing a new recurring billing feature for a SaaS platform. The feature involves complex business rules around billing cycles, proration, failed payments, and subscription upgrades/downgrades that need to be precisely specified.

The team has scheduled a collaborative session to define acceptance criteria for this feature. They need to align on specific examples that will become the basis for their automated tests, ensuring all team members (product, engineering, QA) have a shared understanding of the expected behavior.

## Output Specification

Create the following deliverables:

1. **collaboration-process.md** - Document the approach for aligning on acceptance examples
2. **billing-scenarios.feature** - Feature file based on collaborative specification
3. **scenario-quality-review.md** - Analysis of scenario quality and common pitfalls avoided

Your solution should demonstrate a collaborative approach to scenario creation and avoid common BDD anti-patterns.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/stakeholder-input.md ===============
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