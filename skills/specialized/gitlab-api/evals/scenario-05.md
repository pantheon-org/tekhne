# Scenario 05: Create a GitLab MR Integration Starter Kit

## User Prompt

Your company is building a new internal tool that needs to integrate with GitLab to fetch and display merge request information. The infrastructure team has asked you to create a starter kit that other developers can use as a template when building GitLab integrations.

This starter kit should demonstrate:
- How to properly set up authentication with GitLab's API
- How to make a basic API call to fetch merge request data
- How to handle common edge cases and errors
- Best practices for URL handling and data parsing

Create a Go package called `gitlab-kit` with:
1. A main file `main.go` that demonstrates fetching MR comment data
2. A `README.md` file documenting:
   - Required environment variables
   - How to create a GitLab token with the right permissions
   - Expected URL format
   - Dependencies needed (if any)
3. Example code showing how to make the API call and parse the response

The example should show fetching comments from a merge request URL provided as a command-line argument.

```bash
export GITLAB_TOKEN="your-token-here"
go run main.go "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123"
# Outputs: sample comment data
```

The documentation should be thorough enough that someone unfamiliar with GitLab's API could get started.

## Expected Behavior

1. Document or mention setting `GITLAB_TOKEN` or `GITLAB_PAT` environment variable in the README or code
2. State that the token needs `read_api` scope/permission in the documentation
3. Specify the expected URL format: `https://gitlab.com/group/project/-/merge_requests/ID`
4. Use or reference `get_mr_comments.sh` in the solution
5. Document that `curl` is needed as a dependency
6. Document that `jq` is needed for JSON parsing
7. Demonstrate or mention URL encoding project paths (replacing `/` with `%2F`)
8. Explain how to create a GitLab token (User Settings → Access Tokens or similar)
9. Document script exit codes (0 for success, 1 for errors)

## Success Criteria

- **Documents GITLAB_TOKEN or GITLAB_PAT**: README or documentation mentions setting `GITLAB_TOKEN` or `GITLAB_PAT` environment variable
- **Specifies read_api scope**: Documentation states that token needs `read_api` scope/permission
- **Documents URL format**: Specifies expected URL format: `https://gitlab.com/group/project/-/merge_requests/ID`
- **References get_mr_comments.sh**: Uses or references `get_mr_comments.sh` in the solution
- **Mentions curl requirement**: Documents that `curl` is needed as a dependency
- **Mentions jq requirement**: Documents that `jq` is needed for JSON parsing
- **Shows URL encoding**: Demonstrates or mentions URL encoding project paths (replacing `/` with `%2F`)
- **Token creation guidance**: Explains how to create a GitLab token (User Settings → Access Tokens or similar)
- **Exit code documentation**: Documents script exit codes (0 for success, 1 for errors)

## Failure Conditions

- `GITLAB_TOKEN` or `GITLAB_PAT` is not documented or mentioned
- Token `read_api` scope requirement is not specified
- Expected URL format is not documented
- `get_mr_comments.sh` is not referenced or used
- `curl` dependency is not documented
- `jq` dependency is not documented
- URL encoding of project paths is not demonstrated or mentioned
- No guidance is provided on how to create a GitLab token
- Script exit codes are not documented
