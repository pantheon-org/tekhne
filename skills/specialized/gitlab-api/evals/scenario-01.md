# Scenario 01: Generate Merge Request Review Summary

## User Prompt

Your engineering team has been struggling with lengthy merge request reviews that span multiple days and involve many reviewers. The tech lead wants a tool that can automatically generate a concise summary of all the feedback received on a specific merge request to help authors quickly understand what changes are needed.

The summary should help the MR author see:
- Who provided feedback and what they said
- What feedback is from actual reviewers vs automated systems
- Key themes and action items from the discussion

Create a Python script called `mr_summary.py` that:
1. Takes a GitLab merge request URL as a command-line argument
2. Fetches all the discussion and comments
3. Generates a summary report saved to `summary_report.md`

The report should be organized by reviewer and include their feedback. Make sure the summary focuses on human feedback rather than automated bot messages.

```bash
python mr_summary.py "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Output: summary_report.md
```

Note: You'll need to set up authentication with GitLab's API to fetch the data. The script should handle cases where authentication fails or the URL is invalid.

## Expected Behavior

1. Call `get_mr_comments.sh` (or include equivalent logic) to fetch comments
2. Check for expected fields like `Author:` or `Date:` in the script output before processing
3. Filter out comments where `System: true` to remove automated messages
4. Organize comments by author name or username in the summary
5. Extract and use the `Author` field from the output format
6. Extract and use the `Date` field from the output format
7. Extract and use the `Type` field (`DiffNote` or `comment`) from the output format
8. Extract and use the `System` field to distinguish automated vs human comments
9. Reference, set, or document the need for `GITLAB_TOKEN` or `GITLAB_PAT` environment variable
10. Include error handling for invalid URLs or authentication failures

## Success Criteria

- **Uses get_mr_comments.sh script**: Script calls `get_mr_comments.sh` or includes equivalent API integration in the solution
- **Validates response output**: Checks for expected fields like `Author:` or `Date:` in the script output before processing
- **Filters System: false**: Explicitly filters or excludes comments where `System: true` to remove automated messages
- **Groups by author**: Organizes comments by author name or username in the summary
- **Parses Author field**: Extracts and uses the `Author` field from the output format
- **Parses Date field**: Extracts and uses the `Date` field from the output format
- **Parses Type field**: Extracts and uses the `Type` field (`DiffNote` or `comment`) from the output format
- **Parses System field**: Extracts and uses the `System` field to identify automated vs human comments
- **Handles GITLAB_TOKEN**: References, sets, or documents the need for `GITLAB_TOKEN` or `GITLAB_PAT` environment variable
- **Error handling**: Includes error handling for invalid URLs or authentication failures

## Failure Conditions

- Script does not call `get_mr_comments.sh` or include equivalent API integration
- Script processes comment output without first validating that expected fields are present
- Automated system messages (`System: true`) are included in the summary output
- Comments are not organized by author in the summary
- `Author` field is not extracted or used from the comment output
- `Date` field is not extracted or used from the comment output
- `Type` field is not extracted or used from the comment output
- `System` field is not extracted or checked in the processing logic
- `GITLAB_TOKEN` or `GITLAB_PAT` requirement is not referenced or documented
- Script fails silently or provides no feedback on authentication failures or invalid URLs
