# Phase 6: Delete Original Hub (Breaking Change)

**Status:** Pending  
**Effort:** ~10 minutes  
**Dependencies:** Phase 0 (dependency check complete), Phase 5

## Description

Delete the entire `software-design-principles/` directory as a breaking change, committing with clear migration guidance.

**Prerequisites:** Phase 0 dependency report must be reviewed to ensure all codebase references are cataloged for Phase 7 updates.

## Commands

```bash
# Remove entire original skill directory
git rm -r skills/software-engineering/software-design-principles/

# Verify deletion
ls skills/software-engineering/software-design-principles 2>&1 | grep -q "No such file or directory" && echo "✓ Directory deleted"
```

## Commit Message

```
BREAKING: Split software-design-principles into focused skills

Replaced monolithic hub skill with 4 focused skills under design-principles tile:
- solid-principles: Tactical class-level design (SRP, OCP, LSP, ISP, DIP)
- clean-architecture: Strategic boundaries and layers
- design-patterns: Structural pattern selection
- testable-design: Testing architecture principles

All 44 reference documents redistributed to appropriate skills.

Migration:
- Old: pantheon-ai/software-design-principles
- New: pantheon-ai/design-principles (contains 4 skills)
  - solid-principles
  - clean-architecture
  - design-patterns
  - testable-design

Update agent configs to reference specific focused skills instead of hub.

Closes #<issue-number>
```

## Acceptance Criteria

- [ ] Original directory completely removed
- [ ] Git history shows breaking change commit
- [ ] Commit message includes migration guidance
- [ ] No broken references remain in repository

## Verification

```bash
# Verify directory deleted
! test -d skills/software-engineering/software-design-principles && echo "✓ Original hub deleted"

# Check for broken references in other files
rg "software-design-principles" --type md --glob '!.context/' --glob '!.git/' skills/ AGENTS.md README.md

# Expected: No results (or only in CHANGELOG/migration docs)
```

## Post-Deletion Tasks

- [ ] Update any AGENTS.md references from old skill to new tile
- [ ] Update skill-taxonomy.md with new classifications
- [ ] Add entry to repository CHANGELOG.md (if exists)

## Notes

- This is intentionally a breaking change
- Users must update their agent configurations
- No backward compatibility maintained (cleaner than deprecation)
- Git history preserves all original content via `git mv` from Phase 2
