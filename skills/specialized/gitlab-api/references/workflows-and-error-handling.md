# Common Workflows and Error Handling

## Generate MR Review Summary

1. Fetch and validate comments:
   ```bash
   output=$(scripts/get_mr_comments.sh "$MR_URL")
   echo "$output" | grep -q "Author:" || { echo "Invalid response — check token and URL"; exit 1; }
   ```
2. Filter by `System: false` to exclude automated messages
3. Group by author, extract action items and feedback themes
4. Generate summary report

## Track Review Progress

1. Fetch and validate comments:
   ```bash
   output=$(scripts/get_mr_comments.sh "$MR_URL")
   [ -n "$output" ] && echo "$output" | grep -q "Date:" || { echo "Empty or malformed response — check credentials"; exit 1; }
   ```
2. Compare timestamps to identify recent activity
3. Flag unresolved DiffNotes and report on response times

## Error Handling

**401 Unauthorized** — Token missing or invalid:
- Check `GITLAB_TOKEN` or `GITLAB_PAT` is set
- Verify token has `read_api` scope

**404 Project Not Found** — Invalid project path:
- Confirm URL format: `https://gitlab.com/group/project/-/merge_requests/ID`
- Check token has access to the project

**200 OK with an error body** — the pitfall neither status code above catches:
- GitLab's API can return HTTP 200 with a JSON error object in the body (e.g. for a malformed nested query)
- Always check the response shape (does it contain the expected `Author:`/`Date:` fields, or an `error`/`message` key?) before treating a 200 response as valid data — this is why `get_mr_comments.sh`'s own output format is worth validating field-by-field rather than just checking the exit code
