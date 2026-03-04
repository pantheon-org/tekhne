# Phase 0: Baseline Audit & Dependency Check

**Duration:** 30-45 minutes  
**Purpose:** Establish quality baseline, verify reference count, identify codebase dependencies

## Tasks

### 1. Audit Current Hub Quality

Run quality audit on existing `software-design-principles` to establish baseline scores:

```bash
bun cli/index.ts audit skill software-engineering/software-design-principles
```

**Expected Output:**
- Dimensional scores (9 dimensions)
- Overall grade (current: B+ at 97/120 on old scale)
- Baseline for comparing new skills

**Success Criteria:**
- [ ] Audit report saved to `.context/audits/software-engineering/software-design-principles/latest/`
- [ ] Baseline score documented in this phase README

### 2. Verify Reference Doc Count

Confirm exactly 44 reference docs exist:

```bash
find skills/software-engineering/software-design-principles/references -type f -name '*.md' | wc -l
```

**Expected:** `44`

**Success Criteria:**
- [ ] Count verified as 44 docs

### 3. Identify Codebase Dependencies

Search for all references to `software-design-principles` across the repository:

```bash
rg "software-design-principles" \
  --type md \
  --glob '!.context/' \
  --glob '!.git/' \
  --glob '!node_modules/' \
  . > .context/plans/split-software-design-principles/dependency-report.txt
```

**Review for:**
- Other skill SKILL.md files referencing this hub
- AGENTS.md classifier rules
- README.md skill tables
- Tessl tile.json files

**Success Criteria:**
- [ ] Dependency report created
- [ ] All references cataloged for Phase 7 updates

### 4. Analyze Reference Doc Distribution

Review existing reference docs to determine optimal redistribution:

```bash
ls -1 skills/software-engineering/software-design-principles/references/ | head -20
```

**Categorize by prefix:**
- `dep-*.md` → clean-architecture (6 docs)
- `comp-*.md` → clean-architecture (5 docs)
- `bound-*.md` → clean-architecture (6 docs)
- `frame-*.md` → clean-architecture (5 docs)
- `entity-*.md` → clean-architecture (5 docs)
- `usecase-*.md` → clean-architecture (6 docs)
- `adapt-*.md` → clean-architecture (5 docs)
- `test-*.md` → testable-design (4 docs)
- Anti-patterns/examples → design-patterns (2 docs)
- **To be created:**
  - SOLID refactoring examples → solid-principles (2-3 docs)
  - Pattern implementation guides → design-patterns (3-5 docs)

**Success Criteria:**
- [ ] Distribution plan documented
- [ ] New reference docs identified (5-8 docs to create)

## Outputs

- Baseline audit report
- Reference doc count verification
- Dependency report (all `software-design-principles` references)
- Reference doc distribution plan

## Next Phase

[Phase 1: Create Directory Structure](../phase-1-create-directory-structure/)
