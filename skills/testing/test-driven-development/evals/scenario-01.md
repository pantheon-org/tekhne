# Scenario 01: User Authentication Service

## User Prompt

The development team at SecureApp Inc. needs a robust user authentication system for their new web application. Currently, users can only access the app without any security checks, which poses a significant business risk. The company has grown from 100 to 10,000 users in the past year, and they need to implement proper login functionality before their next funding round.

The authentication service should validate user credentials and return appropriate responses for successful and failed login attempts. The service will eventually integrate with their existing user database and session management system, but for now it needs to focus on the core validation logic.

Implement a user authentication service with the following requirements:

1. Create a `UserAuthService` class that can validate login attempts
2. The service should have a `login(username: string, password: string)` method
3. Return success/failure indicators with appropriate user feedback
4. Include comprehensive unit tests that demonstrate the functionality
5. Structure your code to support future database integration

Required files:
- `UserAuthService.ts` (or equivalent in your chosen language)
- Test files with comprehensive coverage
- A `README.md` explaining your testing approach and any design decisions

## Expected Behavior

1. Provide evidence that failing tests were written before implementation code (commit history, comments, or documentation)
2. Show evidence of running tests to confirm they fail before implementation (comments, logs, or documentation)
3. Keep the initial implementation minimal to make tests pass — not over-engineered
4. Show improvement or cleanup of code after tests pass (multiple versions or comments about refactoring)
5. Make multiple test/implementation cycles visible — not one big implementation
6. Use clear Arrange-Act-Assert structure with logical separation in tests
7. Perform only one action/method call per test in the Act phase
8. Verify one logical concept per test — not multiple behaviors combined
9. Name each test to describe the scenario and expected outcome clearly

## Success Criteria

- **Test-first implementation**: Evidence shows failing tests were written before implementation code (commit history, comments, or documentation)
- **Verified test failures**: Shows evidence of running tests to confirm they fail before implementation (comments, logs, or documentation)
- **Minimal implementation**: Initial implementation is minimal to make tests pass, not over-engineered
- **Refactoring evidence**: Shows improvement/cleanup of code after tests pass (multiple versions or comments about refactoring)
- **Red-green cycle discipline**: Multiple test/implementation cycles visible, not one big implementation
- **Test structure follows AAA**: Tests use clear Arrange-Act-Assert structure with logical separation
- **One Act per test**: Each test performs only one action/method call in the Act phase
- **Single behavior per test**: Each test verifies one logical concept, not multiple behaviors combined
- **Descriptive test names**: Test names describe scenario and expected outcome clearly

## Failure Conditions

- No evidence that tests were written before the implementation code
- Tests are written after implementation with no indication of prior failures
- Initial implementation is over-engineered beyond what the tests require
- No refactoring step is evident after the initial tests pass
- A single large implementation is produced without iterative test/code cycles
- Tests do not follow Arrange-Act-Assert structure
- A test performs multiple actions in the Act phase
- A test verifies multiple unrelated behaviors simultaneously
- Test names do not describe what is being tested or what outcome is expected
