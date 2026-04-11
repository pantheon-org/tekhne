# Scenario 03: Shared Deployment Workflow for Microservices Platform

## User Prompt

A platform engineering team manages ten microservices that all deploy to the same Kubernetes cluster. Currently each repository copy-pastes a 150-line deployment workflow. When the deployment process changes, engineers must manually update all ten repositories — a process that regularly leads to drift and outages when some repositories are missed.

The team wants to extract the shared deployment logic into a single reusable workflow that can be called from each microservice repository. The reusable workflow should accept the target environment (`staging` or `production`) as a parameter, receive a deploy token as a secret, and expose the deployed URL as an output for downstream jobs in the calling workflow. The calling workflow lives in the same repository (`.github/workflows/deploy.yml`) and should wire up the reusable workflow correctly.

## Output Specification

Produce two YAML files:
1. `.github/workflows/reusable-deploy.yml` — the reusable workflow definition
2. `.github/workflows/deploy.yml` — a caller workflow that invokes the reusable workflow

Both files should demonstrate the correct pattern for shared deployment workflows in GitHub Actions.

## Expected Behavior

1. Add `workflow_call:` as a trigger in the reusable workflow under `on:`
2. Define at least one input with an explicit `type:` field (e.g., `type: string`) in the `workflow_call` block
3. Declare the deploy token secret under `secrets:` in the `workflow_call` block — do NOT use `secrets: inherit` in the caller
4. Include an `outputs:` section in the `workflow_call` block that maps to a job output (e.g., `${{ jobs.<job>.outputs.<key> }}`)
5. Include a `permissions:` block in the reusable workflow file
6. Pin all action references with full SHA hashes in both files
7. Have the caller pass the secret explicitly by name (not `secrets: inherit`)

## Success Criteria

- **workflow_call trigger present**: The reusable workflow file contains `workflow_call:` as a trigger under `on:`
- **Typed inputs defined**: The `workflow_call` block defines at least one input with an explicit `type:` field (e.g., `type: string`)
- **No secrets inherit**: The caller workflow does NOT use `secrets: inherit` — secrets are passed explicitly by name
- **Explicit secret declaration**: The reusable workflow declares the deploy token (or equivalent secret) under `secrets:` in the `workflow_call` block with `required: true` or `required: false`
- **Output mapped from job output**: The reusable workflow's `workflow_call` block includes an `outputs:` section whose value references a job output (e.g., `${{ jobs.<job>.outputs.<key> }}`)
- **Top-level permissions block**: The reusable workflow file includes a `permissions:` block
- **SHA-pinned actions**: Every `uses:` step in both files references actions via full SHA, not branch or tag alone
- **Caller passes secret by name**: The calling workflow passes the secret to the reusable workflow using `secrets:` with a named key (not `secrets: inherit`)
- **Caller workflow uses `uses:` syntax**: The caller workflow invokes the reusable workflow using the `uses:` key under a job (not duplicating steps inline)

## Failure Conditions

- The reusable workflow does not have `workflow_call:` as a trigger
- No input in the `workflow_call` block has an explicit `type:` field
- The caller workflow uses `secrets: inherit` instead of passing secrets explicitly
- The deploy secret is not declared under `secrets:` in the `workflow_call` block
- The `workflow_call` block has no `outputs:` section mapping to a job output
- The reusable workflow has no `permissions:` block
- Any action in either file uses a tag or branch reference instead of a full SHA
- The caller workflow duplicates inline steps instead of using the `uses:` key to invoke the reusable workflow
