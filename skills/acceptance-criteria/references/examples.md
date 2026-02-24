# Acceptance Criteria Examples

## Example: Checkout Submission

Given/When/Then:

```gherkin
Scenario: Successful checkout
Given a user has items in cart
When the user submits payment with valid details
Then an order confirmation page is displayed
And a confirmation email is sent within 5 seconds
```

Rule-oriented:

- Checkout confirmation includes order number.
- Order total matches cart total.
- Invalid payment returns actionable error message.
- Session timeout redirects to login and preserves cart.
