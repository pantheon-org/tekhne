# Scenario 05: Draft Pages

## User Prompt

"How do I mark a documentation page as a draft so it doesn't appear in production?"

## Expected Behavior

1. Adds `draft: true` to page frontmatter
2. Notes that draft pages are visible in dev but excluded from production build and sitemap

## Success Criteria

- Frontmatter syntax is correct
- Development vs. production behavior explained

## Failure Conditions

- Suggests a config-level toggle instead of per-page frontmatter
- Does not explain the dev vs. production visibility difference
