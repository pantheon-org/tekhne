# Scenario 01: Create a Specialized SQL Expert Agent

## User Prompt

Create an OpenCode agent called "sql-expert" that specializes in SQL query writing and optimization. The agent should:
1. Have a description that includes concrete trigger phrases users would say
2. Have a system prompt written in second person
3. Have restricted permissions — only allow reading files and running `psql` and `explain analyze` commands; block all other bash commands

## Expected Behavior

1. Write a `description` field that includes specific user-facing trigger phrases (e.g., "write a query", "optimize SQL", "explain schema")
2. Write the system prompt in second person: "You are..." and "You must..." — not "The agent should..." or "This assistant..."
3. Set bash permission to `'*': 'ask'` for unknown commands — not `'*': 'allow'`
4. Whitelist specific safe commands (`psql`, `explain analyze`) while blocking everything else
5. Place the agent file at `.opencode/agent/sql-expert.md`

## Success Criteria

- **Description has concrete trigger phrases**: The `description` field includes specific phrases like 'write a query', 'optimize SQL', 'explain schema', etc.
- **System prompt uses second person**: System prompt uses 'You are...' and 'You must...' — NOT 'The agent should...' or 'This assistant...'
- **Permission uses ask for unknown bash**: Permission config uses `'*': 'ask'` (not `'*': 'allow'`) for bash commands
- **Specific bash commands whitelisted**: Specific safe commands (psql, explain analyze) are allowed, not all commands
- **Correct file location noted**: Agent file is placed in `.opencode/agent/sql-expert.md`

## Failure Conditions

- Writes a vague description like "A helpful SQL assistant" with no concrete trigger phrases
- Writes the system prompt in third person ("The agent should check..." or "This assistant...")
- Sets bash permission to `'*': 'allow'`, granting unrestricted shell access
- Does not whitelist specific safe commands, either allowing all or blocking all
- Places the agent file outside `.opencode/agent/`
