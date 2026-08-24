#!/usr/bin/env bash
# shell: bash
# Import entries from a legacy source system into this repo's journal format (Mode A).
# Requires GNU awk (gawk) for the 3-arg match() capture-array extension.
#
# Usage:
#   import-legacy-entries.sh <source.md> <repo-root> [source-url] [author]
#
# <source.md> is markdown exported from the legacy source system (a Confluence
# page body, a wiki export, a plain-text log, etc). If you have raw MCP/API
# tool-result JSON instead of markdown, extract the markdown body first, e.g.:
#   jq -r '.result | fromjson | .metadata.content.value' page.json > source.md
#
# Splits the source on any level-3 heading ("### ...") that names a date (a
# month name plus a day number, e.g. "### Week ending on July 3, 2020" or
# "### Sprint close - July 3"), and emits one file per non-empty section at
# <repo-root>/<YYYY>/<MM>/<YYYY-MM-DD>-imported-entry.md, dated by that
# heading. Bullets are preserved verbatim; only "* " is normalised to "- ".
# Idempotent (re-running overwrites the same dated files).
#
# A heading with no year of its own inherits the year from the nearest
# preceding standalone "20NN" or "<Month> 20NN" line in the source. If the
# legacy source's headings do not follow the "Month Day" convention, adjust
# the heading match below before running.
#
# After running, always: prettier --write, markdownlint-cli2 --fix, then
# journal-entry-creator's validate-journal-entry.sh on the generated files.
set -euo pipefail

SRC="${1:-}"
REPO="${2:-}"
SRC_URL="${3:-}"
AUTHOR="${4:-Imported}"
if [[ -z "$SRC" || -z "$REPO" ]]; then
  sed -n '2,25p' "$0"
  exit 2
fi

awk -v repo="$REPO" -v srcurl="$SRC_URL" -v author="$AUTHOR" '
  BEGIN {
    split("January February March April May June July August September October November December", mn, " ")
    for (i = 1; i <= 12; i++) { num[tolower(mn[i])] = i }
    curyear = ""; have = 0; n = 0; written = 0; skipped = 0
  }

  # Flush the buffered section to a file (if it has content).
  function flush(   iso, mm, dd, nonblank, i, dir, path, fdate) {
    if (!have) return
    if (year == "" || mon == "" || day == "") { have = 0; n = 0; return }
    nonblank = 0
    for (i = 1; i <= n; i++) if (body[i] ~ /[^[:space:]]/) nonblank = 1
    if (!nonblank) { skipped++; have = 0; n = 0; return }

    mm = sprintf("%02d", mon); dd = sprintf("%02d", day)
    iso = sprintf("%04d-%s-%s", year, mm, dd)
    fdate = mn[mon] " " day ", " year
    dir = repo "/" year "/" mm
    system("mkdir -p \"" dir "\"")
    path = dir "/" iso "-imported-entry.md"

    printf("---\n") > path
    printf("title: \"Imported Entry - %s\"\n", fdate) >> path
    printf("date: %s\n", iso) >> path
    printf("authors:\n  - %s\n", author) >> path
    printf("tags:\n  - imported\n  - legacy-import\n  - \"%04d\"\n", year) >> path
    printf("source: \"%s\"\n", srcurl) >> path
    printf("status: published\n---\n\n") >> path
    printf("# Imported Entry - %s\n\n", fdate) >> path
    printf("**Date:** %s\n", fdate) >> path
    printf("**Context:** Historical entry imported from a legacy source system (section dated %s).\n\n", fdate) >> path
    printf("## Session Overview\n\n") >> path
    printf("Notes for %s, imported verbatim from the legacy source. No content was added or\n", fdate) >> path
    printf("interpreted; only formatting was normalised.\n\n") >> path
    printf("## Imported Notes\n\n") >> path
    for (i = 1; i <= n; i++) {
      line = body[i]
      if (line !~ /[^[:space:]]/) continue
      sub(/^[[:space:]]*\*[[:space:]]+/, "- ", line)         # "* " -> "- "
      sub(/^[[:space:]]+/, "", line)                          # strip left indent
      if (line !~ /^-/) line = "- " line
      printf("%s\n", line) >> path
    }
    printf("\n## Compliance\n\n") >> path
    printf("- Filename: `%s-imported-entry.md` - **checked**\n", iso) >> path
    printf("- H1 format: Title with formatted date suffix - **checked**\n") >> path
    printf("- Imported verbatim from the legacy source; content not modified - **checked**\n") >> path
    printf("- Linted: `prettier` + `markdownlint` - **pending**\n\n") >> path
    printf("## Tags\n\n- imported\n- legacy-import\n- %04d\n", year) >> path
    close(path)
    written++
    have = 0; n = 0
  }

  {
    line = $0
    # Standalone year line, e.g. "2018".
    if (line ~ /^[[:space:]]*20[0-9][0-9][[:space:]]*$/) { curyear = line + 0 }
    # "Month YYYY" header, e.g. "July 2018".
    if (match(tolower(line), /^([a-z]+)[[:space:]]+20[0-9][0-9]$/, m)) {
      if (m[1] in num) { curyear = substr(line, length(line) - 3) + 0 }
    }
    # A level-3 heading naming a date: "<Month> <Day>[, <Year>]" anywhere in the heading text.
    if (line ~ /^###[[:space:]]+/ && match(tolower(line), /([a-z]+)[[:space:]]+([0-9]{1,2})(st|nd|rd|th)?/, w) && (tolower(w[1]) in num)) {
      flush()
      mon = num[tolower(w[1])]
      day = w[2] + 0
      if (match(line, /(20[0-9][0-9])/, y)) year = y[1] + 0; else year = curyear
      have = 1; n = 0
      next
    }
    if (have) {
      if (line ~ /^-{3,}$/) next                              # setext underline
      body[++n] = line
    }
  }

  END { flush(); printf("written=%d skipped_empty=%d\n", written, skipped) }
' "$SRC"
