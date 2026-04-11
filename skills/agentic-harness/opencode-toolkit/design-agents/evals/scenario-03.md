# Scenario 03: Fix Trigger Phrases and System Prompt Anti-Pattern

## User Prompt

An existing agent called "code-reviewer" is not triggering reliably. Its current description is: "A helpful code review assistant."

Improve this agent to trigger correctly when users say: "review my code", "check this file for bugs", "find issues in", "LGTM?", "code quality".

Also, the current system prompt says: "The agent should check for bugs and provide feedback." Fix this to follow best practices.

## Expected Behavior

1. Rewrite the `description` to include the specific trigger phrases provided: "review my code", "check this file for bugs", "find issues in", "LGTM?", "code quality"
2. Add an examples block to the description mapping user phrases to agent actions
3. Rewrite the system prompt in second person: "You check for bugs..." or "You provide feedback..." — never "The agent should..."
4. Avoid adding redundant permission config (e.g., `read: allow, write: allow`) when defaults are sufficient

## Success Criteria

- **Description includes specific trigger phrases**: Improved description contains concrete phrases like 'review my code', 'check for bugs', 'find issues', 'LGTM', 'code quality'
- **System prompt converted to second person**: System prompt uses 'You check for bugs...' or 'You provide feedback...' — NOT 'The agent should...'
- **Description includes examples block**: Description has example user phrases mapped to agent actions
- **No unnecessary permission config added**: Does not add redundant `read: allow, write: allow` permissions (defaults are sufficient)

## Failure Conditions

- Leaves the description as a single vague sentence with no specific trigger phrases
- Keeps the system prompt in third person ("The agent should...") rather than converting to second person
- Omits an examples block from the description, providing only a general description
- Adds unnecessary permission entries that duplicate default behaviour, adding noise to the config
