# Scenario 7: Deployment Workflow with an Environment and Cleanup

## User Prompt

"Write a workflow that deploys our static site to a `production` environment after the tests pass on main. Use the official Pages actions, show a short summary of what was deployed, and always remove the temporary build directory even if the deploy fails."

## Repo State

A Node.js repository with `npm run build` producing `dist/`. No existing workflows.
