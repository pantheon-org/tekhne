# Scenario 01: Categorize Requirements into MoSCoW Tiers

## User Prompt

A startup is launching a customer-facing billing portal in 6 weeks with a team of 3 engineers. The product manager has provided the following requirements:

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

Apply the MoSCoW prioritization skill to categorize all 10 requirements. Produce a decision table saved to `moscow-table.md`.

The output must:

1. Confirm the release constraints at the top of the document.
2. Contain a table with columns: Requirement | Category | Business Rationale | Risk if Omitted | Owner.
3. Assign each of the 10 requirements to Must, Should, Could, or Won't.
4. Include at least one explicit Won't item with a revisit date.
5. Apply the failure-focused test (release fails without this?) for any Must assignment.

## Expected Behavior

1. State the 6-week deadline, 3-engineer team, and no-extension constraint explicitly before the categorization table
2. Assign every requirement (1-10) to a Must/Should/Could/Won't category
3. Populate every row with non-empty business rationale and risk-if-omitted content (not just blanks or "N/A")
4. Categorize at least one requirement as Won't and include an explicit revisit date for it
5. Keep clearly preference-based items (dark mode, PayPal, multi-currency) out of the Must tier

## Success Criteria

- **Release constraints confirmed**: `moscow-table.md` explicitly states the 6-week deadline, 3-engineer team, and no-extension constraint before the categorization table
- **All 10 requirements categorized**: Every requirement (1-10) appears in the table with a Must/Should/Could/Won't category assigned
- **Rationale column populated**: Every row has a non-empty business rationale and risk-if-omitted entry (not just blanks or "N/A")
- **Explicit Won't with revisit date**: At least one requirement is categorized as Won't and includes an explicit revisit date
- **Preference-only items not in Must**: Items that are clearly preference-based (dark mode, PayPal, multi-currency) are NOT in the Must tier

## Failure Conditions

- Does not state release constraints before the categorization table
- Omits one or more requirements from the table
- Leaves rationale or risk-if-omitted cells blank for any row
- Has no Won't category items, or has Won't items without revisit dates
- Places dark mode, PayPal support, or multi-currency display in the Must tier
