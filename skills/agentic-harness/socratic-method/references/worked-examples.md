# Worked Examples

Two full Socratic dialogues showing the five-phase protocol applied to real engineering
scenarios. Each dialogue is annotated with the phase active at each turn.

---

## Example 1 — "Add caching to the API"

### User request

> Our API is slow. Add Redis caching.

### Dialogue

**[Phase 1 — Clarifying]**

> I want to make sure we're optimizing the right thing before adding infrastructure.
> Where is the slowness most visible — specific endpoints, all reads, or only under load?

**User:** The `/search` endpoint. It can take 3–4 seconds.

**[Phase 2 — Probing assumptions]**

> You've framed this as a caching problem. What's your current read on where the 3–4 seconds
> is actually spent — is it query time, network round trips, or response serialization?

**User:** Honestly, I haven't profiled it. I assumed it was the database.

**[Phase 2 — Probing assumptions continued]**

> Reasonable assumption. What does the `/search` query look like — is it a full-text scan,
> does it join multiple tables, and how much data is it returning?

**User:** It does a LIKE query on a 2M-row table with no index on the search column.

**[Phase 3 — Implications]**

> If the bottleneck is a sequential scan on an unindexed column, caching would hide the
> symptom without fixing the cause — and you'd still see the 3–4 seconds on cache misses,
> which for a search endpoint could be frequent. Does the search term distribution tend to
> cluster around common queries, or is each search likely unique?

**User:** Very unique — it's user-generated natural language queries.

**[Phase 4 — Hypothetical]**

> If the cache hit rate for unique natural language queries is near zero, what problem does
> Redis actually solve here?

**User:** ...Not much, I guess. We'd just be adding a dependency.

**[Phase 5 — Synthesis]**

> So the real problem is an unindexed LIKE query on a large table. The solution space is:
> a full-text index (Postgres `tsvector`, Elasticsearch), query optimization, or results
> pagination to bound the scan. Redis might still be useful later for session data, but it
> won't fix this. Should I proceed with a full-text indexing approach?

**User:** Yes, that makes much more sense.

### Annotations

- The user arrived at "Redis" via assumption, not diagnosis
- Phase 2 surfaced the missing profiling step
- Phase 3 introduced the cache hit rate problem specific to the use case
- Phase 4 asked the user to derive the conclusion themselves (maieutics)
- No implementation was proposed until the problem was correctly identified

---

## Example 2 — "Refactor the auth module"

### Incoming request

> The auth module is a mess. It needs a refactor.

### Exchange

**[Phase 1 — Clarifying]**

> When you say "a mess" — is that about the code structure (hard to read, tangled
> responsibilities), the behavior (bugs, edge cases), or working with it day-to-day
> (slow to change, risky to touch)?

**User:** Mainly the last one. Every time we touch it, something breaks in a way we didn't
expect.

**[Phase 2 — Probing assumptions]**

> "Something breaks unexpectedly" often points to hidden coupling or missing test coverage
> more than structure. What does the test coverage on the auth module look like?

**User:** There are basically no tests for it. It was written before we adopted TDD.

**[Phase 2 — continued]**

> So the refactor you have in mind — what's the shape of it? Are you thinking structural
> (split into smaller modules), behavioral (fix the edge cases), or both?

**User:** I was thinking structural — reorganize the files and clean up the code. But I'm
realizing that might make things more fragile, not less.

**[Phase 3 — Implications]**

> A structural refactor without tests tends to move the coupling around rather than eliminate
> it. If a change breaks something today, you have no way to detect it until it hits production.
> What's the release schedule for the next few weeks — is there a freeze coming up?

**User:** We have a release in 10 days.

**[Phase 4 — Hypothetical]**

> If you could only do one thing to the auth module before the release, and it had to reduce
> the risk of breakage rather than eliminate technical debt, what would it be?

**User:** Probably adding tests for the token validation logic — that's where most of the
surprises come from.

**[Phase 5 — Synthesis]**

> The insight that's emerging: the fragility is caused by absent tests, not structure. A
> structural refactor without tests would be high-risk before a release. The right sequence
> is: (1) characterize the existing behavior with tests, especially token validation, then
> (2) refactor safely once you have a regression net. Should I start with the test-writing
> phase and scope the structural work for after the release?

**User:** Exactly. I didn't realize I was reaching for the wrong tool.

### Notes

- "Mess" was undefined; Phase 1 decomposed it into three distinct problems
- The user self-corrected during Phase 2 (aporia in action)
- Phase 3 introduced the release deadline as a constraint the user hadn't mentioned
- Phase 4's hypothetical led the user to name the specific high-value action themselves
- The synthesis reordered the work without dismissing the original goal
