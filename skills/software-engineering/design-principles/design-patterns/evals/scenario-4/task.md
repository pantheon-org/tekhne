# Scenario 4: Apply Factory Pattern to Encapsulate Conditional Object Creation

## User Prompt

The following `NotificationService` constructs notification senders with complex conditional logic inside the constructor. Refactor using the Factory pattern.

## Expected Behavior

1. `INotificationSender.ts` exists with a `send` method returning `Promise<void>`
2. `EmailSender.ts`, `SmsSender.ts`, and `SlackSender.ts` each implement `INotificationSender`
3. `NotificationSenderFactory.ts` has a `create` method containing the channel-based branching logic
4. Refactored `NotificationService` constructor parameter is typed to `INotificationSender`, not a concrete sender class
5. Refactored `NotificationService.ts` has no `if/else` or `switch` statements for channel selection

## Failure Conditions

- `INotificationSender.ts` is missing or has no `send` method
- Any of the three sender files is missing or does not implement `INotificationSender`
- `NotificationSenderFactory.ts` is missing or its `create` method contains no branching logic
- Refactored `NotificationService` still accepts `channel` and `config` in its constructor instead of an `INotificationSender`
- Refactored `NotificationService.ts` still contains `if/else` or `switch` statements for channel selection
