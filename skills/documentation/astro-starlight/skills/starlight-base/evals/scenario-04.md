# Scenario 04: Sitemap Setup

## User Prompt

"How do I enable the sitemap for my Starlight site?"

## Expected Behavior

1. Sets `site: 'https://...'` on `defineConfig({})`, NOT inside `starlight({})`
2. Notes that Starlight generates the sitemap automatically when `site` is set

## Success Criteria

- `site` is at `defineConfig` level
- No manual sitemap plugin mentioned (not needed)

## Failure Conditions

- Places `site` inside `starlight({})` instead of `defineConfig({})`
- Instructs installing a separate sitemap plugin when it is not required
