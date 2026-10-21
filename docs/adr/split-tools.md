---
title: Split the tools out of tekhne into their own repositories
type: decision
branch_type: chore
status: accepted
date: 2026-10-21
author: thoroc
branch: docs/adr-tool-split
tags:
- architecture
- release
- ci
related:
- consolidate-adr-tooling.md
changed_files:
- docs/adr/index.md
- docs/adr/split-tools.md
history:
- from: proposed
  to: accepted
  at: 2026-10-21T00:00:00Z
---

## Split the tools out of tekhne into their own repositories

### Problem Statement

#### Context
tekhne held about 164 skills, six Rust crates, a docs site, a catalogue generator, probes and 16
workflows. One CI, one audit gate and two release systems (release-please for skills, cargo-dist
for tools) were shared by all of it, so a change to a tool crate and a change to a comedy skill
competed for the same gates. The skill auditor and validator were also two binaries doing one job,
and the journal and ADR tools were installed from tekhne release tags (`tool/<name>-vX.Y.Z`).

#### Goals
Single-purpose repositories: tekhne as a skills catalogue, each tool in its own repository with its
own releases, and the shared library extracted. tekhne keeps auditing its skills with a pinned
released auditor.

#### Non-Goals
Splitting the skills themselves (decided against, see D8), publishing to crates.io (deferred), and
touching the Journal repository.

### Decision Record

#### Options Considered
Keep the monorepo and tighten the gates, which leaves the shared-gate friction. Per-package tags in
one repository, which the new layout makes unnecessary. Split the skills by domain (about 8 to 10
repositories plus a hub) or by plugin (about 60): more isolation, but a migration of 164 skills and
a second install target for little gain once the tools left.

#### Chosen Solution
Decisions taken on 19-10-2026:

| Id | Decision |
|---|---|
| D1 | One binary, `pantheon-skill-auditor`, with `validate`, `check`, `probes` and `analyze` subcommands. The validator is merged in, not a separate release |
| D2 | Plain `vX.Y.Z` tags in every repository, starting above the old versions (auditor 0.3.x, journal 0.2.x, adr 0.2.x) |
| D3 | Extraction order: shared kit, then auditor, then journal and adr |
| D4a | The shared library is `agentic-harness-dev-kit` (crates `ahdk-core`, `ahdk-install`). crates.io publication was the intent; see Outcome |
| D4b | Plan the skills split now. Outcome: option C, skills stay in tekhne, with a monthly revisit trigger |
| D5 | The auditor ships a GitHub Action; every repository audits its own skills with it |
| D6 | The `grade_parity` corpus becomes fixtures in the auditor repository |
| D7 | The docs Tools page reads the GitHub releases API at build time, with a weekly scheduled rebuild |
| D8 | Skills stay in tekhne (option C). Revisit a domain split if, in a calendar month after the split, a skill-only pull request is blocked by another domain's failure more than twice, or clone size or CI time becomes a problem |

Repositories, all public: `pantheon-org/agentic-harness-dev-kit`, `pantheon-org/pantheon-skill-auditor`,
`pantheon-org/pantheon-journal`, `pantheon-org/pantheon-adr`. Repository and binary names are
identical; the shared library has no binary and no `pantheon-` prefix.

#### Rationale
After the merge in D1, each repository has one product, so the repository name identifies it and
namespaced tags add nothing. The GitHub Action (D5) removes the cross-repository coupling that
motivated keeping everything together.

### Implementation

#### Key Changes
Waves 0 to 5 (inventory, shared kit, auditor, journal and adr, removal of `crates/` from tekhne,
consumer clean-up). tekhne pull requests: 447 (validator folded into the auditor), 448 (switch to
the pinned binary), 450 (pin from the new repository), 451 (Rust removal and docs releases lookup),
453 (three embedded skills removed, six repointed). tekhne now pins
`pantheon-skill-auditor` 0.3.1 in `mise.toml` with a checksum in `mise.lock`, guarded by
`scripts/stored-audits/check-auditor-pin.sh`. The last commit with the Rust workspace is tagged
`pre-crates-removal`. History was extracted with `git filter-repo` on fresh clones.

#### Testing Strategy
Each new repository builds and releases through cargo-dist; the auditor's frozen parity fixtures
run always, and live-skill parity runs only when `TEKHNE_ROOT` names a tekhne checkout. tekhne's own
gates (hooks, Skill Audit, probes) ran on every pull request.

### Challenges & Solutions
The auditor's first release needed a version-bump pull request because cargo-dist requires the tag
to match a crate version (hence v0.3.1). The skill folder in each tool repository is named after
the skill because the validator requires the match. Removing the last Go fixture left a stale
CodeQL Go job and the `code_quality` ruleset rule blocked merging pull request 451; the rule was
removed by the repository owner for that merge and restored, then verified identical to a backup.
By pull request 453 GitHub no longer ran a Go job.

### Impact Assessment
tekhne is a skills catalogue with one install target. Tool releases are independent. Pin bumps are
manual. Docs show tool versions from the releases API.

### Risks & Pitfalls
Manual pin bumps can lag (mitigated by the consistency check). The kit is consumed by git tag over
https. A future domain split is not blocked by this layout.

### Outcome & Lessons
Deviations from the plan: only `v0.1.0-rc.1` of the kit exists and its crates stay unpublished, so
crates.io publication (D4a) is deferred (agentic-harness-dev-kit issue 3). No source-build fallback
was kept after the switch to the pinned binary; rollback was reverting pull request 448. Pin bumps
are not automated. All four repositories were created private and made public after review.
`journal-entry-import` now validates with `pantheon-journal validate` and fails loudly when the
binary is missing. Open follow-ups are tracked as GitHub issues: pipeline maturity (kit 2, auditor 3,
journal 2, adr 2), auditor taxonomy 12 against 13 domains (auditor 5), and crates.io publication
(kit 3).
