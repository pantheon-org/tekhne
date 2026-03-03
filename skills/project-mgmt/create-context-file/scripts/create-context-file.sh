#!/usr/bin/env sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

adjectives="happy blue quick smart green fast red cool warm bright"
nouns="moon sun star planet comet galaxy cloud ocean forest mountain"

slugify() {
	printf '%s' "$1" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 -]//g' | sed 's/[[:space:]]/-/g' | sed 's/--*/-/g' | sed 's/^-//;s/-$//'
}

random_index() {
	_max="$1"
	_seed=$(date +%s 2>/dev/null || echo $$)
	_rand=$(( (_seed * 1664525 + 1013904223) % 2147483647 ))
	echo $(( _rand % _max ))
}

pick_word() {
	_words="$1"
	_count="$2"
	_idx=$(random_index "$_count")
	echo "$_words" | cut -d' ' -f$((_idx + 1))
}

generate_three_word_id() {
	_adj_count=$(echo "$adjectives" | wc -w | tr -d ' ')
	_noun_count=$(echo "$nouns" | wc -w | tr -d ' ')
	
	_adj=$(pick_word "$adjectives" "$_adj_count")
	_noun=$(pick_word "$nouns" "$_noun_count")
	_num=$(random_index 1000)
	
	printf '%s-%s-%d' "$_adj" "$_noun" "$_num"
}

title_case() {
	_slug="$1"
	_result=""
	IFS='-'
	for _word in $_slug; do
		_first=$(printf '%s' "$_word" | cut -c1 | tr '[:lower:]' '[:upper:]')
		_rest=$(printf '%s' "$_word" | cut -c2-)
		if [ -n "$_result" ]; then
			_result="$_result "
		fi
		_result="${_result}${_first}${_rest}"
	done
	unset IFS
	printf '%s' "$_result"
}

usage() {
	cat <<EOF
Usage: $(basename "$0") --type [plan|justification|scratch] "slug" "content"

Examples:
  $(basename "$0") --type plan "feature-name" "Plan content here"
  $(basename "$0") --type justification "decision-name" "Justification content here"
  $(basename "$0") --type scratch "note-name" "Scratch content here"
EOF
	exit 1
}

if [ $# -lt 4 ]; then
	usage
fi

if [ "$1" != "--type" ]; then
	echo "Error: --type flag is required"
	usage
fi

_type="$2"
_slug="$3"
shift 3
_content="$*"

case "$_type" in
	plan|justification|scratch) ;;
	*)
		echo "Error: Invalid type. Must be one of: plan, justification, scratch"
		exit 1
		;;
esac

if [ -z "$_slug" ] || [ -z "$_content" ]; then
	echo "Error: Both slug and content are required"
	exit 1
fi

_three_word_id=$(generate_three_word_id)
_formatted_slug=$(slugify "$_slug")
_file_name="${_three_word_id}-${_formatted_slug}.md"

case "$_type" in
	plan) _target_dir="plans" ;;
	justification) _target_dir="justifications" ;;
	scratch) _target_dir="scratches" ;;
esac

_target_path="${SCRIPT_DIR}/../.context/${_target_dir}/${_file_name}"
_date=$(date +%Y-%m-%d)
_title=$(title_case "$_slug")

_frontmatter="---
date: ${_date}
title: ${_title}
---"

_full_content="${_frontmatter}

${_content}"

mkdir -p "${SCRIPT_DIR}/../.context/${_target_dir}"
printf '%s\n' "$_full_content" > "$_target_path"

echo "Created $_type file: $_target_path"
echo "Three-word ID: $_three_word_id"
