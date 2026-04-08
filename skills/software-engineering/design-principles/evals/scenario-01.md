# Scenario 01: Clean Arch Boundary Decision

## User Prompt

"Should we extract our authentication system into a separate bounded context? Consider:
- Current: Auth is part of the monolith (3 modules, 2 developers, 2-week release cycle)
- Team wants to own auth independently and deploy weekly
- Auth is used by 5 other services via REST API
- No compliance requirements yet, but HIPAA compliance planned in 6 months

Analyze tradeoffs and recommend a decision."

## Expected Behavior

1. Evaluate boundary costs (network latency, distributed transactions, complexity)
2. Evaluate boundary benefits (independent deployment, team autonomy, clear ownership)
3. Consider alternatives (keep in monolith, partial boundary, full service extraction)
4. Weigh YAGNI vs. future compliance needs
5. Recommend decision with explicit rationale
6. Document risks and mitigations

## Success Criteria

- Evaluates at least 2 boundary costs and 2 benefits
- Considers at least 2 alternatives
- Provides explicit recommendation (extract, defer, or partial boundary)
- Mentions YAGNI, compliance requirements, or team autonomy
- Documents at least 1 risk and mitigation strategy

## Failure Conditions

- Recommends extraction without evaluating costs or risks
- Ignores the upcoming HIPAA compliance requirement entirely
- Provides no concrete alternatives to full service extraction
- Gives a recommendation with no rationale or supporting evidence
