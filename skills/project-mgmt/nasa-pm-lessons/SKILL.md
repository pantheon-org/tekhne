---
name: nasa-pm-lessons
description:
  "Surface a matched excerpt from Jerry Madden's NASA project-management aphorisms (LLIS #1956) at the moment it fits: incident root-cause sections; ticket refinement; plan drafting; status updates;
  escalation calls. Loads only one category at a time — never the full list, never more than three aphorisms per response. DO NOT use this for regulatory, legal, or prize-dispute decisions; route
  those to Legal/Compliance/DPO. Triggers: 'root cause', 'refine this ticket', 'draft a plan', 'status update', 'should I escalate', 'stakeholder comms'."
---

# NASA PM Lessons (LLIS #1956)

## Source

Jerry Madden, former Associate Director of Flight Projects, NASA Goddard Space Flight Center — aphorisms collected over 37 years, first published in ASK Magazine, October 2003.
<https://llis.nasa.gov/lesson/1956>

The reference files below paraphrase the source in each category's own words rather than quoting it at length — treat them as a judgment aid, not a verbatim transcript. Consult the source URL for
Madden's original phrasing and full elaboration.

## Prerequisites

Only apply this skill when the situation actually matches one of the trigger scenarios below. If it doesn't, skip this skill entirely rather than forcing a fit.

## When to Use

| Scenario                                                                  | Load                                                 |
| ------------------------------------------------------------------------- | ---------------------------------------------------- |
| Root cause / incident writeup (journal troubleshooting entries)           | `references/design-engineering.md`                   |
| Ticket refinement, implementation plan drafting, wave planning            | `references/planning-and-decisions.md`               |
| Status comment to management, escalation call                             | `references/working-with-superiors.md`               |
| Stakeholder comms, incident status to non-engineers, vendor/partner comms | `references/customer-stakeholder-relations.md`       |
| Retro notes, team roster entries, 1:1 or standup prep                     | `references/managing-project-staff.md`               |
| Vendor/contractor friction or negotiation                                 | `references/contractor-relations.md`                 |
| Anything else                                                             | Skip this skill unless a genuine match appears above |

## When Not to Use

- Don't quote more than one category, or more than 1-3 aphorisms, in a single response — pick what fits the specific situation
- Not a substitute for your project's own planning/ticketing structure — this skill adds judgment framing, not schema
- Regulatory, legal, or other decisions requiring a documented human sign-off are out of scope for this skill — those route to the responsible function regardless of what a Madden aphorism might suggest

## Workflow

1. Identify which trigger scenario applies (table above)
2. Load only the matching `references/*.md` file
3. Pick the 1-3 aphorisms that actually fit the situation — state the point in a line, attribute it to Madden/LLIS #1956, then apply it to the concrete case at hand
4. Never present this as directive process or a checklist to satisfy — it's judgment support

## Verification

Before treating a Madden aphorism as settled guidance for anything stakeholder-facing or regulated, wait for explicit confirmation from the responsible function (Legal, Compliance, or equivalent)
whenever the topic touches a regulatory or externally-facing decision — this skill only produces internal judgment framing, never the final call.

```bash
cargo run -p pantheon-skill-auditor -- evaluate project-mgmt/nasa-pm-lessons --json --store
```

Result: a JSON score report across the 9-dimension framework — treat anything below a B grade as not yet production-ready.

```text
Working with Superiors — LLIS #1956: "Never ask management to make a decision that you can make."
Applied: this status update is yours to send; don't route it through your manager first.
```

Result: one line, attributed, applied — not a dump of the whole category.

## Troubleshooting

If the matched reference file is missing, or nothing in the matched category actually fits the situation, say so directly and skip the citation — forcing an aphorism where none fits is worse than
citing none.

## Mindset

- TYPICALLY, one well-chosen line beats five loosely relevant ones — brevity is the point, not exhaustiveness
- These are aphorisms, not procedures. ALWAYS use them to frame a judgment call, never to justify a decision already made
- PREFER citing zero aphorisms over forcing a citation that doesn't fit the specific situation
- In a production incident writeup, a single attributed line lands better than a wall of NASA trivia — treat over-citing as the gotcha to avoid, not a feature

## Anti-Patterns

**NEVER** paste an entire category's list into a journal entry, ticket comment, or status update. **WHY:** it turns a judgment nudge into noise and defeats the point of category scoping — the main
pitfall this skill guards against. **BAD:** "Per NASA LLIS #1956, here are the 20 planning lessons to consider..." **GOOD:** one line, attributed, applied to the specific decision in front of you.

**NEVER** treat this skill as authority for a stakeholder-facing regulatory, legal, or customer-dispute decision. **WHY:** those are decision-support domains requiring a documented human decision —
an aphorism is not a substitute. **BAD:** citing Madden's "know your customer" aphorism as the basis for resolving a customer dispute. **GOOD:** use it for internal judgment framing only, and route
anything regulator-facing to the correct function.

**NEVER** force a citation when nothing in the matched category actually fits the situation. **WHY:** a fabricated-fit aphorism reads as filler and erodes trust in the citations that are genuinely
useful. **BAD:** stretching a "Working with Superiors" line to cover a pure design-engineering root cause just because that was the category loaded. **GOOD:** say plainly that nothing fits, skip the
citation, and move on.

## References

| Category                                            | File                                           |
| --------------------------------------------------- | ---------------------------------------------- |
| Design Engineering                                  | `references/design-engineering.md`             |
| Planning, Decision-Making, Documentation, Reporting | `references/planning-and-decisions.md`         |
| Managing Project Staff                              | `references/managing-project-staff.md`         |
| Working with Superiors                              | `references/working-with-superiors.md`         |
| Customer/Stakeholder Relations                      | `references/customer-stakeholder-relations.md` |
| Contractor Relations                                | `references/contractor-relations.md`           |
