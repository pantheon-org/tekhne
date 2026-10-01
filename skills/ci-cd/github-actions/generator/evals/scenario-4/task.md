# Scenario 4: PR Labeler and Coverage Uploader for Open-Source Repository

## User Prompt

A popular open-source project on GitHub receives many pull requests from external contributors (forks). The maintainers want two automated behaviours:

1. **Auto-label PRs** based on the PR title: if the title starts with `fix:` apply the `bug` label; if it starts with `feat:` apply the `enhancement` label. This requires write access to the repository to add labels.

2. **Upload code coverage** to an external service after CI passes. The upload requires a `CODECOV_TOKEN` secret that must not be exposed to untrusted fork code.

A previous engineer drafted a single workflow that uses `pull_request_target` so it can access secrets, and it checks out the PR's head commit directly. Before shipping this, the team wants it reviewed and rewritten following safe patterns. The draft is provided below.
