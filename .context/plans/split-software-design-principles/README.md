# Split software-design-principles Hub Skill

**Status:** Draft  
**Created:** 2026-03-04  
**Updated:** 2026-03-04  
**Effort:** ~12-15 hours

## Overview

Split the monolithic `software-design-principles` hub skill (254 lines, 44 reference docs) into 4 focused, single-responsibility skills under a unified tile, following Agent Skills best practices.

## Target Skills (Single Tile)

**Tile:** `skills/software-engineering/design-principles/tile.json`  
**Namespace:** `pantheon-ai/design-principles`

### 1. solid-principles
**Path:** `skills/software-engineering/design-principles/solid-principles/`  
**Scope:** Tactical class-level design (SRP, OCP, LSP, ISP, DIP)  
**References:** 2-3 docs (code examples, refactoring patterns)

### 2. clean-architecture  
**Path:** `skills/software-engineering/design-principles/clean-architecture/`  
**Scope:** Strategic boundaries, dependency rules, layer isolation  
**References:** 27-29 docs (dependencies, components, boundaries, frameworks, entities, use cases, adapters)

### 3. design-patterns
**Path:** `skills/software-engineering/design-principles/design-patterns/`  
**Scope:** Structural patterns, when/when-not to use, pattern selection  
**References:** 8-9 docs (anti-patterns, examples, pattern selection workflow, implementation guides, boundary pattern applications)

### 4. testable-design
**Path:** `skills/software-engineering/design-principles/testable-design/`  
**Scope:** Testing architecture, boundary verification, layer isolation  
**References:** 4 docs (boundary verification, testable design, layer isolation, tests-as-architecture)

## File Mapping Summary

| Original | New Location | Count |
|----------|-------------|-------|
| `software-design-principles/` | **DELETED** | - |
| `references/dep-*.md` | `clean-architecture/references/` | 6 |
| `references/comp-*.md` | `clean-architecture/references/` | 5 |
| `references/bound-*.md` | `clean-architecture/references/` (3) + `design-patterns/references/` (3) | 6 |
| `references/frame-*.md` | `clean-architecture/references/` | 5 |
| `references/entity-*.md` | `clean-architecture/references/` | 5 |
| `references/usecase-*.md` | `clean-architecture/references/` | 6 |
| `references/adapt-*.md` | `clean-architecture/references/` | 5 |
| `references/test-*.md` | `testable-design/references/` | 4 |
| `references/anti-patterns-and-frameworks.md` | `design-patterns/references/` | 1 |
| `references/detailed-examples.md` | `design-patterns/references/` | 1 |
| **Total** | | **44** |

## Phases

0. [Phase 0: Baseline Audit & Dependency Check](phases/phase-0-baseline-audit/)
1. [Phase 1: Create Directory Structure](phases/phase-1-create-directory-structure/)
2. [Phase 2: Redistribute Reference Documents](phases/phase-2-redistribute-reference-documents/)
3. [Phase 3: Author New SKILL.md Files](phases/phase-3-author-new-skill-files/)
4. [Phase 4: Create Unified tile.json](phases/phase-4-create-unified-tile/)
5. [Phase 5: Create Eval Scenarios](phases/phase-5-create-eval-scenarios/)
6. [Phase 6: Delete Original Hub](phases/phase-6-delete-original-hub/)
7. [Phase 7: Update Repository Documentation](phases/phase-7-update-repository-docs/)
8. [Phase 8: Validation](phases/phase-8-validation/)
9. [Phase 9: Tessl Integration](phases/phase-9-tessl-integration/)

## Success Criteria

- [ ] Single tile contains 4 focused skills
- [ ] All 44 original reference docs redistributed (verified by count)
- [ ] 2-3 new reference docs created for solid-principles
- [ ] 3-5 docs redistributed from clean-architecture to design-patterns (balanced distribution)
- [ ] Each skill scores >= B-grade (112/140 points, 80%)
- [ ] Each skill passes markdownlint and Biome validation
- [ ] Each skill has 3-4 eval scenarios (13 total)
- [ ] Original hub deleted cleanly
- [ ] All codebase references updated (README.md, skill-taxonomy.md)
- [ ] Tile published to Tessl registry with all skills >= 90%

## Risks & Mitigations

**Risk:** Breaking existing workflows that depend on `software-design-principles`  
**Mitigation:** Accept as intentional breaking change; new focused skills provide better UX. Document migration path in README.md and AGENTS.md.

**Risk:** Reference doc loss during redistribution  
**Mitigation:** Use `git mv` to preserve history, verify count before/after (44 docs)

**Risk:** Quality audit failures on new skills  
**Mitigation:** Run audits iteratively during authoring, target weakest dimensions (D3, D5, D2)

**Risk:** clean-architecture dominance (30-32/44 docs)  
**Mitigation:** Redistribute 3 boundary pattern docs (`bound-humble-object.md`, `bound-partial-boundaries.md`, `bound-defer-decisions.md`) to design-patterns; balances to 27-29 clean-arch, 8-9 design-patterns

**Risk:** Undetected references in other skills/docs  
**Mitigation:** Phase 0 includes codebase-wide grep for `software-design-principles` before deletion

**Risk:** Underestimated effort (Phase 3 authoring + Phase 5 eval scenarios)  
**Mitigation:** Budget 12-15h total; each SKILL.md ~2-3h + 13 eval scenarios ~2-3h

## References

- Agent Skills Specification: https://agentskills.io
- Repository AGENTS.md: Best practices for skill authoring
- Skill Quality Auditor: `skills/agentic-harness/skill-quality-auditor/SKILL.md`
- Tessl Registry: https://tessl.io
