# Mode B: Ticket Enrichment (reference)

Turn a ticket reference in an existing log entry into a dated, self-contained detail entry, cross-linked both
ways. The ticket-detail entry type this mode produces extends `journal-entry-creator`'s frontmatter and tag
taxonomy; see [assets/schemas/ticket-detail-frontmatter.schema.json](../assets/schemas/ticket-detail-frontmatter.schema.json)
and [assets/templates/ticket-detail-template.yaml](../assets/templates/ticket-detail-template.yaml) for the full
contract.

## Reachability

- A ticket key resolves through whatever the issue tracker offers: its API, or an MCP connector if one is
  configured (for a Jira-backed tracker this is typically a `jira_get_issue`-style tool). Old instances sometimes
  migrate to a new URL or a new tracker entirely - verify one key before enriching a batch.
- A reference to a system that is not an issue tracker at all (an ITSM ticket, a support-desk case, a plain
  incident number) is often **not reachable** with the tools available. Leave it as a plain link rather than
  guessing at its content.
- A key from a different project, team, or tracker instance may not resolve even though it looks like a ticket
  key. Treat a non-resolving key as an access gap to flag, not as invented content.

## Fetch

Pull the ticket with its comments - the description alone rarely explains the outcome. For a
`jira_get_issue`-style call, request `fields = "summary,description,status,created,resolutiondate,comment"` with
a sane comment limit (20 is plenty for most tickets).

## "Enough data" bar

Create a detail entry only when the ticket has a usable description and/or substantive comments or a resolution.
A pure one-liner with nothing to narrate stays as the plain mention in the log entry - do not manufacture detail
that is not there. Log what you skip and why; silent truncation reads as "covered everything" when it did not.

## Dating

Resolution/closed date first; else the ticket's last-updated date; else the date of the log entry that mentioned
it. Use `YYYY-MM-DD` for the filename and frontmatter, `Month D, YYYY` for the H1, `DD-MM-YYYY` inside prose.

## The Decisions and Resolution section

Synthesise what was decided and how it resolved. Do **not** transcribe the thread comment-by-comment ("he said,
she said"). Names live in the metadata `Contributors` line; name a person in the body only when it carries
weight (who raised a blocker, who closed it).

## Decisive-contribution callout

A blockquote at the end of the section:

```markdown
> **Decisive contribution (Alex Rivera):** <one sentence>.
```

Include it **only** when the author was the pivotal actor - the diagnosis, the design call, the implementation
that shipped. Omit it entirely on team efforts where the author contributed but was not the pivot. This keeps the
callout meaningful rather than decorative.

## Cross-link convention

Use plain-markdown bidirectional links. tekhne's base frontmatter schema has no continuation/supersession field
(nothing equivalent to marking an entry "superseded and hidden" by another) - do not invent one to express this
cross-link. A detail entry elaborates a log entry; it does not replace it.

- Detail entry: `## Related Entries` links to the log entry(ies) that mention the ticket (relative path).
- Log entry: a nested bullet under the ticket's own bullet:

  ```markdown
  - [PROJ-2058](https://.../PROJ-2058): ...original log text...
    - Detailed entry: [PROJ-2058 - Move to fetching credentials at run time](2020-07-06-proj-2058-...md)
  ```

## Batch runs

Enriching many tickets at once is a fan-out job: one agent per ticket. This is where a first attempt hits
avoidable friction - the pitfalls below are drawn from real batch runs.

### The pattern

1. **Scout the work-list inline first.** Extract the distinct ticket keys from the log entries and map each to
   its primary log file. A key is "done" if a detail file already exists for it (`glob 20*/*/*-<key>-*.md`);
   compute remaining from the repo, not from a saved list, so reruns never duplicate.
2. **Fan out one agent per ticket.** Each agent: reads this skill plus a worked example, fetches the ticket with
   comments, applies the enough-data bar, writes the detail file, and **self-validates** with
   `scripts/validate-ticket-detail.sh`, fixing until it passes. It returns a small status record (key, status,
   detail_file, detail_title, log_file), never the file body.
3. **Back-link deterministically afterwards, not in the agents.** Detail files are distinct, so parallel writes
   never conflict. Back-links into log entries are added in a separate main-loop pass grouped by log file, so two
   agents never edit the same log file at once.
4. **Validate, then commit per batch.** Run the ticket-detail validator over every written file and the base
   validator over every touched log entry, then commit.

### Pitfalls

**Trust the filesystem, not the agent's status.** An agent can return `status: "written"` without having written
the file (it may believe a prior run created it). Filter results to entries whose `detail_file` actually exists
on disk before counting or committing. Never commit on the strength of a status field alone.

**Stage every year directory, not just the batch's year.** Entries are dated by the ticket's resolution date, so
a ticket resolved a year after it was raised writes its file under the later year. `git add 2023` alone misses a
file written to `2024/`. Always stage every year directory touched by the batch, not just the one you started in.

**Match log back-links loosely.** Source logs often have messy link formatting: a colon inside the label
(`[PROJ-56:](url)`), comma-run citations (`[PROJ-962](url)[, PROJ-963](url)`), multi-key bullets. Matching the key
literally as `[KEY]` misses these. Match the key on a word boundary (`\bPROJ-963\b`, which also matches the URL in
the bullet) and scan the whole bullet block, inserting one nested `Detailed entry:` line per key.

**Recompute remaining from the repo on every resume.** Runs get interrupted (session limits, transient API
errors). On resume, recompute the remaining set from files-on-disk rather than trusting the pre-interruption
list, and exclude a persistent skip list (tickets deliberately judged too thin). This absorbs partial progress
cleanly.

**Distinguish "not found" from "not reachable".** A ticket that returns "not found" on the configured tracker
instance may simply live on a different instance or project. Verify the project exists on the instance before
concluding a ticket was deleted; flag an unreachable instance as an access gap, not as a dead ticket.

### Cost and pacing

Each ticket agent runs roughly 15-40k tokens (a fetch with comments plus a written, self-validated entry). A
large batch is several million tokens and tens of minutes; concurrency is capped, so it proceeds in waves. Batch
by year (or another natural unit), commit per batch, and expect to resume across interruptions.
