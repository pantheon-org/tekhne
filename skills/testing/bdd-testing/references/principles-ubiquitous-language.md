---
category: principles
priority: CRITICAL
source: bdd-principles, bdd-collaboration
---

# Ubiquitous Language

A shared vocabulary used consistently by both technical and business team members throughout code, conversations, and documentation. The foundation of effective BDD collaboration.

## Core Concept

Ubiquitous Language is:
- **Shared** - Same terms used by everyone (dev, QA, product, stakeholders)
- **Consistent** - One concept = one term
- **Domain-focused** - Business concepts, not technical implementation
- **Living** - Evolves with domain understanding
- **Everywhere** - In code, scenarios, conversations, docs

## Why It Matters

### Without Ubiquitous Language

**Product Manager:** "When a customer's subscription lapses, we need to win them back."

**Developer:** "So you want to reactivate a churned user account?"

**QA:** "Are we testing the expired premium member re-enrollment flow?"

Same concept, three different terms. Confusion follows.

### With Ubiquitous Language

**Everyone:** "When a subscriber churns, we offer a winback promotion."

Terms defined:
- **Subscriber** - Customer with active subscription
- **Churn** - Cancel subscription
- **Winback** - Re-activate churned subscriber

## Building Ubiquitous Language

### 1. Discovery Through Conversation

Ask questions during Three Amigos sessions:

**What do you call this?**
```
PM: "When someone stops paying..."
Dev: "What's the term for that? Cancelled? Inactive?"
PM: "We call them 'churned subscribers'"
✅ Term added to glossary: Churned Subscriber
```

**What's the difference between X and Y?**
```
QA: "What's the difference between 'guest' and 'user'?"
PM: "Guest hasn't registered. User has an account."
✅ Two distinct terms: Guest, User
```

**When does this state change?**
```
Dev: "When exactly does an order become 'completed'?"
PM: "When payment clears and items ship."
✅ Term defined: Completed = Paid + Shipped
```

### 2. Document in Scenarios

```gherkin
Feature: Subscription Management

  # Define domain terms upfront
  Background:
    """
    Domain Terms:
    - Subscriber: Customer with active subscription
    - Churned Subscriber: Cancelled within last 30 days
    - Lapsed Subscriber: Cancelled over 30 days ago
    - Winback Offer: Discount to re-activate churned subscriber
    """

  Scenario: Winback offer for churned subscriber
    Given a churned subscriber from 15 days ago
    When they visit the pricing page
    Then they should see the winback offer

  Scenario: No winback for lapsed subscriber
    Given a lapsed subscriber from 60 days ago
    When they visit the pricing page
    Then they should see standard pricing
    And they should not see the winback offer
```

### 3. Use Domain Terms Consistently

❌ **Inconsistent Terms:**
```gherkin
Feature: User Account Management

  Scenario: User registers
    When a customer signs up
    Then a member account is created
    And the client receives confirmation email
```

Four terms (user, customer, member, client) for one concept!

✅ **Consistent Terms:**
```gherkin
Feature: Member Account Management

  Scenario: Member registers
    When a customer creates an account
    Then a member account is created
    And the member receives confirmation email
```

Distinction:
- **Customer** - Anyone (browsing, not logged in)
- **Member** - Registered account

### 4. Maintain a Glossary

```markdown
# Domain Glossary

## Order Lifecycle
- **Cart** - Temporary collection of items (not yet ordered)
- **Order** - Finalized purchase (payment may be pending)
- **Pending Order** - Order created, awaiting payment
- **Confirmed Order** - Payment received, awaiting fulfillment
- **Shipped Order** - Items dispatched to customer
- **Completed Order** - Items delivered successfully
- **Cancelled Order** - Order terminated before shipping
- **Returned Order** - Items sent back after delivery

## Membership
- **Guest** - Browsing without account
- **Customer** - Anyone (guest or member)
- **Member** - Registered account (free tier)
- **Subscriber** - Paid subscription
- **Premium Member** - Highest paid tier

## Inventory
- **In Stock** - Available for immediate purchase
- **Low Stock** - Less than 10 units remaining
- **Out of Stock** - Zero units, cannot purchase
- **Backordered** - Out of stock, but can pre-order
- **Discontinued** - No longer sold
```

## Ubiquitous Language in Code

### Class Names

```typescript
// GOOD - Uses domain language
class Subscriber {
  churnDate?: Date;
  
  isChurned(): boolean {
    return this.churnDate !== undefined;
  }
  
  isEligibleForWinback(): boolean {
    if (!this.isChurned()) return false;
    const daysSinceChurn = daysBetween(this.churnDate, now());
    return daysSinceChurn <= 30;
  }
}

// AVOID - Technical jargon
class UserAccount {
  deactivationTimestamp?: number;
  
  isDeactivated(): boolean {
    return this.deactivationTimestamp !== undefined;
  }
}
```

### Method Names

```typescript
// GOOD - Domain verbs
order.confirm();
order.ship();
order.complete();
subscription.churn();
subscriber.winback();

// AVOID - Generic CRUD
order.update({ status: 'confirmed' });
subscription.delete();
user.create();
```

### Enums and Constants

```typescript
// GOOD - Domain states
enum OrderStatus {
  Pending = 'PENDING',
  Confirmed = 'CONFIRMED',
  Shipped = 'SHIPPED',
  Completed = 'COMPLETED',
  Cancelled = 'CANCELLED',
}

// AVOID - Technical states
enum OrderStatus {
  State1 = 1,
  State2 = 2,
  State3 = 3,
}
```

## Ubiquitous Language in Scenarios

### High-Level Business Terms

```gherkin
# GOOD - Business domain language
Feature: Order Management

  Scenario: Checkout with insufficient inventory
    Given product "Laptop" is out of stock
    When customer tries to purchase "Laptop"
    Then they should see "Out of stock"
    And order should not be created

# AVOID - Technical implementation
Feature: Database Constraint Handling

  Scenario: Handle zero inventory constraint
    Given inventory.stock_count = 0 for product_id = 123
    When POST /api/orders with product_id = 123
    Then response should be 409 Conflict
    And orders table should have no new records
```

### Consistent Terminology

```gherkin
Feature: Subscription Billing

  Scenario: Monthly subscription renewal
    Given a subscriber with monthly billing cycle
    When their subscription renews on billing date
    Then they should be charged subscription fee
    And subscription expiry should extend 30 days

  # Use "subscriber" consistently, not:
  # - "user"
  # - "customer"  
  # - "member"
  # - "account"
```

## Common Pitfalls

### 1. Technical Terms in Business Scenarios

❌ **Don't expose implementation:**
```gherkin
When I POST to /api/auth with credentials
Then JWT token is stored in localStorage
And user_id foreign key is inserted
```

✅ **Use business language:**
```gherkin
When I log in with valid credentials
Then I should be authenticated
And I should access my dashboard
```

### 2. Ambiguous Terms

❌ **Vague:**
```gherkin
Given the item is inactive
When I process the thing
Then the status should update
```

✅ **Specific:**
```gherkin
Given the product is discontinued
When I attempt to purchase the product
Then I should see "Product no longer available"
```

### 3. Synonyms for Same Concept

❌ **Inconsistent:**
```gherkin
Scenario: Register new account
  When I sign up
  Then I create a profile
  And I complete onboarding

# "register", "sign up", "create profile", "onboarding" - which is it?
```

✅ **Consistent:**
```gherkin
Scenario: Register new member
  When I complete registration
  Then my member account should be created
  And I should receive welcome email
```

### 4. One Term for Multiple Concepts

❌ **Overloaded term "User":**
```gherkin
Given a user is logged in      # Authenticated member?
And a user visits the page     # Anonymous visitor?
And a user account exists      # Member entity?
```

✅ **Distinct terms:**
```gherkin
Given a member is logged in
And a guest visits the page
And a member account exists with email "test@example.com"
```

## Evolving Ubiquitous Language

Language evolves as domain understanding deepens:

**Initial:**
```gherkin
Scenario: Deactivate user
  When admin deactivates user
  Then user cannot login
```

**After discussion:**
```
Team realizes two types of deactivation:
- Temporary suspension (can be reversed)
- Permanent deletion (cannot be reversed)
```

**Evolved:**
```gherkin
Scenario: Suspend member account
  When admin suspends member
  Then member cannot login
  But member data is retained

Scenario: Permanently delete member account
  When admin deletes member
  Then member cannot login
  And member data is anonymized
```

**Update glossary:**
```markdown
- ~~Deactivate~~ (deprecated - ambiguous)
- **Suspend** - Temporarily disable login (reversible)
- **Delete** - Permanently remove member (irreversible)
```

## Benefits

### Reduced Miscommunication
Everyone speaks the same language. No translation layer between business and technical.

### Better Code Quality
Code mirrors business domain, making it easier to understand and maintain.

### Effective Collaboration
Product, dev, and QA collaborate using shared terminology.

### Living Documentation
Scenarios document business rules using terms everyone understands.

### Faster Onboarding
New team members learn domain vocabulary from scenarios and code.

## Building Ubiquitous Language - Workshop Exercise

**Step 1: Brainstorm Terms**
```
What do we call someone who:
- Visits the site? → Visitor? Guest?
- Creates account? → User? Member? Customer?
- Pays subscription? → Subscriber? Premium Member?
- Cancels subscription? → Churned? Inactive? Lapsed?
```

**Step 2: Define Distinctions**
```
Guest: Not logged in, browsing
Member: Free account, logged in
Subscriber: Paid monthly subscription
Churned Subscriber: Cancelled within 30 days
Lapsed Member: Cancelled over 30 days ago
```

**Step 3: Document in Glossary**
```markdown
# Terms

- **Guest** - Visitor without account
- **Member** - Registered free account
- **Subscriber** - Paid subscription
- **Churned** - Cancelled subscription (last 30 days)
- **Lapsed** - Cancelled subscription (30+ days ago)
```

**Step 4: Use in Scenarios**
```gherkin
Scenario: Guest browses products
  Given I am a guest
  When I view products
  Then I see public pricing

Scenario: Subscriber sees discounted pricing
  Given I am a subscriber
  When I view products
  Then I see subscriber pricing
```

**Step 5: Implement in Code**
```typescript
class Guest extends Visitor {
  viewProducts(): Product[] {
    return this.catalog.getPublicProducts();
  }
}

class Subscriber extends Member {
  viewProducts(): Product[] {
    return this.catalog.getSubscriberProducts();
  }
}
```

## Related References

- @see principles-core-philosophy.md - BDD communication focus
- @see collaboration-building-ubiquitous-language.md - Team exercises
- @see principles-living-documentation.md - Document domain terms
- @see patterns-domain-language.md - Using domain terms in scenarios
