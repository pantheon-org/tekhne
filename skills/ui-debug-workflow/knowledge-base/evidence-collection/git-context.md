# Git Context

## Essential Information

Always include in debug reports:

### Current Branch

```bash
git branch --show-current
```

### Latest Commit

```bash
git log -1 --oneline
# or more detailed:
git log -1 --format="%H%n%an <%ae>%n%ad%n%s"
```

### Changes

```bash
# Full diff between baseline and changed
git diff main...HEAD > changes.diff

# Just file names
git diff --name-status main...HEAD

# Statistics
git diff --stat main...HEAD
```

## Including in Reports

### Baseline Section

```markdown
## Baseline

**Branch**: `main`
**Commit**: `abc1234 Fix syntax highlighting`
**Date**: 2026-02-04 14:30 UTC
```

### Changed Section

```markdown
## Changed

**Branch**: `fix/code-annotations`
**Commit**: `def5678 Use plain text wrapper for annotations`
**Date**: 2026-02-05 10:15 UTC

**Files Modified**:
- M packages/ui/components/Viewer.tsx
- M packages/ui/utils/highlighter.ts
```

### Full Diff

Include in appendix:

```markdown
## Appendix A: Git Diff

See [changes.diff](./changes.diff) for complete diff.

**Summary**:
- 2 files changed
- 45 insertions(+)
- 12 deletions(-)
```

## Automated Capture

```bash
#!/bin/bash
# capture-git-context.sh

OUTPUT_DIR="${1:-.}"

mkdir -p "$OUTPUT_DIR/git"

# Branch and commit
git branch --show-current > "$OUTPUT_DIR/git/branch.txt"
git log -1 --oneline > "$OUTPUT_DIR/git/commit.txt"
git log -1 --format="%H%n%an <%ae>%n%ad%n%s%n%n%b" > "$OUTPUT_DIR/git/commit-full.txt"

# Changes
git diff main...HEAD > "$OUTPUT_DIR/git/changes.diff"
git diff --name-status main...HEAD > "$OUTPUT_DIR/git/changed-files.txt"
git diff --stat main...HEAD > "$OUTPUT_DIR/git/stats.txt"

echo "✅ Git context saved to $OUTPUT_DIR/git/"
```

## Related Documentation

- [Logs](./logs.md) - Build and runtime logs
- [Report Structure](../reporting/comprehensive-template.md) - Using git info in reports
