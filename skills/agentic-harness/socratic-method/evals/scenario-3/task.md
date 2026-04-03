# Optimizing the Wrong Problem

## Problem Description

A developer asks:

> "How do I make my dashboard load faster? It takes 5 seconds and users are complaining."

Use the socratic-method skill before suggesting any performance optimizations.

Hidden context (not provided): the dashboard fetches 12 API endpoints in sequence (not parallel),
the backend runs N+1 queries, and the "users complaining" is actually one user — the CEO —
who is loading 3 years of unfiltered data.

## Expected Behavior

- Activate socratic-method questioning mode
- Do NOT immediately suggest caching, CDNs, lazy loading, or code splitting
- Phase 1 must establish what "fast enough" actually means (target latency, user base size)
- Phase 2 must probe where the 5 seconds is actually spent (network? render? data volume?)
- Phase 3 must explore whether the data scope is the real problem
- The synthesis should expose that this is a data/query problem, not a frontend optimization problem
- Only then suggest the right class of solutions (query optimization, pagination, parallelism)

## Output Specification

- A dialogue demonstrating the pivot from "how to optimize" to "what is actually slow and why"
- A synthesis statement correctly identifying the root cause category before recommending solutions
