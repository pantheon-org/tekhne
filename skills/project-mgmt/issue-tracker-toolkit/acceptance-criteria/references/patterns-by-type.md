# Acceptance Criteria Patterns by Feature Type

Use these rule-oriented patterns when requirements are independent constraints rather than a single sequence.

## Form Validation Pattern

```markdown
Input Validation:
- Email accepts only valid format `user@domain.tld`.
- Password requires at least 12 chars, 1 number, and 1 symbol.
- Invalid fields show inline error text below each field.

Submit Behavior:
- Submit is disabled while any required field is invalid.
- Submit triggers only one request per click.
- Success clears the form and shows confirmation.
```

## Search Pattern

```markdown
Input:
- Search accepts up to 200 characters.
- Empty search shows "Please enter a search term".

Execution:
- Search triggers on Enter and on Search button click.
- Results return in <= 2 seconds for 95% of queries.

Results:
- Show hotel name, location, price, and rating.
- Show "No results found" when zero matches are returned.
```

## API Integration Pattern

```markdown
Request:
- Endpoint: POST /api/v1/users
- Requires bearer token with admin scope
- Payload must include email, name, and role

Responses:
- 201 with created user payload on success
- 400 with field-level validation errors for invalid payload
- 401 for missing/invalid token
- 403 for insufficient scope
- 409 for duplicate email
- 500 displays generic retryable service error
```

## UI/UX Behavior Pattern

```markdown
Layout:
- Primary actions remain visible above the fold on desktop and mobile.
- Content reflows to one column below 768px width.

Interaction:
- Interactive elements expose hover/focus/disabled states.
- Modal closes via explicit close action and Escape key.

Accessibility:
- Keyboard navigation reaches all actionable controls.
- Error messages are announced via assistive technology.
```

## Measurement Pattern

Use measurable wording:

- Time: `<= 500ms`, `<= 2s for 95%`
- Quantity: `max 20 results/page`
- Availability: `99.9% monthly uptime`
- Limits: `max 100 requests/min/api key`
