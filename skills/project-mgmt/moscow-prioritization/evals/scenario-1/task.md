# Scenario 1: Categorize Requirements into MoSCoW Tiers

## Context

A startup is launching a customer-facing billing portal in 6 weeks with a team of 3
engineers. The product manager has provided the following requirements:

1. Display current subscription plan and price
2. Allow users to update payment method (credit card)
3. Show invoice history (last 12 months)
4. Export invoices as PDF
5. Support PayPal as an alternative payment method
6. Send email receipt on successful payment
7. Allow users to cancel subscription
8. Support multi-currency display
9. Show payment failure reason with retry option
10. Dark mode theme

**Constraints:** 6-week deadline, 3 engineers, no budget to extend timeline.

## Your Task

Apply the MoSCoW prioritization skill to categorize all 10 requirements. Produce
a decision table saved to `moscow-table.md`.

## Requirements

The output must:

1. Confirm the release constraints at the top of the document.
2. Contain a table with columns: Requirement | Category | Business Rationale | Risk if Omitted | Owner.
3. Assign each of the 10 requirements to Must, Should, Could, or Won't.
4. Include at least one explicit Won't item with a revisit date.
5. Apply the failure-focused test (release fails without this?) for any Must assignment.

## Output Spec

File: `moscow-table.md`
