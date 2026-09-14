#!/usr/bin/env bash
# shell: bash
#
# goal.sh - record and report the goal for a working session.
#
# Goals live at <root>/.context/goals/<YYYY-MM-DD>-<slug>.md. Exactly one may
# carry status: active at a time.
#
# Usage:
#   goal.sh new <title> [--root DIR] [--tags a,b]
#   goal.sh status [--root DIR]
#   goal.sh check  [--root DIR]
#   goal.sh park <item-number> <reason> [--root DIR]
#
# Subcommands:
#   new     Scaffold a goal file from assets/goal-template.md.tmpl.
#   status  Render the "what's left" summary. Capped at 100 words, led by the
#           single next action. Exits 0 with a notice when no goal is active.
#   check   Validate the active goal against the evidence rules. Non-zero exit
#           when an item is done without evidence, or an outside-reach item was
#           closed without a recorded user confirmation.
#   park    Print the create-context-file invocation that files item N as a
#           follow-up. Filing it and deleting the row remain manual, so the
#           follow-up body gets written rather than generated empty.
#
#   -h, --help  Show this help.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE="$SCRIPT_DIR/../assets/goal-template.md.tmpl"

die() { printf 'error: %s\n' "$1" >&2; exit 1; }

usage() { sed -n '3,/^# *-h/p' "$0" | sed 's/^#\{0,1\} \{0,1\}//'; }

# substitute LINE NEEDLE REPLACEMENT - literal replacement of every occurrence.
#
# A goal title is arbitrary user text, so neither of the obvious tools is safe:
# sed's replacement expands & to the matched text and a | in the title collides
# with the s||| delimiter, and bash 5.2's ${var//x/y} likewise expands an
# unescaped & in the replacement. Prefix and suffix expansion has no such
# surface: nothing in the replacement is interpreted.
substitute() {
	_s="$1"; _needle="$2"; _repl="$3"; _out=""
	while [ "${_s#*"$_needle"}" != "$_s" ]; do
		_out="$_out${_s%%"$_needle"*}$_repl"
		_s="${_s#*"$_needle"}"
	done
	printf '%s' "$_out$_s"
}

slugify() {
	printf '%s' "$1" \
		| tr '[:upper:]' '[:lower:]' \
		| sed 's/[^a-z0-9]\{1,\}/-/g; s/-\{1,\}/-/g; s/^-//; s/-$//'
}

# Resolve the repository root: an explicit --root wins, then the harness's
# project dir, then git, then the working directory.
resolve_root() {
	if [ -n "${ROOT_OPT:-}" ]; then
		printf '%s' "$ROOT_OPT"
	elif [ -n "${CLAUDE_PROJECT_DIR:-}" ]; then
		printf '%s' "$CLAUDE_PROJECT_DIR"
	elif git rev-parse --show-toplevel >/dev/null 2>&1; then
		git rev-parse --show-toplevel
	else
		pwd
	fi
}

ROOT_OPT=""
TAGS=""
cmd="${1:-}"
[ -n "$cmd" ] || { usage; exit 1; }
case "$cmd" in -h|--help) usage; exit 0 ;; esac
shift

args=()
while [ $# -gt 0 ]; do
	case "$1" in
		--root) shift; ROOT_OPT="${1:-}"; [ -n "$ROOT_OPT" ] || die "--root needs a value" ;;
		--tags) shift; TAGS="${1:-}"; [ -n "$TAGS" ] || die "--tags needs a value" ;;
		-h|--help) usage; exit 0 ;;
		*) args+=("$1") ;;
	esac
	shift
done

ROOT="$(resolve_root)"
GOALS_DIR="$ROOT/.context/goals"

case "$cmd" in
	new)
		title="${args[0]:-}"
		[ -n "$title" ] || die "new needs a title"
		[ -f "$TEMPLATE" ] || die "template missing: $TEMPLATE"
		date_str="$(date +%Y-%m-%d)"
		slug="$(slugify "$title")"
		[ -n "$slug" ] || die "title produced an empty slug: $title"
		mkdir -p "$GOALS_DIR"
		target="$GOALS_DIR/$date_str-$slug.md"
		[ -e "$target" ] && die "already exists: $target"
		tags_yaml="[]"
		if [ -n "$TAGS" ]; then
			tags_yaml="[$(printf '%s' "$TAGS" | sed 's/ *, */, /g')]"
		fi
		: > "$target"
		while IFS= read -r line || [ -n "$line" ]; do
			# Date before title. substitute() never rescans what it inserted,
			# so doing the title last lets a title containing a literal
			# __DATE__ survive intact.
			line="$(substitute "$line" "__DATE__" "$date_str")"
			line="$(substitute "$line" "__TITLE__" "$title")"
			[ "$line" = "tags: []" ] && line="tags: $tags_yaml"
			printf '%s\n' "$line" >> "$target"
		done < "$TEMPLATE"
		printf '%s\n' "$target"
		;;

	status|check)
		[ -d "$GOALS_DIR" ] || { printf 'No goal recorded. State one to begin.\n'; exit 0; }
		MODE="$cmd" python3 - "$GOALS_DIR" "$ROOT" <<'PY'
import os
import re
import sys
import glob

mode = os.environ["MODE"]
goals_dir = sys.argv[1]
root = sys.argv[2]

active = []
for path in sorted(glob.glob(os.path.join(goals_dir, "*.md"))):
    with open(path) as handle:
        text = handle.read()
    fm = re.match(r"^---\n(.*?)\n---\n", text, re.S)
    if not fm:
        continue
    front = fm.group(1)

    def field(name, default=""):
        hit = re.search(rf"^{name}:\s*(.+)$", front, re.M)
        return hit.group(1).strip() if hit else default

    if field("status") != "active":
        continue
    active.append((path, field("title", os.path.basename(path)),
                   field("goal-status", "new"), text))

if not active:
    print("No active goal. State one to begin.")
    sys.exit(0)

if len(active) > 1:
    print("More than one active goal, so there is no single answer to "
          "\"what's left\". Close or promote all but one:")
    for path, title, _, _ in active:
        print(f"- {title} ({path})")
    sys.exit(1)

path, title, goal_status, text = active[0]

# Items table: | # | Item | State | Reach | Evidence |
rows = []
for line in text.splitlines():
    if not line.startswith("|"):
        continue
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) != 5:
        continue
    if cells[0] in ("#", "-") or set(cells[0]) <= {"-"}:
        continue
    if not cells[0] or not cells[1]:
        # An unfilled template row carries a number but no item text. It is
        # not yet an item and must not be counted as outstanding work.
        continue
    rows.append({"n": cells[0], "item": cells[1], "state": cells[2].lower(),
                 "reach": cells[3].lower(), "evidence": cells[4]})

done = [r for r in rows if r["state"] == "done"]
awaiting = [r for r in rows if r["state"] == "awaiting"]
todo = [r for r in rows if r["state"] == "todo"]

rel = os.path.relpath(path, root)

if mode == "check":
    problems = []
    if not rows:
        problems.append("the items table is empty")
    for r in rows:
        if r["state"] == "done" and not r["evidence"]:
            problems.append(f"item {r['n']} is done with no evidence")
        if r["state"] == "done" and r["reach"] == "outside" \
                and "confirm" not in r["evidence"].lower() \
                and "agree" not in r["evidence"].lower():
            problems.append(
                f"item {r['n']} has outside reach and was closed without a "
                f"recorded user confirmation: {r['evidence']!r}")
        if r["state"] not in ("todo", "awaiting", "done"):
            problems.append(f"item {r['n']} has unknown state {r['state']!r}")
        if r["reach"] not in ("local", "outside"):
            problems.append(f"item {r['n']} has unknown reach {r['reach']!r}")
    if len(rows) > 5:
        problems.append(f"{len(rows)} items exceeds the promotion threshold of 5")
    if todo or awaiting:
        if goal_status == "completed":
            problems.append("goal-status is completed but items remain open")
    elif goal_status not in ("completed", "promoted"):
        problems.append("no items remain open but goal-status is "
                        f"{goal_status!r}")
    if problems:
        print(f"FAILED: {rel}")
        for p in problems:
            print(f"- {p}")
        sys.exit(1)
    print(f"OK: {rel} ({len(done)}/{len(rows)} done)")
    sys.exit(0)

# status: compose under 100 words, next action first.
lines = []
if not rows:
    lines.append("Next: fill in the goal's items table. It is still empty.")
elif todo:
    lines.append(f"Next: {todo[0]['item']}")
elif awaiting:
    lines.append(f"Next: confirm \"{awaiting[0]['item']}\" so it can be closed.")
else:
    lines.append("Next: close the goal out and land it.")

lines.append(f"Goal: {title} ({goal_status}, {len(done)} of {len(rows)} done).")

if len(todo) > 1:
    rest = ", ".join(r["item"] for r in todo[1:])
    lines.append(f"Left: {rest}.")
if awaiting:
    names = ", ".join(r["item"] for r in awaiting)
    lines.append(f"Awaiting your confirmation: {names}.")

parked = len(re.findall(r"^- .*parked", text, re.M | re.I))
if parked:
    lines.append(f"Parked to follow-ups: {parked}.")

lines.append(f"File: {rel}")

# Drop optional detail lines from the middle until the cap is met, rather than
# truncating mid-sentence. The next action and the goal line always survive.
def words(ls):
    return len(" ".join(ls).split())

while words(lines) > 100 and len(lines) > 3:
    del lines[2]

print("\n".join(lines))
PY
		;;

	park)
		n="${args[0]:-}"
		reason="${args[1]:-}"
		[ -n "$n" ] && [ -n "$reason" ] || die "park needs an item number and a reason"
		cat <<EOF
Park item $n. Three steps, in this turn:

1. File the follow-up:
   create-context-file.sh --type follow-ups --title "<item text>" --root "$ROOT/.context"
   Write the reason into the body: $reason
2. Delete row $n from the goal's Items table and renumber the rows below it.
3. Append to the goal's Log:
   - $(date +%Y-%m-%d) item $n parked to <follow-up path>: $reason
EOF
		;;

	*)
		die "unknown subcommand: $cmd"
		;;
esac
