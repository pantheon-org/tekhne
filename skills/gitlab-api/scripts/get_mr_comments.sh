#!/usr/bin/env bash
# shell: bash

set -euo pipefail

MR_URL="${1:-}"

if [[ -z "$MR_URL" ]]; then
    echo "Usage: $0 <merge_request_url>"
    echo "Example: $0 https://gitlab.com/your-group/your-project/-/merge_requests/123"
    echo ""
    echo "Requires GITLAB_TOKEN environment variable or GITLAB_PAT"
    exit 1
fi

GITLAB_TOKEN="${GITLAB_TOKEN:-${GITLAB_PAT:-}}"

if [[ -z "$GITLAB_TOKEN" ]]; then
    echo "Error: GITLAB_TOKEN or GITLAB_PAT environment variable required"
    exit 1
fi

if [[ ! "$MR_URL" =~ ^https://gitlab\.com/(.+)/-/merge_requests/([0-9]+) ]]; then
    echo "Error: Invalid GitLab MR URL format"
    echo "Expected: https://gitlab.com/group/subgroup/project/-/merge_requests/ID"
    exit 1
fi

PROJECT_PATH="${BASH_REMATCH[1]}"
MR_ID="${BASH_REMATCH[2]}"
# URL encode the project path properly (replace / with %2F)
PROJECT_PATH_ENCODED=$(echo -n "$PROJECT_PATH" | jq -sRr @uri | tr -d '\n')

API_URL="https://gitlab.com/api/v4/projects/${PROJECT_PATH_ENCODED}/merge_requests/${MR_ID}/notes"

echo "Fetching comments for MR #${MR_ID} in ${PROJECT_PATH}..."
echo ""

RESPONSE=$(curl -s -H "PRIVATE-TOKEN: ${GITLAB_TOKEN}" "$API_URL")

# Check if response is an error
if echo "$RESPONSE" | jq -e '.message' > /dev/null 2>&1; then
    echo "API Error: $(echo "$RESPONSE" | jq -r '.message')"
    exit 1
fi

# Parse and display comments
echo "$RESPONSE" | jq -r '.[] | "---\nAuthor: \(.author.name) (@\(.author.username))\nDate: \(.created_at)\nType: \(.type // "comment")\nSystem: \(.system)\n\n\(.body)\n"'
