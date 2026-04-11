# Scenario 04: Extract Design Tokens from a React App

## User Prompt

A client has asked you to port the visual theme from their existing React marketing site at `https://app.client-brand.io` into a new project. You have confirmed that `agent-browser` is available in your environment.

The site is a single-page React application — its content is rendered client-side after JavaScript executes.

Write a shell script called `extract.sh` that performs the complete Stage 1 extraction for this site. The script should:
- Set up the artifact directory
- Navigate to the URL, handle the SPA rendering behaviour appropriately, and take screenshots
- Extract the CSS custom properties and computed element styles and save them to the artifact directory

Use `app-client-brand-io` as the website slug and `2026-03-13` as the date.

The script does not need to be runnable in this environment — write it as if you would execute it against the live site.

## Expected Behavior

1. Use `agent-browser` commands (not `curl` or `wget`) as the primary extraction method since it was stated to be available
2. Define `ARTIFACTS` as a variable pointing to `.context/artifacts/app-client-brand-io/2026-03-13`
3. Create the ARTIFACTS directory with `mkdir -p` before writing any files
4. Include a `window.scrollTo(0, document.body.scrollHeight)` eval or equivalent scroll step before extracting tokens (SPA pattern)
5. Wait for `networkidle` after the scroll step (e.g., `agent-browser wait --load networkidle`)
6. Include a JS eval that extracts CSS custom properties from `:root` (filtering for `--` prefixed properties) and saves to a JSON file under ARTIFACTS
7. Include a JS eval that extracts computed styles from key elements (at minimum: body, buttons/CTAs) and saves to a JSON file under ARTIFACTS
8. Save all screenshots to `${ARTIFACTS}/` paths — not to `/tmp` or `docs/`
9. Wait for `networkidle` after opening the page before attempting any extraction
10. Take at least one full-page screenshot and one mobile-viewport screenshot (375px wide)

## Success Criteria

- **Uses agent-browser**: `extract.sh` uses `agent-browser` commands (not `curl` or `wget`) as the primary extraction method
- **ARTIFACTS path defined**: `extract.sh` defines `ARTIFACTS` as a variable pointing to `.context/artifacts/app-client-brand-io/2026-03-13`
- **mkdir -p ARTIFACTS**: `extract.sh` creates the ARTIFACTS directory with `mkdir -p` before writing any files
- **SPA scroll step**: `extract.sh` includes a `window.scrollTo(0, document.body.scrollHeight)` eval or equivalent scroll step before extracting tokens
- **Wait networkidle after scroll**: `extract.sh` waits for `networkidle` after the scroll step
- **CSS vars extraction**: `extract.sh` includes a JS eval that extracts CSS custom properties from `:root` and saves to a JSON file under ARTIFACTS
- **Computed styles extraction**: `extract.sh` includes a JS eval that extracts computed styles from key elements and saves to a JSON file under ARTIFACTS
- **Screenshots under ARTIFACTS**: All screenshot commands save files to `${ARTIFACTS}/` paths
- **wait --load networkidle on open**: `extract.sh` waits for the page to fully load after opening before attempting extraction
- **Full-page and mobile screenshots**: `extract.sh` takes at least one full-page screenshot and one mobile-viewport screenshot

## Failure Conditions

- Agent uses `curl` or `wget` instead of `agent-browser` for the main extraction
- Agent saves files to `/tmp` or `docs/` instead of `.context/artifacts/`
- Agent does not handle SPA rendering (no scroll step or networkidle wait)
- Agent does not extract CSS custom properties from `:root`
- Agent does not extract computed styles from key elements
- Agent takes only a single screenshot with no mobile-viewport variant
- Agent does not `mkdir -p` the ARTIFACTS directory before writing files
