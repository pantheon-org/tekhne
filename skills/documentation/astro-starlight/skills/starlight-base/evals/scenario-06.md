# Scenario 06: Anti-Pattern Detection — Wrong Slug Format

## User Prompt

"My sidebar link isn't working. I set `slug: '/guides/setup.md'`. What's wrong?"

## Expected Behavior

1. Identifies leading slash and `.md` extension as the problem
2. Provides corrected slug: `'guides/setup'`
3. Explains that slugs map to paths under `src/content/docs/` without slash or extension

## Success Criteria

- Both errors (leading slash AND extension) identified
- Correct slug shown
- Cause explained

## Failure Conditions

- Identifies only one of the two errors (leading slash or extension, not both)
- Does not provide a corrected slug example
- Does not explain the relationship between slug and `src/content/docs/` path
