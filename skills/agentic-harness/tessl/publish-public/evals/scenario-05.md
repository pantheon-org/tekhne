# Scenario 05: Agent-Agnostic Compliance Check

## User Prompt

"Verify the github-actions-validator skill is agent-agnostic before public publishing."

## Expected Behavior

1. Agent reads the SKILL.md for `skills/ci-cd/github-actions/validator/SKILL.md`
2. Scans for agent-specific tool references (Claude Code specific, Cursor specific, etc.)
3. Checks for harness-specific instructions ("use Claude Code's X feature")
4. Verifies only universal tools are used (Read, Write, Edit, Bash, Grep, Glob)
5. Identifies any platform-specific behaviors (VS Code commands, IDE integrations)
6. Reports compliance status (pass/fail)
7. If violations found, lists specific lines/sections with issues
8. Suggests agent-agnostic alternatives for any violations

## Success Criteria

- Agent thoroughly scans SKILL.md content
- Agent identifies any agent-specific tool references
- Agent identifies any harness-specific instructions
- Agent verifies universal tool usage
- Agent provides specific line numbers for violations (if any)
- Agent suggests concrete alternatives for violations
- Agent confirms cross-platform compatibility
- Agent reports clear pass/fail compliance status

## Failure Conditions

- Agent performs superficial scan without content analysis
- Agent misses obvious agent-specific references
- Agent approves skills with "Claude Code only" instructions
- Agent doesn't suggest alternatives for violations
- Agent confuses universal tools with agent-specific tools
- Agent reports false positives (flags universal tools as violations)
- Agent skips cross-platform compatibility consideration
