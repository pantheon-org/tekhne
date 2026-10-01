# Scenario 04: Secure a Pull Request Automation Workflow

## User Prompt

A platform team maintains a GitHub Actions workflow that automatically posts a comment on pull requests with a summary of what changed. A security researcher has flagged that the workflow contains a code injection risk but didn't provide specifics. The team needs the workflow analyzed and the security issue corrected before it goes back into production. The workflow runs on every pull request opened or updated against the main branch.

Analyze the workflow below, identify the security issue(s), explain what attack vector they expose, and produce a corrected version of the workflow file.
