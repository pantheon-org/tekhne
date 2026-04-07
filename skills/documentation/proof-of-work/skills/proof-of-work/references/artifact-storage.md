# Artifact Storage Reference

## Default Storage Location

All proof-of-work artifacts are stored under `.context/evidence/` in the repository root.

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
