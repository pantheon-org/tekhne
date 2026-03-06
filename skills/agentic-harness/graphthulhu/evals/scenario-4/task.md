# User Correction — Update the Vault Before Continuing

## Problem/Feature Description

During a previous session, the knowledge graph was updated with an analysis of the deployment pipeline. The stored page `analysis/deployment-pipeline` contains the following (incorrect) finding:

> "The deploy job uses `kubectl apply` with `--force` flag, which can cause resource deletion."

The developer now corrects this: "That's wrong — we use `kubectl apply` without `--force`. We removed that flag three sprints ago. Please fix it."

After correcting the vault, the developer wants to continue with the next task: reviewing the rollback procedure.

## Task

Handle the correction and then acknowledge you are ready to continue.

## Expected Behaviour

- Call `search` to locate the block containing the incorrect statement about `--force`.
- Call `update_block` to replace the incorrect content with: "The deploy job uses `kubectl apply` without `--force` — the flag was removed to prevent unintended resource deletion."
- Do NOT proceed to the rollback review task until the correction is confirmed saved.
- Do NOT simply acknowledge the correction verbally without updating the vault.
- After saving, confirm the correction is persisted, then offer to continue with the rollback review.
