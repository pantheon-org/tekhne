# pantheon-adr CLI Usage

The `pantheon-adr` binary creates and manages Architecture Decision Records. It
is offline and deterministic: every command is a pure function of the records on
disk, the current git branch, and today's date.

```bash
cargo install --path crates/adr   # from the tekhne repository root
pantheon-adr --version
```

## Where records live

Every command that touches records accepts `--dir`, which sits **after** the
subcommand:

```bash
pantheon-adr list --dir architecture/decisions
```

Resolution order is `--dir`, then the `ADR_DIR` environment variable, then the
default `docs/adr`. The default is deliberately a committed path: an earlier
tool defaulted to `.context/decisions/`, which most repositories gitignore, so
records created with defaults could never be committed. Check any directory you
are unsure about:

```bash
git check-ignore -v docs/adr
# no output means the records can be committed
```

## What a record looks like

One markdown file per decision, named for its slug, with YAML frontmatter that
holds everything the tooling reads:

```markdown
---
title: Adopt OpenTelemetry for tracing
type: decision
branch_type: feat
status: accepted
date: 2026-09-15
author: Thomas Roche
branch: feat/adopt-opentelemetry
pr: 42
tags:
- tracing
- observability
related: []
changed_files:
- src/tracing.rs
supersedes: use-vendor-agent
superseded_by: adopt-grafana-tempo
history:
- from: proposed
  to: review-requested
  at: 2026-09-15T08:03:10Z
- from: review-requested
  to: accepted
  at: 2026-09-15T09:14:02Z
---

# Adopt OpenTelemetry for tracing
...
```

`changed_files` is a snapshot taken at creation, not live state: it records what
the branch touched at the time, which is what lets `draft` surface earlier
decisions over the same files.

There is no index file to keep in step. `index.md` is generated and carries a
banner saying so.

## Commands

### `pantheon-adr init`

Creates the ADR directory, and optionally the git hook.

| Flag | Effect |
| --- | --- |
| `--project <name>` | Project name shown in hook output; auto-detected from git if omitted. |
| `--install-hooks` | Install a `pre-push` hook that blocks while any proposed record scores below 80. |
| `--uninstall-hooks` | Remove that hook. Refuses to delete a `pre-push` hook it did not write. |

### `pantheon-adr create [slug]`

Writes one record from the Branch ADR template. With no `slug`, the slug and
type come from the current branch: `feat/adopt-otel` gives slug `adopt-otel` and
type `feat`. Recognised prefixes are `feat`, `fix`, `docs` and `chore`; anything
else is typed `chore`.

| Flag | Effect |
| --- | --- |
| `-d, --description <text>` | One-line description, stored as the record's `title`. Derived from the slug if omitted. |
| `-t, --type <type>` | Override the branch-derived type. |
| `--author <name>` | Override `git config user.name`. |

Fails if the file already exists, and (with no slug given) if there is no branch
to name the record after.

The generated body is the house **Branch ADR**:

```markdown
# <title>

## Problem Statement
### Context
### Goals
### Non-Goals

## Decision Record
### Options Considered
### Chosen Solution
### Rationale

## Implementation
### Key Changes
### Testing Strategy

## Challenges & Solutions

## Impact Assessment
- **Performance**: / **Security**: / **Maintenance**:

## Risks & Pitfalls

## Outcome & Lessons
```

Each section body is an HTML-comment placeholder to replace. There is no
metadata block in the body: frontmatter owns every field, which is why the two
can never disagree.

### `pantheon-adr draft [slug]`

Read-only. Aggregates context for filling a record in. With no slug, the current
branch's record is used.

| Flag | Effect |
| --- | --- |
| `--bootstrap` | Emit a numbered question set instead of the context summary. |
| `--base <branch>` | Diff against this branch instead of `main` or `master`. |
| `--pr` | Add pull request comments as a Reviewer Feedback section; needs `gh`, and is skipped silently without it. |
| `--json` | Machine-readable output. |

Reports the branch, base, commits, changed-file totals, earlier decisions that
touched the same files, and the headings whose bodies are still placeholders.

### `pantheon-adr check <slug>`

Scores the body 0 to 100. **The slug is required.**

| Flag | Effect |
| --- | --- |
| `--strict` | Require 95 rather than 80. |
| `--json` | Emit `{score, pass, missing[], filled[], caps_applied[]}`. |

**The rubric**, applied to `##` and `###` sections by substring match on the
heading name:

| Rule | Weight | Passes when |
| --- | --- | --- |
| No unfilled placeholders | 20 | Zero `<!-- ... -->` comments remain and at least one heading exists. |
| Problem Statement / Context | 25 | 30 or more words, excluding comments. |
| Chosen Solution | 25 | 30 or more words. |
| Rationale | 20 | 20 or more words. |
| Impact Assessment | 10 | At least one `-` bullet whose value is neither empty nor `none`. |

Three caps then apply, and the lowest one wins:

| Trigger | Cap |
| --- | --- |
| Problem Statement thin or absent | 60 |
| Chosen Solution thin or absent | 60 |
| Three or more unfilled placeholders | 70 |

A **boilerplate filter** rejects Problem Statement, Chosen Solution and
Rationale text where more than 60% of the words are `describe`, `decision`,
`context`, `placeholder`, `example`, `tbd` or `todo`. Such a section scores as
if it were empty, cap included.

### `pantheon-adr review <slug>`

Re-runs the check and, at 80 or above, moves the status to `review-requested`
and appends the transition to the record's history. Below 80 it prints the
failing rules and changes nothing.

### `pantheon-adr update <slug>`

Changes metadata. Never touches the prose.

| Flag | Effect |
| --- | --- |
| `-d, --description <text>` | Replace the title. |
| `-s, --status <status>` | One of `proposed`, `review-requested`, `accepted`, `deprecated`, `superseded`. |
| `--tags <a,b,c>` | Replace the tag list. |
| `--superseded-by <slug>` | Retire this record in favour of `<slug>`, which must already exist. |

Every status change appends a `history` entry with `from`, `to` and a UTC
timestamp. `--superseded-by` also writes `supersedes` onto the replacement, so
the chain is navigable from either end. It does not create the replacement:
`create` it first.

Calling `update` with no flags is an error rather than a silent no-op.

### `pantheon-adr list`

```bash
pantheon-adr list
# 2 ADR(s)
#   feat  adopt-grafana-tempo 2026-09-15 proposed   Adopt Grafana Tempo as the tracing backend
#   feat  adopt-opentelemetry 2026-09-15 superseded Adopt OpenTelemetry for tracing
```

| Flag | Effect |
| --- | --- |
| `-s, --status <status>` | Filter by status. |
| `-t, --type <type>` | Filter by type. |
| `--json` | One object per record: slug, title, branch_type, status, date, author, tags, superseded_by. |

A markdown file in the directory that is not a record (`README.md`, the
generated `index.md`) is ignored. A file that *looks* like a record but has
broken frontmatter is reported as an error rather than skipped.

### `pantheon-adr status`

```bash
pantheon-adr status --json
# {"total":2,"by_status":{"proposed":1,"superseded":1},"by_type":{"feat":2}}
```

Useful as a CI gate:

```bash
pantheon-adr status --json | jq -e '(.by_status.proposed // 0) == 0'
```

### `pantheon-adr index`

Regenerates `index.md` from the records' frontmatter, grouped by status in
lifecycle order. Derived output: overwrite it freely, never hand-edit it.

### `pantheon-adr sync`

Detects the agent harnesses this project uses and wires the hook commands into
each. Idempotent and additive: unrelated settings survive, an already-wired
config reports `ok`, and a hook belonging to another tool is left in place.

| Harness | Detected by | Wired |
| --- | --- | --- |
| Claude Code | `.claude/` | `SessionStart`, `PostToolUse` (Bash), `Stop` in `settings.json` |
| OpenCode | `opencode.json` or `.opencode/` | `.opencode/plugins/adr.js` |

A project with no harness config is reported, not failed.

### `pantheon-adr skill install` and `uninstall`

Installs this `adr-creator` skill into agent skills directories. The skill is
embedded in the binary, so no repository checkout is needed.

| Flag | Effect |
| --- | --- |
| `--agent <name>` | Target a specific agent by slug (repeatable). |
| `--all` | Install into every agent in the universal list. |
| `--local` | Install into project-local directories instead of global. |
| `--mode copy` or `--mode symlink` | Copy (default) or symlink the skill files. |
| `--dry-run` | Report what would happen without writing. |
| `--list-agents` | List targetable agents and exit. |

## Hidden commands

Four commands exist for machinery rather than people, and are wired by `sync`
and `init --install-hooks`. Three of them only ever print, because a hook that
fails a session over an untidy record is a hook that gets uninstalled.

| Command | When it runs | What it does |
| --- | --- | --- |
| `session-start` | Agent session begins | Prints status counts and any records awaiting human review. |
| `post-tool-use` | After a Bash tool call | Once per branch, notes that the branch has no record. Silent on `main` and `master`. |
| `session-end` | Agent session ends | Lists unfinished records with their scores. |
| `pre-push` | `git push` | **Fails** the push while any proposed record scores below 80. |

The once-per-branch marker for `post-tool-use` is written under `.git/`, so a
committed `docs/adr` never fills with bookkeeping files.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Success. |
| 1 | General failure, including a score below the threshold. |
| 3 | A record or directory that does not exist. |

```bash
pantheon-adr check my-slug \
  && pantheon-adr index \
  && git add docs/adr \
  && git commit -m "docs(adr): record decision"
```

## Not available

Two capabilities of the retired standalone Go CLI were deliberately not carried
over. `draft --with-analysis` built a Go import graph with `go list`, which says
nothing about a Rust or TypeScript codebase. Shell completion would need a
dependency no sibling CLI in this repository uses. Ask before assuming either
exists.
