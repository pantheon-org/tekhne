# Task: Apply Factory Pattern to Encapsulate Conditional Object Creation

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
