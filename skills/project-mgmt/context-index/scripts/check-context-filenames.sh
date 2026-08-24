#!/usr/bin/env bash
# shell: bash
# check-context-filenames.sh - enforce the date-first filename convention
# create-context-file.sh writes: <root>/<typology>/<YYYY-MM-DD>-<slug>.md.
#
# Only files inside the nine curated typology directories (findings/, plans/,
# guides/, follow-ups/, merge-requests/, tickets/, decisions/, notes/,
# research/) are checked. A file living elsewhere under .context/ - a one-off
# --allow-new-type folder, or content belonging to an unrelated tool - is out
# of scope for this convention and is silently skipped, not flagged.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"

if [[ $# -gt 0 ]]; then
    files=("$@")
else
    files=()
    while IFS= read -r line; do
        files+=("$line")
    done < <(find "$ROOT/.context" -type f -name "*.md")
fi

date_first_re='^[0-9]{4}-[0-9]{2}-[0-9]{2}-[a-z0-9]+(-[a-z0-9]+)*\.md$'
known_dirs=(findings plans guides follow-ups merge-requests tickets decisions notes research)

errors=()

for f in "${files[@]}"; do
    [[ -f "$f" ]] || continue
    case "$f" in
        *.context/*.md) ;;
        *) continue ;;
    esac

    rel="${f#"$ROOT"/}"
    subdir="${rel#.context/}"
    subdir="${subdir%%/*}"
    base="$(basename "$f")"

    known=false
    for d in "${known_dirs[@]}"; do
        if [[ "$subdir" == "$d" ]]; then
            known=true
            break
        fi
    done
    [[ "$known" == false ]] && continue

    if [[ ! "$base" =~ $date_first_re ]]; then
        errors+=("$rel: expected YYYY-MM-DD-slug.md (date-first) for $subdir/")
        continue
    fi
    fn_date="${base:0:10}"

    fm_date=$(grep -m1 '^date:' "$f" | sed -E 's/^date: *"?([0-9-]+)"?.*/\1/')
    if [[ -n "$fm_date" && "$fm_date" != "$fn_date" ]]; then
        errors+=("$rel: filename date ($fn_date) does not match frontmatter date ($fm_date)")
    fi
done

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "ERROR: .context/ filenames violate naming convention:"
    for e in "${errors[@]}"; do
        echo "  $e"
    done
    printf '\nSee create-context-file/SKILL.md and references/typologies.md for the naming rule.\n'
    exit 1
fi

echo "context filenames OK"
