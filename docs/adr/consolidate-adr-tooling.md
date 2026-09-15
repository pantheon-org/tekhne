---
title: Consolidate ADR tooling into pantheon-adr
type: decision
branch_type: feat
status: accepted
date: 2026-09-15
author: thoroc
branch: feat/consolidate-adr-tooling
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
- from: review-requested
  to: accepted
  at: 2026-09-15T09:14:50Z
---

# Consolidate ADR tooling into pantheon-adr

## Problem Statement

### Context
Two ADR tools existed in parallel. The one shipped from this repository created numbered Nygard
records and could do nothing else with them: no way to tell a finished record from an empty
template, no status history, and numbering that collides whenever two branches each add a decision.
A separate prototype, never published, had the capabilities that actually mattered: a scored
completeness gate, branch-context drafting, a five-state lifecycle, agent-config sync and a pre-push
hook.

Two tools that do the same job is a maintenance cost, but the sharper problem was that both
installed a binary whose name a reader could plausibly reach for, so following the companion skill
could invoke either one. A skill maintained against a tool that only one machine has is a skill that
silently rots.

### Goals
One ADR tool, in this repository alongside its companion skill, with every capability either tool
had. Records that can be committed by default. A completeness check a hook can enforce, so "is this
decision documented" has an answer that is not a matter of opinion.

### Non-Goals
Migrating existing records, because neither tool had produced any. Carrying over Go import-graph
analysis, which says nothing about a Rust and TypeScript codebase.

## Decision Record

### Options Considered
Retire this repository's crate and keep the prototype, which would put a skill's CLI outside the
repository meant to hold all of them. Keep both and document the split, which leaves the name
collision in place. Keep numbered-Nygard storage and bolt the missing features on, which gives no
status history, no tags, and keeps the numbering collision. Adopt an authoritative TOML index
alongside the markdown, which means two sources of truth that drift in practice: the prototype's own
supersede command changed its index and left the markdown still reading `proposed`.

### Chosen Solution
pantheon-adr absorbs every capability of the prototype, and the prototype is deleted. Storage
follows the pattern the `journal` crate and this project's `.context/` typologies already use: each
decision is one markdown file whose YAML frontmatter is the single source of truth for status,
author, branch, tags, supersede links and the full history of status transitions, and `pantheon-adr
index` regenerates a browsable catalogue derived from that frontmatter. Records are identified by a
slug taken from the branch name, so nothing needs numbering. The default directory is `docs/adr`,
which is committable, rather than a path under `.context/`, which is gitignored machine-wide and so
produces records that can never be shared.

### Rationale
The frontmatter model is the only option that keeps the queryability without the drift, because a
derived index cannot contradict the records it is generated from. It also makes decisions consistent
with every other context typology already in use, which matters more than decisions being special.
Dropping the metadata block from the record body is what designs the drift out, rather than leaving
it to be fixed by hand in every write path.

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
asserted against the same inputs the earlier implementation was checked with. Every command was then
exercised against a real git repository, including exit codes, sync idempotency against a settings
file carrying unrelated keys, and hook installation refusing to overwrite a hook it did not write.

## Challenges & Solutions
The prototype's sync wired harness hooks calling four subcommands that its own help output did not
list, so reading the interface was not enough to find them; they were recovered from the provider
source. Its once-per-branch nudge also wrote marker files into the records directory, which would
pollute a committed `docs/adr`; those markers now live under `.git/`.

## Impact Assessment

- **Performance**: no meaningful change; every command is a small number of file reads plus git
- **Security**: the pre-push hook can block a push, so a misconfigured records directory could
  obstruct work; the gate is opt-in via `init --install-hooks` and never installed by default
- **Maintenance**: one tool instead of two, and roughly 2,900 lines of a second implementation no
  longer maintained

## Risks & Pitfalls

- **Risk**: a stale binary from the retired prototype left on a developer's PATH still resolves and
  behaves differently  **Mitigation**: the skill's prerequisites name the expected binary and warn
  against substituting another
- **Risk**: the completeness rubric is the definition of "finished", so a badly calibrated rule
  either waves through empty records or blocks good ones  **Mitigation**: the weights, caps and
  boilerplate filter are unit-tested against known inputs, and `--strict` exists for teams that want
  a higher bar

## Outcome & Lessons
Pending. The lesson already banked: a tool and its companion skill must live in the same repository,
or the skill will be maintained against a tool nobody else has.
