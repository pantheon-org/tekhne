# Scenario 05: Production Deployment Pipeline with Manual Approval Gate

## User Prompt

You are building a production deployment pipeline that includes a manual approval gate. The pipeline must build, test, and then deploy to production only on the `main` branch, requiring sign-off from a group called `release-managers` before the deployment proceeds. Artifacts should be archived with fingerprinting enabled.

The pipeline must include stages for: build, test, a conditional production deployment (main branch only) with a manual approval gate, and artifact archiving.

## Expected Behavior

The agent should place the `input` step outside the `steps` block (directly in the stage) to avoid holding a Jenkins agent while waiting for approval. `archiveArtifacts` must include `fingerprint: true`. The production deployment stage must use `when { branch 'main'; beforeAgent true }` so agent allocation is skipped when the condition is false. The `input` block must include a `submitter` field restricted to `release-managers`. The `post` block must include `always { cleanWs() }`.

Capability tested: Manual approval, fingerprint, artifact archiving.

1. Place `input` outside the `steps` block to avoid holding an agent during manual approval
2. Use `archiveArtifacts` with `fingerprint: true`
3. Include a production deployment stage conditional on `branch 'main'`
4. Add `beforeAgent true` to the `when` block on the deployment stage
5. Restrict the `input` approver via `submitter: 'release-managers'` (or equivalent)
6. Include `post { always { cleanWs() } }` for workspace cleanup

## Success Criteria

- **input outside steps**: The `input` directive appears outside the `steps { }` block (at stage level) to prevent holding a build agent during approval wait
- **fingerprint: true**: `archiveArtifacts` directive includes `fingerprint: true`
- **archiveArtifacts present**: Jenkinsfile contains an `archiveArtifacts` directive in the `post success` block or in a stage
- **when branch main**: The production deployment stage has a `when { branch 'main' }` or equivalent condition
- **beforeAgent true**: The `when` block on the deployment stage includes `beforeAgent true` to skip agent allocation when the condition is false
- **submitter restricted**: The `input` block includes a `submitter` field restricting approval to specific users or groups (e.g., `release-managers`)
- **cleanWs in post**: Jenkinsfile `post` block includes `always { cleanWs() }` or equivalent workspace cleanup

## Failure Conditions

- The `input` step is placed inside `steps { }`, which holds a build agent while waiting for approval
- `archiveArtifacts` does not include `fingerprint: true`
- No `archiveArtifacts` directive is present
- The deployment stage is not conditional on the `main` branch
- `beforeAgent true` is absent from the `when` block, causing unnecessary agent allocation
- The `input` block has no `submitter` restriction (anyone can approve)
- No `post` block with `cleanWs()` or equivalent is present
