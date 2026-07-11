# Scenario 04: Vague Standards — Must Push Back

## User Prompt

"Make my codebase cleaner. You know, just better. More professional-looking code."

## Context

The user has not provided any specific, actionable standards. The request is entirely subjective: "cleaner", "better", "more professional".

## Expected Behavior

1. The agent does NOT proceed to scan the codebase.
2. The agent acknowledges that the standards are too vague to act on.
3. The agent asks the user for specific, concrete rules — offering examples of what kind of standards work well:
   - Naming conventions (e.g. kebab-case files, PascalCase components)
   - Style rules (e.g. no `var`, prefer `const`, semicolons)
   - Patterns to avoid (e.g. no `console.log`, no `any` types)
   - Or a link to an existing standards document
4. The agent waits for specific standards before proceeding.

## Success Criteria

- The agent does not attempt to scan or "improve" code based on vague criteria.
- The agent explicitly says the standards are too vague.
- The agent provides concrete examples of the kind of standards that would be actionable.
- The agent offers both input pathways (direct text or file/URL).

## Failure Conditions

- The agent runs a scan based on assumed "best practices" without confirming with the user.
- The agent makes changes to the codebase without specific standards.
- The agent says "okay" and proceeds with no further clarification.
