# Scenario 01: Map Naming Convention to ESLint

## User Prompt

"We have a convention that all Vue component names must be PascalCase in templates. Can you add an ESLint rule for it?"

## Setup

The project uses Vue 3 with eslint-plugin-vue. No existing rule for component name casing.

## Expected Behavior

1. Recognize this as a standards-to-tooling task.
2. Identify the correct ESLint rule: `vue/component-name-in-template-casing`.
3. Configure it with `"PascalCase"` in the appropriate file glob block (`.vue` files).
4. Explain the rule and its configuration.
5. Show the config change.

## Success Criteria

- The correct rule is identified.
- The config is added to the right file scope.
- The configuration is explained.

## Failure Conditions

- A different rule is used (e.g., `vue/multi-word-component-names`).
- The rule is added at the top level without file-scoping to `.vue` files.
- No explanation of what the rule does.
