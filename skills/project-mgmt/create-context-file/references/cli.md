# Generator CLI reference

`scripts/create-context-file.sh` writes a date-prefixed context file into a
typology subfolder and prints the created path.

## Synopsis

```bash
scripts/create-context-file.sh --type <typology> --title "<title>" [options]
# optional body content is read from stdin
```

## Options

| Flag | Argument | Purpose |
| --- | --- | --- |
| `-t`, `--type` | `TYPE` | Typology / subfolder (required). Validated against `KNOWN_TYPES`. |
| `-T`, `--title` | `TITLE` | Human-readable title (required); also seeds the slug. |
| `-s`, `--slug` | `SLUG` | Override the auto-derived slug. |
| `-g`, `--tags` | `TAGS` | Comma-separated tags for frontmatter. |
| `-d`, `--date` | `DATE` | Override the date (`YYYY-MM-DD`); defaults to today. |
| `-r`, `--root` | `DIR` | Context root; defaults to `.context`. |
| `-A`, `--allow-new-type` | | Permit a typology not in `KNOWN_TYPES`. |
| `-n`, `--dry-run` | | Print the target path and file body; write nothing. |
| `-f`, `--force` | | Overwrite an existing target file. |
| `-h`, `--help` | | Show usage. |

## Behavior

- The output path is `<root>/<typology>/<YYYY-MM-DD>-<slug>.md`, where `<root>`
  is resolved against the current working directory (the consuming project).
- The slug is derived from the title (lowercased, non-alphanumeric runs
  collapsed to single hyphens) unless `--slug` overrides it.
- The date must match `YYYY-MM-DD`; other formats are rejected.
- An unknown typology is rejected unless `--allow-new-type` is passed.
- An existing target file is never overwritten unless `--force` is passed.

## Examples

```bash
# Finding with tags
scripts/create-context-file.sh --type findings --title "Auth token analysis" \
  --tags "auth,oauth"

# Plan with a heredoc body
scripts/create-context-file.sh --type plans --title "Retriever rollout" << 'EOF'
## Phase 1
...
EOF

# Preview only
scripts/create-context-file.sh --type merge-requests --title "PROJ-1234 retriever" --dry-run
```
