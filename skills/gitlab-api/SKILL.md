---
name: gitlab-api
description: Fetches and analyzes GitLab merge request (MR) comments, metadata, and review feedback using authenticated API calls. Capabilities include fetching comment threads, retrieving reviewer feedback, filtering unresolved discussions, and generating MR activity reports. Use when the user asks about GitLab MR comments, code review discussions, review feedback, approval status, troubleshooting review delays, or needs to fetch merge request metadata via the GitLab API.
version: 0.2.1
tags:
  - gitlab
  - api
  - merge-requests
  - code-review
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

**DO NOT read script source** for:
- Basic usage (fetching comments, standard workflows)
- Standard error handling (401, 404)

**READ script source** when:
- Debugging unexpected output format
- Extending script for custom metadata
- Understanding URL encoding for nested groups

Script location: `.agents/skills/gitlab-api/scripts/get_mr_comments.sh` (~80 lines)

## Before Analyzing MR Comments

Ask yourself:
- **Purpose**: What insight am I trying to extract? (review summary, blockers, team dynamics)
- **Signal**: What differentiates valuable feedback from noise? (System: false, DiffNote vs comment)
- **Temporal**: Does recency matter, or full history? (timestamps, recent activity)
- **Completeness**: Am I filtering out important automated messages? (CI failures, merge conflicts)

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

**Adapting Workflows**: These patterns are templates. Modify validation checks, add filtering steps, or combine workflows based on your specific analysis needs.

## Error Handling

**401 Unauthorized**: Token missing or invalid
- Check `GITLAB_TOKEN` or `GITLAB_PAT` is set
- Verify token has API read scope

**404 Project Not Found**: Invalid project path
- Confirm URL format: `https://gitlab.com/group/project/-/merge_requests/ID`
- Check token has access to the project

## NEVER Do When Working with GitLab MR Data

- **NEVER parse comment output without checking System field** — automated bot messages will pollute your analysis with CI notifications, merge conflict warnings, and automated code quality reports
- **NEVER assume Type: comment means low priority** — critical architectural feedback and blocking concerns often appear in general comments rather than inline code reviews
- **NEVER use grep on raw API JSON responses** — field order isn't guaranteed by the API spec; always use `jq` for JSON parsing to ensure reliable field extraction
- **NEVER skip response validation** — GitLab API can return 200 OK with error messages embedded in the response body; always check for expected fields like "Author:" before processing
- **NEVER hardcode project paths in URLs** — nested groups require URL encoding (replace `/` with `%2F`), otherwise the API returns 404 even for valid projects
- **NEVER cache MR data without considering staleness** — comments can be edited, deleted, or have their resolved status changed; always re-fetch for accuracy in time-sensitive analysis
- **NEVER ignore DiffNote vs comment distinction** — treating all feedback equally misses the critical difference between code-level technical debt and high-level architectural discussions
- **NEVER process ISO8601 timestamps without timezone awareness** — all timestamps are UTC; converting to local time without proper handling causes incorrect temporal analysis
