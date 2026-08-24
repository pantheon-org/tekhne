#!/usr/bin/env sh
#
# create-context-file.sh - create a context file under <root>/<typology>/
# using a date-prefixed filename: <YYYY-MM-DD>-<slug>.md
#
# The typology is an open, curated set (findings, plans, guides, follow-ups,
# merge-requests, tickets, ...). It maps directly to the subfolder name. The
# set is meant to stay mostly static; extend KNOWN_TYPES below when a genuinely
# new typology is needed, or pass --allow-new-type for a one-off.
#
# Files are written to <root>/<typology>/ where <root> defaults to .context in
# the CURRENT working directory (the consuming project), not the skill package.
#
# Usage:
#   create-context-file.sh --type <typology> --title "<title>" [options]
#   ... optional body content is read from stdin (e.g. via heredoc)
#
# Options:
#   -t, --type    TYPE     Typology / subfolder (required). See KNOWN_TYPES.
#   -T, --title   TITLE    Human-readable title (required).
#   -s, --slug    SLUG     Override the auto-generated slug (kebab-case).
#   -g, --tags    TAGS     Comma-separated tags (e.g. "auth,oauth").
#   -R, --related PATHS    Comma-separated relative paths to related .context
#                          files. Omitted entirely from frontmatter when empty
#                          -- never emitted as `related: []`.
#   -d, --date    DATE     Override the date (YYYY-MM-DD). Defaults to today.
#   -r, --root    DIR      Context root. Defaults to .context.
#   -A, --allow-new-type   Permit a typology not in KNOWN_TYPES.
#   -n, --dry-run          Print the target path and file body; write nothing.
#   -f, --force            Overwrite if the target file already exists.
#   -h, --help             Show this help.

set -eu

# Curated, mostly-static typology set. Extend deliberately.
KNOWN_TYPES="findings plans guides follow-ups merge-requests tickets decisions notes research"

die() { printf 'error: %s\n' "$1" >&2; exit 1; }

usage() { sed -n '2,/^# *-h/p' "$0" | sed 's/^#\{0,1\} \{0,1\}//'; }

# slugify: lowercase, keep alnum, collapse the rest to single hyphens, trim.
slugify() {
	printf '%s' "$1" \
		| tr '[:upper:]' '[:lower:]' \
		| sed 's/[^a-z0-9]\{1,\}/-/g; s/-\{1,\}/-/g; s/^-//; s/-$//'
}

type_known() {
	for _k in $KNOWN_TYPES; do
		[ "$_k" = "$1" ] && return 0
	done
	return 1
}

# The typology (and the subfolder it maps to) is plural, e.g. "follow-ups".
# The frontmatter `type:` field is the singular form, e.g. "follow-up" - that's
# what downstream consumers (regenerate-context-index.sh's type_group_key)
# match against. Curated pairs for KNOWN_TYPES; a typology added later via
# --allow-new-type falls back to stripping a trailing "s", which is correct
# for every typology in the set today.
singular_of() {
	case "$1" in
		findings)       echo "finding" ;;
		plans)          echo "plan" ;;
		guides)         echo "guide" ;;
		follow-ups)     echo "follow-up" ;;
		merge-requests) echo "merge-request" ;;
		tickets)        echo "ticket" ;;
		decisions)      echo "decision" ;;
		notes)          echo "note" ;;
		research)       echo "research" ;;
		*)              printf '%s' "$1" | sed 's/s$//' ;;
	esac
}

type=""
title=""
slug=""
tags=""
related=""
date=""
root=".context"
allow_new=0
dry_run=0
force=0

while [ $# -gt 0 ]; do
	case "$1" in
		-t|--type)           type="${2:-}"; shift 2 ;;
		-T|--title)          title="${2:-}"; shift 2 ;;
		-s|--slug)           slug="${2:-}"; shift 2 ;;
		-g|--tags)           tags="${2:-}"; shift 2 ;;
		-R|--related)        related="${2:-}"; shift 2 ;;
		-d|--date)           date="${2:-}"; shift 2 ;;
		-r|--root)           root="${2:-}"; shift 2 ;;
		-A|--allow-new-type) allow_new=1; shift ;;
		-n|--dry-run)        dry_run=1; shift ;;
		-f|--force)          force=1; shift ;;
		-h|--help)           usage; exit 0 ;;
		*)                   die "unknown argument: $1 (see --help)" ;;
	esac
done

[ -n "$type" ]  || die "--type is required (see --help)"
[ -n "$title" ] || die "--title is required (see --help)"

type="$(slugify "$type")"
[ -n "$type" ] || die "--type produced an empty typology"

if ! type_known "$type" && [ "$allow_new" -ne 1 ]; then
	die "unknown typology '$type'. Known: ${KNOWN_TYPES}.
Add it to KNOWN_TYPES in this script if it should be permanent, or pass --allow-new-type for a one-off."
fi

date="${date:-$(date +%Y-%m-%d)}"
case "$date" in
	[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]) ;;
	*) die "--date must be YYYY-MM-DD (got: $date)" ;;
esac

[ -n "$slug" ] || slug="$(slugify "$title")"
slug="$(slugify "$slug")"
[ -n "$slug" ] || die "could not derive a slug; pass --slug"

dir="${root%/}/${type}"
path="${dir}/${date}-${slug}.md"

# Body content from stdin when piped (leaves body empty for interactive runs).
content=""
if [ ! -t 0 ]; then
	content="$(cat)"
fi

# Build tags block.
if [ -n "$tags" ]; then
	tags_block="tags:"
	_ifs="$IFS"; IFS=','
	for _tag in $tags; do
		_tag="$(printf '%s' "$_tag" | sed 's/^[[:space:]]\{1,\}//; s/[[:space:]]\{1,\}$//')"
		[ -n "$_tag" ] && tags_block="${tags_block}
  - ${_tag}"
	done
	IFS="$_ifs"
else
	tags_block="tags: []"
fi

# Build related block. Unlike tags, an empty related list is omitted
# entirely from the frontmatter -- never emitted as `related: []`.
related_block=""
if [ -n "$related" ]; then
	_ifs="$IFS"; IFS=','
	for _rel in $related; do
		_rel="$(printf '%s' "$_rel" | sed 's/^[[:space:]]\{1,\}//; s/[[:space:]]\{1,\}$//')"
		if [ -n "$_rel" ]; then
			if [ -z "$related_block" ]; then
				related_block="related:
  - ${_rel}"
			else
				related_block="${related_block}
  - ${_rel}"
			fi
		fi
	done
	IFS="$_ifs"
fi

body="---
title: \"${title}\"
type: $(singular_of "$type")
date: ${date}
status: active
${tags_block}"

if [ -n "$related_block" ]; then
	body="${body}
${related_block}"
fi

body="${body}
---

# ${title}
"
[ -n "$content" ] && body="${body}
${content}
"

if [ "$dry_run" -eq 1 ]; then
	printf 'path: %s\n\n%s' "$path" "$body"
	exit 0
fi

[ -e "$path" ] && [ "$force" -ne 1 ] \
	&& die "refusing to overwrite existing file: $path (use --force)"

mkdir -p "$dir"
printf '%s' "$body" > "$path"
printf '%s\n' "$path"
