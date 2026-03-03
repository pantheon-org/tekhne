# Multi-Channel Notification Service

## Problem/Feature Description

TechCorp's customer engagement platform needs a notification system that can send alerts via multiple channels: email, SMS, and push notifications. Currently, notifications are hardcoded to only send emails, but marketing campaigns require reaching customers through their preferred communication channels.

The business requirements specify that the system should gracefully handle failures (like invalid email addresses or SMS delivery issues) without breaking the entire notification flow. The service needs to support different notification types like welcome messages, password resets, and promotional offers.

Additionally, the system must be testable in development without actually sending real emails or SMS messages, as this has caused issues with accidental spam to customers during testing.

## Output Specification

Build a notification service with the following capabilities:

1. Send notifications through multiple channels (email, SMS, push)
2. Handle different notification types and templates
3. Provide error handling for channel failures
4. Support configuration of which channels to use per notification type

Design the system to be:

- Easily testable without external service calls
- Extensible for new notification channels
- Resilient to individual channel failures

**Required files:**

- Notification service implementation
- Channel provider implementations (email, SMS, etc.)
- Comprehensive test suite
- Example usage demonstrating the service capabilities
