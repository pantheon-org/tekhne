# Scenario 04: Apply Factory Pattern to Encapsulate Conditional Object Creation

## User Prompt

The following `NotificationService` constructs notification senders with complex conditional logic inside the constructor. Refactor using the Factory pattern.

## Code to Refactor

```typescript
// src/NotificationService.ts
export class NotificationService {
  private sender: { send(to: string, message: string): Promise<void> }

  constructor(channel: string, config: Record<string, string>) {
    if (channel === 'email') {
      const smtpHost = config.smtpHost
      const smtpPort = parseInt(config.smtpPort)
      // ... complex SMTP setup
      this.sender = {
        send: async (to, message) => {
          console.log(`SMTP to ${to} via ${smtpHost}:${smtpPort}: ${message}`)
        }
      }
    } else if (channel === 'sms') {
      const apiKey = config.twilioApiKey
      const from = config.twilioFrom
      this.sender = {
        send: async (to, message) => {
          console.log(`SMS from ${from} via Twilio apiKey=${apiKey}: ${to} - ${message}`)
        }
      }
    } else if (channel === 'slack') {
      const webhookUrl = config.slackWebhook
      this.sender = {
        send: async (to, message) => {
          console.log(`Slack webhook ${webhookUrl}: @${to} ${message}`)
        }
      }
    } else {
      throw new Error(`Unknown channel: ${channel}`)
    }
  }

  async notify(to: string, message: string): Promise<void> {
    await this.sender.send(to, message)
  }
}
```

## Output Specification

Produce:

1. `INotificationSender.ts` — the sender interface with `send(to: string, message: string): Promise<void>`.
2. `EmailSender.ts`, `SmsSender.ts`, `SlackSender.ts` — one file per channel implementation.
3. `NotificationSenderFactory.ts` — a factory class with a `create(channel: string, config: Record<string, string>): INotificationSender` method containing all the construction logic.
4. `NotificationService.ts` — refactored to accept an `INotificationSender` via constructor (the factory is used externally to construct it).

All TypeScript files must be valid.

## Expected Behavior

1. `INotificationSender.ts` exists with a `send` method returning `Promise<void>`
2. `EmailSender.ts`, `SmsSender.ts`, and `SlackSender.ts` each implement `INotificationSender`
3. `NotificationSenderFactory.ts` has a `create` method containing the channel-based branching logic
4. Refactored `NotificationService` constructor parameter is typed to `INotificationSender`, not a concrete sender class
5. Refactored `NotificationService.ts` has no `if/else` or `switch` statements for channel selection

## Success Criteria

- **INotificationSender interface produced**: `INotificationSender.ts` exists with a `send` method returning `Promise<void>`
- **Three sender implementations produced**: `EmailSender.ts`, `SmsSender.ts`, and `SlackSender.ts` each exist and implement `INotificationSender`
- **Factory class has create method with conditional logic**: `NotificationSenderFactory.ts` has a `create` method containing the channel-based branching logic
- **Refactored NotificationService accepts sender via constructor**: `NotificationService` constructor parameter is typed to `INotificationSender`, not a concrete sender class
- **NotificationService contains no conditional creation logic**: The refactored `NotificationService.ts` has no `if/else` or `switch` statements for channel selection

## Failure Conditions

- `INotificationSender.ts` is missing or has no `send` method
- Any of the three sender files is missing or does not implement `INotificationSender`
- `NotificationSenderFactory.ts` is missing or its `create` method contains no branching logic
- Refactored `NotificationService` still accepts `channel` and `config` in its constructor instead of an `INotificationSender`
- Refactored `NotificationService.ts` still contains `if/else` or `switch` statements for channel selection
