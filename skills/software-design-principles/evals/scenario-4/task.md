# Feature Design: Product Review System

## Problem/Feature Description

The product team has requested a new feature for the e-commerce platform: a product review system. They want to allow customers to leave reviews with ratings and text feedback on products they have purchased.

A junior developer on your team has already drafted an initial design that includes:

- Review entity with rating, comment, photos, and helpful votes
- Support for multiple photo uploads per review
- Helpful/not helpful voting system
- Review moderation queue
- Verified purchase badge
- Review reactions (like, dislike, etc.)
- Review analytics and reporting
- Email notifications when someone replies to your review
- Integration with third-party review aggregation services
- AI-powered sentiment analysis
- Review translation for international users

Your task is to review this feature request and create a focused design that addresses the immediate need without over-engineering.

## Output Specification

Create a design document called `review-feature-design.md` that:

1. **Classifies** the decision type (architectural, tactical, or foundational)
2. **Applies the design decision workflow**:
   - Start with the simplest solution that meets current requirements
   - Identify what NOT to include (YAGNI)
   - Plan for future extension when triggers appear
3. **Uses BAD/GOOD format**:
   - Show what over-designed code looks like (including imagined future features)
   - Show the recommended simple approach
4. **Documents tradeoffs**:
   - What alternatives were considered
   - Why the focused approach was chosen
   - What risks exist and how to mitigate them
5. **Includes validation** step

## Current Business Requirements (Real)

Based on conversations with the product team:

- Customers can leave a 1-5 star rating and text review after purchasing
- Reviews are displayed on product pages
- The team needs to be able to moderate reviews (approve/reject)
- They want this live in 2 weeks
- They are NOT sure about photo uploads yet - "maybe in the future"
- They have mentioned potentially adding voting "someday" but no concrete plans
