# Scenario 4: Add Scoped Migration Override to biome.json for a Legacy Directory

## User Prompt

You are migrating a large Nx monorepo to Biome. The `apps/legacy-portal` directory contains a lot of TypeScript files that use non-null assertions heavily. Rather than globally disabling the `noNonNullAssertion` rule, you need to add a scoped, time-boxed override for just that directory.

Produce an updated `biome.json` that adds this scoped override.
