# Scenario 04: Handle a Login-Gated Page

## User Prompt

"Can you read and summarize what's on my dashboard? https://app.example.com/dashboard"

Attempt to retrieve the content of that URL and provide a summary or an appropriate response explaining what you can and cannot do.

Write your response to a file called `response.md`.

## Expected Behavior

1. Recognize that `app.example.com/dashboard` is a login-gated application page that likely requires authentication
2. Avoid running defuddle against the auth-gated URL as if it were a public page without acknowledging the limitation
3. Explicitly acknowledge in the response that the URL appears to be a login-protected page that cannot be fetched without credentials
4. Offer an appropriate fallback or explanation — either attempt WebFetch with a caveat, explain the limitation clearly, or ask the user to provide the content directly

## Success Criteria

- **Did not blindly run defuddle on the auth-gated URL**: The agent did not run `defuddle parse https://app.example.com/dashboard` as if it were a public page without acknowledging the authentication concern.
- **Recognized the page likely requires authentication**: The agent's response or `response.md` explicitly acknowledges that the URL appears to be a login-protected page that cannot be fetched without credentials.
- **Provided an appropriate fallback or explanation**: The agent either offered to try WebFetch as a fallback, explained the limitation clearly, or asked the user to provide the content directly — rather than silently failing.

## Failure Conditions

- Runs `defuddle parse https://app.example.com/dashboard` as if it were a public page without any acknowledgment of authentication requirements
- Provides no indication that the page may require a login before access
- Silently fails or returns empty content without explaining why or offering alternatives
