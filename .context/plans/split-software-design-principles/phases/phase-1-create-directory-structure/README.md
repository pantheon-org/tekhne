# Phase 1: Create Directory Structure

**Status:** Pending  
**Effort:** ~15 minutes  
**Dependencies:** Phase 0 (baseline established)

## Description

Create the new `design-principles/` tile directory structure with subdirectories for 4 focused skills and their supporting files.

## Activities

- [ ] Create tile root: `skills/software-engineering/design-principles/`
- [ ] Create 4 skill subdirectories: `solid-principles/`, `clean-architecture/`, `design-patterns/`, `testable-design/`
- [ ] Create `references/` and `evals/` within each skill directory
- [ ] Copy `.vscode/` settings from original skill to tile root

## Commands

```bash
mkdir -p skills/software-engineering/design-principles/{solid-principles,clean-architecture,design-patterns,testable-design}/{references,evals}
cp -r skills/software-engineering/software-design-principles/.vscode skills/software-engineering/design-principles/
```

## Acceptance Criteria

- [ ] All 4 skill directories exist
- [ ] Each skill has `references/` and `evals/` subdirectories
- [ ] `.vscode/` settings copied to tile root

## Verification

```bash
# Verify structure
tree -L 3 skills/software-engineering/design-principles

# Expected output:
# design-principles/
# ├── .vscode/
# ├── solid-principles/
# │   ├── references/
# │   └── evals/
# ├── clean-architecture/
# │   ├── references/
# │   └── evals/
# ├── design-patterns/
# │   ├── references/
# │   └── evals/
# └── testable-design/
#     ├── references/
#     └── evals/
```
