# Onboard Biome to a TypeScript Project

## Problem Description

A team is starting a new TypeScript API project and wants consistent code quality from day one. They've decided to use Biome as their single tool for both linting and formatting. They need a biome.json tailored to their conventions: 2-space indentation, double quotes, semicolons enabled, Node.js globals available to the linter, and the `dist/` and `node_modules/` directories excluded from checks.

Set up Biome for the project. Produce:
1. `biome.json` — the configured Biome config file
2. `SETUP.md` — a short guide with the commands to run checks and apply fixes

Include the commands for local development and for CI.
