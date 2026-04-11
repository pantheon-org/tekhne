# Scenario 03: Detect and Fix Anti-Patterns in a Risk Assessment

## User Prompt

A security engineer has written a risk assessment for a cross-functional team that includes product managers, legal counsel, and operations staff. The document contains several plain-english anti-patterns.

Here is the input document:

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

1. Identify every plain-english anti-pattern present in this document.
2. Rewrite the document for the cross-functional audience, fixing all anti-patterns.

## Expected Behavior

1. Explicitly flag jargon terms (OAuth 2.0, JWT, RS256, Redis, attack surface) as anti-patterns requiring translation
2. Call out passive voice in the Risks section (`should be mitigated`, `should be migrated`, `It is recommended`) as an anti-pattern
3. Note that the key risk/decision is buried after background context rather than leading the document
4. Open the rewritten document with what the risk is and what decision or action is required
5. Rewrite each risk item naming a responsible team with a deadline using `[Owner] must [action] by [deadline]` format

## Success Criteria

- **Jargon anti-patterns identified**: Response explicitly flags jargon terms (OAuth 2.0, JWT, RS256, Redis, attack surface) as anti-patterns requiring translation.
- **Passive voice anti-patterns identified**: Response calls out passive voice in the Risks section (`should be mitigated`, `should be migrated`, `It is recommended`) as an anti-pattern.
- **Buried decision anti-pattern identified**: Response notes that key risk/decision is buried after background context rather than leading the document.
- **Rewrite leads with business impact and decision**: Rewritten document opens with what the risk is and what decision or action is required.
- **Risks rewritten with named owners and deadlines**: Each risk item in the rewrite names a responsible team and includes a deadline using `[Owner] must [action] by [deadline]` format.

## Failure Conditions

- Jargon terms (OAuth 2.0, JWT, RS256, Redis, attack surface) are not identified as anti-patterns
- Passive voice in the Risks section is not flagged as an anti-pattern
- Buried key message/decision is not identified as an anti-pattern
- Rewritten document still starts with technical background rather than business impact and decision
- Risk items in the rewrite lack named owners or deadlines
