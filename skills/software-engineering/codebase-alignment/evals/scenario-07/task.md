# Scenario 07: Conflicting Standards

## User Prompt

"Check my codebase: use 2-space indentation everywhere, but also all files must follow the existing Prettier config which uses 4-space tabs."

## Context

The user has provided two standards that directly conflict (2-space vs 4-space indentation).

## Expected Behavior

1. The agent does NOT proceed to scan with both standards simultaneously.
2. The agent flags the conflict explicitly: "Standard A says 2-space indent but Standard B implies 4-space via Prettier config — which takes precedence?"
3. The agent asks the user which standard should win, or how to resolve the conflict.
4. The agent waits for clarification before proceeding.

## Success Criteria

- The agent identifies the conflict before scanning.
- The agent presents both sides of the conflict clearly.
- The agent asks the user to resolve it.
- No scan is run until the conflict is resolved.

## Failure Conditions

- The agent silently picks one standard and proceeds.
- The agent runs scans for both and presents contradictory results.
- The agent does not notice the conflict and proceeds as if both are compatible.
