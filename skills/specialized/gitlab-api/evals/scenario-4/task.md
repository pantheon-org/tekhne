# Create a GitLab MR Integration Starter Kit

## Problem Description

Your company is building a new internal tool that needs to integrate with GitLab to fetch and display merge request information. The infrastructure team has asked you to create a starter kit that other developers can use as a template when building GitLab integrations.

This starter kit should demonstrate:
- How to properly set up authentication with GitLab's API
- How to make a basic API call to fetch merge request data
- How to handle common edge cases and errors
- Best practices for URL handling and data parsing

The goal is to create something that new developers can copy and modify for their specific needs, with clear documentation about what's required.

## Output Specification

Create a Go package called `gitlab-kit` with:
1. A main file `main.go` that demonstrates fetching MR comment data
2. A `README.md` file documenting:
   - Required environment variables
   - How to create a GitLab token with the right permissions
   - Expected URL format
   - Dependencies needed (if any)
3. Example code showing how to make the API call and parse the response

The example should show fetching comments from a merge request URL provided as a command-line argument.

## Example Usage

```bash
export GITLAB_TOKEN="your-token-here"
go run main.go "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123"
# Outputs: sample comment data
```

The documentation should be thorough enough that someone unfamiliar with GitLab's API could get started.
