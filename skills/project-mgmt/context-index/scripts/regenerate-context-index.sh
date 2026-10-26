#!/usr/bin/env bash
# shell: bash
# regenerate-context-index.sh - rebuild .context/index.yaml from the YAML
# frontmatter of every .context/**/*.md file.
#
# The index is a generated cache grouped by typology (the plural directory
# name a file lives under: findings/, plans/, goals/, guides/, follow-ups/,
# handovers/, merge-requests/, tickets/, decisions/, notes/, research/). The
# source of truth is always the frontmatter in each .md file, never this output.
#
# Follow-ups may also carry severity, priority and graded. For follow-ups only,
# they are copied into the index entry after date: (see GRADE_FIELDS below), and
# an invalid value or incomplete set is warned about on stderr.
#
# Usage:
#   regenerate-context-index.sh          # write .context/index.yaml
#   regenerate-context-index.sh --check  # verify it is already up to date
#
# Opt-in split, one yaml and md pair per non-empty typology (index-<type>.yaml,
# index-<type>.md, beside index.yaml in .context/):
#   regenerate-context-index.sh --split                 # write index.yaml and the pairs
#   regenerate-context-index.sh --split --no-aggregate  # write the pairs only
#   regenerate-context-index.sh --split --check         # verify all of them
#   regenerate-context-index.sh --only <type> --stdout [--format yaml|md]
#                                                       # render one file, write nothing
# The per-type files carry no date in their header, so a day passing does not
# dirty them. --split --check also fails on an orphan file (a pair whose type
# is now empty) and on a missing file for a non-empty type. Without --split the
# output is exactly what it was before the flag existed.
set -euo pipefail

CHECK_MODE=false
SPLIT=false
AGGREGATE=true
ONLY=""
TO_STDOUT=false
FORMAT="yaml"
while [ $# -gt 0 ]; do
  case "$1" in
    --check) CHECK_MODE=true ;;
    --split) SPLIT=true ;;
    --no-aggregate) AGGREGATE=false ;;
    --stdout) TO_STDOUT=true ;;
    --only)
      [ $# -ge 2 ] || { echo "usage: --only <type>" >&2; exit 2; }
      ONLY="$2"
      shift
      ;;
    --format)
      [ $# -ge 2 ] || { echo "usage: --format yaml|md" >&2; exit 2; }
      FORMAT="$2"
      shift
      ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
  shift
done
if [ -n "$ONLY" ] && [ "$TO_STDOUT" != true ]; then
  echo "--only needs --stdout (it renders one file and writes nothing)" >&2
  exit 2
fi
if [ "$TO_STDOUT" = true ] && [ -z "$ONLY" ]; then
  echo "--stdout needs --only <type>" >&2
  exit 2
fi
case "$FORMAT" in yaml | md) ;; *) echo "--format must be yaml or md" >&2; exit 2 ;; esac
if [ "$AGGREGATE" = false ] && [ "$SPLIT" != true ]; then
  echo "--no-aggregate only makes sense with --split" >&2
  exit 2
fi

ROOT="$(git rev-parse --show-toplevel)"
INDEX="$ROOT/.context/index.yaml"

python3 - "$ROOT" "$INDEX" "$CHECK_MODE" "$SPLIT" "$AGGREGATE" "$ONLY" "$FORMAT" <<'PYEOF'
import sys
import json
import re
import posixpath
from urllib.parse import quote
from pathlib import Path
from datetime import date

root = Path(sys.argv[1])
index_path = Path(sys.argv[2])
check_mode = sys.argv[3] == "true"
split_mode = sys.argv[4] == "true"
aggregate = sys.argv[5] == "true"
only_type = sys.argv[6]
out_format = sys.argv[7]
context_dir = root / ".context"


def unquote(v):
    # Frontmatter values may use either YAML quoting style. Stripping only '"'
    # left single-quoted values carrying their quotes plus any inner '"', which
    # then broke the emitted index (see emit_scalar).
    v = v.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        inner = v[1:-1]
        if v[0] == "'":
            return inner.replace("''", "'")
        return inner
    return v


def emit_scalar(v):
    # A double-quoted YAML scalar built by naive f-string interpolation breaks
    # on any value containing '"'. JSON string syntax is a valid YAML 1.2
    # double-quoted scalar and escapes what needs escaping; ensure_ascii=False
    # keeps em dashes and other non-ASCII readable.
    return json.dumps(v, ensure_ascii=False)


def parse_list_block(raw, key):
    # Matches a block-style YAML list under `key:`, e.g.:
    #   tags:
    #     - auth
    #     - oauth
    # Does not match flow-style `key: []` or `key: [a, b]` - callers treat an
    # absent block as "no items" either way.
    m = re.search(rf"^{re.escape(key)}:\n((?:  - .+\n?)+)", raw, re.MULTILINE)
    if not m:
        return []
    return [ln.strip()[2:].strip() for ln in m.group(1).splitlines() if ln.strip().startswith("- ")]


def parse_frontmatter(text):
    if not text.startswith("---\n"):
        return None
    try:
        end = text.index("---\n", 4)
    except ValueError:
        return None
    fm_text = text[4:end]
    fm = {}
    for line in fm_text.splitlines():
        if ": " in line and not line.startswith(" "):
            k, _, v = line.partition(": ")
            fm[k.strip()] = unquote(v)
    fm["_raw"] = fm_text
    return fm


# Follow-up grades. `severity`, `priority` and `graded` are optional, set
# together, and meaningful on follow-ups only; other typologies ignore them.
# Values are matched case-insensitively. A value outside the lists, or a
# partial set, is still written to the index as the file has it so readers can
# show it marked as needing a grade; the warning below tells the author.
GRADE_FIELDS = ("severity", "priority", "graded")
SEVERITY_VALUES = {"CRITICAL", "HIGH", "MEDIUM", "LOW"}
PRIORITY_VALUES = {"P1", "P2", "P3"}
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def grade_problems(grades):
    problems = []
    absent = [f for f in GRADE_FIELDS if f not in grades]
    if absent:
        problems.append(
            f"incomplete grades, missing: {', '.join(absent)} (set severity, priority and graded together)"
        )
    if "severity" in grades and grades["severity"].upper() not in SEVERITY_VALUES:
        problems.append(f"severity: {grades['severity']!r} is not one of CRITICAL, HIGH, MEDIUM, LOW")
    if "priority" in grades and grades["priority"].upper() not in PRIORITY_VALUES:
        problems.append(f"priority: {grades['priority']!r} is not one of P1, P2, P3")
    if "graded" in grades and not DATE_RE.match(grades["graded"]):
        problems.append(f"graded: {grades['graded']!r} is not a YYYY-MM-DD date")
    return problems


entries = []
missing = []
bad_grades = []

for md in sorted(context_dir.rglob("*.md")):
    # index-<type>.md are this script's own generated views, not entries.
    if md.name.startswith("index-"):
        continue
    rel = str(md.relative_to(root))
    content = md.read_text()
    fm = parse_frontmatter(content)
    if fm is None:
        missing.append(rel)
        continue
    required = ["title", "type", "status", "date"]
    absent = [f for f in required if not fm.get(f)]
    if absent:
        missing.append(f"{rel} (missing: {', '.join(absent)})")
        continue
    entry = {k: fm[k] for k in required}
    entry["path"] = rel
    if fm["type"] == "follow-up":
        grades = {f: fm[f] for f in GRADE_FIELDS if fm.get(f)}
        if grades:
            entry.update(grades)
            bad_grades.extend(f"{rel}: {p}" for p in grade_problems(grades))
    tags = parse_list_block(fm["_raw"], "tags")
    if tags:
        entry["tags"] = tags
    related = parse_list_block(fm["_raw"], "related")
    if related:
        entry["related"] = related
    blocks = parse_list_block(fm["_raw"], "blocks")
    if blocks:
        entry["blocks"] = blocks
    blocked_by = parse_list_block(fm["_raw"], "blocked-by")
    if blocked_by:
        entry["blocked-by"] = blocked_by
    entries.append(entry)

if missing:
    print(
        "WARNING: files with missing frontmatter (excluded from index):", file=sys.stderr
    )
    for f in missing:
        print(f"  {f}", file=sys.stderr)

if bad_grades:
    print("WARNING: invalid follow-up grades (entry kept as written, readers show it as needing a grade):", file=sys.stderr)
    for b in bad_grades:
        print(f"  {b}", file=sys.stderr)

# The plural directory name a file lives under is the typology; the file's
# own `type:` frontmatter field is the singular form of that same typology
# (findings/ -> finding, follow-ups/ -> follow-up, research/ -> research).
# This mapping must stay in sync with singular_of() in create-context-file's
# scripts/create-context-file.sh.
type_group_key = {
    "finding": "findings",
    "plan": "plans",
    "goal": "goals",
    "evidence": "evidence",
    "guide": "guides",
    "follow-up": "follow-ups",
    "handover": "handovers",
    "merge-request": "merge-requests",
    "ticket": "tickets",
    "decision": "decisions",
    "note": "notes",
    "research": "research",
}
type_order = [
    "findings",
    "plans",
    "goals",
    "evidence",
    "guides",
    "follow-ups",
    "handovers",
    "merge-requests",
    "tickets",
    "decisions",
    "notes",
    "research",
]
type_label = {
    "findings": "Findings",
    "plans": "Plans",
    "goals": "Goals",
    "evidence": "Evidence",
    "guides": "Guides",
    "follow-ups": "Follow-ups",
    "handovers": "Handovers",
    "merge-requests": "Merge Requests",
    "tickets": "Tickets",
    "decisions": "Decisions",
    "notes": "Notes",
    "research": "Research",
}

# A file's `type:` should always match the typology its own directory
# implies - the directory already says what it should be, so any
# disagreement is a mistake in the file, not a legitimate new typology.
# Silently routing it into the `other` catch-all is how such mistakes go
# unnoticed. Warn loudly instead of swallowing it.
dir_to_type = {v: k for k, v in type_group_key.items()}
mismatches = []
for e in entries:
    parts = e["path"].split("/")
    if len(parts) < 3 or parts[0] != ".context":
        continue
    actual_dir = parts[1]
    expected_type = dir_to_type.get(actual_dir)
    if expected_type is None:
        # Directories outside the curated typology set (e.g. one-off folders
        # created with create-context-file's --allow-new-type, or content
        # belonging to an unrelated tool) are not an error - they simply fall
        # into the "other" catch-all below.
        continue
    if e["type"] != expected_type:
        mismatches.append(
            f"{e['path']}: type: {e['type']!r} does not match its directory '{actual_dir}/' (expected type: {expected_type!r})"
        )

if mismatches:
    print("WARNING: type/directory mismatches (entry kept, but check the source file):", file=sys.stderr)
    for m in mismatches:
        print(f"  {m}", file=sys.stderr)

grouped = {}
for e in entries:
    key = type_group_key.get(e["type"], "other")
    grouped.setdefault(key, []).append(e)
other = grouped.pop("other", [])
for t in list(grouped):
    if t not in type_order:
        other.extend(grouped.pop(t))
if other:
    grouped["other"] = other
    type_order.append("other")
    type_label["other"] = "Other"

def summarise(es):
    counts = {}
    for e in es:
        counts[e["status"]] = counts.get(e["status"], 0) + 1
    return ", ".join(f"{n} {s}" for s, n in sorted(counts.items()))


def section_lines(t, lst):
    out = [f"# ── {type_label[t]} ({len(lst)}) ──", f"{t}:"]
    for e in lst:
        out.append(f"  - path: {emit_scalar(e['path'])}")
        out.append(f"    title: {emit_scalar(e['title'])}")
        out.append(f"    status: {emit_scalar(e['status'])}")
        out.append(f"    date: {e['date']}")
        if e.get("severity"):
            out.append(f"    severity: {emit_scalar(e['severity'])}")
        if e.get("priority"):
            out.append(f"    priority: {emit_scalar(e['priority'])}")
        if e.get("graded"):
            # A date stays unquoted like `date:`; anything else is quoted so a
            # malformed value cannot break the emitted YAML.
            graded = e["graded"]
            out.append(f"    graded: {graded if DATE_RE.match(graded) else emit_scalar(graded)}")
        for key in ("tags", "related", "blocks", "blocked-by"):
            if e.get(key):
                out.append(f"    {key}:")
                for item in e[key]:
                    out.append(f"      - {emit_scalar(item)}")
    out.append("")
    return out


HEADER = "# Auto-generated by context-index skill. Do not edit manually."


def type_yaml(t):
    lst = grouped[t]
    lines = [HEADER, f"# {len(lst)} entries: {summarise(lst)}", ""] + section_lines(t, lst)
    return "\n".join(lines) + "\n"


def md_cell(v):
    return re.sub(r"\s+", " ", v).replace("|", "\\|")


def type_md(t):
    lst = grouped[t]
    graded_cols = t == "follow-ups"
    cols = ["Title", "Status", "Date"] + (["Severity", "Priority", "Graded"] if graded_cols else [])
    lines = [
        "<!-- Auto-generated by context-index skill. Do not edit manually. -->",
        f"# {type_label[t]}",
        "",
        f"{len(lst)} entries: {summarise(lst)}",
        "",
        "| " + " | ".join(cols) + " |",
        "| " + " | ".join("---" for _ in cols) + " |",
    ]
    for e in lst:
        rel = quote(posixpath.relpath(e["path"], ".context"), safe="/")
        title = md_cell(e["title"]).replace("[", "\\[").replace("]", "\\]")
        row = [f"[{title}]({rel})", md_cell(e["status"]), e["date"]]
        if graded_cols:
            row += [md_cell(e.get(f, "")) for f in GRADE_FIELDS]
        lines.append("| " + " | ".join(row) + " |")
    return "\n".join(lines) + "\n"


def aggregate_yaml():
    lines = [
        HEADER,
        f"# Last updated: {date.today().isoformat()}",
        f"# {len(entries)} entries: {summarise(entries)}",
        "",
    ]
    for t in type_order:
        if grouped.get(t):
            lines.extend(section_lines(t, grouped[t]))
    return "\n".join(lines) + "\n"


if only_type:
    if only_type not in type_order:
        print(f"ERROR: unknown type {only_type!r} (non-empty types: {', '.join(t for t in type_order if grouped.get(t)) or 'none'})", file=sys.stderr)
        sys.exit(2)
    if grouped.get(only_type):
        sys.stdout.write(type_md(only_type) if out_format == "md" else type_yaml(only_type))
    sys.exit(0)

# path -> content for every file this run owns.
outputs = {}
if aggregate:
    outputs[index_path] = aggregate_yaml()
if split_mode:
    for t in type_order:
        if grouped.get(t):
            outputs[context_dir / f"index-{t}.yaml"] = type_yaml(t)
            outputs[context_dir / f"index-{t}.md"] = type_md(t)

# Split files on disk that no non-empty type accounts for.
orphans = []
if split_mode and context_dir.is_dir():
    for pattern in ("index-*.yaml", "index-*.md"):
        orphans.extend(p for p in sorted(context_dir.glob(pattern)) if p not in outputs)

if check_mode and not split_mode:
    # Unchanged from before --split existed.
    current = index_path.read_text() if index_path.exists() else ""
    if aggregate_yaml() != current:
        print("ERROR: .context/index.yaml is stale — regenerate it and commit the result", file=sys.stderr)
        sys.exit(1)
    print("context index is fresh")
elif check_mode:
    problems = []
    for path, content in outputs.items():
        rel = path.relative_to(root)
        if not path.exists():
            problems.append(f"{rel} is missing")
        elif path.read_text() != content:
            problems.append(f"{rel} is stale")
    for p in orphans:
        problems.append(f"{p.relative_to(root)} is an orphan (no entries of that type)")
    if problems:
        for p in problems:
            print(f"ERROR: {p}", file=sys.stderr)
        print("ERROR: context index is out of date -- regenerate it and commit the result", file=sys.stderr)
        sys.exit(1)
    print("context index is fresh")
else:
    context_dir.mkdir(parents=True, exist_ok=True)
    for path, content in outputs.items():
        path.write_text(content)
    for p in orphans:
        p.unlink()
    if split_mode:
        n_files = len(outputs) - (1 if aggregate else 0)
        print(f"Generated {len(entries)} entries -> {n_files} split file(s) in {context_dir}" + (f" and {index_path}" if aggregate else ""))
    else:
        print(f"Generated {len(entries)} entries -> {index_path}")
PYEOF
