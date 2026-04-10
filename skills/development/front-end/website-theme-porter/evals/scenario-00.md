# Scenario 00: Theme Extraction Setup

## User Prompt

A client has asked you to begin porting the visual identity of their competitor's marketing site at `https://example.com` into a React project.

Before touching any code, you need to take initial screenshots of the site and save a brief overview document describing what you found. You do not need to actually open a browser or download any files — instead, write a shell script (`setup.sh`) that represents exactly the commands you would run to set up the working directory and capture initial screenshots, and create the placeholder directory structure and documentation files that would be produced at the end of Stage 1.

Produce the following:

1. `setup.sh` — the shell script that sets up the artifact directory and captures screenshots (using `agent-browser` commands). The script should be executable.
2. The directory structure and empty placeholder files that the script would create. Use `example-com` as the website slug and today's date (`2026-03-13`) as the date.
3. `overview.md` inside the theme subdirectory — filled in with: the source URL, today's date, extraction method used, and a placeholder note about what was found.

## Expected Behavior

1. Define `ARTIFACTS` in `setup.sh` as a path starting with `.context/artifacts/`
2. Use a lowercase hyphenated slug (`example-com`) as the directory name under `.context/artifacts/`
3. Include a `YYYY-MM-DD` date subdirectory under the slug directory
4. Do not save any file to `/tmp`
5. Do not save any file to `docs/` or `docs/screenshots/`
6. Use `${ARTIFACTS}/` as the base path for all `agent-browser` screenshot commands
7. Create a `theme/` subdirectory inside the ARTIFACTS path
8. Place `overview.md` inside `.context/artifacts/example-com/2026-03-13/theme/`
9. Note the extraction method in `overview.md`
10. Include the source URL (`https://example.com`) in `overview.md`

## Success Criteria

- **Correct base path**: `setup.sh` defines `ARTIFACTS` as a path starting with `.context/artifacts/`
- **Slug format**: `setup.sh` uses a lowercase hyphenated slug (e.g., `example-com`) as the directory name under `.context/artifacts/`
- **Date subdirectory**: `setup.sh` includes a `YYYY-MM-DD` date subdirectory under the slug directory
- **No /tmp usage**: `setup.sh` does NOT save any file to `/tmp`
- **No docs/ usage**: `setup.sh` does NOT save any file to `docs/` or `docs/screenshots/`
- **Screenshots under ARTIFACTS**: All `agent-browser` screenshot commands in `setup.sh` use `${ARTIFACTS}/` as the base path
- **theme/ subdirectory**: `setup.sh` creates a `theme/` subdirectory inside the ARTIFACTS path
- **overview.md location**: `overview.md` is placed inside `.context/artifacts/example-com/2026-03-13/theme/`
- **Extraction method noted**: `overview.md` contains a field or note identifying which extraction method was used
- **Source URL noted**: `overview.md` contains the source URL (`https://example.com`)

## Failure Conditions

- Agent writes artifact files to `/tmp` instead of `.context/artifacts/`
- Agent writes artifact files to `docs/` or `docs/screenshots/`
- Agent does not include a date subdirectory under the slug
- Agent does not create a `theme/` subdirectory
- Agent places `overview.md` in the project root or under `docs/` instead of the theme subdirectory
- Agent omits the extraction method from `overview.md`
- Agent omits the source URL from `overview.md`
