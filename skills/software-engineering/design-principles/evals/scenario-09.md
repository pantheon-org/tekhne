# Scenario 09: Solid Srp Violation

## User Prompt

"Review this class and identify SRP violations. Suggest how to refactor it.

```typescript
class UserService {
  async register(email: string, password: string): Promise<void> {
    // Validate email format
    if (!email.includes('@')) throw new Error('Invalid email')

    // Hash password
    const hash = await bcrypt.hash(password, 10)

    // Save to database
    await db.users.insert({ email, passwordHash: hash })

    // Send welcome email
    await emailService.send({
      to: email,
      subject: 'Welcome!',
      body: 'Thanks for signing up'
    })

    // Log analytics event
    analytics.track('user_registered', { email })
  }
}
```"

## Expected Behavior

1. Identify 4+ responsibilities: validation, hashing, persistence, notifications, analytics
2. Suggest splitting into focused classes: UserRepository, NotificationService, AnalyticsService
3. Recommend orchestration via use case
4. Explain "one reason to change" principle

## Success Criteria

- Identifies multiple responsibilities (at least 3)
- Proposes extraction of at least 2 new classes
- Mentions SRP or "single reason to change"
- Suggests dependency injection for new services

## Failure Conditions

- Identifies fewer than 3 distinct responsibilities in the class
- Proposes breaking up the method into private helper methods within the same class
- Does not extract any responsibilities into separate classes or services
- Omits SRP or "single reason to change" from the explanation
- Does not suggest dependency injection for the extracted services
