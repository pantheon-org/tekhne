# Scenario 5: Draw the Boundary between ESLint and Biome During Migration

## User Prompt

An Nx workspace has `apps/web`, `apps/admin` and `libs/legacy-reports`, all linted by ESLint with targets covering `**/*.ts`. The team wants `apps/web` and `apps/admin` on Biome now and `libs/legacy-reports` left on ESLint until next quarter. Describe the configuration and target changes you would make and what you would check so no file is linted by both tools.
