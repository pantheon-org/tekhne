---
category: principles
priority: HIGH
source: bdd-principles, bdd-collaboration
---

# Example Mapping

A structured workshop technique using colored cards to explore features through rules and examples. Helps teams quickly identify what they know, what they don't know, and what needs clarification.

## The Four Colors

| Color | Represents | Purpose |
|-------|------------|---------|
| **Yellow** | User Story / Feature | The capability being built |
| **Blue** | Rules | Business rules and acceptance criteria |
| **Green** | Examples | Concrete scenarios illustrating rules |
| **Red** | Questions | Uncertainties that need answering |

## Workshop Format

**Time-boxed:** 25-30 minutes maximum
**Participants:** Business, Development, Testing (Three Amigos)
**Materials:** Colored index cards or sticky notes

### Step-by-Step Process

**1. Start with the Story (Yellow Card)**
```
┌─────────────────────────────────────┐
│ As a customer                       │
│ I want to apply discount codes      │
│ So that I can save money on orders  │
└─────────────────────────────────────┘
```

**2. Identify Rules (Blue Cards)**
```
┌─────────────────────────────────┐
│ Discount codes are case-        │
│ insensitive                     │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ One discount code per order     │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ Expired codes cannot be applied │
└─────────────────────────────────┘
```

**3. Provide Examples (Green Cards)**

For each rule, add concrete examples:

```
Blue Rule: "Discount codes are case-insensitive"
  └─ Green: "SAVE20" and "save20" both work
  └─ Green: "SaVe20" also works

Blue Rule: "One discount code per order"
  └─ Green: Apply "SAVE20" → Works
  └─ Green: Then try "FREESHIP" → Rejected

Blue Rule: "Expired codes cannot be applied"
  └─ Green: Code expired yesterday → Rejected
  └─ Green: Code expires tomorrow → Works
```

**4. Capture Questions (Red Cards)**
```
┌─────────────────────────────────┐
│ ? Can codes be combined with    │
│   promotional pricing?          │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ ? What happens if item already  │
│   discounted by 50%?            │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ ? Who can create discount codes?│
└─────────────────────────────────┘
```

## Example Mapping Session

### Story: User Registration

**Yellow Card (Story):**
```
As a new visitor
I want to create an account
So that I can make purchases
```

**Blue Cards (Rules):**
1. Email must be unique
2. Password must be strong (8+ chars, number, special char)
3. Age must be 18+
4. Email verification required

**Green Cards (Examples):**

Rule 1: Email must be unique
- ✅ Register with "new@example.com" → Success
- ❌ Register with existing "taken@example.com" → Error: "Email already registered"

Rule 2: Password must be strong
- ❌ Password "abc" → Error: "Too short"
- ❌ Password "abcdefgh" → Error: "Needs number"
- ❌ Password "abcdefgh1" → Error: "Needs special character"
- ✅ Password "abcdefgh1!" → Success

Rule 3: Age must be 18+
- ❌ Age 17 → Error: "Must be 18 or older"
- ✅ Age 18 → Success
- ✅ Age 65 → Success

Rule 4: Email verification required
- ✅ Register → Email sent → Click link → Verified
- ❌ Login before verifying → Error: "Please verify email"

**Red Cards (Questions):**
- ❓ Can users change email after registration?
- ❓ How long is verification link valid?
- ❓ What if email bounces?
- ❓ Do we need parent consent for users under 18?

## Visual Layout

```
┌────────────────────────────────────────────────────────┐
│  YELLOW: User Registration Story                       │
├────────────────────────────────────────────────────────┤
│                                                         │
│  BLUE: Email unique     BLUE: Password strong          │
│    GREEN: new@test.com    GREEN: "abc" fails           │
│    GREEN: existing fails  GREEN: "abc123!" works       │
│                                                         │
│  BLUE: Age 18+          BLUE: Email verification       │
│    GREEN: 17 rejected     GREEN: Click link works     │
│    GREEN: 18 accepted     GREEN: No click = blocked   │
│                                                         │
│  RED: Email change?     RED: Link expiry?              │
│  RED: Bounced emails?   RED: Parent consent?           │
└────────────────────────────────────────────────────────┘
```

## When to Stop

Stop the session if:

### Too Many Red Cards (Uncertainty)
```
RED RED RED RED RED RED
```
**Signal:** Not ready to develop. Need more discovery.
**Action:** Schedule follow-up with stakeholders.

### Too Many Blue Cards (Complexity)
```
BLUE BLUE BLUE BLUE BLUE BLUE BLUE
```
**Signal:** Story too large.
**Action:** Split into multiple stories.

### Just Right (Ready for Development)
```
YELLOW
  BLUE BLUE BLUE
    GREEN GREEN GREEN GREEN GREEN
      RED RED
```
**Signal:** Clear rules, good examples, few questions.
**Action:** Proceed to development.

## From Example Mapping to Gherkin

After the session, convert cards to Gherkin scenarios:

**Blue Rule + Green Examples → Gherkin Scenario**

```
Blue: "Password must be strong"
Green: "abc" → Error "Too short"
Green: "abcdefgh1!" → Success

Becomes:

Scenario Outline: Password strength validation
  When I register with password "<password>"
  Then I should see "<result>"

  Examples:
    | password    | result                      |
    | abc         | Too short                   |
    | abcdefgh    | Needs number                |
    | abcdefgh1   | Needs special character     |
    | abcdefgh1!  | Registration successful     |
```

**Red Questions → Follow-up Tasks**
```
RED: "How long is verification link valid?"

Actions:
- [ ] Ask product owner
- [ ] Document decision
- [ ] Add scenario when answered
```

## Example Mapping Anti-Patterns

### ❌ Writing Code in Examples
```
GREEN: When POST /api/register with {email, password}
       Then return 201 with JWT token
```

Instead:
```
GREEN: Register with valid email → Account created
```

### ❌ Too Many Rules for One Story
```
YELLOW: Complete checkout flow
  BLUE: Validate cart
  BLUE: Process payment
  BLUE: Update inventory
  BLUE: Send confirmation
  BLUE: Calculate tax
  BLUE: Apply shipping
  BLUE: Validate address
  BLUE: Check fraud rules
```

**Too complex!** Split into multiple stories.

### ❌ Abstract Examples
```
GREEN: Valid input → Success
GREEN: Invalid input → Error
```

Instead:
```
GREEN: Register with "alice@example.com" → Success
GREEN: Register with "not-an-email" → Error: "Invalid email format"
```

### ❌ Ignoring Red Cards
```
RED: What about international addresses?
RED: How do we handle gift cards?
RED: Can businesses register?

[Session ends, questions never answered]
```

**Don't ignore questions!** Assign owners and follow up.

## Example Mapping Variations

### Remote Teams
Use virtual whiteboard tools:
- Miro
- Mural
- FigJam
- Google Jamboard

Digital colored cards work the same way.

### Large Features
Break into multiple sessions:
```
Session 1: Story A - Core registration
Session 2: Story B - Email verification
Session 3: Story C - Social login
```

### Spike Investigations
When too many red cards:
```
┌─────────────────────────┐
│ SPIKE: Research         │
│ International address   │
│ validation requirements │
└─────────────────────────┘

Questions to answer:
- RED: Which countries?
- RED: Postal code formats?
- RED: Address validation API?
```

Run spike, then reconvene for Example Mapping.

## Benefits

### Fast Feedback
25-30 minutes reveals if story is:
- **Ready** (few red cards, clear examples)
- **Needs refinement** (some red cards, follow up)
- **Blocked** (many red cards, stop and investigate)

### Shared Understanding
All three perspectives (business, dev, test) align on:
- What we're building
- Why we're building it
- How we'll know it works

### Right-Sized Stories
Too many blue cards → Story too big → Split it
Too few rules → Story too small → Combine with others

### Uncovers Assumptions
Red cards surface:
- Missing requirements
- Edge cases no one considered
- Technical constraints
- Integration questions

## Example Mapping → BDD Workflow

```
1. Example Mapping Session (30 min)
   ↓
2. Review and Prioritize Examples
   ↓
3. Write Gherkin Scenarios (from green cards)
   ↓
4. Implement Step Definitions
   ↓
5. Develop Feature (TDD)
   ↓
6. All Scenarios Pass → Done
```

## Complete Example

**Story:** Password Reset

**Session Output:**

```
YELLOW: Password Reset
  
  BLUE: Reset link sent to registered email
    GREEN: Request reset for "user@example.com" → Email sent
    GREEN: Request reset for "unknown@example.com" → Email sent (security: don't reveal if email exists)
  
  BLUE: Reset link expires after 1 hour
    GREEN: Click link after 30 minutes → Works
    GREEN: Click link after 90 minutes → Expired error
  
  BLUE: Multiple requests invalidate previous links
    GREEN: Request reset twice → First link no longer works
  
  RED: Rate limiting for reset requests?
  RED: What if email bounces?
  RED: Can user reset password while logged in?
```

**Resulting Gherkin:**

```gherkin
Feature: Password Reset

  Rule: Reset link sent to registered email
    Scenario: Request reset with registered email
      Given user "alice@example.com" exists
      When I request password reset for "alice@example.com"
      Then I should receive reset email
      And I should see "If email exists, reset link sent"

    Scenario: Request reset with unknown email
      When I request password reset for "unknown@example.com"
      Then I should see "If email exists, reset link sent"
      But no email should be sent

  Rule: Reset link expires after 1 hour
    Scenario: Use reset link within expiry window
      Given I requested reset 30 minutes ago
      When I click the reset link
      Then I should be able to set new password

    Scenario: Use reset link after expiry
      Given I requested reset 90 minutes ago
      When I click the reset link
      Then I should see "Reset link expired"

  Rule: Multiple requests invalidate previous links
    Scenario: Request reset twice
      Given I requested reset and received link
      When I request another reset
      Then the first link should no longer work
      And the second link should work
```

**Follow-up Tasks:**
```
[ ] Product owner: Define rate limiting rules
[ ] Tech lead: Investigate email bounce handling
[ ] UX: Design in-app password reset flow
```

## Related References

- @see principles-three-amigos.md - The three perspectives
- @see collaboration-example-mapping-workshop.md - Detailed workshop facilitation
- @see principles-specification-by-example.md - Using examples to drive development
- @see collaboration-discovery-workshop.md - Alternative discovery format
