# Scenario 03: Sidebar Configuration

## User Prompt

"How do I configure the sidebar with explicit links and an auto-generated section?"

## Expected Behavior

1. Shows `sidebar` array in `starlight({})` config
2. Demonstrates both `items` (explicit) and `autogenerate` approaches
3. Uses `slug` values without leading slashes or `.md` extensions
4. Does NOT use both `items` and `autogenerate` on the same group

## Success Criteria

- Correct slug format (e.g., `'guides/getting-started'` not `'/guides/getting-started.md'`)
- `autogenerate: { directory: '...' }` syntax correct
- No mixed `items`/`autogenerate` in same group

## Failure Conditions

- Slugs include a leading slash or `.md` extension
- Uses `items` and `autogenerate` together in the same group
- `autogenerate` syntax is incorrect or missing the `directory` key
