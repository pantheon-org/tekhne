# Build a Diagnostic Tool for GitLab MR Access Issues

## Problem Description

Your engineering team frequently encounters issues when trying to access GitLab merge requests programmatically. New team members often struggle with authentication, incorrect URLs, or missing permissions, and there's no quick way to diagnose what's wrong.

You've been asked to create a diagnostic tool that can help troubleshoot why access to a merge request might be failing. The tool should check common problems and give helpful feedback about what's wrong.

## Output Specification

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

## Example Usage

```bash
node diagnose.js "https://gitlab.com/myorg/myproject/-/merge_requests/42"
# Output: diagnostic_report.json
```

The script should provide clear, actionable feedback when something goes wrong rather than just failing silently.
