# Scenario 06: Anti-Pattern Detection — Props Access

## User Prompt

"I have a custom PageTitle component but `Astro.props.title` is undefined. Why?"

## Expected Behavior

1. Explains that override components don't receive page data as props
2. Provides correct path: `Astro.locals.starlightRoute.entry.data.title`

## Success Criteria

- Root cause explained (override components don't get page data via props)
- `Astro.locals.starlightRoute.entry.data` path shown
- TypeScript type hint optional but appreciated

## Failure Conditions

- Does not explain why `Astro.props.title` is undefined
- Does not provide the `Astro.locals.starlightRoute.entry.data.title` path as the fix
