# Scenario 03: Extract Page Metadata Only

## User Prompt

"I just need the title and description of this page — nothing else: https://github.com/kepano/defuddle"

Retrieve only the title and description metadata from that URL and write them to a file called `metadata.md` in this format:

```
Title: <value>
Description: <value>
```

## Expected Behavior

1. Use `defuddle parse <url> -p title` and `-p description` flags (or `--json`) to extract only metadata, rather than fetching the full page with `--md`
2. Avoid running `defuddle parse <url> --md` when only metadata was requested
3. Write a `metadata.md` file containing both the title and description values in the requested format

## Success Criteria

- **Used -p flag or --json for metadata extraction**: The agent used `defuddle parse <url> -p title` and/or `-p description` (or `--json`) rather than fetching the full page with `--md`.
- **Did not fetch full page content unnecessarily**: The agent did not run `defuddle parse <url> --md` when only metadata was requested, avoiding wasteful full-content extraction.
- **Produced metadata.md with title and description**: The file `metadata.md` exists and contains both the title and description values in the requested format.

## Failure Conditions

- Fetches the full page content with `--md` when only title and description metadata was requested
- Omits the `-p` flags or `--json` and returns unfiltered full-page content
- Does not produce a `metadata.md` file
- `metadata.md` is missing either the title or the description value
