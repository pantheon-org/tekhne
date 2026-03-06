# Navigation Hub Completion Plan

**Status:** Draft
**Created:** 2026-03-05
**Source:** `.context/investigations/navigation-hub-candidates-2026-03-04.md`
**Effort:** ~4-6 hours

## Context

The skill-quality-auditor flagged five skills for Navigation Hub conversion. Since the investigation was written (2026-03-04), three of the five have already been converted. This plan completes the remaining work and verifies the existing hubs are healthy.

## Current State (2026-03-05)

| Skill | Audit Lines | Status | Action |
|-------|-------------|--------|--------|
| `nx-bun-integration` | 897 | ✅ Hub done (128-line SKILL.md + 4 refs) | Verify refs are substantive |
| `nx-biome-integration` | 383 | ✅ Hub done (135-line SKILL.md + 3 refs) | Verify refs are substantive |
| `nx-workspace-patterns` | 457 | ✅ Hub done (178-line SKILL.md + 4 refs) | Verify refs are substantive |
| `conventional-commits` | 636 | ⚠️ Reduced to 109 lines, no hub refs | Assess — may be complete as-is |
| `extending-nx-plugins` | 553 | ❌ Still 432-line monolith | Full Navigation Hub conversion |

## Phases

0. [Phase 0: State Audit](phases/phase-0-state-audit/) — Confirm exact current state of all 5 skills
1. [Phase 1: Convert extending-nx-plugins](phases/phase-1-extending-nx-plugins/) — Main work: split into hub + reference files
2. [Phase 2: Conventional Commits](phases/phase-2-conventional-commits/) — Assess and resolve open question
3. [Phase 3: Reference File Audit](phases/phase-3-reference-audit/) — Verify existing hub reference files are populated
4. [Phase 4: Quality Validation](phases/phase-4-quality-validation/) — Re-run audits; confirm score improvement

## Success Criteria

- [ ] `extending-nx-plugins` SKILL.md ≤ 200 lines, hub structure with ≥3 reference files
- [ ] `extending-nx-plugins` audit score improves from 88/120 (C) to ≥96/120 (B)
- [ ] `conventional-commits` status resolved (hub or confirmed complete)
- [ ] All existing hub reference files verified non-empty and coherent
- [ ] Post-conversion audit run; results saved to `.context/audits/`

## Risks

**Risk:** extending-nx-plugins has no tile.json — needs creation alongside hub conversion
**Mitigation:** Create tile.json in Phase 1 as part of the hub setup

**Risk:** Existing hub reference files may be empty stubs
**Mitigation:** Phase 3 reads each reference file before declaring done

**Risk:** conventional-commits at 109 lines may score fine without hub conversion
**Mitigation:** Phase 2 runs an audit first; only convert if score is below B
