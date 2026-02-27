# Architecture Decision: Notification Service

## Problem/Feature Description

Your team needs to create a new notification microservice that handles sending emails, SMS, and push notifications to users. The service must integrate with the existing platform which includes:

- **User Service**: Manages user profiles and preferences
- **Auth Service**: Handles authentication and authorization
- **Template Service**: Provides notification templates with variable substitution

The notification service will receive events from other services (order placed, password reset, etc.) and dispatch notifications based on user preferences.

Your task is to design the architecture for this new service and document the key decisions. The architecture must be maintainable, testable, and integrate cleanly with existing services without creating tight coupling.

## Output Specification

Create an Architecture Decision Record (ADR) document called `adr-notification-service.md` that covers:

1. **Classification**: Identify whether this is an architectural, tactical, or foundational decision

2. **Strategic Architecture**:
   - Define service boundaries and where the service fits in the overall system
   - Specify dependency direction (which services depend on which)
   - Identify potential circular dependencies and how to avoid them
   - Define who owns the contracts/interfaces

3. **Domain Design**:
   - Define the core entities (Notification, Template, Recipient, etc.)
   - Ensure entities remain pure - no database or infrastructure concerns in domain objects
4. **Tradeoffs**:
   - List at least 2 alternatives considered
   - Explain why you chose your recommended approach
   - Document risks and mitigation strategies
5. **Validation**: Include a validation step before proceeding to implementation

## Existing System Context

The current platform structure:

- User Service exposes REST API for user preferences
- Auth Service provides JWT tokens for authenticated requests
- Template Service provides template rendering via GraphQL
- All services use PostgreSQL for persistence
- Services communicate via REST APIs and a message queue (RabbitMQ)
