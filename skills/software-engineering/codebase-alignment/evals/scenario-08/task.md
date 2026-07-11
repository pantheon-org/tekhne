# Scenario 08: Subjective Standards — Mark as Manual Review

## Setup

The user provides these standards:
- "Use descriptive variable names"
- "Functions should do one thing only"
- "All API endpoints must be versioned with /v1/ prefix"
- "Code should be easy to read"

## User Prompt

"Check these standards against our codebase: Use descriptive variable names, Functions should do one thing only, All API endpoints must be versioned with /v1/ prefix, Code should be easy to read."

The codebase is a standard Express.js REST API.

## Expected Behavior

1. Recognize that "descriptive variable names," "functions should do one thing," and "code should be easy to read" are subjective standards that cannot be automatically checked.
2. Recognize that "API endpoints must be versioned with /v1/ prefix" IS an objective checkable standard.
3. Run the /v1/ prefix check against the codebase.
4. For the subjective standards, report them as "manual review needed" — do NOT attempt an automated check.
5. Do NOT guess or fabricate a checking methodology for subjective standards.

## Success Criteria

- The objective standard (API versioning) is checked automatically.
- The three subjective standards are reported as manual review needed.
- No automated check is attempted for subjective standards.

## Failure Conditions

- The agent tries to auto-check "descriptive variable names" with a regex or heuristic.
- The agent reports all standards as checked when some are subjective.
- The agent says "checking variable names" and fabricates results.
- The agent assumes the standards without asking for confirmation.
