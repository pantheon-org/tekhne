# Validation Workflow

Use this flow to validate a CloudFormation update behavior safely.

## Step 1: Research

- Confirm resource/property in AWS docs.
- Note expected behavior (`No interruption`, `Some interruption`, `Replacement`).

## Step 2: Design Minimal Reproduction

- One stack with minimal resources.
- One property change per test run.
- Explicit pass/fail criteria before execution.

## Step 3: Execute

1. Deploy baseline stack.
2. Capture initial resource IDs/ARNs.
3. Apply single change.
4. Redeploy and inspect stack events.

## Step 4: Verify Outcome

- Confirm whether resource was updated, replaced, or unchanged.
- Validate runtime behavior (e.g., new subscription endpoint receives confirmation).

## Step 5: Decision

- Workaround needed: yes/no.
- Record evidence (events, timestamps, screenshots/log links).
