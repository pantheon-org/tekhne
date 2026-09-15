---
name: adr-creator
description: "Creates, scores, reviews and supersedes Architecture Decision Records with the pantheon-adr CLI. Use when recording an architectural decision, writing an ADR, documenting a technical choice, checking whether an ADR is finished, superseding a prior decision, or bootstrapping an ADR log under docs/adr."
---

# Authoring Architecture Decision Records

## Mindset

An ADR is a dated, immutable record of one decision and the forces that shaped it, not living documentation. Its value is archaeological: a reader six months from now must understand *why* a choice was made without asking anyone. Once accepted, an ADR is never edited to change its meaning. When the decision changes you write a new record and point the old one at it, preserving the chain of reasoning.

**A record's YAML frontmatter is the single source of truth.** Status, author, branch, tags, supersede links and the full history of status transitions all live in the file's own frontmatter. There is no separate index to keep in step, so the metadata cannot contradict the prose. `pantheon-adr index` regenerates a browsable catalogue *from* that frontmatter, which makes the catalogue derived and safe to overwrite, never authored.

**Records are identified by slug, not by number.** `pantheon-adr create` on branch `feat/adopt-otel` writes `docs/adr/adopt-otel.md` and types the record `feat`. There is no `max(existing) + 1` to compute and no numbering to collide when two branches each add a decision.

**Completeness is machine-checked, not a matter of taste.** `pantheon-adr check <slug>` scores the prose 0 to 100 against a fixed rubric and `pantheon-adr review <slug>` refuses to run below 80. Treat the score as the definition of "finished" and write to satisfy the rubric rather than arguing with it. The point of the tool is that "is this ADR done?" has an answer a hook can enforce.

## Prerequisites

This skill drives the `pantheon-adr` CLI from the `tekhne` repository, and is
distributed by it (`pantheon-adr skill install`). Confirm the binary is on
`PATH`:

```bash
pantheon-adr --version
```

It is not published to crates.io, so `cargo install pantheon-adr` will fail.
Install a `tekhne` release binary (cargo-dist), or build from a checkout:

```bash
cargo install --path crates/adr   # from the tekhne repository root
```

There is no self-contained fallback for these commands. Do not substitute a
different binary named `adr`: an unrelated Go CLI of that name has an
incompatible command set and record format.

## When to Use

- The user asks to record, write, or draft an architectural or technical decision.
- A significant, hard-to-reverse choice was just made (framework, data store, protocol, boundary) and needs a durable rationale.
- A branch is ready for review and its record needs finishing or scoring.
- An earlier decision is being replaced and the old record must be retired while keeping its history.
- A repository needs an ADR log bootstrapped under `docs/adr`.
- A decision already exists in a planning or review document and needs capturing as a record after the fact. See [Deriving an ADR from an Existing Document](references/context-extraction.md).

## When Not to Use

- The change is routine and reversible (a dependency bump, a rename, a config tweak). Use a commit message, not an ADR.
- The user wants prose design documentation or a runbook. A record captures a single decision, not a system overview.
- A decision is still being debated with no chosen option. Reach a decision first, or leave the record at `proposed` and say in the prose that nothing is ratified.

## Principles

1. One record captures exactly one decision. If you are tempted to write "and also", split it in two.
2. The slug comes from the branch or an explicit argument. Never hand-create a markdown file; `create` writes the frontmatter the rest of the tooling reads.
3. Accepted records are immutable except for their status. Supersede, do not rewrite.
4. The Problem Statement must state the forces at play so the decision reads as inevitable, not arbitrary.
5. Options Considered is mandatory evidence of due diligence, even when the answer was obvious.
6. `check` is the gate. A record below 80 is unfinished, whatever it reads like.

## Procedure

1. **Bootstrap the log if there is none.** Run `pantheon-adr init`. The default directory is `docs/adr`, which is committable; `--dir` and the `ADR_DIR` variable override it. **Verify:** `pantheon-adr list` prints `No ADRs found.` or the existing records.
2. **Create the record on its branch.** Run `pantheon-adr create -d "<one-line description>"`. The slug is the branch name with its `feat/`, `fix/`, `docs/` or `chore/` prefix stripped, and the type comes from that prefix. Pass an explicit slug (`create <slug>`) when not on a feature branch, and `-t` when the prefix is not the type you want. **Verify:** the printed path is `docs/adr/<slug>.md` and `list` shows it as `proposed`.
3. **Gather context before writing.** Run `pantheon-adr draft` for the branch's commits, its changed-file totals, any earlier decisions over the same files, and the list of sections still unfilled. `draft --bootstrap` turns the same material into a numbered question set. This is optional, and faster than reconstructing the branch by hand.
4. **Fill the template in place.** Replace every `<!-- ... -->` placeholder with real prose and keep every heading exactly as generated. The rubric needs 30 or more words under `### Context`, 30 under `### Chosen Solution`, 20 under `### Rationale`, and one `## Impact Assessment` entry whose value is not `none`. **Stop if:** you cannot name a real rejected alternative under Options Considered; that gap means the decision is not yet understood.
5. **Score it.** Run `pantheon-adr check <slug>` and iterate until the score is at least 80, or 95 with `--strict`. **Verify:** exit code 0 and `[complete]` in the output.
6. **Request review.** Run `pantheon-adr review <slug>` to move the status to `review-requested`. It re-runs the check and refuses below 80.
7. **Accept once ratified.** Run `pantheon-adr update <slug> -s accepted`. The transition is appended to the record's history automatically.
8. **Regenerate the catalogue and commit the record with it.** `pantheon-adr index`, then commit both. Do not hand-edit `index.md`.
9. **Supersede when the decision changes.** Create the replacement first, then run `pantheon-adr update <old-slug> --superseded-by <new-slug>`. The old record's status becomes `superseded`, the replacement gains a `supersedes` link back, and neither record's prose is touched. **Verify:** `pantheon-adr list -s superseded` shows the old slug.

### Deriving an ADR from an existing document

Sometimes step 4 isn't a blank page: the decision was already made and written
down in a design doc, review, or planning note, and the task is to capture it
rather than author it. The steps above still apply, but recognising that a
document contains a binding decision, and linking the record back to it for
provenance, takes more care than filling in a decision you just made yourself.
See [Deriving an ADR from an Existing Document](references/context-extraction.md).

## Quick Commands

```bash
# Bootstrap an ADR log under docs/adr.
pantheon-adr init
```

Expected result: `Initialised the ADR log for <project> at docs/adr`.

```bash
# Create the record for the current branch.
pantheon-adr create -d "Adopt OpenTelemetry for tracing"
```

Expected result: on branch `feat/adopt-opentelemetry`, prints `Created docs/adr/adopt-opentelemetry.md` with frontmatter `branch_type: feat`, `status: proposed`.

```bash
# Read the branch, or turn it into a question set.
pantheon-adr draft
pantheon-adr draft --bootstrap
```

Expected result: branch, base, commit list, changed-file totals, earlier decisions over the same files, and the sections still to address.

```bash
# Score the record. The slug is required.
pantheon-adr check adopt-opentelemetry
```

Expected result: `Score: 100/100  [complete]` and exit 0 at or above 80; below that, the failing rules with their weights and exit 1.

```bash
# Move through the lifecycle.
pantheon-adr review adopt-opentelemetry
pantheon-adr update adopt-opentelemetry -s accepted --tags tracing,observability
```

Expected result: `adopt-opentelemetry marked for review (score 100/100)`, then `Updated adopt-opentelemetry: tags, status to accepted`.

```bash
# Retire a decision in favour of a newer one (create the replacement first).
pantheon-adr update adopt-opentelemetry --superseded-by adopt-grafana-tempo
```

Expected result: the old record becomes `superseded` and the replacement gains `supersedes: adopt-opentelemetry`.

```bash
# Inspect the log and regenerate the catalogue.
pantheon-adr list
pantheon-adr list -s accepted
pantheon-adr status --json
pantheon-adr index
```

Expected result: one line per record as `<type> <slug> <date> <status> <description>`; `status --json` emits `{"total":N,"by_status":{...},"by_type":{...}}` for CI gates.

```bash
# Wire the checks into the harness and into git.
pantheon-adr sync
pantheon-adr init --install-hooks
```

Expected result: `sync` patches `.claude/settings.json` or the OpenCode plugin and is idempotent; `--install-hooks` adds a pre-push hook that blocks while any proposed record scores below 80.

```bash
# Find planning documents with a decision that no record points back at.
scripts/check-undocumented-decisions.sh
```

Expected result: exit 0 with a confirmation line when everything is covered, or exit 2 with a list of undocumented files. See [Deriving an ADR from an Existing Document](references/context-extraction.md).

## Anti-Patterns

### NEVER hand-write a record's markdown file

- **WHY:** `create` writes the YAML frontmatter that every other command reads. A file without it fails to parse, and because `list` reports a malformed record rather than skipping it, one hand-made file breaks the whole log.
- **BAD:** `cat > docs/adr/adopt-grpc.md` with a copied template.
- **GOOD:** `pantheon-adr create adopt-grpc -d "..."`, then editing the body it generated.
- **Consequence:** `list`, `status` and `check` all fail with a parse error until someone finds the file you wrote.

### NEVER hand-edit the frontmatter to change status

- **WHY:** `review` and `update` append a timestamped entry to the record's `history` as they move the status. Editing `status:` directly skips that, so the audit trail silently claims the record went straight from `proposed` to `accepted` with nobody reviewing it.
- **BAD:** opening the record and changing `status: proposed` to `status: accepted`.
- **GOOD:** `pantheon-adr review <slug>` then `pantheon-adr update <slug> -s accepted`.
- **Consequence:** the history is a lie, and the one thing an ADR log is for is being trustworthy about what happened when.

### NEVER edit an accepted record to change its decision

- **WHY:** Records are an audit trail. Rewriting the Chosen Solution erases the evidence that a different choice was once correct, and severs the reasoning chain reviewers rely on.
- **BAD:** opening `use-rest-for-internal-services.md` and replacing "We will use REST" with "We will use gRPC".
- **GOOD:** create `adopt-grpc-for-internal-services`, then `pantheon-adr update use-rest-for-internal-services --superseded-by adopt-grpc-for-internal-services`.
- **Consequence:** history lies; a future reader cannot tell the decision ever changed or why.

### NEVER rename or reorder the template headings

- **WHY:** `check` scores sections by heading name. Renaming `### Chosen Solution` drops its 25 points *and* triggers a cap that holds the whole record at 60, which is below the review threshold.
- **BAD:** replacing `### Rationale` with `### Why`.
- **GOOD:** keep the generated heading text verbatim and put your prose beneath it.
- **Consequence:** a fully written record that cannot reach 80, and so cannot pass `review`.

### NEVER leave `<!-- ... -->` placeholders in a record you are calling done

- **WHY:** Placeholder count is a scored rule in its own right: any remaining comment forfeits 20 points, and three or more cap the total at 70.
- **BAD:** filling Context and Chosen Solution but leaving the Outcome & Lessons comment in place.
- **GOOD:** replace every comment with real text, or delete the ones that genuinely do not apply.
- **Consequence:** `review` refuses the record, and the reason ("15 unfilled placeholder(s) remain") reads as carelessness rather than judgement.

### NEVER leave Options Considered empty or padded with filler

- **WHY:** The section is the evidence the decision was weighed against real options. The checker also runs a boilerplate filter: prose that is mostly the words describe, decision, context, placeholder, example, tbd or todo scores as if the section were empty, cap included.
- **BAD:** `### Options Considered` followed by `TBD`.
- **GOOD:** each rejected option with the concrete reason it lost.
- **Consequence:** reviewers cannot judge whether the decision was sound, so they either block it or rubber-stamp it.

### NEVER hand-edit the generated catalogue

- **WHY:** `index.md` is rendered from the records' frontmatter and carries a banner saying so. The next `pantheon-adr index` overwrites whatever you wrote.
- **BAD:** correcting a title or status directly in `index.md`.
- **GOOD:** fix the record's frontmatter (or its status, through `update`), then re-run `pantheon-adr index`.
- **Consequence:** the correction disappears on the next run, and until then the catalogue disagrees with the records it claims to describe.

### NEVER record multiple unrelated decisions in one record

- **WHY:** A record that decides two things cannot be superseded independently. When one half changes you must either fork it or retire a still-valid decision.
- **BAD:** one record titled "Database and CI runner choices".
- **GOOD:** two records, `choose-postgresql` and `adopt-self-hosted-runners`.
- **Consequence:** the log tangles; superseding the database choice wrongly retires the CI decision too.

## References

- [ADR Lifecycle](references/adr-lifecycle.md) — the five statuses, their transitions, the history trail, and how superseding links both records
- [CLI Usage](references/cli-usage.md) — every command and flag, the completeness rubric and its weights, exit codes, and JSON output shapes
- [Deriving an ADR from an Existing Document](references/context-extraction.md) — recognising a binding decision already written down in a design doc, review, or planning note, and linking the record back to it for provenance
- [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) — Michael Nygard's original essay grounding the ADR practice
- [MADR templates](https://adr.github.io/madr/) — widely used Markdown ADR template variants for comparison
