---
name: gitlab-api
description: Fetches and analyzes GitLab merge request (MR) comments, metadata, and review feedback using authenticated API calls. Capabilities include fetching comment threads, retrieving reviewer feedback, filtering unresolved discussions, and generating MR activity reports. Use when the user asks about GitLab MR comments, code review discussions, review feedback, approval status, troubleshooting review delays, or needs to fetch merge request metadata via the GitLab API.
---

# GitLab API Integration

Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls.

## Prerequisites

- Set `GITLAB_TOKEN` or `GITLAB_PAT` with `read_api` scope
- Requires: `curl`, `jq`

## Available Scripts

### get_mr_comments.sh

Fetches all comments from a GitLab merge request.

**Location:** `.agents/skills/gitlab-api/scripts/get_mr_comments.sh`

**Usage:**
```bash
.agents/skills/gitlab-api/scripts/get_mr_comments.sh <merge_request_url>
```

**Example:**
```bash
# Prerequisites assumed: GITLAB_TOKEN with read_api scope
.agents/skills/gitlab-api/scripts/get_mr_comments.sh "https://gitlab.com/your-group/your-project/-/merge_requests/123"
```

**Output Format:**
```
---
Author: Name (@username)
Date: ISO8601 timestamp (UTC)
Type: DiffNote|comment
System: true|false

Comment body text
```

- `System: true` — automated/system-generated message; `System: false` — human comment
- `Type: DiffNote` — inline code review comment; `Type: comment` — general MR comment
- Timestamps are UTC in ISO8601 format

**Exit Codes:**
- `0`: Success
- `1`: Invalid URL format, missing token, or API error

**When to Examine Script Internals:**

Read script source when debugging unexpected output, extending for custom metadata, or understanding URL encoding. For basic usage, the examples above suffice.

Script location: `.agents/skills/gitlab-api/scripts/get_mr_comments.sh` (~80 lines)

## Common Workflows

### Generate MR Review Summary

1. Fetch and validate comments:
   ```bash
   output=$(.agents/skills/gitlab-api/scripts/get_mr_comments.sh "$MR_URL")
   echo "$output" | grep -q "Author:" || { echo "Invalid response — check token and URL"; exit 1; }
   ```
2. Filter by `System: false` to exclude automated messages
3. Group by author, extract action items and feedback themes
4. Generate summary report

### Track Review Progress

1. Fetch and validate comments:
   ```bash
   output=$(.agents/skills/gitlab-api/scripts/get_mr_comments.sh "$MR_URL")
   [ -n "$output" ] && echo "$output" | grep -q "Date:" || { echo "Empty or malformed response — check credentials"; exit 1; }
   ```
2. Compare timestamps to identify recent activity
3. Flag unresolved DiffNotes and report on response times

## Error Handling

**401 Unauthorized**: Token missing or invalid
- Check `GITLAB_TOKEN` or `GITLAB_PAT` is set
- Verify token has API read scope

**404 Project Not Found**: Invalid project path
- Confirm URL format: `https://gitlab.com/group/project/-/merge_requests/ID`
- Check token has access to the project

## Critical Pitfalls

- Check System field to exclude bot messages
- Use `jq` for JSON parsing, never grep on raw API JSON
- Validate responses before processing (API returns 200 OK with errors)
- URL-encode nested groups (`%2F` for `/` in project paths)
- Re-fetch for time-sensitive analysis (comments can be edited/deleted)
- Respect Type distinction: DiffNote (code-level) vs comment (architectural)
- Handle UTC timestamps with proper timezone conversion
