# Phase 8: Validation

**Status:** Pending  
**Effort:** ~45 minutes  
**Dependencies:** Phase 7

## Description

Run comprehensive validation on all 4 skills to ensure quality thresholds, formatting compliance, and reference doc integrity.

## Validation Checks

### 1. Quality Audits (Target: B-grade minimum, ≥112/140 points)

```bash
# Audit each skill
bun cli/index.ts audit skill software-engineering/design-principles/solid-principles
bun cli/index.ts audit skill software-engineering/design-principles/clean-architecture
bun cli/index.ts audit skill software-engineering/design-principles/design-patterns
bun cli/index.ts audit skill software-engineering/design-principles/testable-design

# Generate summary report
bun cli/index.ts audit summary
```

**Expected scores:**
- Minimum: 112/140 (B-grade, 80%)
- Target: 126/140 (A-grade, 90%)

**Focus on weak dimensions:**
- D3 Anti-Pattern Quality (68% avg) - Add Common Mistakes sections
- D5 Progressive Disclosure (73% avg) - Improve information architecture
- D2 Mindset & Procedures (74% avg) - Establish mental models

### 2. Linting

```bash
# Markdown linting
bunx markdownlint-cli2 "skills/software-engineering/design-principles/**/*.md"

# JSON linting
bunx @biomejs/biome check skills/software-engineering/design-principles/tile.json
```

### 3. Reference Doc Count Verification

```bash
# Count all reference docs (should be 44 original + 2-3 new solid + 0-2 new design-patterns = 46-49)
find skills/software-engineering/design-principles -type f -name "*.md" -path "*/references/*" | wc -l

# Expected: 46-49 total with balanced distribution

# Breakdown by skill (after Phase 2 redistribution)
echo "solid-principles: $(find skills/software-engineering/design-principles/solid-principles/references -type f -name "*.md" 2>/dev/null | wc -l) (expected: 2-3)"
echo "clean-architecture: $(find skills/software-engineering/design-principles/clean-architecture/references -type f -name "*.md" | wc -l) (expected: 30-32)"
echo "design-patterns: $(find skills/software-engineering/design-principles/design-patterns/references -type f -name "*.md" | wc -l) (expected: 6-8)"
echo "testable-design: $(find skills/software-engineering/design-principles/testable-design/references -type f -name "*.md" | wc -l) (expected: 4)"
```

### 4. Eval Scenario Verification

```bash
# Count eval scenarios (should be 13 total: 3+4+3+3)
for skill in solid-principles clean-architecture design-patterns testable-design; do
  count=$(find skills/software-engineering/design-principles/$skill/evals -type f -name "*.md" | wc -l)
  echo "$skill: $count scenarios"
done

total=$(find skills/software-engineering/design-principles/*/evals -type f -name "*.md" | wc -l)
echo "Total: $total (expected: 13)"
```

### 5. SKILL.md Completeness Check

```bash
# Verify all SKILL.md files have required sections
for skill in solid-principles clean-architecture design-patterns testable-design; do
  file="skills/software-engineering/design-principles/$skill/SKILL.md"
  echo "Checking $skill..."
  
  grep -q "^name:" "$file" && echo "  ✓ Frontmatter: name"
  grep -q "^description:" "$file" && echo "  ✓ Frontmatter: description"
  grep -q "## When to Use" "$file" && echo "  ✓ Section: When to Use"
  grep -q "## When Not to Use" "$file" && echo "  ✓ Section: When Not to Use"
  grep -q "## Anti-Patterns" "$file" && echo "  ✓ Section: Anti-Patterns"
done
```

### 6. tile.json Validation

```bash
# Verify tile.json structure
tile_json="skills/software-engineering/design-principles/tile.json"

jq -e '.name' "$tile_json" && echo "✓ Has name field"
jq -e '.version' "$tile_json" && echo "✓ Has version field"
jq -e '.private' "$tile_json" && echo "✓ Has private field"
jq -e '.summary' "$tile_json" && echo "✓ Has summary field"
jq -e '.skills | length == 4' "$tile_json" && echo "✓ Has 4 skills"

# Verify skill paths exist
for skill_path in $(jq -r '.skills[].path' "$tile_json"); do
  full_path="skills/software-engineering/design-principles/$skill_path"
  test -f "$full_path" && echo "✓ $skill_path exists" || echo "✗ $skill_path NOT FOUND"
done
```

## Acceptance Criteria

- [ ] All 4 skills score ≥ B-grade (112/140 points, 80%)
- [ ] All markdown files pass markdownlint
- [ ] tile.json passes Biome validation
- [ ] Reference doc count is 46-49 (verified with balanced distribution)
- [ ] Eval scenario count is 13 (verified: 3+4+3+3)
- [ ] All SKILL.md files have required sections
- [ ] All skill paths in tile.json resolve correctly
- [ ] No broken internal references between skills

## Remediation Steps (If Validation Fails)

**If quality audit fails (<112 points):**
1. Review `.context/audits/<skill-name>/latest/remediation-plan.md`
2. Focus on D3 (Anti-Patterns), D5 (Progressive Disclosure), D2 (Mindset)
3. Add missing sections, improve examples
4. Re-run audit until B-grade achieved

**If linting fails:**
1. Review markdownlint errors
2. Fix formatting issues (heading levels, line length, code block syntax)
3. Re-run linting

**If reference count is wrong:**
1. Verify all 44 original files moved from original skill
2. Verify 2-3 new solid-principles reference docs created
3. Verify 3-5 docs redistributed from clean-architecture to design-patterns
4. Check git log for any missed files
5. Verify pattern-selection-workflow.md was created (design-patterns)

## Verification Report Template

After all checks pass, document results:

```markdown
# Validation Report: design-principles Tile

**Date:** YYYY-MM-DD
**Status:** ✓ PASSED / ✗ FAILED

## Quality Audit Scores

| Skill | Score | Grade | Status |
|-------|-------|-------|--------|
| solid-principles | X/140 | Y | ✓/✗ |
| clean-architecture | X/140 | Y | ✓/✗ |
| design-patterns | X/140 | Y | ✓/✗ |
| testable-design | X/140 | Y | ✓/✗ |

## Linting

- [ ] Markdownlint: PASSED/FAILED
- [ ] Biome: PASSED/FAILED

## File Counts

- [ ] Reference docs: 44 (verified)
- [ ] Eval scenarios: 13 (verified)
- [ ] SKILL.md files: 4 (verified)

## Issues Found

None / List issues here

## Next Steps

Proceed to Phase 9: Tessl Integration / Address issues above
```
