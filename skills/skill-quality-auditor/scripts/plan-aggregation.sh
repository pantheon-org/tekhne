#!/usr/bin/env sh
# Plan aggregation for a skill family
# Usage: sh plan-aggregation.sh --family <prefix>

set -e

FAMILY=""
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
TEMP_DIR=""

usage() {
    echo "Usage: sh plan-aggregation.sh --family <prefix>" >&2
    echo "Example: sh plan-aggregation.sh --family bdd" >&2
    exit 1
}

parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            --family)
                if [ -z "$2" ]; then
                    usage
                fi
                FAMILY="$2"
                shift 2
                ;;
            *)
                shift
                ;;
        esac
    done

    if [ -z "$FAMILY" ]; then
        usage
    fi
}

cleanup() {
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}

trap cleanup EXIT

get_description() {
    _file="$1"
    _desc=$(grep '^description:' "$_file" 2>/dev/null | head -1 | sed 's/^description:[[:space:]]*//')
    echo "$_desc"
}

get_line_count() {
    _file="$1"
    wc -l < "$_file" | tr -d ' '
}

find_skills_by_family() {
    _prefix="$1"
    _skills_dir="$2"
    _result_file="$3"

    if [ ! -d "$_skills_dir" ]; then
        return
    fi

    for _entry in "$_skills_dir"/*; do
        [ -d "$_entry" ] || continue
        _name=$(basename "$_entry")
        _skill_file="$_entry/SKILL.md"

        if [ -f "$_skill_file" ]; then
            case "$_name" in
                "${_prefix}"|"${_prefix}-"*)
                    _desc=$(get_description "$_skill_file")
                    _lines=$(get_line_count "$_skill_file")
                    echo "${_name}|${_skill_file}|${_lines}|${_desc}" >> "$_result_file"
                    ;;
            esac
        fi
    done
}

analyze_duplication() {
    _skills_file="$1"
    _skill_count=$(wc -l < "$_skills_file" | tr -d ' ')

    if [ "$_skill_count" -lt 2 ]; then
        echo "0"
        return
    fi

    _total_overlap=0
    _pair_count=0
    _seen_pairs="$TEMP_DIR/seen_pairs_$$_tmp"
    : > "$_seen_pairs"

    while IFS='|' read -r _name1 _path1 _lines1 _desc1; do
        _content1_file="$TEMP_DIR/content1_$$_tmp"
        grep -v '^[[:space:]]*$' "$_path1" 2>/dev/null | \
            awk 'length($0) > 10' | \
            tr '[:upper:]' '[:lower:]' | \
            sort -u > "$_content1_file" || true

        _size1=$(wc -l < "$_content1_file" | tr -d ' ')
        [ "$_size1" -eq 0 ] && continue

        while IFS='|' read -r _name2 _path2 _lines2 _desc2; do
            [ "$_name2" = "$_name1" ] && continue

            _pair_key=$(printf '%s|%s\n' "$_name1" "$_name2" | sort | tr '\n' '|')
            if grep -qF "$_pair_key" "$_seen_pairs" 2>/dev/null; then
                continue
            fi
            printf '%s\n' "$_pair_key" >> "$_seen_pairs"

            _content2_file="$TEMP_DIR/content2_$$_tmp"
            grep -v '^[[:space:]]*$' "$_path2" 2>/dev/null | \
                awk 'length($0) > 10' | \
                tr '[:upper:]' '[:lower:]' | \
                sort -u > "$_content2_file" || true

            _size2=$(wc -l < "$_content2_file" | tr -d ' ')
            [ "$_size2" -eq 0 ] && continue

            _intersection=$(comm -12 "$_content1_file" "$_content2_file" 2>/dev/null | wc -l | tr -d ' ')

            _avg_size=$(awk "BEGIN {printf \"%.0f\", ($_size1 + $_size2) / 2}")
            if [ "$_avg_size" -gt 0 ]; then
                _overlap_pct=$(awk "BEGIN {printf \"%.2f\", ($_intersection / $_avg_size) * 100}")
                _total_overlap=$(awk "BEGIN {printf \"%.2f\", $_total_overlap + $_overlap_pct}")
                _pair_count=$((_pair_count + 1))
            fi
        done < "$_skills_file"
    done < "$_skills_file"

    if [ "$_pair_count" -gt 0 ]; then
        awk "BEGIN {printf \"%.0f\", $_total_overlap / $_pair_count}"
    else
        echo "0"
    fi
}

propose_categories() {
    _skills_file="$1"
    _categories_file="$TEMP_DIR/categories_$$_tmp"
    : > "$_categories_file"

    while IFS='|' read -r _name _path _lines _desc; do
        _rest=$(echo "$_name" | sed 's/^[^-]*-//')
        if [ -n "$_rest" ] && [ "$_rest" != "$_name" ]; then
            _cat=$(echo "$_rest" | cut -d'-' -f1)
            if [ -n "$_cat" ]; then
                echo "$_cat" >> "$_categories_file"
            fi
        fi
    done < "$_skills_file"

    if [ ! -s "$_categories_file" ]; then
        echo "core advanced"
    else
        sort -u "$_categories_file" | tr '\n' ' ' | sed 's/ $//'
    fi
}

generate_plan() {
    _family="$1"
    _skills_file="$2"
    _duplication="$3"

    _total_lines=0
    while IFS='|' read -r _name _path _lines _desc; do
        _total_lines=$((_total_lines + _lines))
    done < "$_skills_file"

    _skill_count=$(wc -l < "$_skills_file" | tr -d ' ')
    _categories=$(propose_categories "$_skills_file")

    _estimated_hub_lines=65
    _estimated_refs=$(awk "BEGIN {printf \"%.0f\", $_total_lines / 350}")
    if [ "$_estimated_refs" -lt 1 ]; then
        _estimated_refs=1
    fi

    _estimated_new_total=$((_estimated_hub_lines + _estimated_refs * 200))
    if [ "$_total_lines" -gt 0 ]; then
        _reduction=$(awk "BEGIN {printf \"%.1f\", (1 - $_estimated_new_total / $_total_lines) * 100}")
    else
        _reduction="0.0"
    fi

    echo "# Aggregation Plan: $_family"
    echo ""
    echo "## Summary"
    echo ""
    echo "- **Source Skills**: $_skill_count"
    echo "- **Total Lines**: $_total_lines"
    echo "- **Estimated Hub**: ~$_estimated_hub_lines lines"
    echo "- **Estimated References**: ~$_estimated_refs files"
    echo "- **Estimated Reduction**: ${_reduction}%"
    echo ""
    echo "## Source Skills"
    echo ""
    echo "| Skill | Lines | Description |"
    echo "|-------|-------|-------------|"

    while IFS='|' read -r _name _path _lines _desc; do
        _short_desc=$(echo "$_desc" | cut -c1-50)
        if [ ${#_desc} -gt 50 ]; then
            _short_desc="${_short_desc}..."
        fi
        echo "| $_name | $_lines | $_short_desc |"
    done < "$_skills_file"

    echo ""
    echo "## Proposed Categories"
    echo ""
    echo "| Priority | Category | Prefix | Suggested Content |"
    echo "|----------|----------|--------|-------------------|"

    _priority_idx=0
    for _cat in $_categories; do
        case "$_priority_idx" in
            0) _priority="CRITICAL" ;;
            1) _priority="HIGH" ;;
            2) _priority="MEDIUM" ;;
            *) _priority="LOW" ;;
        esac
        echo "| $_priority | $_cat | $_cat- | TBD |"
        _priority_idx=$((_priority_idx + 1))
    done

    echo ""
    echo "## Implementation Steps"
    echo ""
    echo "1. **Design Structure** - Finalize category organization"
    echo "2. **Create Hub** - Write SKILL.md navigation (60-90 lines)"
    echo "3. **Create AGENTS.md** - Reference guide with file listing"
    echo "4. **Extract References** - Move content to categorized files"
    echo "5. **Deprecate Sources** - Move to .deprecated/"
    echo "6. **Verify** - Run skill-judge evaluation"
    echo ""
    echo "## Recommendations"
    echo ""

    _rec_num=1

    if [ "$_skill_count" -lt 3 ]; then
        echo "$_rec_num. Only 1-2 skills found - consider if aggregation is necessary"
        _rec_num=$((_rec_num + 1))
    elif [ "$_skill_count" -ge 6 ]; then
        echo "$_rec_num. $_skill_count skills found - strong candidate for aggregation"
        _rec_num=$((_rec_num + 1))
    fi

    if [ "$_duplication" -gt 30 ]; then
        echo "$_rec_num. High duplication (${_duplication}%) - high priority for consolidation"
        _rec_num=$((_rec_num + 1))
    elif [ "$_duplication" -gt 20 ]; then
        echo "$_rec_num. Moderate duplication (${_duplication}%) - good candidate for consolidation"
        _rec_num=$((_rec_num + 1))
    else
        echo "$_rec_num. Low duplication (${_duplication}%) - review if skills are truly related"
        _rec_num=$((_rec_num + 1))
    fi

    if [ "$_total_lines" -gt 2000 ]; then
        echo "$_rec_num. Large collection (${_total_lines} lines) - significant reduction potential"
        _rec_num=$((_rec_num + 1))
    fi

    echo "$_rec_num. Follow aggregation-implementation.md 6-step process"
    _rec_num=$((_rec_num + 1))
    echo "$_rec_num. Create navigation hub (SKILL.md) with priority categories"
    _rec_num=$((_rec_num + 1))
    echo "$_rec_num. Extract content to references/ directory by category"

    echo ""
    echo "## Next Actions"
    echo ""
    echo "- [ ] Review this plan with team"
    echo "- [ ] Refine category structure"
    echo "- [ ] Estimate effort (hours)"
    echo "- [ ] Schedule implementation"
    echo "- [ ] Execute aggregation-implementation.md"
    echo ""
    echo "---"
    echo ""
    echo "*Generated by skill-quality-auditor*"
}

main() {
    parse_args "$@"

    TEMP_DIR=$(mktemp -d)
    SKILLS_FILE="$TEMP_DIR/skills_$$_tmp"
    : > "$SKILLS_FILE"

    cd "$REPO_ROOT"

    for _dir in "skills" ".agents/skills"; do
        if [ -d "$_dir" ]; then
            find_skills_by_family "$FAMILY" "$_dir" "$SKILLS_FILE"
        fi
    done

    if [ ! -s "$SKILLS_FILE" ]; then
        echo "No skills found with prefix: $FAMILY" >&2
        exit 1
    fi

    sort -t'|' -k1 "$SKILLS_FILE" -o "$SKILLS_FILE"

    DUPLICATION=$(analyze_duplication "$SKILLS_FILE")
    generate_plan "$FAMILY" "$SKILLS_FILE" "$DUPLICATION"
}

main "$@"
