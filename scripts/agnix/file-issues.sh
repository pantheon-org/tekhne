#!/usr/bin/env bash
#
# file-issues.sh
#
# Keeps one GitHub issue open per agnix error, so a finding the agnix check
# reports is tracked without anyone filing it by hand. Run by the agnix workflow
# after the lint step; the step is continue-on-error, so it never blocks a pull
# request.
#
#   - Warnings never get an issue; only errors do.
#   - One issue per rule and file. An open issue is found by a marker in its
#     body, "<!-- agnix-finding: RULE FILE -->", or, for an issue filed by hand,
#     by the title "agnix RULE: FILE". A closed issue is ignored, so a finding
#     that comes back gets a new one.
#   - If none exists, one is created, labelled "bug". Its body names the pull
#     request (MODE=pr) or "main" (MODE=main) and the workflow run.
#   - If one exists, a pull request run adds a single comment per pull request
#     ("Also found on #N"), never one per push. A main run adds nothing.
#   - On a main run only, an issue carrying a marker whose finding agnix no
#     longer reports is closed with a comment. A pull request run never closes
#     anything: a branch that fixes a finding has not fixed main yet. Nothing is
#     closed unless agnix checked at least one file, so a broken run cannot close
#     every issue.
#   - After creating an issue, the open issues are listed again. If another run
#     created the same one first, the later number is closed as a duplicate.
#   - Values from agnix are validated as plain rule ids and paths before they go
#     into a title, a marker or a gh argument, and messages are HTML-escaped so
#     a finding cannot inject markup or mentions.
#
# The issue numbers are written to $TRACKED_FILE as a Markdown list for the
# agnix comment on the pull request to include.
#
# Usage: file-issues.sh <agnix.json>   (agnix --format json output)
# Env:   GITHUB_REPOSITORY  owner/name (required)
#        MODE               pr or main (required)
#        PR_NUMBER          pull request number (required when MODE=pr)
#        RUN_URL            link to the workflow run (optional)
#        TRACKED_FILE       output list (default: tracked-issues.md)
#        DRY_RUN=1          report what would happen, change nothing

set -uo pipefail

JSON="${1:-}"
REPO="${GITHUB_REPOSITORY:-}"
MODE="${MODE:-}"
PR="${PR_NUMBER:-}"
RUN_URL="${RUN_URL:-}"
TRACKED_FILE="${TRACKED_FILE:-tracked-issues.md}"
DRY_RUN="${DRY_RUN:-0}"

if [ -z "$JSON" ] || [ -z "$REPO" ]; then
    echo "usage: GITHUB_REPOSITORY=owner/name MODE=pr|main file-issues.sh <agnix.json>" >&2
    exit 2
fi

case "$MODE" in
    pr)
        if ! [[ "$PR" =~ ^[0-9]+$ ]]; then
            echo "::error::MODE=pr needs a numeric PR_NUMBER." >&2
            exit 2
        fi
        ;;
    main) ;;
    *)
        echo "::error::MODE must be pr or main." >&2
        exit 2
        ;;
esac

: >"$TRACKED_FILE"

BODY_FILE="$(mktemp)"
trap 'rm -f "$BODY_FILE"' EXIT

if ! jq -e '(.diagnostics | type == "array")
            and (.files_checked | type == "number")
            and (.summary.errors | type == "number")' "$JSON" >/dev/null 2>&1; then
    echo "::warning::No usable agnix output in ${JSON}; nothing to file or close."
    exit 0
fi

CAN_CLOSE=0
if [ "$MODE" = "main" ] && [ "$(jq '.files_checked' "$JSON")" -gt 0 ]; then
    CAN_CLOSE=1
fi

valid_rule() {
    [[ "$1" =~ ^[A-Z0-9]+(-[A-Z0-9]+)+$ ]]
}

# Plain path segments only; anything else is skipped rather than passed to gh.
valid_file() {
    [[ "$1" =~ ^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$ ]]
}

escape() {
    sed -e 's/&/\&amp;/g' -e 's/</\&lt;/g' -e 's/>/\&gt;/g' -e 's/`/\&#96;/g' -e 's/@/\&#64;/g'
}

marker() {
    printf '<!-- agnix-finding: %s %s -->' "$1" "$2"
}

# One entry per rule and file, with every line it was reported on. A runner path
# becomes a path under skills/.
ERRORS="$(jq -c '
    def plain: if startswith("/") then sub("^.*?/skills/"; "skills/") else ltrimstr("./") end;
    [ .diagnostics[] | select(.level == "error")
      | { rule, file: (.file | plain), line,
          message: ((.message // "") | gsub("[\n\r\t]+"; " ") | .[0:300]) } ]
    | group_by([.rule, .file])
    | map({ rule: .[0].rule, file: .[0].file,
            hits: map({ line, message }) })
' "$JSON")"

if [ "$(jq 'length' <<<"$ERRORS")" -eq 0 ] && [ "$CAN_CLOSE" -ne 1 ]; then
    echo "No agnix errors; no issues to file."
    exit 0
fi

if ! OPEN_ISSUES="$(gh issue list --repo "$REPO" --state open --limit 500 \
    --json number,title,body 2>/dev/null)"; then
    echo "::warning::Could not list the open issues; nothing filed or closed."
    exit 1
fi

existing_for() {
    jq -r --arg m "$(marker "$1" "$2")" --arg t "agnix $1: $2" '
        [ .[] | select((.body // "" | contains($m)) or .title == $t) ]
        | sort_by(.number) | (.[0].number // empty)
    ' <<<"$OPEN_ISSUES"
}

run_link() {
    if [ -n "$RUN_URL" ]; then
        printf ' ([run](%s))' "$RUN_URL"
    fi
}

status=0
while IFS= read -r entry; do
    rule="$(jq -r '.rule' <<<"$entry")"
    file="$(jq -r '.file' <<<"$entry")"

    if ! valid_rule "$rule" || ! valid_file "$file"; then
        echo "::warning::Skipping a finding whose rule or path is not plain."
        continue
    fi

    existing="$(existing_for "$rule" "$file")"

    if [ -n "$existing" ]; then
        if [ "$MODE" = "pr" ]; then
            linked="$(gh issue view "$existing" --repo "$REPO" --json body,comments </dev/null 2>/dev/null \
                | jq -r --arg pr "#${PR}" '
                    ([.body] + [.comments[].body])
                    | any(test($pr + "([^0-9]|$)"))' 2>/dev/null || echo false)"
            if [ "$linked" != "true" ]; then
                if [ "$DRY_RUN" = "1" ]; then
                    echo "would comment on #${existing} linking #${PR} (${rule} ${file})"
                else
                    gh issue comment "$existing" --repo "$REPO" \
                        --body "Also found on #${PR}: agnix still reports \`${rule}\` for \`${file}\` there. The agnix check does not block a pull request, so it was not held up." \
                        </dev/null || status=1
                fi
            fi
        fi
        printf '%s\n' "- \`${rule}\` in \`${file}\`: #${existing}" >>"$TRACKED_FILE"
        continue
    fi

    title="agnix ${rule}: ${file}"
    if [ "$DRY_RUN" = "1" ]; then
        echo "would create: ${title}"
        continue
    fi

    {
        echo "## What"
        echo ""
        echo "[agnix](https://github.com/agent-sh/agnix) reports \`${rule}\` for \`${file}\`:"
        echo ""
        jq -r '.hits[] | "- line \(.line): \(.message)"' <<<"$entry" | escape
        echo ""
        if [ "$MODE" = "pr" ]; then
            echo "Found by the agnix check on #${PR}$(run_link)."
        else
            echo "Found by the agnix check on \`main\`$(run_link)."
        fi
        echo "The check does not block a pull request. Filed automatically; one open issue is kept per rule and file, and it is closed automatically once \`main\` no longer has the finding."
        echo ""
        echo "## Done when"
        echo ""
        echo "- \`agnix -c .agnix.toml --format github skills\` no longer reports \`${rule}\` for \`${file}\`."
        echo ""
        marker "$rule" "$file"
        echo ""
    } >"$BODY_FILE"

    if url="$(gh issue create --repo "$REPO" --title "$title" --body-file "$BODY_FILE" --label bug </dev/null)"; then
        number="${url##*/}"
        if ! [[ "$number" =~ ^[0-9]+$ ]]; then
            echo "::warning::Filed an issue but could not read its number."
            status=1
            continue
        fi
        echo "Filed ${url}."

        # Another run may have filed the same finding at the same moment. The
        # lowest number wins; this run closes its own if it is the later one.
        first="$(gh issue list --repo "$REPO" --state open --limit 500 \
            --json number,title,body </dev/null 2>/dev/null \
            | jq -r --arg m "$(marker "$rule" "$file")" '
                [ .[] | select(.body // "" | contains($m)) | .number ] | min // empty
            ' 2>/dev/null || true)"
        if [[ "$first" =~ ^[0-9]+$ ]] && [ "$first" -lt "$number" ]; then
            gh issue close "$number" --repo "$REPO" \
                --comment "Duplicate of #${first}, filed by a run that overlapped this one." \
                </dev/null || status=1
            number="$first"
        fi
        printf '%s\n' "- \`${rule}\` in \`${file}\`: #${number}" >>"$TRACKED_FILE"
    else
        echo "::warning::Could not file an issue for a ${rule} finding."
        status=1
    fi
done < <(jq -c '.[]' <<<"$ERRORS")

if [ "$CAN_CLOSE" -eq 1 ]; then
    CURRENT="$(jq -r '.[] | "\(.rule) \(.file)"' <<<"$ERRORS")"
    while IFS=$'\t' read -r number rule file; do
        [ -n "$number" ] || continue
        if ! valid_rule "$rule" || ! valid_file "$file"; then
            continue
        fi
        if grep -qxF -- "${rule} ${file}" <<<"$CURRENT"; then
            continue
        fi
        if [ "$DRY_RUN" = "1" ]; then
            echo "would close #${number} (${rule} ${file})"
            continue
        fi
        gh issue close "$number" --repo "$REPO" \
            --comment "agnix no longer reports \`${rule}\` for \`${file}\` on \`main\`$(run_link). Closing automatically; a new issue is filed if the finding comes back." \
            </dev/null || status=1
    done < <(jq -r '
        .[]
        | select((.body // "") | test("<!-- agnix-finding: [^ ]+ [^ ]+ -->"))
        | (.body | capture("<!-- agnix-finding: (?<rule>[^ ]+) (?<file>[^ ]+) -->")) as $m
        | "\(.number)\t\($m.rule)\t\($m.file)"
    ' <<<"$OPEN_ISSUES")
fi

exit "$status"
