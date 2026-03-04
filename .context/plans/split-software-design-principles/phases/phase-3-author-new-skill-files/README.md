# Phase 3: Author New SKILL.md Files

**Status:** Pending  
**Effort:** ~5-6 hours (2-3h authoring + 2-3h iterative quality audits)  
**Dependencies:** Phase 2

## Description

Author 4 focused SKILL.md files, extracting and reorganizing content from the original hub skill. Each skill should be ~120-180 lines with clear scope and actionable guidance.

## Skills to Author

### 1. solid-principles/SKILL.md (~150 lines)

**Content structure:**
- Frontmatter (name, description with tactical triggers)
- When to Use (class design, refactoring, code reviews)
- When Not to Use (architecture decisions, framework selection)
- The Five Principles (SRP, OCP, LSP, ISP, DIP with embedded examples)
- Compliance Checklist (table from original lines 60-66)
- Anti-Patterns (god classes, tight coupling, concrete dependencies)
- Quick Commands (search for design smells)

**Key extracts from original:**
- Lines 56-75: SOLID Checks table
- Lines 123-159: Anti-Patterns section (filter for class-level issues)

### 2. clean-architecture/SKILL.md (~180 lines)

**Content structure:**
- Frontmatter (name, description with strategic triggers)
- When to Use (service boundaries, module structure, dependency direction)
- When Not to Use (single-module apps, prototypes)
- Architecture Layers (entities → use cases → adapters → frameworks)
- Dependency Rules (lines 42-45 from original)
- Boundary Management (cross-boundary data, interface ownership)
- Quick Reference (organized sections from lines 179-241)
- Anti-Patterns (circular dependencies, infrastructure in domain)
- Quick Commands

**Key extracts from original:**
- Lines 36-54: Strategic checks + ADR template
- Lines 179-241: Quick Reference (all sections)
- Lines 162-175: Quick Commands

### 3. design-patterns/SKILL.md (~120 lines)

**Content structure:**
- Frontmatter (name, description with pattern selection triggers)
- When to Use (repeated structures, extensibility needs)
- When Not to Use (premature abstraction, YAGNI violations)
- Pattern Selection Workflow (extract from lines 76-92)
- Common Patterns (Strategy, Factory, Adapter, Repository)
- Anti-Patterns (overengineering, pattern obsession)
- Quick Reference (3 docs)

**Key extracts from original:**
- Lines 76-92: Pattern selection workflow (Step 4)
- Lines 127-128: YAGNI anti-pattern

**Additional content:**
- Create `references/pattern-selection-workflow.md` with expanded workflow

### 4. testable-design/SKILL.md (~140 lines)

**Content structure:**
- Frontmatter (name, description with testing architecture triggers)
- When to Use (designing testable systems, refactoring untestable code)
- When Not to Use (performance-critical paths)
- Testing Architecture Principles (isolation, boundary verification, layer isolation)
- Design for Testability Checklist
- Anti-Patterns (test-induced damage, mocking everything)
- Quick Reference (4 docs)
- Quick Commands (test execution, coverage)

**Key extracts from original:**
- Lines 244-249: Test references (now distributed to testable-design)
- Lines 162-175: Adapt quick commands for testing context

## Quality Audit Integration

After authoring each SKILL.md, run iterative quality audit:

```bash
# Audit each skill individually
bun cli/index.ts audit skill software-engineering/design-principles/solid-principles
bun cli/index.ts audit skill software-engineering/design-principles/clean-architecture
bun cli/index.ts audit skill software-engineering/design-principles/design-patterns
bun cli/index.ts audit skill software-engineering/design-principles/testable-design
```

**Target:** B-grade minimum (112/140 points, 80%)  
**Focus dimensions:** D3 (Anti-Patterns), D5 (Progressive Disclosure), D2 (Mindset/Procedures)

**Iterative improvement:**
1. Author initial SKILL.md
2. Run quality audit
3. Review remediation plan
4. Apply improvements for weak dimensions
5. Re-audit until B-grade achieved

## Acceptance Criteria

- [ ] All 4 SKILL.md files created
- [ ] Each file has valid frontmatter (name, description)
- [ ] Each file has When to Use / When Not to Use sections
- [ ] Anti-Patterns sections present in all skills (addresses D3 weakness)
- [ ] Progressive disclosure structure implemented (addresses D5 weakness)
- [ ] Quick Reference sections link to correct reference docs
- [ ] All files pass markdownlint
- [ ] Each skill achieves >= B-grade (112/140) in quality audit

## Verification

```bash
# Check all SKILL.md files exist
ls skills/software-engineering/design-principles/*/SKILL.md

# Verify frontmatter
for skill in solid-principles clean-architecture design-patterns testable-design; do
  echo "Checking $skill..."
  head -5 skills/software-engineering/design-principles/$skill/SKILL.md | grep -q "^name:" && echo "✓ Has name"
  head -5 skills/software-engineering/design-principles/$skill/SKILL.md | grep -q "^description:" && echo "✓ Has description"
done

# Run linting
bunx markdownlint-cli2 "skills/software-engineering/design-principles/*/SKILL.md"
```

## Notes

- Embed SOLID principle examples directly in SKILL.md (no separate reference docs)
- Preserve original ADR template format from lines 47-54
- Keep code block examples using the same BAD/GOOD format
- Create `pattern-selection-workflow.md` reference doc as part of this phase
