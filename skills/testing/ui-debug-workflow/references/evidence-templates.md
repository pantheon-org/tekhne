# Evidence Templates

Use these templates to standardize debug output.

## Session Header

```markdown
# UI Debug Session

- Baseline branch:
- Changed branch:
- URL:
- Viewport:
- Build command:
- Start command:
- Timestamp:
```

## Comparison Summary

```markdown
## Comparison Summary

| Check | Baseline | Changed | Status |
| --- | --- | --- | --- |
| Initial render | Pass | Pass | ✅ |
| Target bug reproduction | Reproducible | Not reproducible | ✅ |
| Console errors | 2 errors | 0 errors | ✅ |
```

## Findings Block

```markdown
## Findings

- Root cause:
- Fix implemented:
- Residual risks:
- Follow-up tasks:
```
