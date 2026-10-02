# Scenario 8: Enforce Dependency Rules With Architecture Tests

## User Prompt

Developers keep adding shortcuts: `src/domain/Order.ts` recently imported `prisma` and `src/application/CreateOrderUseCase.ts` imported a Stripe client directly. The team wants this caught automatically. Add architecture tests for the TypeScript layout `src/domain`, `src/application`, `src/infrastructure`, `src/interface` and wire them into CI.

Repo state: GitHub Actions workflow at `.github/workflows/ci.yml`, no architecture checks yet.
