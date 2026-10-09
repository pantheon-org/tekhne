#!/usr/bin/env bash
# Record one agent run against a fixture, ready for stream-to-trace.ts.
#
# Usage:
#   record-run.sh --skill-file <SKILL.md> --skill-name <name> --fixture <dir> \
#                 --prompt <text> --out <stream.jsonl> [--model sonnet] [--budget 0.50]
#
# It copies the fixture into a fresh temporary folder, installs the skill
# version under test at .claude/skills/<name>/SKILL.md, commits everything (so
# `git status` afterwards shows only what the agent changed), and runs Claude
# Code there in headless mode with stream-json output.
#
# The agent is limited to reading and searching files and to running the
# auditor binary: no edits, no other commands, no MCP servers, only the
# project's own settings, nothing stored, and a cost cap. It uses your normal
# Claude login, so it spends from that account. Your global CLAUDE.md is still
# read (the minimal --bare mode would skip it but accepts only an API key), so
# the wording of the output carries that influence.
#
# On success it prints `fixture=<folder>` as its last line, for
# stream-to-trace.ts --fixture. The folder is left in place so you can look at it.
set -euo pipefail

skill_file="" skill_name="" fixture="" prompt="" out="" model="sonnet" budget="0.50"

fail() {
  printf 'error: %s\n' "$1" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --skill-file) skill_file=${2:-}; shift 2 ;;
    --skill-name) skill_name=${2:-}; shift 2 ;;
    --fixture) fixture=${2:-}; shift 2 ;;
    --prompt) prompt=${2:-}; shift 2 ;;
    --out) out=${2:-}; shift 2 ;;
    --model) model=${2:-}; shift 2 ;;
    --budget) budget=${2:-}; shift 2 ;;
    *) fail "unknown option: $1" ;;
  esac
done

[ -n "$skill_file" ] || fail "--skill-file is required"
[ -n "$skill_name" ] || fail "--skill-name is required"
[ -n "$fixture" ] || fail "--fixture is required"
[ -n "$prompt" ] || fail "--prompt is required"
[ -n "$out" ] || fail "--out is required"
[ -f "$skill_file" ] || fail "skill file not found: $skill_file"
[ -d "$fixture" ] || fail "fixture folder not found: $fixture"

work=$(mktemp -d "${TMPDIR:-/tmp}/record-run-XXXXXX")
cp -R "$fixture"/. "$work"/
mkdir -p "$work/.claude/skills/$skill_name"
cp "$skill_file" "$work/.claude/skills/$skill_name/SKILL.md"

git -C "$work" init -q
git -C "$work" add -A
git -C "$work" -c user.email=record@run -c user.name=record -c commit.gpgsign=false commit -q -m "fixture"

mkdir -p "$(dirname "$out")"
status=0
(
  cd "$work"
  claude -p "$prompt" \
    --output-format stream-json --verbose \
    --model "$model" --max-budget-usd "$budget" \
    --allowedTools Read Glob Grep "Bash(pantheon-skill-auditor:*)" \
    --setting-sources project --strict-mcp-config \
    --no-session-persistence --permission-mode dontAsk
) >"$out" || status=$?

if [ "$status" -ne 0 ]; then
  printf 'error: claude exited with %s (fixture left at %s)\n' "$status" "$work" >&2
  exit "$status"
fi

printf 'fixture=%s\n' "$work"
