# Task: Create a wave execution plan from prose requirements

A backend team needs to migrate their authentication service from session-based
auth to a stateless token-based system. Create a wave execution plan from the
requirements below.

## Requirements

The work must start with a codebase audit: identify every call site that reads
or writes a session, and document a migration checklist. Nothing else can begin
until this audit is complete.

Once the audit is done, two tracks can run in parallel:

- Add a token issuance endpoint that accepts credentials and returns a signed token.
- Add a token validation middleware that verifies the token on incoming requests.

These two are completely independent of each other.

After both the issuance endpoint and the validation middleware exist, implement
the protected route guard that uses the middleware to enforce authentication. This
is a single task that depends on both tracks above.

Once the route guard is live, two cleanup tasks can run in parallel:

- Remove the legacy session handler (the class and its routes).
- Remove the session cleanup background job.

These two touch different files and have no dependency on each other.

Finally, after all legacy session code is removed, run the full regression suite
and update the service runbook. This final step is a single task.

## Output constraints

- Write the wave document to `.context/plans/auth-token-migration.md`
- One branch per parallel task, named `feat/auth-<slug>`
- Each wave must have a verification checklist with at least one concrete command
- Include the dependency graph (ASCII DAG) and a branch strategy table
