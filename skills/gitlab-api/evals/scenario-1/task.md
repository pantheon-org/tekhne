# Monitor Merge Request Review Activity

## Problem Description

Your team's DevOps manager needs visibility into the review process for critical merge requests. They want to know which MRs have had recent activity, which have stalled, and which have outstanding inline code comments that haven't been addressed yet.

The manager needs a monitoring tool that can analyze a merge request and produce a status report showing:
- When the last review activity occurred
- Whether there are pending inline code review comments
- How quickly reviewers are responding

This will help prioritize which MRs need attention and identify bottlenecks in the review process.

## Output Specification

Create a bash script called `mr_monitor.sh` that:
1. Takes a GitLab merge request URL as input
2. Analyzes the comment history
3. Produces a file called `activity_report.txt` with:
   - Timestamp of most recent comment
   - Count of pending inline code review comments
   - List of dates when reviews occurred

The script should identify which comments are actual code review feedback vs general discussion.

## Example Usage

```bash
./mr_monitor.sh "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Output: activity_report.txt
```

The script should fail gracefully if credentials are missing or the MR doesn't exist.
