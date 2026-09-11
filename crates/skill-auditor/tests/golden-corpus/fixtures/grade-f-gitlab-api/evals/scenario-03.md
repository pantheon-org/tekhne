# Scenario 03: Build a Diagnostic Tool for GitLab MR Access Issues

## User Prompt

Your engineering team frequently encounters issues when trying to access GitLab merge requests programmatically. New team members often struggle with authentication, incorrect URLs, or missing permissions, and there's no quick way to diagnose what's wrong.

You've been asked to create a diagnostic tool that can help troubleshoot why access to a merge request might be failing. The tool should check common problems and give helpful feedback about what's wrong.

Create a Node.js script called `diagnose.js` that:
1. Takes a GitLab merge request URL as a command-line argument
2. Attempts to retrieve information about the merge request
3. Saves diagnostic results to `diagnostic_report.json`

The diagnostic report should include:
- Whether authentication is properly configured
- Whether the URL format appears valid
- What error occurred (if any)
- Suggestions for fixing the problem

The tool should be able to distinguish between authentication problems and other types of errors.

```bash
node diagnose.js "https://gitlab.com/myorg/myproject/-/merge_requests/42"
# Output: diagnostic_report.json
```

The script should provide clear, actionable feedback when something goes wrong rather than just failing silently.

## Expected Behavior

1. Call or integrate `get_mr_comments.sh` in the solution
2. Verify that the `GITLAB_TOKEN` or `GITLAB_PAT` environment variable exists before making requests
3. Validate that the URL matches the pattern `https://gitlab.com/*/merge_requests/[number]` or expected format
4. Detect authentication failures (HTTP 401) and provide feedback about token issues
5. Detect project-not-found errors (HTTP 404) and provide feedback about URL or access issues
6. Examine script exit codes (0 for success, 1 for errors) to detect failures
7. Reference or check that the token has `read_api` scope as part of diagnostics
8. Validate that the response contains expected fields like `Author:` or `Date:` to ensure valid output
9. Include suggestions for fixing detected problems (e.g., "set GITLAB_TOKEN", "verify URL format")

## Success Criteria

- **Uses get_mr_comments.sh**: Calls or integrates `get_mr_comments.sh` in the solution
- **Checks for GITLAB_TOKEN**: Verifies that `GITLAB_TOKEN` or `GITLAB_PAT` environment variable exists before making requests
- **Validates URL format**: Checks that URL matches pattern `https://gitlab.com/*/merge_requests/[number]` or validates against expected format
- **Handles 401 errors**: Detects authentication failures and provides feedback about token issues
- **Handles 404 errors**: Detects project not found errors and provides feedback about URL or access issues
- **Checks exit codes**: Examines script exit code (0 for success, 1 for errors) to detect failures
- **Mentions read_api scope**: References or checks that token has `read_api` scope as part of diagnostics
- **Validates response format**: Checks that response contains expected fields like `Author:` or `Date:` to ensure valid output
- **Provides actionable suggestions**: Includes suggestions for fixing detected problems (e.g., "set GITLAB_TOKEN", "verify URL format")

## Failure Conditions

- Script does not call or integrate `get_mr_comments.sh`
- Script does not check for `GITLAB_TOKEN` or `GITLAB_PAT` before making requests
- URL format is not validated before attempting API calls
- HTTP 401 authentication errors are not detected or explained
- HTTP 404 not-found errors are not detected or explained
- Script exit codes are not checked to detect failures
- `read_api` token scope is not mentioned or checked
- Response format is not validated for expected fields
- Script provides no actionable suggestions when problems are detected
