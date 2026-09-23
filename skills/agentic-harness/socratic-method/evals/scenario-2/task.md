# Optimizing the Wrong Problem

## User Prompt

A developer asks:

> "How do I make my dashboard load faster? It takes 5 seconds and users are complaining."

Use the socratic-method skill before suggesting any performance optimizations.

Hidden context (not provided): the dashboard fetches 12 API endpoints in sequence (not parallel),
the backend runs N+1 queries, and the "users complaining" is actually one user — the CEO —
who is loading 3 years of unfiltered data.
