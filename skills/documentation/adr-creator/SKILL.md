---
name: adr-creator
description: "Creates, lists, and supersedes Architecture Decision Records with the adr CLI, following the house ADR template. Use when recording an architectural decision, writing an ADR, documenting a technical choice, superseding a prior decision, numbering a new decision record, bootstrapping an ADR log under docs/adr."
---

# Authoring Architecture Decision Records

## Mindset

An ADR is a dated, immutable record of one decision and the forces that shaped it, not living documentation. Its value is archaeological: a reader six months from now must understand *why* a choice was made without asking anyone. Once accepted, an ADR is never edited to change its meaning. When the decision changes, you write a new ADR and mark the old one superseded, preserving the chain of reasoning.

Two rules make an ADR log trustworthy over time. First, **numbering is append-only and gap-tolerant**: the next number is always `max(existing) + 1`, never a count of files, so deleting or archiving a record never reuses an identifier. Second, **status is the only mutable field**: an accepted decision's Context and Decision text are frozen; only its Status flips to `Superseded` when a later ADR replaces it. The `adr` CLI enforces both rules, which is why you should reach for it rather than hand-writing files that drift from the template.

Treat the template's section order as a contract. Downstream tooling and reviewers scan for `## Context`, `## Decision`, and `## Consequences` in that order. Reordering or renaming them breaks that expectation even when the prose is good.

## Prerequisites

This skill drives the `pantheon-adr` CLI and is distributed by it
(`pantheon-adr skill install`). Every step below invokes that binary, so it must
be on `PATH`. Confirm before proceeding:

```bash
pantheon-adr --version
```

If it is not found, the skill was installed without its companion CLI. Install
the `pantheon-adr` CLI (its release binary, or `cargo install`) and retry; there
is no self-contained fallback for these commands.

## When to Use

- The user asks to record, write, or draft an architectural or technical decision.
- A significant, hard-to-reverse choice was just made (framework, data store, protocol, boundary) and needs a durable rationale.
- An earlier decision is being replaced and the old record must be marked superseded while keeping its history.
- A repository needs an ADR log bootstrapped under `docs/adr`.

## When Not to Use

- The change is routine and reversible (a dependency bump, a rename, a config tweak). Use a commit message, not an ADR.
- The user wants prose design documentation or a runbook. ADRs capture a single decision, not a system overview.
- A decision is still being debated with no chosen option. Reach a decision first, or record it with status `Proposed` only if the team parks proposals as ADRs.

## Principles

1. One ADR records exactly one decision. If you are tempted to write "and also", split it into two records.
2. Numbers are assigned by the tool from the highest existing record, never guessed or hand-typed.
3. Accepted ADRs are immutable except for their Status line. Supersede, do not rewrite.
4. The Context must state the forces at play so the decision reads as inevitable, not arbitrary.
5. Alternatives Considered is mandatory evidence of due diligence, even when the answer was obvious.

## Procedure

1. **Locate the ADR directory.** Default is `docs/adr`; a repository may override it with the `ADR_DIR` environment variable or a `--dir` flag. Confirm which applies before creating records. **Verify:** `pantheon-adr list --dir <path>` runs without error.
2. **Create the record.** Run `pantheon-adr new "<Title>"`. The tool computes the next number, slugs the title into `NNNN-kebab-title.md`, and stamps today's date with status `Proposed`. **Verify:** the printed path matches the number you expected from `pantheon-adr list`.
3. **Fill the template in place.** Replace the placeholder prose under Context, Decision, and Consequences. Keep every heading; delete only the placeholder bullet text. **Stop if:** you cannot articulate at least one entry under each of Positive, Negative, and Neutral consequences; that gap means the decision is not yet understood.
4. **Record alternatives honestly.** For each option not chosen, give its pros, cons, and the specific reason it was rejected. An empty Alternatives section fails review.
5. **Set the final status.** Change `Proposed` to `Accepted` once the decision is ratified. Do not touch any other field after acceptance.
6. **Supersede when the decision changes.** Run `pantheon-adr supersede <old-number> "<New Title>"`. This flips the old record's Status to `Superseded by ADR-NNNN` and creates a new Accepted record that references the old one. **Verify:** `pantheon-adr list` shows the old record as superseded and the new record directly after it.

## Quick Commands

```bash
# Create the next-numbered ADR from the house template.
pantheon-adr new "Adopt OpenTelemetry for tracing"
```

Expected result: prints `Created docs/adr/0001-adopt-opentelemetry-for-tracing.md` (number varies with existing records).

```bash
# List every ADR with its number, status, and title.
pantheon-adr list
```

Expected result: one line per record, e.g. `ADR-0001  Accepted  Adopt OpenTelemetry for tracing`.

```bash
# Supersede an earlier decision, marking it and linking the replacement.
pantheon-adr supersede 1 "Adopt Grafana Tempo for tracing"
```

Expected result: prints the superseded path and the new `Created` path; the old record's Status becomes `Superseded by ADR-0002`.

```bash
# Work against a non-default ADR directory.
pantheon-adr new "Split the monolith" --dir architecture/decisions
```

Expected result: the record is created under `architecture/decisions`.

## Anti-Patterns

### NEVER hand-number a new ADR

- **WHY:** Guessing the next number races with other records and reuses identifiers after deletions, corrupting cross-references. The tool derives the number from the highest existing record, which is gap-tolerant.
- **BAD:** creating `docs/adr/0003-...md` by hand because "there are three files" when the highest existing number is 0005.
- **GOOD:** `pantheon-adr new "..."`, which assigns `max(existing) + 1`.
- **Consequence:** two decisions share ADR-0003 and every link to "ADR-0003" becomes ambiguous.

### NEVER edit an accepted ADR to change its decision

- **WHY:** ADRs are an audit trail. Rewriting the Decision text erases the record that a different choice was once correct and severs the reasoning chain reviewers rely on.
- **BAD:** opening `0002-...md` and replacing "We will use REST" with "We will use gRPC".
- **GOOD:** `pantheon-adr supersede 2 "Adopt gRPC for internal services"`, leaving ADR-0002 intact and marked superseded.
- **Consequence:** history lies; a future reader cannot tell the decision ever changed or why.

### NEVER leave the Alternatives Considered section empty

- **WHY:** The section is the evidence that the decision was weighed against real options. An empty section reads as an unexamined default and fails review.
- **BAD:** `## Alternatives Considered` followed by `- N/A`.
- **GOOD:** each rejected option with its pros, cons, and the concrete reason it lost.
- **Consequence:** reviewers cannot judge whether the decision was sound, so they either block it or rubber-stamp it.

### NEVER rename or reorder the template headings

- **WHY:** Tooling and reviewers scan for the exact headings `## Context`, `## Decision`, `## Consequences` in order. Renaming `## Consequences` to `## Trade-offs` or moving it breaks that contract.
- **BAD:** replacing `## Decision` with `## What We Chose`.
- **GOOD:** keep the heading text verbatim; put your prose beneath it.
- **Consequence:** automated ADR indexes and diff reviews silently skip your record's key sections.

### NEVER record multiple unrelated decisions in one ADR

- **WHY:** A record that decides two things cannot be superseded independently. When one half changes you must either fork the record or supersede a still-valid decision.
- **BAD:** one ADR titled "Database and CI runner choices".
- **GOOD:** two records, `... Choose PostgreSQL` and `... Adopt self-hosted runners`.
- **Consequence:** the log tangles; superseding the database choice wrongly retires the CI decision too.

### NEVER commit an ADR while its status is still Proposed as if it were final

- **WHY:** `Proposed` signals an open question. Merging it as the decision of record makes readers act on a choice the team never ratified.
- **BAD:** merging `**Status:** Proposed` and treating it as accepted in downstream work.
- **GOOD:** flip the Status to `Accepted` (or `Rejected`) once the team decides, then merge.
- **Consequence:** teams build on a decision that was never actually agreed.

## References

- [ADR Lifecycle](references/adr-lifecycle.md) — status transitions, superseding chains, and why accepted records are immutable
- [CLI Usage](references/cli-usage.md) — every adr command, its flags, the ADR_DIR variable, and exit behaviour
- [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) — Michael Nygard's original essay grounding the ADR practice
- [MADR templates](https://adr.github.io/madr/) — widely used Markdown ADR template variants for comparison
