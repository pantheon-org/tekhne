# Plan Structure Reference

Common sections found in `.context/plans/` files and their purpose.

## Core Sections (present in >= 40% of plans)

| Section | Purpose | Always include? |
|---------|---------|-----------------|
| `## Goal` | Desired end state in one paragraph | Yes |
| `## Steps` / `## Phases` | Numbered implementation steps grouped by phase | Yes |
| `## Open Questions` | Unresolved items for reviewers to address | Yes |

## Common Sections (20-39% of plans)

| Section | Purpose | When to include |
|---------|---------|-----------------|
| `## Scope` / `## Out of Scope` | What's included and excluded | When scope is non-obvious |
| `## Related` / `## Related findings` | Links to source findings or dependent plans | When there are related context files |
| `## Verification` | How to confirm the plan worked | When the plan has concrete deliverables |
| `## Risks` / `## Sequencing & Risk` | Known risks and mitigation | When the plan has significant unknowns |

## Rare Sections (< 20% of plans)

| Section | Purpose |
|---------|---------|
| `## Background` | Context and motivation (use if Goal doesn't cover it) |
| `## File-by-file change list` | Specific files to modify |
| `## Academic References` | Research papers supporting the approach |
| `## Critical Review` | Findings from a plan-review audit |

## File naming

Two conventions are used:

- **Timestamped:** `short-description-YYYY-MM-DD.md` — use for plans with a
  specific creation date (most common)
- **Topic-only:** `short-description.md` — use when the plan is versioned
  alongside the code it describes

Follow whichever convention the majority of existing plans use.
