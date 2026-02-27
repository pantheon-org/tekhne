---
category: framework
priority: CRITICAL
source: skill-judge quality requirements
---

# A-Grade Skill Quality Standards

Requirements for achieving A-grade (≥108/120) on skill-judge evaluation. Use this as a checklist when creating or improving skills.

## A-Grade Requirements Summary

**Minimum Score**: 108/120 (90%)  
**Target Score**: 114/120 (95%) for A+  
**Perfect Score**: 120/120 (100%)

## Dimension Requirements for A-Grade

### D1: Knowledge Delta (≥18/20)

**Requirements:**
- <5% redundant content
- Expert-only knowledge throughout
- No basic tutorials or installation guides
- Domain-specific patterns AI assistants don't know
- Project-specific conventions

**Checklist:**
- [ ] No basic syntax explanations
- [ ] No copied official documentation
- [ ] No generic best practices
- [ ] Contains non-obvious insights
- [ ] Includes production-tested patterns

### D2: Mindset + Procedures (≥13/15)

**Requirements:**
- Clear philosophical framing
- Step-by-step numbered workflow
- Explicit "when to apply" criteria
- Explicit "when NOT to apply" scenarios

**Checklist:**
- [ ] Philosophy/core principle stated
- [ ] Numbered workflow steps
- [ ] Entry/exit points defined
- [ ] Use case examples provided
- [ ] Non-applicable scenarios listed

### D3: Anti-Pattern Quality (≥13/15)

**Requirements:**
- NEVER statements with WHY explanations
- Concrete code examples of anti-patterns
- Consequences explained
- ❌ BAD / ✅ GOOD comparisons

**Checklist:**
- [ ] At least 3 NEVER statements
- [ ] Each NEVER has WHY explanation
- [ ] Code examples for anti-patterns
- [ ] Consequences articulated
- [ ] Side-by-side comparisons present

### D4: Specification Compliance (≥13/15)

**Requirements:**
- Description field with activation keywords
- Proper YAML frontmatter
- Clear trigger scenarios in description

**Checklist:**
- [ ] Description is comprehensive
- [ ] Domain keywords included
- [ ] Trigger scenarios mentioned
- [ ] Frontmatter syntax correct
- [ ] Name matches directory

### D5: Progressive Disclosure (≥13/15)

**Requirements:**
- SKILL.md <100 lines for aggregations
- References directory with categorized files
- Navigation hub approach
- Content organized by priority

**Checklist:**
- [ ] Main file is navigation hub
- [ ] Detailed content in references/
- [ ] Files organized by prefix
- [ ] Priority labels present
- [ ] Clear category structure

### D6: Freedom Calibration (≥13/15)

**Requirements:**
- Rigidity matches skill type
- Mindset skills: Strong rules
- Process skills: Balanced flexibility
- Tool skills: Options and trade-offs

**Checklist:**
- [ ] Calibration appropriate for type
- [ ] Mindset skills use NEVER/ALWAYS
- [ ] Process skills allow adaptation
- [ ] Tool skills present options
- [ ] Consistent throughout

### D7: Pattern Recognition (≥9/10)

**Requirements:**
- Rich domain keywords in description
- Comprehensive trigger scenarios
- Clear activation signals

**Checklist:**
- [ ] 5+ domain keywords in description
- [ ] Trigger examples provided
- [ ] When-to-use scenarios clear
- [ ] Unambiguous activation criteria

### D8: Practical Usability (≥13/15)

**Requirements:**
- Concrete, runnable code examples
- Real-world scenarios
- Clear, scannable structure
- Complete implementations (not fragments)

**Checklist:**
- [ ] Code examples are runnable
- [ ] Examples use realistic scenarios
- [ ] Headings are scannable
- [ ] Code blocks properly formatted
- [ ] No pseudocode fragments

## Common A-Grade Patterns

### Navigation Hub Pattern (Best for Aggregations)
```
skill-name/
├── SKILL.md (60-90 lines)
├── AGENTS.md
└── references/
    ├── category-topic1.md
    └── category-topic2.md
```

### Single-File Pattern (Best for Focused Skills)
```
skill-name/
├── SKILL.md (100-250 lines)
└── AGENTS.md (optional)
```

## Red Flags (Immediate Disqualification from A-Grade)

- Description <50 characters
- No activation keywords
- >50% tutorial content
- Missing anti-patterns
- No code examples
- >500 lines in single file
- Generic "best practices" without specificity

## Quality Verification Commands

```bash
# Check line count
wc -l skills/*/SKILL.md

# Verify frontmatter
head -10 skills/*/SKILL.md | grep -A5 "^---"

# Check for references directory
ls -la skills/*/references/

# Count keywords in description
grep "^description:" skills/*/SKILL.md | wc -w
```

## See Also

- `framework-skill-judge-dimensions.md` - Dimension definitions
- `framework-scoring-rubric.md` - Scoring methodology
