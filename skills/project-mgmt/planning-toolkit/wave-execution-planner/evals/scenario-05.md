# Scenario 05: Create a Wave Execution Plan from Prose Requirements

## User Prompt

A backend team needs to migrate their authentication service from session-based auth to a stateless token-based system. Create a wave execution plan from the requirements below.

## Requirements

The work must start with a codebase audit: identify every call site that reads or writes a session, and document a migration checklist. Nothing else can begin until this audit is complete.

Once the audit is done, two tracks can run in parallel:

- Add a token issuance endpoint that accepts credentials and returns a signed token.
- Add a token validation middleware that verifies the token on incoming requests.

These two are completely independent of each other.

After both the issuance endpoint and the validation middleware exist, implement the protected route guard that uses the middleware to enforce authentication. This is a single task that depends on both tracks above.

Once the route guard is live, two cleanup tasks can run in parallel:

- Remove the legacy session handler (the class and its routes).
- Remove the session cleanup background job.

These two touch different files and have no dependency on each other.

Finally, after all legacy session code is removed, run the full regression suite and update the service runbook. This final step is a single task.

## Output constraints

- Write the wave document to `.context/plans/auth-token-migration.md`
- One branch per parallel task, named `feat/auth-<slug>`
- Each wave must have a verification checklist with at least one concrete command
- Include the dependency graph (ASCII DAG) and a branch strategy table

## Expected Behavior

1. Structure the plan with exactly 5 waves: (1) audit, (2) issuance + validation in parallel, (3) route guard, (4) legacy removal in parallel, (5) regression + runbook
2. No wave contains two tasks where one depends on the other (e.g. issuance and validation are in the same wave, not split across waves)
3. Waves 2 and 4 are labelled "parallel"; waves 1, 3, and 5 are labelled "sequential" (single task each)
4. Write the wave document to `.context/plans/auth-token-migration.md`
5. Include an ASCII DAG section showing the dependency structure between waves and tasks
6. Include a branch strategy table assigning one branch per parallel task using the `feat/auth-<slug>` pattern
7. Each wave has a Verification checklist with at least one concrete, runnable command
8. Do not add ordering constraints not stated in the requirements (e.g. do not force issuance before validation or vice versa)

## Success Criteria

- **correct-wave-count-and-structure**: Plan has exactly 5 waves matching the topological sort: (1) audit, (2) issuance + validation in parallel, (3) route guard, (4) legacy removal in parallel, (5) regression + runbook
- **no-intra-wave-dependencies**: No wave contains two tasks where one depends on the other
- **parallel-sequential-labels-correct**: Waves 2 and 4 are labelled "parallel"; waves 1, 3, and 5 are labelled "sequential"
- **output-file-written**: Wave document is written to `.context/plans/auth-token-migration.md`
- **dependency-graph-present**: An ASCII DAG section is included showing the dependency structure between waves and tasks
- **branch-strategy-present**: A branch strategy table is included assigning one branch per parallel task using the `feat/auth-<slug>` pattern
- **verification-checklists-present**: Each wave has a Verification checklist with at least one concrete, runnable command
- **no-invented-dependencies**: Agent does not add ordering constraints not stated in the requirements

## Failure Conditions

- Creates more or fewer than 5 waves
- Places issuance and validation in separate waves (they should be parallel in the same wave)
- Labels parallel waves (2, 4) as "sequential" or sequential waves (1, 3, 5) as "parallel"
- Writes to a path other than `.context/plans/auth-token-migration.md`
- Omits the ASCII DAG section
- Omits the branch strategy table or uses a different naming pattern
- A wave is missing its Verification checklist
- Forces issuance before validation (or vice versa) when they are stated as independent
