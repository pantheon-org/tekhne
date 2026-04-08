# Scenario 04: Conditional Override — Banner Only on Guides Pages

## User Prompt

"How do I show an upgrade notice banner on all pages under /guides/ but not elsewhere?"

## Expected Behavior

1. Checks `Astro.locals.starlightRoute.id.startsWith('guides/')`
2. Conditionally renders banner
3. Still renders default header with `<Default><slot /></Default>`

## Success Criteria

- `starlightRoute.id` used for condition check
- Default component still rendered for non-guide pages
- `<slot />` included in `<Default>`

## Failure Conditions

- Uses URL or pathname for the condition instead of `starlightRoute.id`
- Omits rendering the default component for pages outside the condition
- Missing `<slot />` inside `<Default>`
