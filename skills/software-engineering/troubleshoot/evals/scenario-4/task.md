# Scenario 4: Runtime Crash with Stack Trace

## User Prompt

"Our Node.js service crashed in production with this stack trace:

```
UnhandledPromiseRejectionWarning: TypeError: Cannot read properties of undefined (reading 'id')
    at UserService.getProfile (src/services/user.service.ts:84:32)
    at async ProfileController.show (src/controllers/profile.ts:21:18)
```"
