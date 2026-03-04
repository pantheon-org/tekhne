# Phase 7: Update Repository Documentation

**Status:** Pending  
**Effort:** ~20 minutes  
**Dependencies:** Phase 6

## Description

Update repository-level documentation to reflect the new tile structure and remove references to the old hub skill.

**Input:** Phase 0 dependency report (`.context/plans/split-software-design-principles/dependency-report.txt`) identifying all codebase references.

## Files to Update

### 1. AGENTS.md

**Location:** `/Users/thomas.roche/Projects/github/pantheon-org/tekhne/AGENTS.md`

**Changes:**
- Remove any references to `software-design-principles`
- Add references to `design-principles` tile in software-engineering domain
- Update skill management examples if they reference the old skill

**Search pattern:**
```bash
rg "software-design-principles" AGENTS.md
```

### 2. skill-taxonomy.md

**Location:** `skills/agentic-harness/skill-quality-auditor/references/skill-taxonomy.md`

**Changes:**
- Remove `software-design-principles` classification
- Add 4 new skill classifications:
  - `solid-principles` → software-engineering / tactical design
  - `clean-architecture` → software-engineering / strategic architecture
  - `design-patterns` → software-engineering / structural patterns
  - `testable-design` → software-engineering / testing architecture

### 3. CHANGELOG.md (if exists)

**Location:** `CHANGELOG.md` (root, if present)

**Entry:**
```markdown
## [Unreleased]

### Breaking Changes

- **Split software-design-principles into focused skills**: The monolithic `software-design-principles` hub has been replaced by a unified `design-principles` tile containing 4 focused skills:
  - `solid-principles`: Tactical class-level design
  - `clean-architecture`: Strategic architecture and boundaries
  - `design-patterns`: Structural pattern selection
  - `testable-design`: Testing architecture principles
  
  **Migration:** Update skill references in agent configs from `pantheon-ai/software-design-principles` to specific focused skills under `pantheon-ai/design-principles`.
```

### 4. Repository README

**Location:** `README.md` (root)

**Known reference:** Skill table entry with B+ rating and Tessl registry link (from Phase 0 grep)

**Changes:**
- Remove `software-design-principles` table row
- Add 4 new rows for focused skills:
  - `solid-principles` with rating, path, Tessl link
  - `clean-architecture` with rating, path, Tessl link
  - `design-patterns` with rating, path, Tessl link
  - `testable-design` with rating, path, Tessl link
- Update skill count if documented (total increases by 3: 1 hub → 4 focused)

## Acceptance Criteria

- [ ] AGENTS.md updated with new tile references
- [ ] skill-taxonomy.md includes 4 new skill classifications
- [ ] CHANGELOG.md entry added (if file exists)
- [ ] No broken references to old skill in documentation
- [ ] All documentation passes markdownlint

## Verification

```bash
# Search for remaining references
rg "software-design-principles" --type md --glob '!.context/' --glob '!.git/' .

# Expected: Only in CHANGELOG or migration notes

# Verify taxonomy updates
grep -E "(solid-principles|clean-architecture|design-patterns|testable-design)" \
  skills/agentic-harness/skill-quality-auditor/references/skill-taxonomy.md

# Run linting
bunx markdownlint-cli2 "AGENTS.md" "README.md" "CHANGELOG.md" \
  "skills/agentic-harness/skill-quality-auditor/references/skill-taxonomy.md"
```

## Notes

- Be thorough in searching for references
- Update both human-readable docs and machine-readable configs
- Preserve historical references in changelogs/migration guides
