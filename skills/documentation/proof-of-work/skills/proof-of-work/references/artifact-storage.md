# Artifact Storage Reference

## Check for a Project-Specific Convention First

Before defaulting to `.context/evidence/`, check whether the project already has its own established location for evidence tied to a specific document (a journal entry, a report, a ticket write-up). A generic cross-cutting bucket is wrong for evidence that belongs to one specific artifact — e.g. a project that documents work as dated entries may already store an entry's supporting files in a directory named after that entry (`<entry-slug>/assets/`), sitting next to it, rather than in a shared `.context/evidence/` folder. Look for an existing sibling directory pattern near similar past documents before assuming the default below applies, and check that project's own skill/instruction docs for an explicit convention.

## Default Storage Location (no project-specific convention found)

Otherwise, proof-of-work artifacts are stored under `.context/evidence/` in the repository root.

```
.context/
└── evidence/
    ├── 2026-04-07-dashboard-post-deploy.png
    ├── 2026-04-07-test-run.txt
    └── 2026-04-07-migration-output.json
```

## Naming Convention

```
YYYY-MM-DD-<slug>.<ext>
```

| Part | Rules | Example |
|---|---|---|
| `YYYY-MM-DD` | ISO date, today | `2026-04-07` |
| `<slug>` | kebab-case, describes content | `checkout-page-after-fix` |
| `<ext>` | matches content type | `png`, `txt`, `json` |

### Extension Guide

| Content | Extension |
|---|---|
| Browser screenshot | `.png` |
| Terminal / log output | `.txt` |
| Structured API response | `.json` |
| Multi-page report | `.md` |

## Supported Capture Methods

### playwright-mcp

```javascript
await page.goto('https://example.com/page');
await page.screenshot({
  path: '.context/evidence/2026-04-07-page-name.png',
  fullPage: true
});
```

### agent-browser

```
action: screenshot
save_to: .context/evidence/2026-04-07-page-name.png
```

### Shell log capture

```bash
<command> 2>&1 | tee .context/evidence/2026-04-07-output.txt
```

### Structured output

```bash
<command> > .context/evidence/2026-04-07-result.json
```

## Evidence Summary Block Template

Include in every response where artifacts were captured:

```markdown
## Evidence

| Artifact | Path | Description |
|---|---|---|
| Screenshot | `.context/evidence/YYYY-MM-DD-slug.png` | What it shows |
| Log | `.context/evidence/YYYY-MM-DD-slug.txt` | What was captured |
```

## Gitignore Considerations

`.context/evidence/` should be committed for audit trails, or gitignored if artifacts are large or contain secrets. Check the project's `.gitignore` and follow the existing convention.
