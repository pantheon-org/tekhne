# Scenario 7: Route a request correctly and scan the codebase for violations

## User Prompt

"Our team is deciding where to put the boundary between the billing service and the ordering service, and whether to use the Strategy or Decorator pattern for discounts. Separately, can you find likely SRP, OCP and DIP problems in `src/`?"

## Repo state

- TypeScript codebase under `src/` with `*Service`, `*Repository`, `*Manager` and `*Handler` classes.
- Some switch statements on a `type` field.
