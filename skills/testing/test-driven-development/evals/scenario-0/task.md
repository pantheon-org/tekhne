# User Authentication Service

## Problem/Feature Description

The development team at SecureApp Inc. needs a robust user authentication system for their new web application. Currently, users can only access the app without any security checks, which poses a significant business risk. The company has grown from 100 to 10,000 users in the past year, and they need to implement proper login functionality before their next funding round.

The authentication service should validate user credentials and return appropriate responses for successful and failed login attempts. The service will eventually integrate with their existing user database and session management system, but for now it needs to focus on the core validation logic.

## Output Specification

Implement a user authentication service with the following requirements:

1. Create a `UserAuthService` class that can validate login attempts
2. The service should have a `login(username: string, password: string)` method
3. Return success/failure indicators with appropriate user feedback
4. Include comprehensive unit tests that demonstrate the functionality
5. Structure your code to support future database integration

**Required files:**

- `UserAuthService.ts` (or equivalent in your chosen language)
- Test files with comprehensive coverage
- A `README.md` explaining your testing approach and any design decisions
