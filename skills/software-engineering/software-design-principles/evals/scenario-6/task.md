# Architecture Decision: Message Queue vs Direct API Calls

## Problem/Feature Description

Your team is designing a notification system for a social media platform. When users create posts, the system needs to notify followers, update recommendation engines, trigger content moderation, and send push notifications to mobile apps.

Currently, the system makes direct synchronous API calls to each service, but this is causing performance bottlenecks and reliability issues when downstream services are slow or unavailable.

The team is debating between three approaches:

1. **Message queue with async processing** (Kafka/RabbitMQ)
2. **Direct API calls with circuit breakers** (current + resilience)
3. **Webhook-based notifications** (HTTP callbacks)

You need to make an architectural decision and document it properly using ADR format.

## Output Specification

Create an architecture decision record called `adr-001-notification-architecture.md` that:

1. **Classifies** this as an architectural decision
2. **Documents the decision** using proper ADR format with alternatives, rationale, and risks
3. **Applies architectural checks** for dependency direction and coupling
4. **Considers tradeoffs** systematically between all three options
5. **Includes validation steps** for the chosen approach

The ADR should follow standard format: Status, Context, Decision, Alternatives, Consequences, and Validation.

## Input Files

=============== FILE: current-notification.ts ===============
export class NotificationService {
  constructor(
    private followerService: FollowerService,
    private recommendationService: RecommendationService,
    private moderationService: ModerationService,
    private pushService: PushNotificationService
  ) {}

  async notifyPostCreated(post: Post): Promise\<void> {
    // All synchronous - blocks user experience
    const followers = await this.followerService.getFollowers(post.authorId);

    // These calls can take 200ms each - user waits 800ms+
    await this.recommendationService.updateFor(post);
    await this.moderationService.scanContent(post);
    await this.pushService.notifyFollowers(followers, post);
    
    // If any service fails, entire operation fails
  }
}

=============== FILE: performance-metrics.md ===============

## Current Performance Issues

- Average post creation time: 1.2 seconds
- 99th percentile: 4.5 seconds  
- Service failures cause user-visible errors
- Recommendation service timeout: 12% of requests
- Moderation service SLA: 99.5% (fails 0.5% of posts)

## Requirements

- Post creation should complete in <200ms for user
- Notification delivery can be eventual (within 30 seconds)
- System should handle 10,000 posts/minute at peak
- Individual service failures should not block post creation
