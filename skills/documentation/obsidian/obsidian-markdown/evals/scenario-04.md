# Scenario 04: Create a Collapsed Important Callout with Highlighted Date

## User Prompt

You are writing a project status note. Add a callout block to `status-update.md` that:

1. Signals high importance to readers
2. Has a custom heading that reads "Key Deadline"
3. Starts hidden so it does not clutter the reading view — the reader must click to expand it
4. Contains a sentence noting that the release is due on ==April 15th==

Write the complete callout to the file `status-update.md`.

## Expected Behavior

1. Use the `[!important]` callout type on the opening blockquote line
2. Include a minus sign after the type bracket to make the callout collapsed by default: `> [!important]-`
3. Place the custom title "Key Deadline" on the same line as the callout type declaration: `> [!important]- Key Deadline`
4. Wrap the date text inside the callout body using `==...==` highlight syntax: `==April 15th==`

## Success Criteria

- **Callout uses 'important' type**: The callout opening line uses `[!important]` (e.g., `> [!important]-` or `> [!important]+`)
- **Callout is collapsed by default**: The callout opening line includes a minus sign after the type bracket: `> [!important]-` (not `+` and not missing the symbol entirely)
- **Custom title on the opening line**: The text "Key Deadline" appears on the same line as the callout type declaration (e.g., `> [!important]- Key Deadline`)
- **Highlighted date uses == syntax**: The date or relevant text inside the callout body is wrapped in `==...==` highlight syntax (e.g., `==April 15th==`)

## Failure Conditions

- Callout opening line uses a type other than `[!important]` (e.g., `[!warning]` or `[!note]`)
- Callout is not collapsed by default (uses `+` or omits the collapse symbol, making it open by default)
- "Key Deadline" title does not appear on the same line as the callout type declaration
- The release date inside the callout is not wrapped in `==...==` highlight syntax
