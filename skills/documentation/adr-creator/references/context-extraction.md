# Deriving an ADR from an Existing Document

Most records get written forwards: a decision is made, then `pantheon-adr
create` opens a fresh one for it. Sometimes the order is reversed. A decision
was already made and written down, just not as a record: it is sitting inside a
design doc, a review, a retrospective, a planning note, or an analysis document
that existed for some other purpose. This reference covers that extraction case:
how to recognise a binding decision buried in existing prose, and how to turn it
into a record without losing the link back to where it came from.

Everything else about records is unchanged: immutability once accepted, the
supersede workflow, the status transitions. See
[ADR Lifecycle](adr-lifecycle.md) for those rules; this document only covers the
extraction step that happens before `create` runs.

## When this applies

- A design doc, spike write-up, or planning note already states a choice ("we
  will use X", "the recommended approach is Y") but no record exists yet.
- A retrospective or incident review reaches a conclusion that changes how the
  team will build something going forward.
- Someone asks you to "write up that decision from the doc" rather than handing
  you a decision to record from scratch.

If the source document only observes or analyses without landing on a choice,
there is nothing to extract yet. See **Recognising a binding decision** below
before creating anything.

## Recognising a binding decision in existing prose

Not every sentence that sounds decisive is a decision worth a record, and not
every decision announces itself with a heading. Look for a choice that meets
both of these:

1. **It is settled, not proposed.** The document states what *will* happen or
   what *was* chosen, not a menu of options still being weighed. "We should
   probably consider Postgres" is not yet a decision; "we will use Postgres for
   the audit log" is.
2. **It is binding on future work.** The choice constrains architecture, a
   convention, a process, or a hard-to-reverse technical direction, rather than
   a one-off implementation detail.

A quick scan of section headings and phrasing narrows down where to look:

| Signal in the source document | Likely a decision? |
| --- | --- |
| A section titled "Decision", "Recommendation", "Recommended Approach", or "Proposed Approach" | Yes, read it first |
| "We will...", "Adopt Option A", "Going forward, ..." | Yes |
| An "Open Questions" or "Options" section, later resolved elsewhere | Yes once resolved; do not extract while still open |
| A "Summary" or "Findings" section with no stated direction | No, this is observational |
| An inline aside ("worth noting that...") | No, too ephemeral for a record |

When in doubt, ask: if this document disappeared tomorrow, would future
contributors need to know this choice was made and why? If yes, it earns a
record.

## Workflow

1. **Confirm no record already covers this decision.** Run `pantheon-adr list`
   and skim the slugs and descriptions. Extracting a decision that already has a
   record produces a duplicate; supersede instead if the decision has since
   changed (see [ADR Lifecycle](adr-lifecycle.md)).
2. **Create the record with an explicit slug.** Run `pantheon-adr create <slug>
   -d "<description>"`. Pass the slug rather than relying on the branch name: an
   extracted decision rarely has a branch of its own, and the slug should name
   the decision, not whatever branch you happen to be on. The record is stamped
   with today's date and status `proposed`; the fact that the decision was made
   earlier does not change how it is created.
3. **Fill Problem Statement, Chosen Solution and Rationale from the source
   document, not from memory.** Pull the forces, constraints and reasoning out
   of the original prose and rewrite them so the record stands alone: a reader
   should not need the source document to understand the decision. Do not
   copy-paste a paragraph that assumes context the record does not have.
4. **Backfill Options Considered if the source document lacks it.** Planning
   notes often settle on an approach without formally listing what else was
   considered. If the document names other options anywhere, even in passing,
   pull them in with the reason each was rejected. An empty section here is not
   acceptable just because the source document had one. See the skill's own
   [Anti-Patterns](../SKILL.md#anti-patterns).
5. **Record the provenance** in frontmatter, using the convention below.
6. **Score it.** Run `pantheon-adr check <slug>`. An extracted record is held to
   the same threshold as one written forwards, and extraction tends to leave
   Impact Assessment and Risks & Pitfalls untouched, because the source document
   had nothing to say about them. Fill them from your own judgement rather than
   leaving placeholders.

## Linking back to the source document

The record's frontmatter has a `related` list for exactly this. It is structured
data, so the link is machine-readable and there is only one place to look:

```yaml
related:
- ../../.context/plans/2026-02-03-caching-spike.md
- https://example.atlassian.net/browse/PROJ-1234
```

Paths are relative to the record, links are absolute URLs, and a ticket
reference counts. Put the PR number in the `pr` field when the decision came out
of a pull request discussion rather than a document.

Do not also write a `Source:` bullet into the prose. Two copies of the same link
is the drift this record format was designed to avoid; the frontmatter is the
one to keep.

The convention is a documentation practice, not something the CLI validates.
Nothing breaks if it is omitted, but omitting it loses the chain between the
decision and the document that motivated it, so treat it as expected for any
record created this way.

## Anti-patterns specific to extraction

**NEVER** extract a decision that is still framed as an open question in the
source document.
**WHY:** A record states what *was* decided. Recording an unresolved option as
settled misrepresents the state of the decision, and will need correcting or
superseding as soon as the team actually decides.
**BAD:** turning a planning doc's "Option A looks promising" into a record
titled "Adopt Option A" before anyone signed off.
**GOOD:** wait until the source document, or a follow-up, states the choice was
made. Or leave the record at its creation status of `proposed` and say in the
prose that nothing is ratified yet.

**NEVER** skip the provenance link because the decision "is obvious from
context".
**WHY:** The person reading the record in six months has neither the source
document open nor the memory of which review it came from. Without the link they
cannot verify the reasoning or find the detail the record necessarily left out.
**GOOD:** always add the source to `related`, even for a decision that feels
self-explanatory today.

## Optional: spotting decisions with no record yet

If a team accumulates design docs, reviews, or planning notes as a matter of
course, it is worth periodically checking whether any of them contain a decision
(per the signals above) that never got a record.
`scripts/check-undocumented-decisions.sh` automates this: it scans a
planning-document directory (`.context` by default) for the same
decision-shaped headings and phrasing listed above, cross-references them
against the `related` lists of every existing record, and reports any document
that looks decided but is not linked from one (exit 2), or confirms everything
is covered (exit 0).

Pass `--adr-dir` when records do not live in `docs/adr`, and `--source-dir` when
planning documents live somewhere other than `.context`. The ADR directory is
excluded from the scan, so it is safe to run even when the two overlap.

Like the `related` convention itself this is a documentation-side check, not
something `pantheon-adr` runs internally. Wire it into CI or a pre-commit hook
if you want it enforced rather than run on demand.
