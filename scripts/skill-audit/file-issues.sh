#!/usr/bin/env bash
#
# file-issues.sh
#
# For every skill the audit classified as known debt (graded below B, but not
# lowered by the pull request, see classify.sh), make sure one open GitHub issue
# tracks it, and link that issue to the pull request that found it. A skill the
# pull request made worse blocks instead and gets no issue: it is fixed in that
# pull request. Run by the Skill Audit workflow after the audit.
#
#   - An open issue is found by a marker in its body, or, for issues filed by
#     hand, by a title that starts "Raise <skill> to grade B". Closed issues are
#     ignored, so a skill that falls below B again gets a new one.
#   - If none exists, one is created. Its body names the pull request and the
#     workflow run, which puts the issue on the pull request's timeline.
#   - If one exists, it gets a single comment per pull request ("Also found on
#     #N"), never one per push.
#   - The issue numbers are written to $TRACKED_FILE as a Markdown list for the
#     Skill Audit comment to include.
#
# This never decides the gate (classify.sh does). A failure here exits non-zero
# after every skill has been tried, and the workflow step is continue-on-error.
#
# Usage: file-issues.sh <verdict.json>   (the output of classify.sh)
# Env:   GITHUB_REPOSITORY  owner/name (required)
#        PR_NUMBER          pull request number (required)
#        RUN_URL            link to the workflow run (optional)
#        TRACKED_FILE       output list (default: tracked-issues.md)
#        DRY_RUN=1          report what would happen, change nothing
set -uo pipefail

BATCH="${1:?usage: file-issues.sh <verdict.json>}"
REPO="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY must be set}"
PR="${PR_NUMBER:?PR_NUMBER must be set}"
RUN_URL="${RUN_URL:-}"
TRACKED_FILE="${TRACKED_FILE:-tracked-issues.md}"
DRY_RUN="${DRY_RUN:-0}"
BODY_FILE="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/skill-audit-issue-body.md"

: >"$TRACKED_FILE"

if [ ! -f "$BATCH" ]; then
    echo "No ${BATCH} found; nothing to file."
    exit 0
fi

# Skill names come from paths in the pull request, so only plain path segments
# are accepted; anything else is skipped rather than passed to gh.
valid_skill() {
    [[ "$1" =~ ^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$ ]]
}

FAILING="$(jq -c '
    [ .[] | select(.tier == "debt")
      | { skill, grade, total, maxTotal, dimensions, baseTotal } ]
' "$BATCH")"

if [ "$(jq 'length' <<<"$FAILING")" -eq 0 ]; then
    echo "No known-debt skill; no issues to file."
    exit 0
fi

OPEN_ISSUES="$(gh issue list --repo "$REPO" --state open --limit 500 --json number,title,body 2>/dev/null || echo '[]')"

# The three biggest losses, in the order the auditor's dimensions are named.
losses_table() {
    jq -r '
        { knowledgeDelta: ["D1 Knowledge Delta", 20],
          mindsetProcedures: ["D2 Mindset and Procedures", 15],
          antiPatternQuality: ["D3 Anti-Pattern Quality", 15],
          specificationCompliance: ["D4 Specification Compliance", 15],
          progressiveDisclosure: ["D5 Progressive Disclosure", 15],
          freedomCalibration: ["D6 Freedom Calibration", 15],
          patternRecognition: ["D7 Pattern Recognition", 10],
          practicalUsability: ["D8 Practical Usability", 15],
          evalValidation: ["D9 Eval Validation", 20] } as $names
        | [ .dimensions | to_entries[]
            | select($names[.key] != null)
            | { name: $names[.key][0], score: .value, max: $names[.key][1] }
            | select(.max > .score) ]
        | sort_by(-(.max - .score))
        | .[0:4][]
        | "| \(.name) | \(.score) | \(.max) | \(.max - .score) |"
    ' <<<"$1"
}

status=0
while IFS= read -r entry; do
    skill="$(jq -r '.skill' <<<"$entry")"
    grade="$(jq -r '.grade' <<<"$entry")"
    total="$(jq -r '.total' <<<"$entry")"
    max="$(jq -r '.maxTotal // 140' <<<"$entry")"

    if ! valid_skill "$skill"; then
        echo "::warning::Skipping a skill name that is not a plain path."
        continue
    fi

    marker="<!-- skill-audit-below-b: ${skill} -->"
    existing="$(jq -r --arg m "$marker" --arg p "Raise ${skill} to grade B" '
        [ .[]
          | select((.body // "" | contains($m))
                   or (.title | startswith($p + " ") or . == $p)) ]
        | sort_by(.number) | (.[0].number // empty)
    ' <<<"$OPEN_ISSUES")"

    if [ -n "$existing" ]; then
        linked="$(gh issue view "$existing" --repo "$REPO" --json body,comments 2>/dev/null \
            | jq -r --arg pr "#${PR}" '
                ([.body] + [.comments[].body])
                | any(test($pr + "([^0-9]|$)"))' 2>/dev/null || echo false)"
        if [ "$linked" != "true" ]; then
            if [ "$DRY_RUN" = "1" ]; then
                echo "would comment on #${existing} linking #${PR} (${skill})"
            else
                gh issue comment "$existing" --repo "$REPO" \
                    --body "Also found on #${PR}: \`${skill}\` is still graded ${grade} (${total}/${max}) there. That pull request did not lower its score, so it was not blocked." \
                    || status=1
            fi
        fi
        printf '%s\n' "- \`${skill}\`: #${existing}" >>"$TRACKED_FILE"
        continue
    fi

    title="Raise ${skill} to grade B (${total}/${max}, grade ${grade})"
    if [ "$DRY_RUN" = "1" ]; then
        echo "would create: ${title}"
        continue
    fi

    {
        echo "## What"
        echo ""
        echo "\`skills/${skill}\` grades **${grade} (${total}/${max})**, below the B (112/${max}) the Skill Audit expects."
        echo ""
        echo "Found by the Skill Audit on #${PR}${RUN_URL:+ ([run](${RUN_URL}))}, which did not lower its score and so was not blocked. A pull request that lowers this skill's score, or adds a skill below B, is blocked. Filed automatically; one open issue is kept per skill."
        echo ""
        echo "## Where the points are lost"
        echo ""
        echo "| Dimension | Score | Max | Lost |"
        echo "| --- | --- | --- | --- |"
        losses_table "$entry"
        echo ""
        echo "## Done when"
        echo ""
        echo "- \`pantheon-skill-auditor evaluate ${skill} --json --store\` reports 112 or more, and the refreshed stored audit is committed."
        echo "- Points are earned by real content. Do not add coverage figures or instruction inventories the evals do not support."
        echo ""
        echo "$marker"
    } >"$BODY_FILE"

    if url="$(gh issue create --repo "$REPO" --title "$title" --body-file "$BODY_FILE" --label enhancement)"; then
        number="${url##*/}"
        printf '%s\n' "- \`${skill}\`: #${number}" >>"$TRACKED_FILE"
        echo "Filed ${url}."
    else
        echo "::warning::Could not file an issue for ${skill}."
        status=1
    fi
done < <(jq -c '.[]' <<<"$FAILING")

exit "$status"
