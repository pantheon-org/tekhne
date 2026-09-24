# Compliance

Purpose

- Ensure every journal entry follows the project's structure, formatting, and quality standards so entries are discoverable, consistent, and machine-validated.

Quick automation (recommended)

- Format: `npx prettier --write path/to/entry.md`
- Lint: `npx markdownlint-cli2 path/to/entry.md`
- Validator: `bash scripts/validate-journal-entry.sh path/to/entry.md` (accepts multiple files)

Required tools

- Node.js + `npx` (for Prettier and markdownlint-cli2)
- `bash`, `perl`, `rg` (ripgrep), and GNU awk (or `gawk`) for the repository validator

YAML frontmatter (standardized)

All templates now include a standard YAML frontmatter that journal entries should adopt when appropriate. Fields:

- `title` (string): descriptive title
- `date` (YYYY-MM-DD): entry date — must match filename prefix
- `authors` (array): list of author names
- `tags` (array): lowercase, hyphen-separated tags
- `source` (string): optional URL or origin
- `status` (string): `draft`, `published`, or other workflow states

Example frontmatter:

```yaml
---
title: "Example — Investigating Service Crash"
date: 2025-09-01
authors:
  - Alex Rivera
tags:
  - troubleshooting
  - systemd
source: ""
status: draft
---
```

Checklist (run for every new or updated entry)

- [ ] Filename: `YYYY-MM-DD-slug.md` (ISO 8601 date prefix must match entry date)
- [ ] Location: placed under `YYYY/MM-Month/DD-Weekday/` where YYYY, MM, and DD match the filename
- [ ] Single H1 with full date: `# Title - Month D, YYYY` (exactly one H1)
- [ ] `## Session Overview` (or an equivalent short context section) present
- [ ] Required sections for the entry type present:
  - Troubleshooting: Problem Description, Investigation Process, Resolution Steps, Session Outcome
  - Learning: Context, Key Learning, Solution/Process, Use Cases
  - Ticket Refinement (`refinement_ticket` set): Current Ticket State, Findings, Proposed Ticket Description
  - Ticket Kickoff (`kickoff_ticket` set): Ticket Summary, Conditions of Satisfaction & Acceptance Criteria, Open Questions & Gaps, Supporting Information, Work Checklist, Proof of Work Plan
- [ ] Continuation links (`continues_from`/`continued_by`, if set): linked entry exists, links back reciprocally, and carries the matching `## Related Entries` section or "Superseded" banner
- [ ] Executive Summary (if present): placed as the H2 immediately after `## Session Overview`, nowhere else
- [ ] All fenced code blocks have language specifiers (e.g., ```bash)
- [ ] No duplicate headings (MD024) and no multiple consecutive blank lines (MD012)
- [ ] Tags present and correctly formatted (lowercase, hyphen-separated, nested allowed)
- [ ] Images include alt text where present
- [ ] No trailing whitespace; prefer line length ≤ 120 characters when practical
- [ ] Prettier-format and markdownlint report zero errors
- [ ] Commit message follows convention: `Add journal entry: [Brief Description] (YYYY-MM-DD)`

## Compliance Section

Every dated, non-template entry MUST include a `## Compliance` section (validate-journal-entry.sh returns exit code 8 if it's missing — this applies regardless of entry type). It is a self-assessed
checklist, placed near the end of the entry (conventionally just before `## Tags`), using GitHub-flavored checkboxes:

```markdown
## Compliance

- [x] Filename matches the entry date (ISO 8601, `YYYY-MM-DD-slug.md`)
- [x] Location matches YYYY/MM/DD from the filename
- [x] Single H1 with the full date
- [x] Session Overview present
- [x] All fenced code blocks have language specifiers
- [x] Tags present, lowercase/hyphenated, matching frontmatter
- [x] No special-category or third-party personal data recorded (or the entry states what is involved and why recording it is permitted)
- [x] No credentials, tokens, secrets, or account/org identifiers included verbatim
- [x] Any code or ticket changes made during this session are disclosed
- [x] No proprietary third-party source code was inspected or reproduced without confirming license terms
```

Check an item honestly against what the entry actually contains — don't tick a box that doesn't hold. If an item doesn't apply (e.g. no personal data was ever in scope), tick it and say so briefly
rather than leaving it unchecked.

Common validator failures & fixes

- `Single H1` errors: remove extra H1s or convert them to H2 (`##`) and ensure date is present in the H1.
- `Code block language missing`: add the language after the opening fence, e.g., ```bash
- `Duplicate headings` or `multiple blank lines`: remove the duplicate section or extra blank lines.
- `Filename/location mismatch`: move or rename the file so the path and filename date match the H1 date.

`validate-journal-entry.sh` exit codes (non-zero = failed; each check is a no-op when its triggering frontmatter field is absent, so it never fires for entries that don't use the feature)

| Exit | Meaning | Fix |
| ---- | ------- | --- |
| 2 | Usage error / gawk missing | Pass at least one file; install `gawk` |
| 3 | File not found | Check the path |
| 4 | Not exactly one H1 | Ensure a single `# Title - Month D, YYYY` heading |
| 5 | Filename missing `YYYY-MM-DD` prefix | Rename the file |
| 7-9 | Missing `## Session Overview` / `## Compliance` / `## Tags` | Add the missing section |
| 10-11 | Bad filename slug or code fence without a language specifier | Lowercase the slug; add a language to the fence |
| 11 | Heading with trailing punctuation | Remove the trailing `.`/`:`/`;`/`!`/`?` |
| 12-14 | Tags section empty, no tokens, or non-conforming | Match tags to the frontmatter array; lowercase, hyphenated |
| 15 | H1 date does not match filename date | Reformat the H1 date to `Month D, YYYY` |
| 16 | `refinement_ticket` set but no `## Proposed Ticket Description` | Add the section (see Proposed Ticket Description in SKILL.md) |
| 17 | `continues_from` target file does not exist | Fix the relative path |
| 18 | Target does not reciprocally set `continued_by` back to this file | Set `continued_by` on the older entry |
| 19 | `continues_from` set but no `## Related Entries` section | Add the section (see Continuation Links in SKILL.md) |
| 20 | `continued_by` target file does not exist | Fix the relative path |
| 21 | `continued_by` set but no "Superseded" banner after the H1 | Add the banner (see Continuation Links in SKILL.md) |
| 22 | `kickoff_ticket` set but a required section is missing | Add all five sections (see Ticket Kickoff in SKILL.md) |
| 23 | `## Executive Summary` present but not immediately after `## Session Overview` | Move it to directly follow `## Session Overview` |
| 24 | Emoji found outside a fenced code block | Remove it; use bold plain text instead (e.g. `**Resolved**`) |
| 25 | `jira_ticket` set but no `## Jira Comment Draft` | Add the section (see Jira Comment Draft below) |

## Ticket Kickoff Rules

Applies to ticket-kickoff sessions: understanding an issue-tracker ticket before implementation — pulling its CoS/AC and supporting information, surfacing gaps, building a work checklist, and planning the evidence that will prove it's done. Distinct from Ticket Refinement: kickoff plans the work assuming the ticket is already implementation-ready; it never rewrites the ticket description. Full section-by-section requirements and examples live in `ticket-kickoff.yaml` — read it before generating.

When an entry kicks off a ticket, you MUST:

1. Set `kickoff_ticket: <KEY>` in the frontmatter. This makes five sections REQUIRED (enforced by the validator, no-op otherwise): `## Conditions of Satisfaction & Acceptance Criteria`, `## Open Questions & Gaps`, `## Supporting Information`, `## Work Checklist`, `## Proof of Work Plan`.
2. Ground CoS/AC in what was actually pulled from the tracker — quote or closely paraphrase, don't lose specifics.
3. Never leave `## Open Questions & Gaps` blank — state `None identified` explicitly when nothing is open. Before writing that, verify the first Work Checklist item is actually self-service (not gated behind an unconfirmed process, tool, or permission) — a well-specified ticket can still hide an execution-mechanics gap.
4. Hyperlink every reference in `## Supporting Information` — no bare keys or paths.
5. Make `## Work Checklist` GitHub-flavored checkboxes (`- [ ]`), granular and traceable back to specific CoS/AC items.
6. Make `## Proof of Work Plan` a table (CoS/AC item -> evidence -> where captured). It's a PLAN, not the evidence itself — the artifacts are collected in a later completion entry, using the `<slug>/assets/` convention in SKILL.md.

If a kickoff surfaces gaps serious enough to need rewriting the ticket description, do a Ticket Refinement instead (or first).

## Jira Comment Draft

Applies to ANY entry type whose work maps to a Jira ticket (a review, an incident, a spike, a piece of learning done under a ticket). When an entry relates to a ticket, you MUST:

1. Set `jira_ticket: <KEY>` in the frontmatter (e.g. `jira_ticket: CC-1218`).
2. Add a `## Jira Comment Draft` section, placed after the narrative sections and before `## Tags`.

The section holds a ready-to-paste draft comment for that ticket, written in the voice you would post to it:

```markdown
## Jira Comment Draft

> Draft for CC-1218 - review before posting; not yet posted.

Reviewed the runbook against the acceptance criteria: five of seven MUST conditions are fully met... [and so on]
```

Rules for the draft:

- It is a DRAFT. Lead with the `> Draft for [TICKET] - review before posting; not yet posted.` banner and never post it to Jira from this skill. Posting to a ticket is a separate step the user
  explicitly confirms.
- It must be self-contained. A ticket reader has not seen the journal entry, so restate what was done, the key findings, the recommendation, and any decision the ticket owner needs to make — do not
  write "see the journal".
- Keep it concise and ticket-appropriate; the journal entry holds the full detail, the comment is the summary.

## Continuation Links

Applies to ANY entry type when work spans more than one dated entry (e.g. a multi-day spike). Never fold a later day's narrative into an earlier entry as an inline "Update (DD-MM-YYYY)" — that breaks the single-date H1 and buries current status behind old content. Instead, create a new dated entry and link both directions so a reader landing on either one finds the other.

**Older entry (continued):** frontmatter gets `continued_by: <relative-path>` and `status: superseded`, plus a banner immediately after the H1. **Newer entry (continuing):** frontmatter gets `continues_from: <relative-path>`, plus a `## Related Entries` section immediately after the metadata block, before `## Session Overview`:

```markdown
[on the older entry, after the H1] > **Superseded — see latest:** [Title](RELATIVE-PATH-TO-NEWER-ENTRY.md) (DD-MM-YYYY) — what changed.

[on the newer entry, after the metadata block]
## Related Entries

- **Continues from:** [Title](RELATIVE-PATH-TO-OLDER-ENTRY.md) (DD-MM-YYYY) — what that entry covered.
```

The validator checks this bidirectionally whenever `continues_from`/`continued_by` is set: the linked file must exist, must link back, and the matching banner/section must be present. No-op otherwise.

CI and pre-commit behavior

- Pre-commit (lefthook) runs Prettier, markdownlint, then the validator for staged files under `202*/**/*.md`.
- CI should treat validator failures as blocking; run the same commands used locally.
- If Prettier fixes files during pre-commit, re-run linters/validator to ensure a clean state before committing.

Notes for AI agents

- Run the validator and linter automatically on any files you create or update, fix issues programmatically when safe (formatting, adding code fence languages), and prompt the user for ambiguous fixes
  (renaming files, content deletions).

Reference

- This checklist is canonical: `.opencode/checklist/compliance.md` — link to this file from `AGENT.md` and other contributing docs.
