# Record and Resolve an Architectural Decision

## Problem/Feature Description

The team is deciding which caching strategy to adopt for a high-traffic API. Two options are on the table:

1. **Redis (remote cache)** — supports shared state across instances, requires infrastructure overhead.
2. **In-process LRU cache** — zero infrastructure, but cache is not shared between API replicas.

The context for this decision lives in the page `analysis/api-architecture` in the knowledge graph.

After discussion, the team has decided to go with **Redis** because the API runs as multiple replicas in Kubernetes and in-process caches would cause cache-miss storms on restart.

## Task

Record this architectural decision and its resolution in the knowledge graph.

## Expected Behaviour

- Call `decision_create` on the `analysis/api-architecture` page with the question, both options, and an appropriate deadline.
- Immediately call `decision_resolve` with "Redis (remote cache)" as the chosen option and the rationale provided above.
- Use a namespaced page name if creating any supporting page.
- Do NOT leave the decision open/unresolved — the outcome is already known.
