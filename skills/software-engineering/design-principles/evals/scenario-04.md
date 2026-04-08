# Scenario 04: Design Patterns Adapter

## User Prompt

"We need to integrate a third-party email service with a poor API. Our domain uses this interface:

```typescript
interface IEmailService {
  send(email: Email): Promise<void>
}

interface Email {
  to: string[]
  subject: string
  body: string
  attachments?: Attachment[]
}
```

The external API looks like this:

```typescript
class LegacyEmailClient {
  sendMail(recipients: string, subj: string, msg: string, files: string[]): boolean
}
```

How do we integrate this without polluting our domain?"

## Expected Behavior

1. Identify interface incompatibility
2. Suggest Adapter pattern
3. Create adapter class implementing IEmailService
4. Wrap LegacyEmailClient and translate calls
5. Keep domain interface unchanged
6. Explain how adapter isolates external dependency

## Success Criteria

- Identifies Adapter pattern as solution
- Creates adapter class implementing domain interface
- Shows translation logic between domain and external types
- Keeps domain interface unchanged
- Mentions isolation or anti-corruption benefits

## Failure Conditions

- Modifies the domain IEmailService interface to match the legacy API
- Does not name or explain the Adapter pattern
- Translates types in calling code rather than encapsulating them in a dedicated class
- Exposes LegacyEmailClient types to the domain layer
- Omits error handling or ignores the boolean return value from sendMail
