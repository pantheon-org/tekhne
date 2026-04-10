# Scenario 02: Enforce the 60% Must Effort Cap

## User Prompt

A team has submitted the following draft MoSCoW table for a 4-week sprint. Total estimated effort is 80 story points.

| Requirement | Category | Effort (SP) |
|---|---|---|
| User authentication | Must | 13 |
| Password reset flow | Must | 8 |
| OAuth2 login (Google) | Must | 13 |
| Profile editing | Must | 8 |
| Email notifications | Must | 13 |
| Activity feed | Should | 8 |
| Dark mode | Could | 5 |
| Mobile app (React Native) | Won't | — |

Must total: 55 SP out of 80 SP total = 68.75%

Apply the MoSCoW skill's validation step to this draft. Produce a rebalancing recommendation saved to `rebalancing-plan.md`.

Your output must:

1. State the current Must effort percentage and confirm it violates the 60% cap.
2. Apply the failure-focused test to each Must item to challenge whether it truly causes release failure if absent.
3. Propose specific items to move from Must to Should (with justification for each).
4. Show the recalculated Must percentage after rebalancing.
5. If rebalancing alone cannot bring Must under 60%, provide an escalation recommendation.

## Expected Behavior

1. Explicitly state the current Must percentage (68.75%) and identify it as exceeding the 60% cap
2. Apply the "release fails without this?" test to at least 3 of the 5 Must items
3. Propose at least one specific Must item to move to Should, with justification that references the failure test outcome
4. Show the Must effort percentage after the proposed moves, confirming it is at or below 60%
5. Mention the escalation option (scope split, timeline extension, or capacity increase) if rebalancing cannot achieve 60%

## Success Criteria

- **Violation identified with percentage**: `rebalancing-plan.md` explicitly states the current Must percentage (68.75%) and identifies it as exceeding the 60% cap
- **Failure-focused test applied**: `rebalancing-plan.md` applies the "release fails without this?" test to at least 3 of the 5 Must items
- **Specific Must-to-Should moves proposed**: `rebalancing-plan.md` proposes at least one specific Must item to move to Should, with a justification that references the failure test outcome
- **Recalculated percentage shown**: `rebalancing-plan.md` shows the Must effort percentage after the proposed moves, confirming it is at or below 60%
- **Escalation path mentioned**: `rebalancing-plan.md` mentions the escalation option (scope split, timeline extension, or capacity increase) if rebalancing cannot achieve 60%

## Failure Conditions

- Does not state the current Must percentage or identify it as a violation
- Does not apply the failure-focused test to any Must items
- Proposes moves without justification referencing the failure test
- Does not show the recalculated percentage after proposed moves
- Does not mention any escalation option
