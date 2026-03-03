# Cognitive Complexity Reduction Plan

**Date:** 2026-03-03  
**Biome Rule:** `noExcessiveCognitiveComplexity` (max: 15)  
**Status:** 5 functions need refactoring

## Complexity Violations

| File | Function | Score | Threshold | Priority |
|------|----------|-------|-----------|----------|
| `cli/lib/readme/skill-parser.ts` | `parseSkillDescription` | 37 | 15 | 🔴 Critical |
| `cli/lib/audit/audit-summary.ts` | `auditSummary` | 28 | 15 | 🔴 High |
| `cli/lib/install/install-skills.ts` | `installSkills` | 25 | 15 | 🟡 High |
| `cli/lib/readme/update-readme.ts` | `updateReadme` | 20 | 15 | 🟡 Medium |
| `cli/lib/tessl/publish-check.ts` | `tesslPublishCheck` | 19 | 15 | 🟡 Medium |

**Total Complexity Debt:** 129 (should be ≤75)

---

## Refactoring Strategy

### Principles
1. **Extract Method** - Break complex functions into smaller helpers
2. **Early Returns** - Reduce nesting with guard clauses
3. **Strategy Pattern** - Replace conditionals with polymorphism
4. **Pure Functions** - Isolate side effects for easier testing

### Target Metrics
- All functions ≤15 complexity
- 80%+ test coverage for refactored code
- No functionality regression

---

## Priority 1: parseSkillDescription (37 → ≤15)

**File:** `cli/lib/readme/skill-parser.ts:4`  
**Current:** 37 complexity (147% over limit)

**Refactoring Plan:**
1. Extract YAML frontmatter parsing to `extractFrontmatter()`
2. Extract description section parsing to `extractDescriptionSection()`
3. Extract trigger detection to `extractTriggers()`
4. Create `DescriptionParser` class with small methods

**Expected Outcome:**
- `parseSkillDescription`: 8 complexity (orchestration)
- `extractFrontmatter`: 5 complexity
- `extractDescriptionSection`: 6 complexity
- `extractTriggers`: 4 complexity

**Tests to Add:**
```typescript
describe("parseSkillDescription", () => {
  test("should parse frontmatter description");
  test("should extract description section");
  test("should detect trigger phrases");
  test("should handle missing SKILL.md");
  test("should handle malformed frontmatter");
});
```

---

## Priority 2: auditSummary (28 → ≤15)

**File:** `cli/lib/audit/audit-summary.ts:12`  
**Current:** 28 complexity (87% over limit)

**Refactoring Plan:**
1. Extract skill collection to `collectAuditedSkills()`
2. Extract grade distribution calculation to `calculateGradeDistribution()`
3. Extract dimension averages to `calculateDimensionAverages()`
4. Extract report formatting to `formatSummaryReport()`

**Expected Outcome:**
- `auditSummary`: 6 complexity (orchestration)
- `collectAuditedSkills`: 8 complexity
- `calculateGradeDistribution`: 5 complexity
- `calculateDimensionAverages`: 7 complexity
- `formatSummaryReport`: 4 complexity

**Tests to Add:**
```typescript
describe("auditSummary", () => {
  test("should generate summary from multiple audits");
  test("should calculate grade distribution");
  test("should compute dimension averages");
  test("should handle empty audit directory");
});
```

---

## Priority 3: installSkills (25 → ≤15)

**File:** `cli/lib/install/install-skills.ts:106`  
**Current:** 25 complexity (67% over limit)

**Refactoring Plan:**
1. Extract skill discovery to `findSkillDirectories()`
2. Extract symlink creation to `createSkillSymlink()`
3. Extract agent path resolution to `resolveAgentPath()`
4. Extract dry-run reporting to `reportInstallPlan()`

**Expected Outcome:**
- `installSkills`: 7 complexity (orchestration)
- `findSkillDirectories`: 6 complexity
- `createSkillSymlink`: 8 complexity
- `resolveAgentPath`: 3 complexity
- `reportInstallPlan`: 4 complexity

**Tests to Add:**
```typescript
describe("installSkills", () => {
  test("should install skills to agent directories");
  test("should skip existing symlinks");
  test("should handle dry-run mode");
  test("should support global installation");
  test("should namespace skill names correctly");
});
```

---

## Priority 4: updateReadme (20 → ≤15)

**File:** `cli/lib/readme/update-readme.ts:74`  
**Current:** 20 complexity (33% over limit)

**Refactoring Plan:**
1. Extract skill table generation to `generateSkillTable()`
2. Extract domain grouping to `groupSkillsByDomain()`
3. Extract markdown replacement to `replaceMarkdownSection()`

**Expected Outcome:**
- `updateReadme`: 8 complexity (orchestration)
- `generateSkillTable`: 9 complexity
- `groupSkillsByDomain`: 5 complexity
- `replaceMarkdownSection`: 4 complexity

**Tests to Add:**
```typescript
describe("updateReadme", () => {
  test("should update skill tables in README");
  test("should group skills by domain");
  test("should preserve README structure");
  test("should handle dry-run mode");
});
```

---

## Priority 5: tesslPublishCheck (19 → ≤15)

**File:** `cli/lib/tessl/publish-check.ts:6`  
**Current:** 19 complexity (27% over limit)

**Refactoring Plan:**
1. Extract tile validation to `validateTile()`
2. Extract eval scenario check to `checkEvalScenarios()`
3. Extract quality gate check to `checkQualityGates()`

**Expected Outcome:**
- `tesslPublishCheck`: 7 complexity (orchestration)
- `validateTile`: 8 complexity
- `checkEvalScenarios`: 6 complexity
- `checkQualityGates`: 5 complexity

**Tests to Add:**
```typescript
describe("tesslPublishCheck", () => {
  test("should validate tiles before publish");
  test("should check eval scenario coverage");
  test("should enforce quality thresholds");
  test("should report validation failures");
});
```

---

## Implementation Timeline

### Phase 1: Critical (Week 1)
- [ ] Refactor `parseSkillDescription` (37 → ≤15)
- [ ] Add comprehensive tests (target: 90% coverage)
- [ ] Verify Biome warning cleared

### Phase 2: High Priority (Week 2)
- [ ] Refactor `auditSummary` (28 → ≤15)
- [ ] Refactor `installSkills` (25 → ≤15)
- [ ] Add test coverage (target: 85% coverage)

### Phase 3: Medium Priority (Week 3)
- [ ] Refactor `updateReadme` (20 → ≤15)
- [ ] Refactor `tesslPublishCheck` (19 → ≤15)
- [ ] Add test coverage (target: 80% coverage)

### Phase 4: Validation (Week 4)
- [ ] Run full test suite (target: 100% pass)
- [ ] Verify Biome reports 0 complexity warnings
- [ ] Document refactoring patterns for future work

---

## Success Metrics

### Code Quality
- ✅ All functions ≤15 cognitive complexity
- ✅ Test coverage ≥80% for refactored modules
- ✅ Zero Biome warnings
- ✅ TypeScript compiles cleanly

### Maintainability
- ✅ Average function length ≤50 lines
- ✅ Clear single responsibility per function
- ✅ Comprehensive test coverage
- ✅ Self-documenting code with minimal comments

### Performance
- ✅ No performance regression
- ✅ Test suite runs in <500ms
- ✅ CLI commands execute within SLA

---

## Testing Strategy

### Unit Tests
- Test each extracted helper function independently
- Mock file system operations where appropriate
- Use Bun's built-in test runner

### Integration Tests
- Test orchestrator functions end-to-end
- Use temporary directories for file operations
- Verify output correctness

### Regression Tests
- Compare output before/after refactoring
- Ensure existing CLI commands work unchanged
- Validate edge cases and error handling

---

## Monitoring

### Pre-Commit Checks
- Biome complexity linting (enforced)
- Test suite execution (enforced)
- TypeScript compilation (enforced)

### Continuous Improvement
- Track complexity trends over time
- Review new code for complexity
- Refactor when functions exceed threshold

---

## References

- [Biome Cognitive Complexity Rule](https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity/)
- [Refactoring: Improving the Design of Existing Code](https://martinfowler.com/books/refactoring.html)
- [Clean Code by Robert C. Martin](https://www.oreilly.com/library/view/clean-code-a/9780136083238/)
- CLI Architecture Review: `.context/cli-architecture-review.md`
- Refactoring Summary: `.context/cli-refactoring-summary.md`
