# Scenario 02: Investigate with Partial MCP Coverage

## User Prompt

Why don't we use a message queue for this background job instead of polling?

## Expected Behavior

1. The agent enumerates which MCP servers are actually connected this session and maps each to one of the seven evidence categories, rather than assuming coverage.
2. One investigator is spawned per category that has a matching MCP, each scoped to exactly one tool/MCP — no investigator is asked to cover more than one category.
3. For a category with no connected MCP (e.g. infrastructure observability, error tracking, product analytics warehouse in this session's stack), the agent reports it as skipped for lack of a
   connected MCP, not skipped because "it probably wouldn't have anything."
4. The final output includes a "Sources Consulted" section listing what was searched per category, including categories that returned nothing and categories that were skipped, with a reason for each
   skip.

## Success Criteria

- The coverage map names all seven categories, each marked searched, empty, or skipped-with-reason.
- No category is merged into another investigator's scope.
- Every skip reason is either "no MCP available" or a provably-irrelevant justification — never a bare assumption.
- "Sources Consulted" is present and specific per category.

## Failure Conditions

- A category is silently omitted from the coverage map with no mention at all.
- Two categories are handled by a single investigator.
- A category is skipped with reasoning like "probably nothing there" instead of a concrete MCP-availability or provable-irrelevance justification.
- "Sources Consulted" is missing, or is a single vague sentence instead of a per-category breakdown.
