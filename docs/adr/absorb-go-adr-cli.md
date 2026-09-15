---
title: Absorb the standalone Go adr CLI into pantheon-adr
type: decision
branch_type: feat
status: review-requested
date: 2026-09-15
author: thoroc
branch: feat/absorb-go-adr-cli
tags: []
related:
- ../../crates/adr/docs/design/agent-driven-adr-lifecycle.md
changed_files:
- Cargo.lock
- crates/adr/Cargo.toml
- crates/adr/src/adr.rs
- crates/adr/src/commands.rs
- crates/adr/src/completeness.rs
- crates/adr/src/date.rs
- crates/adr/src/draft.rs
- crates/adr/src/git.rs
- crates/adr/src/hooks.rs
- crates/adr/src/index.rs
- crates/adr/src/lib.rs
- crates/adr/src/main.rs
- crates/adr/src/record.rs
- crates/adr/src/store.rs
- crates/adr/src/sync.rs
- crates/adr/tests/integration.rs
- docs/src/content/docs/tiles.md
- skills/documentation/adr-creator/.tessl-plugin/plugin.json
- skills/documentation/adr-creator/SKILL.md
- skills/documentation/adr-creator/evals/instructions.json
- skills/documentation/adr-creator/evals/scenario-1/criteria.json
- skills/documentation/adr-creator/evals/scenario-1/task.md
- skills/documentation/adr-creator/evals/scenario-2/criteria.json
- skills/documentation/adr-creator/evals/scenario-2/task.md
- skills/documentation/adr-creator/evals/scenario-3/criteria.json
- skills/documentation/adr-creator/evals/scenario-3/task.md
- skills/documentation/adr-creator/evals/scenario-4/criteria.json
- skills/documentation/adr-creator/evals/scenario-4/task.md
- skills/documentation/adr-creator/evals/summary.json
- skills/documentation/adr-creator/references/adr-lifecycle.md
- skills/documentation/adr-creator/references/cli-usage.md
- skills/documentation/adr-creator/references/context-extraction.md
- skills/documentation/adr-creator/scripts/check-undocumented-decisions.sh
history:
- from: proposed
  to: review-requested
  at: 2026-09-15T08:23:17Z
---

# Absorb the standalone Go adr CLI into pantheon-adr

## Problem Statement

### Context
Two ADR tools existed. `pantheon-adr` shipped from this repository and created numbered Nygard
records, and could do nothing else with them. A second, unrelated Go CLI installed a binary called
`adr` and carried the capabilities that actually mattered: a scored completeness gate, branch-context
drafting, a five-state lifecycle with history, agent-config sync and a pre-push hook. That second
tool had no git remote, so it existed on exactly one machine and one disk failure from gone.

The collision was not theoretical. A session following the adr-creator skill invoked the wrong
binary, concluded the skill was stale, and rewrote it against the Go tool before the duplication was
spotted. Two tools sharing one name in one person's PATH will keep producing that failure.

### Goals
One ADR tool, in this repository, with every capability either tool had. Records that can be
committed by default. A completeness check a hook can enforce, so "is this decision documented" has
an answer that is not a matter of opinion.

### Non-Goals
Migrating existing records, because neither tool had produced any. Preserving the Go tool's Go
import-graph analysis, which says nothing about a Rust and TypeScript codebase.

## Decision Record

### Options Considered
Keep the Go tool and retire the Rust crate, which would move a skill and its CLI out of the
repository that is meant to hold all of them. Keep both and document the split, which leaves the
name collision in place. Keep the Rust crate's numbered-Nygard storage and bolt the Go features on,
which gives no status history, no tags and keeps the numbering collision between branches. Adopt the
Go tool's authoritative TOML index, which has two sources of truth that already drifted in practice:
its own `update --superseded-by` changed the index and left the markdown reading `proposed`.

### Chosen Solution
pantheon-adr absorbs every capability of the Go CLI, and the Go repository is deleted. Storage
follows the pattern the Journal's `.context/decisions/` records and the `journal` crate already use:
each decision is one markdown file whose YAML frontmatter is the single source of truth for status,
author, branch, tags, supersede links and the full history of status transitions, and `pantheon-adr
index` regenerates a browsable catalogue derived from that frontmatter. Records are identified by a
slug taken from the branch name, so nothing needs numbering. The default directory is `docs/adr`,
which is committable, rather than the Go tool's `.context/decisions/`, which is gitignored
machine-wide.

### Rationale
The frontmatter model is the only option that keeps the Go tool's queryability without its drift,
because the derived index cannot contradict the records it is generated from. It also makes ADRs
consistent with every other context typology already in use, which matters more than ADRs being
special. Dropping the body's metadata block is what designs the drift out rather than leaving it to
be fixed by hand in each write path.

## Implementation

### Key Changes
Seven new modules in `crates/adr/src`: the record model and its frontmatter, the store, the
completeness rubric, git branch context, drafting, the catalogue, and the harness and git hooks. The
CLI gains init, create, draft, check, review, status, update, index and sync, plus four hidden hook
commands. The numbered-record module and the `new` and `supersede` commands are removed. The
adr-creator skill, its three references, its script and all four eval scenarios were rewritten
against the new surface.

### Testing Strategy
155 unit tests and four integration tests, with the rubric's weights, caps and boilerplate filter
asserted against the same inputs the Go implementation was checked with. Every command was then
exercised against a real git repository, including exit codes, sync idempotency, and hook install
and removal.

## Challenges & Solutions
The Go tool's `sync` wired hooks calling four subcommands that `--help` does not list, which is why
an initial reading of its interface missed them; they were found by reading the provider source
rather than the help output. Its once-per-branch nudge wrote marker files into the records
directory, which would pollute a committed `docs/adr`; those markers now live under `.git/`.

## Impact Assessment

- **Performance**: no meaningful change; every command is a small number of file reads plus git
- **Security**: the pre-push hook can block a push, so a misconfigured record directory could
  obstruct work; the gate is opt-in via `init --install-hooks` and never installed by default
- **Maintenance**: one tool instead of two, and roughly 2,900 lines of Go no longer maintained

## Risks & Pitfalls

- **Risk**: the Go repository has no remote, so deleting it is irreversible  **Mitigation**: its
  design rationale is preserved in `crates/adr/docs/design/`, and an archive is taken before removal
- **Risk**: a stale `adr` binary left on PATH still resolves and behaves differently  **Mitigation**:
  the skill's prerequisites warn against substituting a binary named `adr`

## Outcome & Lessons
Pending. The lesson already banked: a tool and its companion skill must live in the same repository,
or the skill will be maintained against a tool nobody else has.
