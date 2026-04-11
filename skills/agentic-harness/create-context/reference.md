# create-context Reference

## Manifest Schema (.context/session/ctx/manifest.yaml)

```yaml
project: string                    # Auto-inferred from directory
created: ISO8601                   # Timestamp of creation
source_folder: .context/session/in/  # Reference to immutable bootstrap folder

sources:
  high:                            # ≤1500 tokens inline, >1500 summarized
    - file: string                 # Path relative to .context/session/in/
      desc: string                 # Human-readable description
      action: inline|summarized    # Auto-set based on size
  medium:                          # ≤2500 tokens inline, >2500 summarized
    - file: string
      desc: string
      action: inline|summarized
  low:                             # Reference only, no content copied
    - file: string
      desc: string

tags:                              # Optional categorization
  category_name: [file1, file2]
```

## Context Sizing Behavior

| Priority | Token Count | Action |
|----------|-------------|--------|
| HIGH | ≤1500 | Copy to `.context/session/ctx/`, inline in baseline |
| HIGH | >1500, ≤25K | Copy to `.context/session/ctx/`, summarize directly |
| HIGH | >25K | Copy to `.context/session/ctx/`, summarize via sub-agent |
| MEDIUM | ≤2500 | Copy to `.context/session/ctx/`, inline in baseline |
| MEDIUM | >2500, ≤25K | Copy to `.context/session/ctx/`, summarize directly |
| MEDIUM | >25K | Copy to `.context/session/ctx/`, summarize via sub-agent |
| LOW | any | Reference only (path to `.context/session/in/`, no copy) |

**Token estimation**: `tokens ≈ words / 0.75` (using `wc -w`)

**25K limit**: Files exceeding this use `summarize-for-context` sub-agent with chunked reads.

## Baseline Template (.context/session/CONTEXT-baseline-llm.md)

```markdown
# Baseline Context

saved: {ISO 8601 timestamp}
stream: baseline
manifest: .context/session/ctx/manifest.yaml
source: .context/session/in/

## Project

ref: .context/session/ctx/manifest.yaml
source_folder: .context/session/in/
files: {total_count}

## Sources (inline)

### {filename}
{Full content from .context/session/ctx/{file}}

## Sources (summarized)

### {filename}
See: `.context/session/ctx/{nn}-{basename}-summary-llm.md`

## Sources (reference only)

- `.context/session/in/{path}`: {description}

## Refs

manifest: .context/session/ctx/manifest.yaml
summaries: [.context/session/ctx/01-*.md, .context/session/ctx/02-*.md, ...]
source: .context/session/in/
```

**Token budget**: ≤2000 tokens total.

## Validation Rules

1. **Required**: project, created, source_folder, sources (high/medium/low)
2. **File paths**: Relative to `.context/session/in/`, must exist at generation time
3. **Descriptions**: Required, non-empty, concise (1-2 sentences)
4. **Action field**: Auto-set, user can override
5. **Tags**: Optional, keys alphanumeric/hyphens/underscores
6. **All priority sections**: Must be present (can be empty arrays)

## Security

Auto-skipped: `.env*`, `*credentials*`, `*secrets*`, `*token*`, `*.key`, `*.pem`, `*.crt`

## RISEN INPUT Table (output)

```markdown
| Priority | File | Description |
|----------|------|-------------|
| 1 (HIGH) | @.context/session/ctx/proposal.md | Main objectives |
| 2 (MED) | @.context/session/ctx/style-guide.md | Style reference |
| 3 (LOW) | @.context/session/in/old-notes.md | Background (ref only) |
```

## Scripts

- `scripts/scan-in-folder.sh` — Scan .context/session/in/ for supported files
- `scripts/validate-manifest.sh` — Validate manifest schema

## Error Messages

| Check | Message |
|-------|---------|
| `[ -d .context/session/in/ ]` | "No .context/session/in/ folder found. Create it and add source files first." |
| `ls .context/session/in/` empty | "No files in .context/session/in/ folder. Add documents first." |
| `[ -d .context/session/ctx/ ]` | ".context/session/ctx/ exists. Use --force to recreate." |

## Related

- `/save-context [stream]` - Save session to named stream
- `/load-context [stream]` - Load saved stream
- `scripts/validate-manifest.sh` - Validate manifest schema
- `.claude/agents/summarize-for-context.md` - Sub-agent for large files
