# Scenario 02: Save a Documentation Page to a Local File

## User Prompt

"Save the Bun installation docs page to a local file so I can reference it offline. Here's the URL: https://bun.sh/docs/installation"

Fetch the page and save its content as markdown to a file called `bun-installation.md` in the current working directory.

## Expected Behavior

1. Use `defuddle parse <url>` rather than WebFetch or curl to retrieve the page content
2. Include the `--md` flag to request markdown output
3. Use the `-o bun-installation.md` flag to write directly to the output file rather than using shell redirection (`>`)
4. Confirm the output file exists and contains non-empty markdown content

## Success Criteria

- **Used defuddle parse**: The agent invoked `defuddle parse` with the target URL rather than using WebFetch or curl.
- **Used --md flag**: The defuddle command included the `--md` flag to request markdown output.
- **Used -o flag for file output**: The agent used the `-o bun-installation.md` flag rather than shell redirection (`>`) to write the file.
- **Output file exists and contains content**: The file `bun-installation.md` exists in the working directory and contains non-empty markdown content.

## Failure Conditions

- Uses WebFetch or curl instead of `defuddle parse` to retrieve the page
- Runs defuddle without the `--md` flag
- Uses shell redirection (`>`) instead of the `-o` flag to write the file
- The `bun-installation.md` file does not exist or is empty after the command
