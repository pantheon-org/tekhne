# Scenario 03: Standards from a URL

## User Prompt

"Our coding standards are documented here: https://example.com/team-standards
Can you check the codebase against these?"

Assume the URL when fetched returns a markdown document with these rules:
- "All Vue components must use `<script setup>` syntax (no Options API)"
- "Props should use type-based declarations, not runtime-only"
- "Component names in templates must be PascalCase"
- "No `any` types in TypeScript files"

The codebase is a Vue 3 + TypeScript project under transition from JS.

## Expected Behavior

1. The agent fetches the URL to read the standards document.
2. Parses the markdown into actionable, structured standards.
3. For each standard, determines checkability:
   - `<script setup>` — auto-checkable (grep for `export default {` in `.vue` files)
   - Type-based props — semi-auto (can flag runtime-only props, needs judgment)
   - PascalCase template names — auto-checkable
   - No `any` types — auto-checkable via grep for `: any` or TS compiler
4. Runs appropriate scans and produces a report.
5. Classifies each finding as auto/semi/manual and labels clearly.

## Success Criteria

- The URL was fetched and standards were extracted from it.
- Each standard is classified by checkability coverage.
- Semi-auto standards are flagged as requiring human review, not presented as definitive violations.
- Report includes file paths and line numbers for auto-checked violations.
- If the URL is inaccessible, the agent reports the error and offers the direct-input alternative.

## Failure Conditions

- The agent ignores the URL and works from an assumed standard set instead.
- The agent treats semi-auto findings as definitive violations without caveat.
- The agent does not attempt to fetch the URL.
- The agent proceeds without standards if the URL is inaccessible, instead of offering alternatives.
