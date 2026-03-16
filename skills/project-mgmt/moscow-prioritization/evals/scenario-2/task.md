# Scenario 2: Enforce the 60% Must Effort Cap

## Context

A team has submitted the following draft MoSCoW table for a 4-week sprint.
Total estimated effort is 80 story points.

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

## Your Task

Apply the MoSCoW skill's validation step to this draft. Produce a rebalancing
recommendation saved to `rebalancing-plan.md`.

## Requirements

Your output must:

1. State the current Must effort percentage and confirm it violates the 60% cap.
2. Apply the failure-focused test to each Must item to challenge whether it truly
   causes release failure if absent.
3. Propose specific items to move from Must to Should (with justification for each).
4. Show the recalculated Must percentage after rebalancing.
5. If rebalancing alone cannot bring Must under 60%, provide an escalation recommendation.

## Output Spec

File: `rebalancing-plan.md`
