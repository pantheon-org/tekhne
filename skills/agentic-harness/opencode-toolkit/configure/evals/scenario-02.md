# Scenario 02: Separate Behavioral Rules from Runtime Configuration

## User Prompt

A team wants OpenCode to:
1. Always run `npm test` after every code change
2. Never modify `package.json` without asking first
3. Use conventional commits format for all commits

Where should these rules be placed (opencode.json or AGENTS.md)? Show the configuration.

## Expected Behavior

1. Place all workflow rules (run tests after changes, ask before modifying package.json, conventional commits) in `AGENTS.md` — not in `opencode.json`
2. Explain the distinction: `AGENTS.md` is for behavioral guidance read by the agent, `opencode.json` is for runtime/provider configuration
3. Avoid adding workflow instructions as JSON fields in `opencode.json`
4. Place `AGENTS.md` in the project root directory

## Success Criteria

- **Rules placed in AGENTS.md**: Workflow rules (test after change, conventional commits, etc.) go in `AGENTS.md`, NOT in `opencode.json`
- **Explains the distinction**: Explains that AGENTS.md is for behavioral guidance and opencode.json is for runtime config
- **Does not add instructions to opencode.json**: Does not attempt to add workflow rules as JSON fields in opencode.json
- **AGENTS.md in project root**: Places AGENTS.md in the project root directory

## Failure Conditions

- Adds workflow rules as JSON fields in `opencode.json` (e.g., a `rules` or `instructions` key)
- Fails to explain the distinction between `AGENTS.md` (behavioral) and `opencode.json` (runtime config)
- Places `AGENTS.md` in a subdirectory instead of the project root
- Puts behavioral instructions in both files, creating inconsistency
