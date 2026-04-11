# Scenario 05: Security Skip for Sensitive Files in .context/session/in/

## User Prompt

"Create context. My .context/session/in/ has a README, an API spec, a .env.production file, and a secrets.yaml."

## Expected Behavior

1. Agent scans `.context/session/in/` and identifies four files
2. Agent automatically skips `.env.production` (matches `.env*` pattern) without prompting the user
3. Agent automatically skips `secrets.yaml` (matches `*secrets*` pattern) without prompting the user
4. Agent emits a warning or note for each skipped file explaining it was excluded for security reasons
5. Agent proceeds to prioritize only the two non-sensitive files (README, API spec)
6. Agent asks the user to assign HIGH / MEDIUM / LOW to the two safe files
7. Manifest lists only the safe files; no reference to the skipped files appears in `.context/session/ctx/`

## Success Criteria

- **Auto-skip applied**: `.env.production` and `secrets.yaml` are excluded without user intervention
- **Skip patterns matched**: All patterns from `reference.md` are enforced (`.env*`, `*credentials*`, `*secrets*`, `*token*`, `*.key`, `*.pem`, `*.crt`)
- **Warning emitted**: Agent reports which files were skipped and why (security pattern match)
- **Manifest clean**: Skipped files do not appear anywhere in `manifest.yaml` or the baseline
- **Safe files processed**: README and API spec proceed through normal prioritization and copy workflow

## Failure Conditions

- Agent includes `.env.production` or `secrets.yaml` in the manifest or copies them to `.context/session/ctx/`
- Agent asks the user whether to include a sensitive file instead of auto-skipping it
- Agent silently skips files without reporting which files were excluded
- Only a subset of the security patterns are enforced (e.g., `.env*` but not `*secrets*`)
- Agent aborts entirely instead of processing the safe files
