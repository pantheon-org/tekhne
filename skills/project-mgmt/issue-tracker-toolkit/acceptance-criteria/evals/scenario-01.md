# Scenario 01: E-commerce Checkout Implementation

## User Prompt

Your team is building a new e-commerce platform for a mid-sized retail company. The product manager has identified that checkout abandonment is a major issue with their current system - users frequently leave during the payment process due to slow performance and unclear error handling. The company processes about 1,000 transactions per day and needs a robust, user-friendly checkout experience.

The development team has the basic shopping cart functionality complete, but the checkout flow needs to be clearly defined before implementation begins. The system needs to handle various payment methods, provide clear feedback to users, and perform reliably under normal load conditions.

Create a comprehensive acceptance criteria document for the checkout flow feature. The document should include:

- User story with clear business value
- Complete acceptance criteria covering the checkout process
- Edge cases and error handling scenarios
- Scope definition to guide development priorities

Save your work as `checkout-acceptance-criteria.md`.

The following files are provided as context. Extract them before beginning.

```text
=============== FILE: inputs/current-issues.md ===============

# Current Checkout Issues

## Performance Problems

- Payment processing taking 8-12 seconds
- Page timeouts during peak hours
- No loading indicators for users

## User Experience Issues

- Generic error messages ("Something went wrong")
- No confirmation of successful orders
- Users unsure if payment was processed

## Business Requirements

- Must support credit cards and PayPal
- Order confirmation required within 5 seconds
- Need to track conversion metrics
```

## Expected Behavior

1. Use Given/When/Then format to structure checkout flow scenarios rather than rule-oriented format
2. Include specific timing requirements with precise measurements (e.g., `<= 3 seconds`)
3. Write Then statements describing observable user outcomes, not internal system behavior
4. Cover failure scenarios such as payment declined, timeout, and invalid input
5. Write When statements containing single, clear actions
6. Include only essential prerequisites in Given statements, avoiding irrelevant setup details
7. Name scenarios after user behavior (e.g., "User completes checkout")
8. Add an explicit Out of Scope section to define boundaries and prevent scope creep

## Success Criteria

- **Given/When/Then format**: Uses Given/When/Then structure for checkout flow scenarios rather than rule-oriented format
- **Measurable timing**: Includes specific timing requirements with precise measurements (e.g., `<= 3 seconds`, `within 500ms`)
- **Observable outcomes**: Then statements describe what user can observe or measure, not internal system behavior
- **Negative scenarios**: Includes failure scenarios like payment declined, timeout, or invalid input
- **Single action When**: When statements contain single, clear actions that trigger behavior being tested
- **Essential Given setup**: Given statements include only prerequisites that matter, avoiding irrelevant setup details
- **User behavior scenarios**: Scenario names describe user behavior (e.g., `User completes checkout`) not system features
- **Out of scope section**: Includes explicit `Out of Scope` section to define boundaries and prevent scope creep
- **Checkbox format**: Uses checkbox format (`- [ ]`) for criteria to enable clear pass/fail validation
- **User outcome focus**: Focuses on user outcomes and business value rather than implementation details

## Failure Conditions

- Uses rule-oriented checklists instead of Given/When/Then for a sequential workflow
- Omits specific timing measurements, using vague terms like "fast" or "quickly"
- Then statements describe internal system state rather than observable user outcomes
- Fails to include any payment failure or error handling scenarios
- When statements contain multiple actions or are ambiguous
- Given statements include unnecessary context that does not affect the scenario outcome
- Scenario names describe system features rather than user behavior
- No Out of Scope section is included, leaving boundaries undefined
- Uses prose descriptions instead of checkbox format for criteria
- Focuses on implementation details rather than what the user experiences
