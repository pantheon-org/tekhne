---
title: Writing Effective Acceptance Criteria
description:
  Skill for creating clear, testable acceptance criteria that define feature completion and enable effective development
type: skill
category: Development
version: 1.0.0
tags:
  - requirements
  - testing
  - agile
  - user-stories
  - quality-assurance
last_updated: 2026-01-26
---

# Skill: Writing Effective Acceptance Criteria

This skill guides you through creating clear, testable acceptance criteria that align teams, enable accurate estimation,
and ensure successful feature delivery.

## When to Use This Skill

Use this skill when you need to:

- Define what "done" means for a user story
- Create testable requirements for QA
- Align stakeholder expectations
- Enable accurate effort estimation
- Document edge cases and error scenarios
- Prepare for sprint planning

## Prerequisites

- User story defined (As a [role], I want [action], so that [benefit])
- Understanding of the feature's purpose and value
- Knowledge of the target users
- Awareness of system constraints and dependencies

## Understanding Acceptance Criteria

**Acceptance Criteria (AC)** are conditions that a feature must meet to be accepted by users, customers, or other
systems.

**Purpose:**

- Define feature boundaries and scope
- Describe both positive and negative scenarios
- Synchronize team and stakeholder visions
- Enable acceptance testing
- Support accurate estimation

**Key Distinction:**

- **User Story** = WHAT and WHY (high-level narrative)
- **Acceptance Criteria** = HOW WE KNOW IT'S DONE (specific conditions)

## Step-by-Step Guide

### Step 1: Choose the Right Format

Select a format based on your needs:

#### Decision Tree for Format Selection

**Ask these questions:**

1. **Is this a user interaction with a clear sequence?**
   - YES → Use Given/When/Then (Scenario format)
   - NO → Continue

2. **Will this be used for automated testing?**
   - YES → Use Given/When/Then (Scenario format)
   - NO → Continue

3. **Is this UI/UX focused with design constraints?**
   - YES → Use Rule-Oriented (Checklist format)
   - NO → Continue

4. **Are there multiple independent rules?**
   - YES → Use Rule-Oriented (Checklist format)
   - NO → Continue

5. **Does a custom format better fit your needs?**
   - YES → Use Custom format (but keep it clear and testable)

#### Format Comparison

| Format              | Best For                                    | Example Use Case                      |
| ------------------- | ------------------------------------------- | ------------------------------------- |
| **Given/When/Then** | User interactions, API flows, sequences     | Login flow, checkout process          |
| **Rule-Oriented**   | UI/UX requirements, validation rules, lists | Search interface, form validation     |
| **Custom**          | Unique requirements, complex constraints    | Password strength, special conditions |

### Step 2: Write Given/When/Then Scenarios (BDD Format)

Use this format for sequential user interactions and testable scenarios.

#### Structure

```gherkin
Scenario: [Name for the behavior]

Given [the beginning state]
When [specific user action]
Then [expected outcome]
And [additional condition]
```

#### Step-by-Step Process

**2A. Define the Scenario Name**

- Use descriptive, action-oriented names
- Focus on user perspective
- Be specific about the outcome

**Good:**

```
Scenario: Successful user login with valid credentials
Scenario: Password recovery via email
Scenario: Insufficient funds during ATM withdrawal
```

**Poor:**

```
Scenario: Login
Scenario: Test 1
Scenario: Something happens
```

**2B. Set Up the Context (Given)**

- Describe the starting state
- Include all necessary preconditions
- Use multiple "And" statements for multiple conditions

**Example:**

```gherkin
Given: The user is on the login page
And: The user has a valid registered account
And: The user's account is not locked
```

**2C. Define the Action (When)**

- Describe the specific user action
- Be explicit about what the user does
- Include any input data

**Example:**

```gherkin
When: The user enters their email "user@example.com"
And: The user enters their password
And: The user clicks the "Log In" button
```

**2D. Specify the Expected Outcome (Then)**

- Describe what should happen
- Be specific and measurable
- Cover all relevant changes

**Example:**

```gherkin
Then: The user is redirected to the dashboard
And: A welcome message displays: "Welcome back, [Name]"
And: The session remains active for 24 hours
```

#### Complete Example: Password Recovery

```gherkin
Scenario: User requests password reset

Given: The user is on the login page
When: The user clicks "Forgot password" link
And: The user enters their registered email address
And: The user clicks "Send reset link"
Then: The system sends a password reset email to the address
And: A confirmation message displays: "Check your email for reset instructions"
And: The reset link expires after 1 hour

Scenario: User completes password reset

Given: The user received a password reset email
And: The reset link has not expired
When: The user clicks the reset link in the email
And: The user enters a new valid password
And: The user confirms the new password
And: The user clicks "Reset password"
Then: The password is updated in the system
And: The user is redirected to the login page
And: A success message displays: "Password reset successful"
And: Previous reset links become invalid
```

### Step 3: Write Rule-Oriented Criteria (Checklist Format)

Use this format for UI/UX requirements, validation rules, and independent conditions.

#### Structure

```markdown
- Rule 1
- Rule 2
- Rule 3
```

#### Step-by-Step Process

**3A. List All Requirements**

- Write each requirement as a standalone bullet point
- Use complete sentences
- Be specific and measurable

**3B. Cover All Categories**

For UI/UX features, include:

- Layout and positioning
- Interactions and behavior
- Validation rules
- Error handling
- Responsive behavior
- Accessibility requirements

**3C. Make Each Item Testable**

- Include specific values, formats, or behaviors
- Define success and failure conditions
- Avoid vague terms like "good," "fast," "user-friendly"

#### Complete Example: Search Interface

```markdown
User Story: As a traveler, I want to search hotels by multiple criteria so that I can find options that match my needs.

Acceptance Criteria:

Layout:

- Search field is positioned in the top navigation bar
- Search field width is 400px on desktop, full-width on mobile
- Search button is positioned immediately to the right of the search field

Input Behavior:

- Field contains placeholder text: "Where are you going?"
- Placeholder disappears when user starts typing
- User can search by city name, hotel name, street name, or combination
- User cannot enter more than 200 characters
- Search supports English, French, German, and Ukrainian languages

Search Execution:

- Search triggers when user clicks "Search" button
- Search triggers when user presses Enter key
- Search displays loading indicator while fetching results
- Results display within 2 seconds for 95% of queries

Validation:

- System prevents searching with empty field
- System displays error "Please enter a search term" for empty searches
- System rejects special characters (!@#$%^&\*)
- System displays error "Search cannot contain special characters" if special characters entered

Results:

- Results display in grid format (3 columns desktop, 1 column mobile)
- Each result shows: hotel name, location, price, rating
- Results are sorted by relevance score
- System displays "No results found" message if no matches
- System displays up to 20 results per page
```

### Step 4: Address the Four Essential Qualities

Every acceptance criterion must possess these qualities:

#### Quality 1: Clarity

**Checklist:**

- [ ] All team members can understand it
- [ ] No ambiguous terms or jargon
- [ ] Specific actions and outcomes defined

**Example Transformation:**

```
❌ Bad: "The system should handle errors"
✅ Good: "When a user enters an invalid email format, the system displays
         the error message 'Please enter a valid email address' below the
         email field"
```

#### Quality 2: Conciseness

**Checklist:**

- [ ] Communicates necessary information
- [ ] No unnecessary background or context
- [ ] Gets to the point quickly

**Example Transformation:**

```
❌ Bad: "The search functionality, which is one of the core features of
         our application and is used by millions of users every day,
         should return results that match what the user is looking for..."

✅ Good: "Search returns results matching user-entered keywords within
         2 seconds"
```

#### Quality 3: Testability

**Checklist:**

- [ ] Clear pass/fail criteria
- [ ] Measurable outcomes
- [ ] Can be verified through testing

**Example Transformation:**

```
❌ Bad: "The page should load quickly"
✅ Good: "The page loads in under 2 seconds for 95% of requests"
```

#### Quality 4: Result-Oriented

**Checklist:**

- [ ] Describes the outcome, not the process
- [ ] Focuses on value delivered
- [ ] Avoids implementation details

**Example Transformation:**

```
❌ Bad: "Use React hooks to manage state"
✅ Good: "User session state persists across page refreshes"
```

### Step 5: Cover Both Positive and Negative Scenarios

Don't just cover the happy path—include error cases and edge scenarios.

#### Scenario Coverage Checklist

**For every feature, define:**

- [ ] Happy path (everything works correctly)
- [ ] Invalid input scenarios
- [ ] Missing required data
- [ ] Boundary conditions
- [ ] Error handling and recovery
- [ ] Timeout or network failure cases
- [ ] Permission/authorization failures

#### Example: Login Feature

```gherkin
Scenario: Successful login (Happy Path)
Given: User has valid credentials
When: User logs in
Then: User accesses dashboard

Scenario: Invalid password (Error Case)
Given: User enters wrong password
When: User clicks Log In
Then: Error message displays: "Incorrect email or password"
And: User remains on login page
And: Email field retains entered value
And: Password field is cleared

Scenario: Locked account (Edge Case)
Given: User account is locked after 5 failed attempts
When: User tries to log in
Then: Error displays: "Account locked. Contact support or try again in 30 minutes"
And: Login attempt is logged

Scenario: Network failure (Error Case)
Given: Network connection is lost
When: User submits login form
Then: Error displays: "Connection lost. Please check your internet and try again"
And: User can retry when connection restored
```

### Step 6: Integrate with MoSCoW Prioritization

Use MoSCoW to prioritize your acceptance criteria within a user story.

#### Process

**6A. Categorize Conditions:**

- **Must Have** → Core acceptance criteria (feature fails without these)
- **Should Have** → Important but can be deferred
- **Could Have** → Enhancements, nice-to-haves
- **Won't Have** → Explicitly out of scope for this iteration

**6B. Focus on Must Haves:**

- These become your required acceptance criteria
- Feature cannot be accepted without these
- Should be ~60% of total effort

**6C. Document Should/Could Haves:**

- Track for future iterations
- Provide context for why deferred
- May become Must Haves in later sprints

#### Example: ATM Withdrawal

```markdown
User Story: As a bank customer, I want to withdraw cash from an ATM so that I can access my money quickly.

## Conditions of Satisfaction (MoSCoW)

### Must Have (Core AC)

- Verify account has sufficient funds before dispensing
- Dispense exact amount requested
- Debit account accurately and immediately
- Return card to user after transaction
- Handle network failures gracefully

### Should Have

- Allow selection from preset amounts ($20, $40, $60, $100)
- Print receipt
- Display remaining balance after withdrawal

### Could Have

- Remember user's preferred withdrawal amounts
- Offer to check balance before withdrawal
- Support multiple currency denominations

### Won't Have (This Iteration)

- Cryptocurrency withdrawal
- Split withdrawal across multiple accounts
- Deposit functionality
```

**Resulting Core Acceptance Criteria (from Must Haves):**

```gherkin
Scenario: Successful withdrawal with sufficient funds
Given: Account balance is $500
And: ATM has sufficient cash
And: Card is valid
When: User requests $100 cash
Then: Account is debited $100
And: Cash is dispensed
And: Card is returned
And: Transaction completes within 10 seconds

Scenario: Insufficient funds
Given: Account balance is $50
And: Card is valid
When: User requests $100 cash
Then: Error displays: "Insufficient funds. Available balance: $50"
And: No cash is dispensed
And: Account is not debited
And: Card is returned
```

### Step 7: Review and Refine

Before finalizing, review against this comprehensive checklist:

#### Quality Checklist

**Clarity & Understanding:**

- [ ] No ambiguous terms ("fast," "good," "user-friendly")
- [ ] All team members understand the criteria
- [ ] Stakeholders agree with the criteria

**Completeness:**

- [ ] Happy path covered
- [ ] Error scenarios covered
- [ ] Edge cases covered
- [ ] All user roles/permissions addressed

**Testability:**

- [ ] Each criterion is independently testable
- [ ] Clear pass/fail scenarios
- [ ] Measurable outcomes defined
- [ ] Can be automated (if applicable)

**Result-Oriented:**

- [ ] Focuses on WHAT not HOW
- [ ] No implementation details
- [ ] Describes outcomes, not technical solutions

**Language & Format:**

- [ ] Written in active voice
- [ ] Uses simple, concise sentences
- [ ] First-person perspective
- [ ] Follows chosen format consistently

**Business Value:**

- [ ] Aligns with user story
- [ ] Delivers on the stated benefit
- [ ] Achievable within sprint timeframe

## Common Patterns by Feature Type

### Pattern 1: Form Validation

```markdown
Field Validation:

- Email field validates format: user@domain.com
- Email displays error "Invalid email format" when validation fails
- Password requires minimum 8 characters
- Password requires at least 1 number and 1 special character
- Password displays strength indicator (weak/medium/strong) as user types
- Phone accepts formats: (555) 555-5555, 555-555-5555, 5555555555

Error Display:

- Error messages appear below their respective fields
- Error messages display in red text (#d32f2f)
- Error messages remain until user corrects input
- All errors clear when form is successfully submitted

Submit Behavior:

- Submit button is disabled when any validation errors exist
- Submit button is enabled when all fields are valid
- Loading spinner displays during submission
- Form clears after successful submission
- Success message displays after submission
```

### Pattern 2: API Integration

```markdown
Endpoint Configuration:

- POST request to /api/v1/users endpoint
- Request includes Authorization: Bearer token in header
- Request body format: { "email": "string", "name": "string", "role": "string" }
- Content-Type header is application/json

Success Response:

- Status code: 201 Created
- Response body: { "id": "uuid", "email": "string", "name": "string", "role": "string", "createdAt": "ISO8601" }
- Response time: under 500ms for 95% of requests

Error Responses:

- 400 Bad Request: Invalid input data with specific error messages
- 401 Unauthorized: Invalid or missing token, redirects to login
- 403 Forbidden: Insufficient permissions, displays permission error
- 409 Conflict: Email already exists, displays duplicate error
- 500 Server Error: Displays "Service temporarily unavailable" message
- Network timeout: 30 seconds, displays retry option
```

### Pattern 3: UI/UX Requirements

```markdown
Layout:

- Navigation bar fixed at top of page
- Logo positioned in top-left corner
- User menu in top-right corner
- Main content has maximum width of 1200px
- Main content is centered on screen

Interactions:

- Buttons change color on hover (#primary → #primary-dark)
- Buttons display loading spinner during async operations
- Buttons are disabled during loading state
- Modal overlay closes when clicking outside modal
- Modal closes when pressing Escape key
- Form inputs show focus state with blue border (#2196f3)

Responsive Behavior:

- On desktop (≥1024px): 3-column layout
- On tablet (768px-1023px): 2-column layout
- On mobile (<768px): single-column layout
- Navigation collapses to hamburger menu on mobile
- Images scale proportionally to container width
- Font size scales: desktop 16px, tablet 14px, mobile 12px
```

## Troubleshooting

### Issue: Acceptance criteria are too vague

**Symptoms:** Terms like "fast," "good," "user-friendly," "efficient"

**Solution:**

1. Ask: "How will we measure this?"
2. Define specific metrics or thresholds
3. Add concrete examples

**Example Fix:**

```
❌ "The system should be fast"
✅ "The system responds to user actions within 200ms"
✅ "Search results display within 2 seconds"
```

### Issue: Mixing technical implementation with business requirements

**Symptoms:** Mentions of technologies, frameworks, libraries, data structures

**Solution:**

1. Focus on user-visible behavior
2. Describe the outcome, not the implementation
3. Move technical details to technical design docs

**Example Fix:**

```
❌ "Use Redux with local storage persistence and middleware"
✅ "User session state persists across page refreshes"
✅ "User preferences are remembered between visits"
```

### Issue: Criteria are too narrow or too broad

**Symptoms:**

- Too narrow: Overly specific UI details, exact pixel measurements
- Too broad: "Provide good UX," "Make it intuitive"

**Solution:**

1. Find the middle ground: specific enough to test, flexible enough to implement
2. Use design guidelines instead of exact specifications
3. Focus on user outcomes

**Example Fix:**

```
❌ Too narrow: "Button is 120px wide, 40px tall, #0066CC background,
               12px padding, 4px border-radius"

✅ Just right: "Submit button follows brand design guidelines and is
               easily clickable"

❌ Too broad: "System provides good user experience"

✅ Just right: "User can complete checkout in 3 steps or fewer"
```

### Issue: Not covering error scenarios

**Symptoms:** Only happy path is defined

**Solution:**

1. Ask: "What could go wrong?"
2. Define error messages and recovery paths
3. Cover invalid input, missing data, network failures

**Example Addition:**

```
Add these scenarios:
- What happens with invalid input?
- What happens when required data is missing?
- What happens during network failure?
- What happens with insufficient permissions?
- What happens at boundary conditions?
```

### Issue: Cannot decide between Given/When/Then and checklist

**Solution:**

- Use Given/When/Then for sequential flows with clear steps
- Use checklist for independent rules or UI requirements
- Mix both formats if needed for different aspects of the same story

## Quick Reference

### Writing Checklist

**Before writing:**

- [ ] User story is clear and understood
- [ ] Stakeholders are available for questions
- [ ] Format selected (Given/When/Then or checklist)

**While writing:**

- [ ] Each criterion is independently testable
- [ ] Both positive and negative scenarios covered
- [ ] Clear pass/fail criteria defined
- [ ] Measurable outcomes specified
- [ ] Written in active voice, first person
- [ ] No implementation details included
- [ ] Simple, concise language used

**After writing:**

- [ ] Reviewed with development team
- [ ] Validated with QA team
- [ ] Confirmed with product owner
- [ ] Stakeholder sign-off obtained
- [ ] Added to user story in project management tool

### Format Quick Reference

**Given/When/Then:**

```gherkin
Scenario: [Descriptive name]
Given: [Starting state]
And: [Additional precondition]
When: [User action]
And: [Additional action]
Then: [Expected outcome]
And: [Additional outcome]
```

**Rule-Oriented:**

```markdown
- Specific requirement 1
- Specific requirement 2
- Error handling: specific behavior
- Edge case: specific behavior
```

### Common Measurable Metrics

- **Time:** "within 2 seconds," "in under 500ms"
- **Quantity:** "displays 10 results per page," "maximum 5 error messages"
- **Quality:** "95% success rate," "supports 10,000 concurrent users"
- **Format:** "YYYY-MM-DD format," "user@domain.com pattern"
- **Behavior:** "button changes from blue to gray," "modal closes"

## Related Resources

- [Behavior-Driven Development (BDD)](https://en.wikipedia.org/wiki/Behavior-driven_development)
- [Gherkin Language Specification](https://cucumber.io/docs/gherkin/)
- [Agile Alliance: Acceptance Criteria](https://www.agilealliance.org/glossary/acceptance-criteria/)
