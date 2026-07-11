# Scenario 05: Generate Complete Multi-Tool Config

## User Prompt

"Set up our tooling from scratch. We use Vue 3 with TypeScript, single quotes, 100-char print width, strict null checks, no unused variables, and Vue component PascalCase in templates."

## Setup

The project has no ESLint, Prettier, or TypeScript config yet. Fresh project.

## Expected Behavior

1. Identify all conventions and sort them by tool:
   - ESLint: `@typescript-eslint/no-unused-vars`, `vue/component-name-in-template-casing`
   - Prettier: `singleQuote: true`, `printWidth: 100`
   - TypeScript: `strict: true` (covers strict null checks)
2. Generate or instruct for each config file.
3. Explain what each rule/setting does and how it enforces the convention.
4. Note any dependencies needed (eslint-plugin-vue, typescript-eslint, etc.).

## Success Criteria

- All conventions are mapped to the correct tool.
- Config entries are correct for their respective tools.
- Dependencies are mentioned.

## Failure Conditions

- A convention is mapped to the wrong tool (e.g., single quotes in ESLint instead of Prettier).
- A convention is missed entirely.
- No dependency guidance is given.
