---
name: gitlab-api
description: Fetches and analyzes GitLab merge request (MR) comments, metadata, and review feedback using authenticated API calls. Capabilities include fetching comment threads, retrieving reviewer feedback, filtering unresolved discussions, and generating MR activity reports. Use when the user asks about GitLab MR comments, code review discussions, review feedback, approval status, troubleshooting review delays, or needs to fetch merge request metadata via the GitLab API.
---

# GitLab API Integration

Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls.

## Mindset

Every GitLab MR comment fetch is a **snapshot of a moving target**, not a stable record. Comments get edited, resolved, and deleted between when a review happens and when it's analyzed, and the GitLab API returns `200 OK` even when the payload inside is an error object — HTTP status alone does not mean "this data is valid." Treat every fetch as provisional: validate the shape of what came back before trusting it, and re-fetch rather than cache when the analysis is time-sensitive (an "unresolved discussion" report generated from an hour-old cache is actively misleading, not just stale).

The second load-bearing habit: `System` and `Type` are two independent axes, not one. `System: true/false` answers "who/what generated this" (bot vs human); `Type: DiffNote/comment` answers "where does this comment live" (inline on a diff line vs a general MR-level remark). A report that conflates them — e.g. treating all `DiffNote`s as human or all `comment`s as automated — will misclassify real feedback.

## When to Use

- Fetching, filtering, or summarizing GitLab MR comment threads, reviewer feedback, or approval/discussion status via the API
- Building a script or report that needs to distinguish human review feedback from automated/bot comments, or inline code comments from general discussion
- Diagnosing why an MR API call fails (401/404, malformed URL, missing scope)

## When NOT to Use

- GitLab CI/CD pipeline configuration or `.gitlab-ci.yml` authoring — unrelated to the MR comments API this skill wraps
- Working with GitLab issues, epics, or wikis — this skill's script and output format are scoped to merge request discussions only

## Prerequisites

- Set `GITLAB_TOKEN` or `GITLAB_PAT` with `read_api` scope
- Requires: `curl`, `jq`

## Available Scripts

### get_mr_comments.sh

Fetches all comments from a GitLab merge request. Run it relative to this skill's own directory:

```bash
scripts/get_mr_comments.sh <merge_request_url>
```

**Example:**
```bash
# Prerequisites assumed: GITLAB_TOKEN with read_api scope
scripts/get_mr_comments.sh "https://gitlab.com/your-group/your-project/-/merge_requests/123"
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

Read script source (`scripts/get_mr_comments.sh`, ~80 lines) when debugging unexpected output, extending for custom metadata, or understanding URL encoding. For basic usage, the examples above suffice.

## Common Workflows and Error Handling

See [`references/workflows-and-error-handling.md`](references/workflows-and-error-handling.md) for the step-by-step review-summary and progress-tracking workflows, plus 401/404/200-with-error-body diagnosis.

## Anti-Patterns

### NEVER hardcode personal access tokens in scripts

**WHY:** PATs stored in source code are exposed in git history even after removal and create a permanent security risk — anyone with repo access, at any point in the future, can recover a token committed once.

❌ BAD:
```bash
GITLAB_TOKEN="glpat-xxxxxxxxxxxxxxxxxxxx"   # inline in a script committed to version control
```

✅ GOOD:
```bash
GITLAB_TOKEN="${GITLAB_TOKEN:?Set GITLAB_TOKEN with read_api scope}"
```

**Consequence:** A leaked PAT grants `read_api` access to every project the token owner can see, not just the one repo it was committed to.

### NEVER assume a single page has all the results

**WHY:** The API always paginates; requesting `per_page=100` does not guarantee the result set fits on one page, and both under- and over-estimating the true count silently truncate downstream analysis.

❌ BAD:
```bash
curl "$API_URL/merge_requests?per_page=100"   # assumes this is everything
```

✅ GOOD:
```bash
page=1
while :; do
  resp=$(curl -sD headers.txt "$API_URL/merge_requests?per_page=100&page=$page")
  # process $resp
  next=$(grep -i '^x-next-page:' headers.txt | tr -d '\r' | cut -d' ' -f2)
  [ -z "$next" ] && break
  page=$next
done
```

**Consequence:** Reports built on the first page only silently omit older or later comments/MRs, and nothing in the response signals that truncation happened — it looks like a complete, valid result.

### NEVER call the GitLab API without respecting rate limits

**WHY:** The API enforces rate limits (typically 2000 req/min for REST); bulk operations without backoff must always check for `429` responses rather than assuming every request succeeds.

❌ BAD: firing parallel bulk API calls with no retry logic.

✅ GOOD: check for `429` status codes and implement exponential backoff, honoring the `Retry-After` header when the API provides one.

**Consequence:** A burst of unthrottled calls gets throttled mid-run, and any call that doesn't check its own status code silently treats a `429` error body as if it were real data.

### NEVER use the API `v3` endpoint path

**WHY:** GitLab removed API v3 in GitLab 11.0; all integrations must use `v4`.

❌ BAD:
```
https://gitlab.example.com/api/v3/projects/
```

✅ GOOD:
```
https://gitlab.example.com/api/v4/projects/
```

**Consequence:** `v3` endpoints return 404 or are silently redirected on current GitLab instances, producing confusing failures that look like an auth or URL problem rather than a versioning one.

## Critical Pitfalls

- Check `System` to exclude bot messages — it is independent of `Type`, so don't conflate the two axes (see Mindset)
- Use `jq` for JSON parsing; never grep on raw API JSON
- Validate responses before processing — the API can return `200 OK` with an error body
- URL-encode nested groups (`%2F` for `/` in project paths)
- Re-fetch for time-sensitive analysis — comments can be edited or deleted between fetches
- Handle UTC timestamps with proper timezone conversion when displaying to users in another zone

## Eval Scenarios

- [Scenario 1: Generate MR review summary](evals/scenario-1/task.md)
- [Scenario 2: Monitor MR review activity](evals/scenario-2/task.md)
- [Scenario 3: Diagnostic tool for MR access issues](evals/scenario-3/task.md)
- [Scenario 4: Separate code review from discussion comments](evals/scenario-4/task.md)
- [Scenario 5: GitLab MR integration starter kit](evals/scenario-5/task.md)

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Step-by-step review-summary/progress-tracking workflows and 401/404/200-with-error-body diagnosis | [Workflows and Error Handling](references/workflows-and-error-handling.md) | Building a report/monitoring script, or diagnosing a failed API call |
