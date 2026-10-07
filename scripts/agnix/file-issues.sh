#!/usr/bin/env bash
#
# file-issues.sh
#
# Keeps GitHub issues open for what the agnix check reports, so a finding is
# tracked without anyone filing it by hand. Run by the agnix workflow after the
# lint step; the step is continue-on-error, so filing never blocks a pull
# request. Whether a pull request fails is decided separately, by
# introduced-errors.sh.
#
# Errors: one issue per rule and file, labelled "bug".
#   - An open issue is found by a marker in its body,
#     "<!-- agnix-finding: RULE FILE -->", or, for an issue filed by hand, by the
#     title "agnix RULE: FILE". A closed issue is ignored, so a finding that
#     comes back gets a new one.
#   - If none exists, one is created. Its body names the pull request (MODE=pr)
#     or "main" (MODE=main) and the workflow run.
#   - An existing issue is left alone: no comment is added for each pull request.
#
# Warnings: one issue per rule, labelled "enhancement", listing every file and
# line the rule is reported on (the first 100 files).
#   - Found by the marker "<!-- agnix-warnings: RULE -->" or the title
#     "agnix warnings: RULE".
#   - The body is deterministic (no run link, no pull request), so a main run can
#     compare it and refresh the issue with "gh issue edit" only when the file
#     list changed. A pull request run never edits an issue.
#
# Closing, on a main run only: an error issue whose finding agnix no longer
# reports, and a warnings issue whose rule no file reports any more, is closed
# with a comment. A pull request run never closes anything: a branch that fixes a
# finding has not fixed main yet. Nothing is closed unless agnix checked at least
# one file, so a broken run cannot close every issue.
#
# After creating an issue, the open issues are listed again. If another run
# created the same one first, the later number is closed as a duplicate.
#
# Values from agnix are validated as plain rule ids and paths before they go into
# a title, a marker or a gh argument, and messages are HTML-escaped so a finding
# cannot inject markup or mentions.
#
# Known limits, accepted for now:
#   - Open issues are listed 500 at a time. With more than 500 open, a marker
#     past the first 500 is not seen: a duplicate could be filed and a fixed
#     finding left open. Open issues were 26 on 07-10-2026.
#   - Two runs creating the same issue at the same moment are reconciled after
#     the fact (the later is closed as a duplicate), not prevented.
#   - A rule, file or message agnix reports that fails the plain-path check is
#     skipped with a warning and gets no issue.
#   - A warnings issue lists at most 100 files; the rest are counted, not named.
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
MAX_FILES=100

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

warnings_marker() {
    printf '<!-- agnix-warnings: %s -->' "$1"
}

# A runner path becomes a path under skills/.
JQ_PLAIN='def plain: if startswith("/") then sub("^.*?/skills/"; "skills/") else ltrimstr("./") end;'

# Errors: one entry per rule and file, with every line it was reported on.
ERRORS="$(jq -c "${JQ_PLAIN}"'
    [ .diagnostics[] | select(.level == "error")
      | { rule, file: (.file | plain), line,
          message: ((.message // "") | gsub("[\n\r\t]+"; " ") | .[0:300]) } ]
    | group_by([.rule, .file])
    | map({ rule: .[0].rule, file: .[0].file,
            hits: map({ line, message }) })
' "$JSON")"

# Warnings: one entry per rule, with every file and the lines in it. Findings
# whose rule or path is not plain are dropped here and counted.
WARNING_PATTERN_RULE='^[A-Z0-9]+(-[A-Z0-9]+)+$'
WARNING_PATTERN_FILE='^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$'
ALL_WARNINGS="$(jq -c "${JQ_PLAIN}"'
    [ .diagnostics[] | select(.level == "warning")
      | { rule, file: (.file | plain), line,
          message: ((.message // "") | gsub("[\n\r\t]+"; " ") | .[0:300]) } ]
' "$JSON")"
SKIPPED_WARNINGS="$(jq --arg r "$WARNING_PATTERN_RULE" --arg f "$WARNING_PATTERN_FILE" \
    '[ .[] | select((.rule | test($r) | not) or (.file | test($f) | not)) ] | length' <<<"$ALL_WARNINGS")"
if [ "$SKIPPED_WARNINGS" -gt 0 ]; then
    echo "::warning::Skipping ${SKIPPED_WARNINGS} warning(s) whose rule or path is not plain."
fi
WARNINGS="$(jq -c --arg r "$WARNING_PATTERN_RULE" --arg f "$WARNING_PATTERN_FILE" '
    map(select((.rule | test($r)) and (.file | test($f))))
    | group_by(.rule)
    | map({ rule: .[0].rule,
            messages: ([ .[].message ] | unique | .[0:5]),
            files: (group_by(.file)
                    | map({ file: .[0].file, lines: ([ .[].line ] | unique) })) })
' <<<"$ALL_WARNINGS")"

if [ "$(jq 'length' <<<"$ERRORS")" -eq 0 ] && [ "$(jq 'length' <<<"$WARNINGS")" -eq 0 ] \
    && [ "$CAN_CLOSE" -ne 1 ]; then
    echo "No agnix findings; no issues to file."
    exit 0
fi

if ! OPEN_ISSUES="$(gh issue list --repo "$REPO" --state open --limit 500 \
    --json number,title,body 2>/dev/null)"; then
    echo "::warning::Could not list the open issues; nothing filed or closed."
    exit 1
fi

# existing_for <marker> <title>: the lowest open issue number carrying either.
existing_for() {
    jq -r --arg m "$1" --arg t "$2" '
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
CREATED=""

# create_issue <title> <marker> <label>: creates an issue from $BODY_FILE and sets
# CREATED to the surviving number (this run's, or a lower one from an overlapping
# run). CREATED stays empty if nothing was filed.
create_issue() {
    local title="$1" mark="$2" label="$3" url number first
    CREATED=""
    if url="$(gh issue create --repo "$REPO" --title "$title" --body-file "$BODY_FILE" --label "$label" </dev/null)"; then
        number="${url##*/}"
        if ! [[ "$number" =~ ^[0-9]+$ ]]; then
            echo "::warning::Filed an issue but could not read its number."
            status=1
            return
        fi
        echo "Filed ${url}."

        # Another run may have filed the same finding at the same moment. The
        # lowest number wins; this run closes its own if it is the later one.
        first="$(gh issue list --repo "$REPO" --state open --limit 500 \
            --json number,title,body </dev/null 2>/dev/null \
            | jq -r --arg m "$mark" '
                [ .[] | select(.body // "" | contains($m)) | .number ] | min // empty
            ' 2>/dev/null || true)"
        if [[ "$first" =~ ^[0-9]+$ ]] && [ "$first" -lt "$number" ]; then
            gh issue close "$number" --repo "$REPO" \
                --comment "Duplicate of #${first}, filed by a run that overlapped this one." \
                </dev/null || status=1
            number="$first"
        fi
        CREATED="$number"
    else
        echo "::warning::Could not file an issue: ${title}."
        status=1
    fi
}

# --- Errors ---------------------------------------------------------------

while IFS= read -r entry; do
    [ -n "$entry" ] || continue
    rule="$(jq -r '.rule' <<<"$entry")"
    file="$(jq -r '.file' <<<"$entry")"

    if ! valid_rule "$rule" || ! valid_file "$file"; then
        echo "::warning::Skipping a finding whose rule or path is not plain."
        continue
    fi

    title="agnix ${rule}: ${file}"
    existing="$(existing_for "$(marker "$rule" "$file")" "$title")"

    if [ -n "$existing" ]; then
        printf '%s\n' "- \`${rule}\` in \`${file}\`: #${existing}" >>"$TRACKED_FILE"
        continue
    fi

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
        echo "A pull request fails the check only for an error it introduces, so this does not hold up other work. Filed automatically; one open issue is kept per rule and file, and it is closed automatically once \`main\` no longer has the finding."
        echo ""
        echo "## Done when"
        echo ""
        echo "- \`agnix -c .agnix.toml --format github skills\` no longer reports \`${rule}\` for \`${file}\`."
        echo ""
        marker "$rule" "$file"
        echo ""
    } >"$BODY_FILE"

    create_issue "$title" "$(marker "$rule" "$file")" bug
    if [ -n "$CREATED" ]; then
        printf '%s\n' "- \`${rule}\` in \`${file}\`: #${CREATED}" >>"$TRACKED_FILE"
    fi
done < <(jq -c '.[]' <<<"$ERRORS")

# --- Warnings -------------------------------------------------------------

# write_warnings_body <entry>: the body of a warnings issue, with no run link or
# pull request so that two runs over the same findings produce the same text.
write_warnings_body() {
    local entry="$1" rule total
    rule="$(jq -r '.rule' <<<"$entry")"
    total="$(jq '.files | length' <<<"$entry")"
    {
        echo "## What"
        echo ""
        if [ "$total" -eq 1 ]; then
            echo "[agnix](https://github.com/agent-sh/agnix) reports warnings for \`${rule}\` in 1 file:"
        else
            echo "[agnix](https://github.com/agent-sh/agnix) reports warnings for \`${rule}\` in ${total} files:"
        fi
        echo ""
        jq -r --argjson max "$MAX_FILES" '
            .files[0:$max][]
            | "- `\(.file)`: " + (if (.lines | length) == 1
                                    then "line \(.lines[0])"
                                    else "lines \(.lines | map(tostring) | join(", "))" end)
        ' <<<"$entry"
        if [ "$total" -gt "$MAX_FILES" ]; then
            echo "- and $((total - MAX_FILES)) more files"
        fi
        echo ""
        echo "Messages:"
        echo ""
        jq -r '.messages[] | "- \(.)"' <<<"$entry" | escape
        echo ""
        echo "Warnings never fail a pull request. Filed automatically; one open issue is kept per rule. It is refreshed when the file list on \`main\` changes and closed automatically once no file has the rule."
        echo ""
        echo "## Done when"
        echo ""
        echo "- \`agnix -c .agnix.toml --format github skills\` reports no \`${rule}\` warnings."
        echo ""
        warnings_marker "$rule"
        echo ""
    } >"$BODY_FILE"
}

while IFS= read -r entry; do
    [ -n "$entry" ] || continue
    rule="$(jq -r '.rule' <<<"$entry")"
    count="$(jq '.files | length' <<<"$entry")"
    plural="files"
    if [ "$count" -eq 1 ]; then plural="file"; fi
    title="agnix warnings: ${rule}"
    mark="$(warnings_marker "$rule")"
    existing="$(existing_for "$mark" "$title")"

    write_warnings_body "$entry"

    if [ -n "$existing" ]; then
        printf '%s\n' "- warnings \`${rule}\` in ${count} ${plural}: #${existing}" >>"$TRACKED_FILE"
        if [ "$MODE" = "main" ]; then
            current="$(jq -r --argjson n "$existing" \
                '.[] | select(.number == $n) | (.body // "") | gsub("\r"; "")' <<<"$OPEN_ISSUES")"
            if [ "$current" != "$(cat "$BODY_FILE")" ]; then
                if [ "$DRY_RUN" = "1" ]; then
                    echo "would update #${existing} (${title})"
                else
                    gh issue edit "$existing" --repo "$REPO" --body-file "$BODY_FILE" \
                        </dev/null || status=1
                fi
            fi
        fi
        continue
    fi

    if [ "$DRY_RUN" = "1" ]; then
        echo "would create: ${title}"
        continue
    fi

    create_issue "$title" "$mark" enhancement
    if [ -n "$CREATED" ]; then
        printf '%s\n' "- warnings \`${rule}\` in ${count} ${plural}: #${CREATED}" >>"$TRACKED_FILE"
    fi
done < <(jq -c '.[]' <<<"$WARNINGS")

# --- Closing (main only) --------------------------------------------------

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

    CURRENT_RULES="$(jq -r '.[].rule' <<<"$WARNINGS")"
    while IFS=$'\t' read -r number rule; do
        [ -n "$number" ] || continue
        if ! valid_rule "$rule"; then
            continue
        fi
        if grep -qxF -- "$rule" <<<"$CURRENT_RULES"; then
            continue
        fi
        if [ "$DRY_RUN" = "1" ]; then
            echo "would close #${number} (warnings ${rule})"
            continue
        fi
        gh issue close "$number" --repo "$REPO" \
            --comment "agnix no longer reports \`${rule}\` warnings on \`main\`$(run_link). Closing automatically; a new issue is filed if they come back." \
            </dev/null || status=1
    done < <(jq -r '
        .[]
        | select((.body // "") | test("<!-- agnix-warnings: [^ ]+ -->"))
        | (.body | capture("<!-- agnix-warnings: (?<rule>[^ ]+) -->")) as $m
        | "\(.number)\t\($m.rule)"
    ' <<<"$OPEN_ISSUES")
fi

exit "$status"
