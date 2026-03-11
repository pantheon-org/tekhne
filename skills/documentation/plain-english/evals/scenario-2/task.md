# Scenario 2: Detect and Fix Anti-Patterns in a Risk Assessment

## Context

A security engineer has written a risk assessment for a cross-functional team that includes product managers, legal counsel, and operations staff. The document contains several plain-english anti-patterns.

## Input Document

```
Risk Assessment: Authentication Service Refactor

Background:
We are planning to refactor the OAuth 2.0 authentication service to address
technical debt accumulated over three years of incremental feature additions.
The current implementation uses a legacy JWT validation library that lacks
RS256 support, increasing our attack surface.

Risks:
1. Token replay attacks should be mitigated.
2. The session store should be migrated from Redis to a distributed cache.
3. It is recommended that penetration testing be conducted.

Timeline:
The migration will happen in Q3. Engineers will need to coordinate.
```

## Task

1. Identify every plain-english anti-pattern present in this document.
2. Rewrite the document for the cross-functional audience, fixing all anti-patterns.
