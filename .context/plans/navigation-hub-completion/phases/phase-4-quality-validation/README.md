# Phase 4: Quality Validation

**Duration:** 30 minutes
**Purpose:** Run audits on all converted skills; confirm Navigation Hub pattern achieves score improvement

## Baseline (from 2026-02-21 audit)

| Skill | Baseline Score | Grade |
|-------|---------------|-------|
| nx-bun-integration | 87/120 | C |
| conventional-commits | 87/120 | C |
| extending-nx-plugins | 88/120 | C |
| nx-workspace-patterns | 88/120 | C |
| nx-biome-integration | 85/120 | C |

**Target:** All five skills at ≥ 96/120 (B grade)

## Tasks

### 1. Run Audits on All Five Skills

```bash
bun cli/index.ts audit skill repository-mgmt/nx/bun-integration
bun cli/index.ts audit skill repository-mgmt/nx/biome-integration
bun cli/index.ts audit skill repository-mgmt/nx/workspace-patterns
bun cli/index.ts audit skill documentation/conventional-commits
bun cli/index.ts audit skill repository-mgmt/nx/extending-plugins
```

### 2. Record Results

Fill in the table:

| Skill | Baseline | Post-Conversion | Delta | Pass? |
|-------|----------|----------------|-------|-------|
| nx-bun-integration | 87/120 | | | |
| nx-biome-integration | 85/120 | | | |
| nx-workspace-patterns | 88/120 | | | |
| conventional-commits | 87/120 | | | |
| extending-nx-plugins | 88/120 | | | |

### 3. Remediate Any Failing Skills

If a skill scores < 96/120, inspect the dimensional breakdown:

```bash
bun cli/index.ts audit skill <path> --verbose
```

Common fixes:
- **D2 (description)**: Tighten the SKILL.md frontmatter `description:` — must be ≤ 200 chars, action-oriented
- **D5 (anti-patterns)**: Add or sharpen anti-pattern entries (BAD/GOOD/WHY format)
- **D6 (scope)**: Ensure hub SKILL.md has clean When to Use / When Not to Use sections
- **D7 (references)**: Confirm Quick Reference table links are valid and reference files are substantive

Iterate until all scores reach target.

### 4. Save Audit Results

```bash
mkdir -p .context/audits/documentation/conventional-commits/2026-03-05
mkdir -p .context/audits/repository-mgmt/nx/bun-integration/2026-03-05
mkdir -p .context/audits/repository-mgmt/nx/biome-integration/2026-03-05
mkdir -p .context/audits/repository-mgmt/nx/workspace-patterns/2026-03-05
mkdir -p .context/audits/repository-mgmt/nx/extending-plugins/2026-03-05
```

Save each audit output to the corresponding directory.

### 5. Update Investigation File

Add a status update to the original investigation:

`.context/investigations/navigation-hub-candidates-2026-03-04.md`

Append a "Resolution" section with final scores and link to audit outputs.

## Success Criteria

- [ ] All five skills score ≥ 96/120 (B)
- [ ] Audit results saved to `.context/audits/`
- [ ] Investigation file updated with resolution status

## Outputs

- Post-conversion audit scores for all 5 skills
- Dimensional breakdown for any skill that required remediation
- Updated investigation file

## Plan Complete

When all success criteria are met, this plan is done.
Update `README.md` status from `Draft` → `Complete` and record the completion date.
