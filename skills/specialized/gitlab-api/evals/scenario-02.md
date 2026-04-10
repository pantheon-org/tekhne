# Scenario 02: Monitor Merge Request Review Activity

## User Prompt

Your team's DevOps manager needs visibility into the review process for critical merge requests. They want to know which MRs have had recent activity, which have stalled, and which have outstanding inline code comments that haven't been addressed yet.

The manager needs a monitoring tool that can analyze a merge request and produce a status report showing:
- When the last review activity occurred
- Whether there are pending inline code review comments
- How quickly reviewers are responding

This will help prioritize which MRs need attention and identify bottlenecks in the review process.

Create a bash script called `mr_monitor.sh` that:
1. Takes a GitLab merge request URL as input
2. Analyzes the comment history
3. Produces a file called `activity_report.txt` with:
   - Timestamp of most recent comment
   - Count of pending inline code review comments
   - List of dates when reviews occurred

The script should identify which comments are actual code review feedback vs general discussion.

```bash
./mr_monitor.sh "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Output: activity_report.txt
```

The script should fail gracefully if credentials are missing or the MR doesn't exist.

## Expected Behavior

1. Call `get_mr_comments.sh` in the solution to fetch comments
2. Include logic to compare or sort comment timestamps to identify recent activity
3. Check the `Type` field to distinguish between `DiffNote` (inline code comments) and general comments
4. Extract and use the `Date` field from the comment output
5. Treat or reference timestamps as ISO8601 format without converting to other formats
6. Validate that fetched data contains expected fields before processing
7. Count or flag `Type: DiffNote` comments specifically as pending inline reviews
8. Document or handle the `GITLAB_TOKEN`/`GITLAB_PAT` environment variable requirement
9. Include checks for missing credentials or invalid MR URLs with appropriate error messages

## Success Criteria

- **Uses get_mr_comments.sh**: Calls `get_mr_comments.sh` in the solution
- **Compares timestamps**: Includes logic to compare or sort comment timestamps to identify recent activity
- **Identifies DiffNote type**: Checks the `Type` field to distinguish between `DiffNote` (inline code comments) and general comments
- **Processes Date field**: Extracts and uses the `Date` field from the comment output
- **Handles ISO8601 format**: Treats or references timestamps as ISO8601 format (no manual conversion to other formats)
- **Validates response**: Checks that the fetched data contains expected fields before processing
- **Identifies inline comments**: Specifically counts or flags `Type: DiffNote` comments as pending inline reviews
- **Authentication setup**: Documents or handles `GITLAB_TOKEN`/`GITLAB_PAT` environment variable requirement
- **Error handling**: Includes checks for missing credentials or invalid MR URLs with appropriate error messages

## Failure Conditions

- Script does not call `get_mr_comments.sh` for data retrieval
- Script has no logic to compare or sort timestamps
- Script does not check the `Type` field to distinguish `DiffNote` from general comments
- `Date` field is not extracted or used from the comment output
- Timestamps are manually converted to a non-ISO8601 format
- Script processes output without validating that expected fields are present
- `DiffNote` type comments are not counted or flagged as inline reviews
- `GITLAB_TOKEN`/`GITLAB_PAT` requirement is not documented or handled
- Script fails silently or without clear error messages for missing credentials or invalid URLs
