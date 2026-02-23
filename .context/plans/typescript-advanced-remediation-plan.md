---
plan_date: 2026-02-23
skill_name: typescript-advanced
source_audit: .context/audits/typescript-advanced-audit-2026-02-22.md
---

# Remediation Plan: typescript-advanced

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 87/120 (72%) | 100/120 (83%) |
| **Grade** | C | B+ |
| **Priority** | High | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Anti-pattern quality (D3), Practical usability (D8), Pattern recognition (D7)

**Verdict**: Priority improvements required. Multiple dimensions need enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak anti-patterns | D3 (8/15) | High | Common mistakes may be repeated |
| Missing executable commands | D8 (8/15) | High | Reduced practical usability |
| Weak trigger keywords | D7 (6/10) | High | Skill may not activate |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add explicit anti-patterns

**File**: `skills/typescript-advanced/SKILL.md`

````markdown
## Anti-Patterns

### NEVER Use `any` Type

**WHY**: Disables type checking completely.

**BAD**:
```typescript
function process(data: any) { return data.value; }
```

**GOOD**:
```typescript
function process<T extends { value: unknown }>(data: T) { return data.value; }
```

### NEVER Use Type Assertions to Silence Errors

**WHY**: Assertions bypass compiler without runtime checks.

**BAD**:
```typescript
const value = data as string;
```

**GOOD**:
```typescript
const value = typeof data === 'string' ? data : throw new TypeError('Expected string');
```

### NEVER Ignore Strict Mode Errors

**WHY**: Strict mode catches real bugs before runtime.

**BAD**:
```typescript
// @ts-ignore or disabling strict mode
```

**GOOD**: Fix the underlying type issue.
````

---

### Phase 2: Practical Usability (D8) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add Quick Commands section

**File**: `skills/typescript-advanced/SKILL.md`

````markdown
## Quick Commands

### Type Checking
```bash
npx tsc --noEmit
npx tsc --noEmit src/types/index.ts
```

### Documentation Generation
```bash
npx typedoc --out docs src/index.ts
```
````

#### Step 2.2: Add common patterns with copy/paste examples

````markdown
## Common Patterns

### Branded Types
```typescript
type Brand<T, B> = T & { __brand: B };
type UserId = Brand<string, 'UserId'>;
```
````

---

### Phase 3: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 3.1: Expand frontmatter description

**File**: `skills/typescript-advanced/SKILL.md`

```yaml
---
name: typescript-advanced
description: |
  Comprehensive TypeScript guidance. Use when: configuring tsconfig, working with complex types,
  implementing type-safe patterns, making illegal states unrepresentable.
  
  Keywords: TypeScript, tsconfig, strict mode, generics, conditional types, utility types,
  type guards, discriminated unions, branded types
---
```

---

### Phase 4: Specification Compliance (D4) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 4.1: Ensure complete frontmatter

Add missing fields: version, author, tags, scope.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh typescript-advanced --json
bunx markdownlint-cli2 "skills/typescript-advanced/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| Overall Score | >= 100/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | M | 1 hour |
| Phase 2: Commands | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| Phase 4: Spec Compliance | S | 20 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/typescript-advanced/SKILL.md
```

## Notes

- Rating: **7/10** - Good plan with detailed anti-patterns and code examples
- Already includes Timeline table
- Anti-patterns have good BAD/GOOD format
- Minor: Could add more verification steps
