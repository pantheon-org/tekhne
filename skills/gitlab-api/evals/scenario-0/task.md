# Generate Merge Request Review Summary

## Problem Description

Your engineering team has been struggling with lengthy merge request reviews that span multiple days and involve many reviewers. The tech lead wants a tool that can automatically generate a concise summary of all the feedback received on a specific merge request to help authors quickly understand what changes are needed.

The summary should help the MR author see:
- Who provided feedback and what they said
- What feedback is from actual reviewers vs automated systems
- Key themes and action items from the discussion

## Output Specification

Create a Python script called `mr_summary.py` that:
1. Takes a GitLab merge request URL as a command-line argument
2. Fetches all the discussion and comments
3. Generates a summary report saved to `summary_report.md`

The report should be organized by reviewer and include their feedback. Make sure the summary focuses on human feedback rather than automated bot messages.

## Example Usage

```bash
python mr_summary.py "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Output: summary_report.md
```

Note: You'll need to set up authentication with GitLab's API to fetch the data. The script should handle cases where authentication fails or the URL is invalid.
