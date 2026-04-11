# Scenario 01: Secure a Pull Request Automation Workflow

## User Prompt

A platform team maintains a GitHub Actions workflow that automatically posts a comment on pull requests with a summary of what changed. A security researcher has flagged that the workflow contains a code injection risk but didn't provide specifics. The team needs the workflow analyzed and the security issue corrected before it goes back into production. The workflow runs on every pull request opened or updated against the main branch.

Analyze the workflow below, identify the security issue(s), explain what attack vector they expose, and produce a corrected version of the workflow file.

## Output Specification

Produce:
- A corrected `pr-comment.yml` with the security issue(s) fixed
- A `validation-report.md` documenting: each issue found, the type of issue, and what fix was applied

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/pr-comment.yml ===============
name: PR Comment

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  comment:
    runs-on: ubuntu-latest
    permissions:
      pull-requests: write
    steps:
      - name: Post PR summary
        run: |
          echo "PR Title: ${{ github.event.pull_request.title }}"
          echo "Author: ${{ github.event.pull_request.user.login }}"
          gh pr comment ${{ github.event.pull_request.number }} --body "Review requested for: ${{ github.event.pull_request.title }}"
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

## Expected Behavior

1. Identify the direct `${{ github.event.pull_request.title }}` interpolation in `run:` steps as a script injection risk
2. Explain that a PR author could inject shell commands via a crafted PR title or username
3. Move `github.event.pull_request.title` out of direct `run:` interpolation and into an `env:` block variable
4. Move `github.event.pull_request.user.login` out of direct `run:` interpolation and into an `env:` block variable
5. Ensure the shell script references sanitized values using `$ENV_VAR` syntax (quoted) rather than `${{ }}` expressions
6. Label the issue type as security/injection in the validation report

## Success Criteria

- **Injection identified**: validation-report.md (or equivalent) identifies the direct ${{ }} interpolation in run: steps as a script injection risk
- **Attack vector explained**: The report explains that a PR author could inject shell commands via the title or username values
- **PR title via env var**: In pr-comment.yml, github.event.pull_request.title is no longer interpolated directly in run: — it is passed through an env: block variable
- **User login via env var**: In pr-comment.yml, github.event.pull_request.user.login is no longer interpolated directly in run: — it is passed through an env: block variable
- **PR number via env var or safe**: github.event.pull_request.number is either passed through an env: variable or is used in a context where it is safe (numeric values)
- **Env vars referenced safely**: The shell script references the sanitized values using $ENV_VAR syntax (quoted, e.g., "$PR_TITLE") rather than ${{ }} expressions
- **Issue type labeled**: The validation report labels this as a security/injection issue (not just a warning or style issue)
- **Fix applied in corrected file**: The corrected pr-comment.yml has no direct ${{ github.event.* }} interpolation inside run: blocks

## Failure Conditions

- The direct ${{ }} interpolation in run: steps is not identified as a security risk
- The attack vector (shell command injection via PR title or username) is not explained
- Corrected workflow still interpolates `github.event.pull_request.title` directly in a `run:` step
- Corrected workflow still interpolates `github.event.pull_request.user.login` directly in a `run:` step
- Shell script references values via ${{ }} expressions inside run: blocks instead of $ENV_VAR syntax
- The report does not label the issue as security or injection
