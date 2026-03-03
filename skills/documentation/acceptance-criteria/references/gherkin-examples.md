# Gherkin Examples for Acceptance Criteria

Use these examples when the story has ordered interactions or when scenarios will be automated.

## Login

```gherkin
Scenario: Successful login with valid credentials
Given the user has a registered account
And the account is not locked
When the user submits valid email and password
Then the user is redirected to the dashboard
And a session is created for 24 hours

Scenario: Login fails with invalid password
Given the user has a registered account
When the user submits a valid email and invalid password
Then an "Incorrect email or password" message is shown
And the user remains on the login page
```

## Password Reset

```gherkin
Scenario: Request reset link
Given the user is on the sign-in page
When the user requests password reset for a registered email
Then a reset email is sent
And the UI shows a confirmation message

Scenario: Expired reset token
Given the user opened a reset link older than 1 hour
When the user submits a new password
Then the reset is rejected
And the UI prompts the user to request a new link
```

## API Create User

```gherkin
Scenario: Create user succeeds
Given a valid auth token with admin scope
When a POST request is sent to /api/v1/users with valid payload
Then the response status is 201
And the response includes id, email, role, and createdAt

Scenario: Create user fails with duplicate email
Given a valid auth token with admin scope
And a user already exists with the requested email
When a POST request is sent to /api/v1/users
Then the response status is 409
And the response includes a duplicate email error
```

## Writing Notes

- Use one behavior-focused scenario name per scenario.
- Keep `Given` to prerequisites that matter for outcome.
- Keep `When` as a single user or system action.
- Keep `Then` strictly observable and verifiable.
