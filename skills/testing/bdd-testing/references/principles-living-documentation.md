---
category: principles
priority: CRITICAL
source: bdd-principles
---

# Living Documentation

BDD scenarios serve as executable specifications that remain synchronized with actual system behavior, providing always-current documentation that technical and non-technical stakeholders can trust.

## Core Concept

Living Documentation is documentation that:
1. **Executes** - Runs as automated tests
2. **Validates** - Verifies actual behavior
3. **Updates** - Fails when behavior changes
4. **Communicates** - Readable by business stakeholders

## Four Functions of BDD Scenarios

### 1. Executable Specifications

```gherkin
Feature: Promotional Discount Application
  To attract customers and increase sales
  As a marketing manager
  I want to offer promotional discounts

  Rule: Percentage discounts apply to order subtotal
    Example: 20% off for orders over $100
      Given I have a $150 order
      When I apply a "20% off" promotion
      Then my discount should be $30
      And my order total should be $120
```

This scenario IS the specification AND the test.

### 2. Up-to-Date Documentation

Unlike traditional documentation:
- **Never stale** - Fails when behavior changes
- **Self-verifying** - Green = accurate, Red = outdated
- **Automatic maintenance** - Forces updates during development

### 3. Common Language Bridge

```gherkin
Feature: Subscription Management

  # Define domain terms upfront
  # A "subscriber" is a customer with an active subscription
  # A "churned subscriber" has cancelled within last 30 days
  # "Winback" is re-activating a churned subscriber

  Scenario: Winback offer for churned subscriber
    Given a churned subscriber from 15 days ago
    When they visit the pricing page
    Then they should see the winback offer
```

Developers, testers, and product managers all understand this.

### 4. Regression Safety Net

Every scenario documents and protects a behavior:
```gherkin
Scenario: Order total includes tax
  Given I have a $100 order
  And my state has 8% sales tax
  When I view my order total
  Then the total should be $108
```

If someone changes tax calculation logic, this scenario fails immediately.

## Documentation Patterns

### Link to External Documentation

```gherkin
@jira:PAY-123
@confluence:payment-rules
@since:v2.1
Feature: Payment Processing
  
  # Links business context to technical implementation
  # Updated: 2024-01-15
  # Owner: Payments Team
```

### Version Tracking

```gherkin
@deprecated
@replaced-by:new-payment-flow.feature
Scenario: Legacy payment flow
  # Kept for backward compatibility until v3.0
  # Will be removed: 2024-06-01
```

### Document Business Rules

```gherkin
Feature: Order Refunds

  Rule: Full refunds available within 30 days
    Scenario: Refund within return window
      Given an order placed 15 days ago
      When customer requests refund
      Then full refund should be processed

    Scenario: Refund after return window
      Given an order placed 45 days ago
      When customer requests refund
      Then refund should be denied
      And customer should see "Return window expired"

  Rule: Only one promotion per order
    Scenario: Cannot stack promotions
      Given I have $100 order with "10% off" applied
      When I try to apply "Free shipping"
      Then I should see "One promotion per order"
      And only "10% off" should be active
```

Each Rule documents a business constraint with concrete examples.

## Living Documentation Reports

Generate readable HTML reports from scenarios:

```bash
npx cucumber-js --format html:reports/features.html
```

Reports show:
- All features and scenarios
- Pass/fail status
- Execution time
- Tags and metadata
- Historical trends

## Benefits

### For Business Stakeholders
- **Visibility** - See what's implemented vs planned
- **Confidence** - Green tests = working features
- **Understanding** - Plain language, concrete examples

### For Developers
- **Specification** - Clear requirements before coding
- **Regression protection** - Know when behavior breaks
- **Onboarding** - New devs learn from scenarios

### For Testers
- **Test coverage** - What's tested is documented
- **Edge cases** - Captured in scenarios
- **Automation** - Tests run continuously

## Best Practices

**Document key behaviors, not every detail:**
```gherkin
# GOOD - Documents important business rule
Scenario: Premium users get free expedited shipping
  Given I am a Premium member
  When I select expedited shipping
  Then shipping cost should be $0

# AVOID - Too granular for documentation
Scenario: Button color changes on hover
  When I hover over the submit button
  Then the button should be #007bff
```

**Keep scenarios readable:**
```gherkin
# GOOD - Business language
Scenario: Checkout with insufficient inventory
  Given product "Laptop" has 0 stock
  When I try to purchase "Laptop"
  Then I should see "Out of stock"

# AVOID - Technical details
Scenario: Database constraint violation
  Given inventory table has stock_count = 0
  When POST /api/orders with product_id = 123
  Then response should be 409 Conflict
```

**Update scenarios when requirements change:**
```gherkin
# When business rule changes from 14 to 30 days,
# update scenario immediately:

Scenario: Refund available within 30 days  # Was: 14 days
  Given order placed 25 days ago            # Was: 10 days ago
  When customer requests refund
  Then full refund should be processed
```

## Anti-Patterns

❌ **Outdated scenarios:**
```gherkin
@ignore  # Don't ignore! Fix or delete
Scenario: Old feature that changed
```

❌ **Non-executable documentation:**
```gherkin
# This is just a comment, not a runnable scenario
# Users can reset passwords via email
# Tokens expire after 1 hour
```

Instead:
```gherkin
Scenario: Password reset token expiration
  Given I requested password reset 61 minutes ago
  When I try to use the reset link
  Then I should see "Reset link expired"
```

❌ **Technical-only documentation:**
```gherkin
Scenario: JWT refresh token rotation
  When POST /auth/refresh with valid refresh_token
  Then response includes new access_token
  And old refresh_token is revoked
```

Better:
```gherkin
Scenario: Stay logged in with automatic token refresh
  Given I logged in 55 minutes ago
  When I continue using the application
  Then my session should remain active
  And I should not be logged out
```

## Living Documentation Mindset

Think of scenarios as:
- **Specifications first, tests second**
- **Business communication, technical automation**
- **Documentation that enforces itself**
- **Requirements that never lie**

When scenarios fail, they're telling you:
- "Behavior changed" (update scenario), or
- "Bug introduced" (fix code)

Never ignore failing scenarios. They're your documentation screaming "I'm outdated!"

## Example: Complete Living Documentation

```gherkin
Feature: Gift Card Purchase and Redemption
  As a customer
  I want to buy and use gift cards
  So I can give gifts or save for later purchases

  @since:v2.0
  @owner:payments-team
  
  Rule: Gift cards have minimum and maximum amounts
    
    @acceptance
    Scenario: Purchase $50 gift card
      When I purchase a $50 gift card
      Then a unique gift card code should be generated
      And the gift card balance should be $50

    @edge-case
    Scenario: Minimum purchase amount enforced
      When I try to purchase a $5 gift card
      Then I should see "Minimum gift card amount is $10"

    @edge-case
    Scenario: Maximum purchase amount enforced
      When I try to purchase a $1000 gift card
      Then I should see "Maximum gift card amount is $500"

  Rule: Gift cards can be partially redeemed
    
    @acceptance
    Scenario: Partial redemption
      Given I have a gift card with $50 balance
      When I apply it to a $30 order
      Then $30 should be deducted from gift card
      And remaining balance should be $20
      And my order total should be $0

    @edge-case
    Scenario: Multiple partial redemptions
      Given I have a gift card with $100 balance
      When I apply it to a $30 order
      And later apply it to another $40 order
      Then remaining balance should be $30
```

This feature file is:
- ✅ Executable (runs as automated test)
- ✅ Documentation (explains gift card rules)
- ✅ Specification (defines required behavior)
- ✅ Communication tool (business + technical can read)
- ✅ Regression safety (catches breaking changes)

## Related References

- @see principles-core-philosophy.md - Discovery-Development-Delivery cycle
- @see principles-ubiquitous-language.md - Shared vocabulary
- @see collaboration-stakeholder-involvement.md - Keeping stakeholders engaged
- @see patterns-domain-language.md - Writing business-readable scenarios
