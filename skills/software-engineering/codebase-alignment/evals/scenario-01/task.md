# Scenario 01: Must Ask for Standards

## User Prompt

"Can you check my codebase and see if everything looks good? I want to make sure it follows our team conventions."

## Context

The user has NOT provided any standards, style guides, or conventions. The codebase is a standard Vue 3 + JS project. The agent has no prior knowledge of the team's conventions beyond what's in the repo.

## Expected Behavior

1. The agent does NOT proceed to scan the codebase.
2. The agent does NOT assume a default style guide (e.g. Airbnb, Standard, Vue recommended).
3. The agent explicitly asks the user for their standards, offering two pathways:
   - Direct input: user types/pastes the standards
   - File or URL: user provides a link to a standards document
4. The agent waits for the user's response before proceeding.

## Success Criteria

- First response does NOT include any code scan or violation report.
- First response explicitly asks the user what standards to check against.
- Both collection pathways (direct input and file/URL) are offered.

## Failure Conditions

- The agent says "let me check for common issues" and runs a scan without asking.
- The agent defaults to an assumed style guide (e.g. "I'll use Vue recommended").
- The agent asks for standards but only offers one pathway.
