# Deriving an ADR from an Existing Document

Most ADRs get written forwards: a decision is made, then `pantheon-adr new`
creates a fresh record for it. Sometimes the order is reversed — a decision
was already made and written down, just not as an ADR. It is sitting inside
a design doc, a review, a retrospective, a planning note, or an analysis
document that existed for some other purpose. This reference covers that
extraction case: how to recognize a binding decision buried in existing
prose, and how to turn it into a proper ADR without losing the link back to
where it came from.

Everything else about ADRs — immutability once accepted, the supersede
workflow, status transitions — is unchanged. See
[ADR Lifecycle](adr-lifecycle.md) for those rules; this document only covers
the extraction step that happens before `pantheon-adr new` runs.

## When this applies

- A design doc, spike write-up, or planning note already states a choice
  ("we will use X", "the recommended approach is Y") but no ADR exists yet.
- A retrospective or incident review reaches a conclusion that changes how
  the team will build something going forward.
- Someone asks you to "write up that decision from the doc" rather than
  handing you a decision to record from scratch.

If the source document only observes or analyzes without landing on a
choice, there is nothing to extract yet — see **Recognizing a binding
decision** below before creating anything.

## Recognizing a binding decision in existing prose

Not every sentence that sounds decisive is a decision worth an ADR, and not
every decision announces itself with a heading. Look for a choice that meets
both of these:

1. **It is settled, not proposed.** The document states what *will* happen
   or what *was* chosen, not a menu of options still being weighed. "We
   should probably consider Postgres" is not yet a decision; "we will use
   Postgres for the audit log" is.
2. **It is binding on future work.** The choice constrains architecture,
   a convention, a process, or a hard-to-reverse technical direction —
   not a one-off, easily-reversed implementation detail.

A quick scan of section headings and phrasing narrows down where to look:

| Signal in the source document | Likely a decision? |
| --- | --- |
| A section literally titled "Decision", "Recommendation", "Recommended Approach", or "Proposed Approach" | Yes — read it first |
| "We will...", "Adopt Option A", "Going forward, ..." | Yes |
| An "Open Questions" or "Options" section, later resolved elsewhere in the doc or in a follow-up | Yes, once resolved — don't extract while still open |
| A "Summary" or "Findings" section with no stated direction | No — this is observational, not a decision |
| An inline aside or comment ("worth noting that...") | No — too ephemeral for an ADR |

When in doubt, ask: if this document disappeared tomorrow, would future
contributors need to know this choice was made and why? If yes, it earns an
ADR.

## Workflow

1. **Confirm no ADR already covers this decision.** Run `pantheon-adr list`
   and skim titles. Extracting a decision that already has a record produces
   a duplicate; supersede instead if the decision has since changed (see
   [ADR Lifecycle](adr-lifecycle.md)).
2. **Create the record exactly as you would for a fresh decision.** Run
   `pantheon-adr new "<Title>"`. The tool still derives the next number and
   stamps today's date — the fact that the decision was made earlier does
   not change how the record is created. **Verify:** the printed path
   matches the number expected from `pantheon-adr list`.
3. **Fill Context, Decision, and Consequences from the source document, not
   from memory.** Pull the forces, constraints, and reasoning out of the
   original prose and rewrite them so the ADR stands alone — a reader should
   not need the source document to understand the decision. Do not just
   copy-paste a paragraph verbatim if it assumes context the ADR doesn't
   have.
4. **Backfill Alternatives Considered if the source document doesn't have
   it.** Planning notes and reviews often settle on an approach without
   formally listing what else was considered. If the source document names
   other options anywhere (even in passing, or in an "Options" section that
   was later resolved), pull them into this section with the reason each
   was rejected. An empty section here is not acceptable just because the
   source document didn't have one — see the CLI skill's own
   [Anti-Patterns](../SKILL.md#anti-patterns) on this.
5. **Link back to the source document for provenance**, using the
   convention below.

## Linking back to the source document

`pantheon-adr` does not currently have a structured field dedicated to "this
ADR was extracted from document X" — checked in the CLI's own template
renderer, which only defines `**Technical Story:**` (a link/ticket line) and
a `## References` section with generic link bullets. Until the tool grows a
purpose-built field, use this convention so provenance is still visible and
consistent between ADRs:

- Add a `- Source: <path-or-link>` bullet under **Context**, right after the
  opening paragraph, naming the document the decision was extracted from
  (a relative repo path, a doc link, or a review/ticket URL).
- Also add the same link under **References** if the document is durable
  and worth pointing readers to directly (the `## References` section
  already invites "[Link to relevant docs]" — a source document counts).

Example `## Context` opening:

```markdown
## Context

- Source: docs/design/ingestion-pipeline-review.md

What is the issue we're facing in a given context?
...
```

This is a documentation convention, not something the CLI validates or
enforces — nothing breaks if it's omitted, but omitting it silently loses
the provenance chain between the decision and the document that motivated
it, so treat it as expected practice for any ADR created this way.

## Anti-patterns specific to extraction

**NEVER** extract a decision that is still framed as an open question in
the source document.
**WHY:** An ADR states what *was* decided. Recording an unresolved option
as if it were settled misrepresents the actual state of the decision and
will need correcting (or superseding) as soon as the team actually decides.
**BAD:** Turning a planning doc's "Option A looks promising" into an ADR
titled "Adopt Option A" before anyone signed off on it.
**GOOD:** Wait until the source document (or a follow-up) states the choice
was made, then extract it — or create the ADR with status `Proposed` only if
the team already treats proposals as ADRs (see the main SKILL's
[When Not to Use](../SKILL.md#when-not-to-use)).

**NEVER** skip the source link because the decision "is obvious from
context".
**WHY:** The person reading the ADR in six months has neither the source
document open nor the memory of which review it came from. Without the
link, they cannot verify the reasoning or find related detail the ADR
necessarily left out.
**GOOD:** Always add the `Source:` line under Context, even for a decision
that feels self-explanatory today.

## Optional: spotting decisions with no ADR yet

If a team accumulates design docs, reviews, or planning notes as a matter of
course, it can be worth periodically checking whether any of them contain a
decision (per the signals above) that never got an ADR.
`scripts/check-undocumented-decisions.sh` automates this: it scans a
planning-document directory (`.context` by default) for the same
decision-shaped headings and phrasing listed above, cross-references them
against every existing ADR's `Source:` line, and reports any document that
looks decided but isn't linked from an ADR yet (exit 2), or confirms
everything is covered (exit 0). Pass `--source-dir` if planning documents
live somewhere other than `.context`, or `--adr-dir` to match a non-default
`pantheon-adr new --dir`. Like the `Source:` convention itself, this is a
documentation-side check, not something `pantheon-adr` runs internally —
wire it into CI or a pre-commit hook if you want it enforced rather than
run on demand.
