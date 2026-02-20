# The Three Amigos Practice

A practice where three perspectives collaborate to explore and define features.

## The Three Perspectives

### 1. Business Perspective (Product Owner/BA)

- What problem are we solving?
- What value does it provide?
- What are the business rules?

### 2. Development Perspective (Developer)

- How might we build this?
- What are the technical constraints?
- What are the edge cases?

### 3. Testing Perspective (Tester/QA)

- What could go wrong?
- What are we missing?
- How will we verify this works?

## Example Three Amigos Session

**Feature**: Password Reset

**Business**: "Users who forget their password need a way to reset it via email."

**Developer**: "We'll need to generate secure tokens with expiration. How long should tokens be valid?"

**Tester**: "What happens if they request multiple reset emails? Can old tokens still be used?"

**Business**: "Tokens should be valid for 1 hour. Multiple requests should invalidate old tokens."

**Developer**: "Should we rate-limit reset requests to prevent abuse?"

**Tester**: "What if the email address doesn't exist in our system?"

**Business**: "For security, show the same success message whether or not the email exists."

## Outcome: Concrete Scenarios

```gherkin
Scenario: Request password reset with valid email
  Given a user account exists for "user@example.com"
  When I request a password reset for "user@example.com"
  Then I should receive a reset email
  And the reset link should be valid for 1 hour

Scenario: Request password reset with non-existent email
  When I request a password reset for "nonexistent@example.com"
  Then I should see a success message
  But no email should be sent

Scenario: Multiple password reset requests
  Given I have requested a password reset
  When I request another password reset
  Then the previous reset link should be invalidated
  And I should receive a new reset email
```

## Session Structure

```
1. Product Owner presents the user story
2. Developer asks clarifying questions
3. Tester identifies potential edge cases
4. Team writes scenarios together
5. Everyone agrees on acceptance criteria
```

## Best Practices

- Schedule regular Three Amigos sessions
- Time-box to 30-60 minutes
- Focus on one story at a time
- Write scenarios during the session
- Capture questions for later follow-up
