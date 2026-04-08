# Scenario 03: Access Page Title in a Custom Component

## User Prompt

"How do I access the current page's title inside a custom Starlight component?"

## Expected Behavior

1. Uses `Astro.locals.starlightRoute.entry.data.title`
2. Does NOT use `Astro.props.title`

## Success Criteria

- `Astro.locals.starlightRoute` path used
- `Astro.props` approach not suggested
- Working code example provided

## Failure Conditions

- Suggests `Astro.props.title` as the access path
- Does not show the full `Astro.locals.starlightRoute.entry.data.title` path
- No working code example provided
