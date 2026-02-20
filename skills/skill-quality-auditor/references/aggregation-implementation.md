---
category: aggregation
priority: HIGH
source: session 2026-02-10 experience + supabase-postgres-best-practices pattern
---

# Skill Aggregation Implementation Guide

Step-by-step guide to consolidating related skills using the Navigation Hub + References pattern. Proven to achieve 96%+ size reduction with <5% duplication.

## When to Aggregate

Aggregate skills when you have:
- **Skill families** - Related skills with shared domain (bdd-*, typescript-*, bun-*)
- **>20% duplication** - Redundant content across multiple skills
- **User confusion** - "Which skill should I use?"
- **Maintenance burden** - Same concept updated in 3+ places
- **Oversized skills** - Individual skills >500 lines

**Do NOT aggregate when:**
- <20% similarity (creates confusion)
- Different domains (don't mix BDD with Docker)
- Skills already well-organized
- <3 skills (not worth overhead)

## The 6-Step Process

### Step 1: Identify Aggregation Candidates

**Analyze your skill collection:**

```bash
# Find skill families (naming pattern)
ls -1 .agents/skills/ | grep "^bdd-"
# Output: bdd-collaboration, bdd-gherkin, bdd-patterns, bdd-principles, bdd-scenarios

# Check for duplication
./scripts/detect-duplication.sh
# Look for >20% similarity pairs

# Measure sizes
wc -l .agents/skills/bdd-*/SKILL.md
# Identify oversized skills (>500 lines)
```

**Decision Criteria:**

| Criteria | Threshold | Weight |
|----------|-----------|--------|
| Family relationship | Same prefix | HIGH |
| Duplication | >20% | CRITICAL |
| User confusion | >2 similar skills | HIGH |
| Total lines | >2000 combined | MEDIUM |
| Maintenance pain | >3 update locations | HIGH |

**Example Analysis:**
```
BDD Skills Family:
- bdd-collaboration (237 lines)
- bdd-gherkin (646 lines) ⚠️ oversized
- bdd-patterns (285 lines)
- bdd-principles (252 lines)
- bdd-scenarios (129 lines)
- cucumber-best-practices (483 lines)

Total: 2,032 lines
Duplication: 35% (Gherkin syntax in 3 skills)
User confusion: High (which BDD skill to use?)
Maintenance: Updating Given-When-Then in 3 places

DECISION: AGGREGATE ✅
```

### Step 2: Design Category Structure

**Organize content by priority and topic:**

```markdown
## Categories by Priority

| Priority | Category | Topics | Prefix |
|----------|----------|--------|--------|
| CRITICAL | Principles | Philosophy, Three Amigos, Living Docs | principles- |
| HIGH | Gherkin | Syntax, step definitions, Cucumber.js | gherkin- |
| HIGH | Patterns | Given-When-Then, scenarios, tags | patterns- |
| MEDIUM | Collaboration | Example Mapping, discovery workshops | collaboration- |
| MEDIUM | Scenarios | Acceptance criteria, edge cases | scenarios- |
| LOW | Practices | Best practices, anti-patterns | practices- |
```

**Naming Convention:**
- Use `prefix-specific-topic.md` format
- Example: `principles-three-amigos.md`, `gherkin-syntax.md`
- Lowercase with hyphens

**Priority Guidelines:**
- **CRITICAL**: Foundation concepts, load first
- **HIGH**: Core functionality, frequently used
- **MEDIUM**: Specialized features
- **LOW**: Advanced/optional, rarely needed

### Step 3: Create Navigation Hub (SKILL.md)

**Target: 60-100 lines**

**Template:**

```markdown
---
name: [aggregation-name]
description: [Comprehensive description with ALL keywords and trigger scenarios]
consolidates: [list of original skills]
---

# [Aggregation Title]

[2-3 sentence overview]

## When to Apply

Use this skill when:
- [Specific trigger scenario 1]
- [Specific trigger scenario 2]
- [Specific trigger scenario 3]

## Categories by Priority

[Priority table from Step 2]

## How to Use This Skill

This skill follows **progressive disclosure**:

### Quick Start
1. [Basic workflow]
2. [Common use case]

### [Category 1]
1. Load `[filename].md`
2. [Specific usage]

### [Category 2]
...

## Anti-Patterns

❌ NEVER [common mistake]
❌ NEVER [another mistake]

## Related Skills

- [skill-name] - [relationship]

## References

All detailed content in `references/` directory.
See `AGENTS.md` for complete file listing.
```

**Real Example (bdd-testing):**
```markdown
---
name: bdd-testing
description: Behavior-Driven Development with Given-When-Then scenarios, Cucumber.js, Three Amigos, Example Mapping, living documentation, acceptance criteria. Use when writing BDD tests, feature files, or planning discovery workshops.
consolidates: bdd-collaboration, bdd-gherkin, bdd-patterns, bdd-principles, bdd-scenarios, cucumber-best-practices
---

# BDD Testing & Practices

Complete Behavior-Driven Development workflow covering collaboration, Gherkin scenarios, and Cucumber.js implementation.

## When to Apply

- Writing acceptance criteria with Given-When-Then
- Implementing Cucumber.js tests
- Running Three Amigos discovery sessions
- Creating living documentation
- Planning Example Mapping workshops

## Categories by Priority

| Priority | Category | Impact | Prefix | Files |
|----------|----------|--------|--------|-------|
| CRITICAL | Principles | Foundation | principles- | 3 |
| HIGH | Gherkin | Syntax | gherkin- | 2 |
...
```

### Step 4: Migrate Content to References

**For each category:**

1. **Create reference file**:
```bash
touch .agents/skills/[aggregation]/references/[prefix]-[topic].md
```

2. **Add frontmatter**:
```markdown
---
category: [category-name]
priority: CRITICAL|HIGH|MEDIUM|LOW
source: [original-skill-name]
---

# [Topic Title]

[2-3 sentence overview]
```

3. **Extract expert knowledge**:
   - Read original skill
   - Copy ONLY expert-level content
   - Remove tutorials, installation, basic syntax
   - Add code examples
   - Include best practices
   - Include anti-patterns with WHY

4. **Structure content**:
```markdown
## Overview
[What this topic covers]

## Key Concepts
### [Concept 1]
[Explanation + code example]

### [Concept 2]
...

## Best Practices
- ✅ [Practice 1]
- ✅ [Practice 2]

## Common Pitfalls
- ❌ [Pitfall 1] - WHY: [reason]
- ❌ [Pitfall 2] - WHY: [reason]

## Related References
- @see [other-reference-file].md
```

**Example (principles-three-amigos.md):**
```markdown
---
category: principles
priority: CRITICAL
source: bdd-collaboration, bdd-principles
---

# Three Amigos Practice

Collaborative discovery session bringing together Business, Development, and Testing perspectives to explore requirements through concrete examples.

## Overview

Three Amigos prevents costly rework by aligning understanding BEFORE implementation. Each perspective asks different questions:
- Business: What problem are we solving?
- Development: How will we build it?
- Testing: What could go wrong?

## The Session Structure

### 1. Present the Story (5 minutes)
Product Owner presents user story and business context.

### 2. Explore with Questions (20 minutes)
- Developer asks: "How does this work if...?"
- Tester asks: "What happens when...?"
- Business clarifies: "We need to handle..."

### 3. Document Scenarios (15 minutes)
Write Given-When-Then scenarios together.

### 4. Identify Questions (10 minutes)
Capture unknowns as action items.

## Best Practices

✅ **Schedule before sprint planning** - Reduces estimation uncertainty
✅ **Time-box to 50 minutes** - Prevents analysis paralysis
✅ **Focus on examples, not implementation** - Business-level discussion
✅ **Write scenarios during session** - Shared understanding is built live

## Common Pitfalls

❌ **Skipping tester perspective** - WHY: Misses edge cases and risks
❌ **Getting into implementation details** - WHY: Loses business focus
❌ **Writing scenarios alone after** - WHY: Loses shared understanding

## Real Example

**User Story:** Password reset functionality

**Three Amigos Session:**
- Business: "User gets email with reset link, valid 24 hours"
- Developer: "What if they request multiple resets?"
- Tester: "What if link expires while they're typing new password?"

**Scenarios Created:**
```gherkin
Scenario: Successful password reset
  Given I requested a password reset 1 hour ago
  When I click the reset link
  And I enter a new valid password
  Then my password is updated
  
Scenario: Expired reset link
  Given I requested a password reset 25 hours ago
  When I click the reset link
  Then I see "Link expired, request new reset"
```

## Related References

- @see principles-example-mapping.md - Structured discovery technique
- @see principles-ubiquitous-language.md - Shared vocabulary
- @see gherkin-scenarios.md - Writing Given-When-Then
```

### Step 5: Create AGENTS.md Navigation Guide

**Template:**

```markdown
# [Aggregation Name] - Navigation Guide

## Overview

**Total Files**: [X] reference files + [Y] scripts
**Categories**: [N] ([list categories])
**Pattern**: Navigation Hub + Expert References
**Origin**: Consolidates [original skills]

## Usage Instructions

1. Start with SKILL.md
2. Identify your task
3. Load specific references
4. Apply methodology

## Reference Categories

### [Category 1] (PRIORITY)

[Description]

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `file1.md` | [purpose] | ~X | [when] |
| `file2.md` | [purpose] | ~X | [when] |

**Load first**: [recommended starting file]

### [Category 2] (PRIORITY)
...

## Complete File Listing

```
.agents/skills/[aggregation]/
├── SKILL.md
├── AGENTS.md
├── references/
│   ├── category-file1.md
│   └── category-file2.md
└── scripts/
    └── script.sh
```

## Navigation Workflow

### For [Use Case 1]
1. Load `[file].md`
2. [Steps]

### For [Use Case 2]
1. Load `[file].md`
2. [Steps]

## Success Criteria

After using this skill:
- ✅ [Outcome 1]
- ✅ [Outcome 2]
```

### Step 6: Deprecate Original Skills

**Move to .deprecated/ directory:**

```bash
# Create deprecated directory
mkdir -p .agents/skills/.deprecated

# Move original skills
for skill in bdd-collaboration bdd-gherkin bdd-patterns bdd-principles bdd-scenarios cucumber-best-practices; do
  mv .agents/skills/$skill .agents/skills/.deprecated/
done

# Create deprecation README
cat > .agents/skills/.deprecated/README.md << 'EOF'
# Deprecated Skills

These skills have been consolidated into aggregation skills using the Navigation Hub pattern.

## BDD Skills → bdd-testing
- bdd-collaboration
- bdd-gherkin
- bdd-patterns
- bdd-principles
- bdd-scenarios
- cucumber-best-practices

See `.agents/skills/bdd-testing/` for consolidated content.

## Why Deprecated

- 96% size reduction (2,032 lines → 64-line hub)
- Zero duplication (was 35%)
- Clear navigation (was confusing which to use)
- Single source of truth (was updating 3 places)

Kept for historical reference only.
EOF
```

**Verify deprecation:**
```bash
# Count active skills
ls -1 .agents/skills/ | grep -v "^\." | wc -l

# Count deprecated
ls -1 .agents/skills/.deprecated/ | wc -l

# Verify aggregation exists
test -f .agents/skills/bdd-testing/SKILL.md && echo "✅ Aggregation created"
```

## Verification Checklist

After completing all 6 steps:

- [ ] Navigation hub (SKILL.md) is 60-100 lines
- [ ] AGENTS.md lists all references with descriptions
- [ ] References directory exists with categorized files
- [ ] Each reference has proper frontmatter
- [ ] Original skills moved to .deprecated/
- [ ] Deprecation README explains consolidation
- [ ] No broken references (all @see links work)
- [ ] Description field includes ALL trigger keywords
- [ ] Scripts still work (if any were moved)
- [ ] Test loading aggregation in practice

## Success Metrics

Target outcomes:
- **90%+ size reduction** from original total lines
- **<5% duplication** across collection
- **A-grade score** (≥108/120) on skill-judge evaluation
- **Zero confusion** about which skill to use
- **Single source of truth** for each concept

## Common Mistakes

❌ **Creating aggregation without duplication** (arbitrary grouping)
❌ **Too many source skills** (>10 creates confusion)
❌ **Missing categories** (everything in one bucket)
❌ **Generic description** (doesn't include activation keywords)
❌ **Not testing references** (@see links broken)
❌ **Deleting originals** (move to .deprecated/ instead)

## Real Results

From session 2026-02-10:

| Aggregation | Sources | Before | After | Reduction |
|-------------|---------|--------|-------|-----------|
| bdd-testing | 6 | 2,032 | 64 | 96.8% |
| bun-development | 6 | 1,658 | 61 | 96.3% |
| markdown-authoring | 4 | 2,199 | 59 | 97.3% |
| typescript-advanced | 5 | 3,372 | 87 | 97.4% |
| mise-complete | 3 | 1,404 | 59 | 95.8% |
| biome-complete | 2 | 1,065 | 59 | 94.5% |

**Average: 96.4% size reduction, A+ grades (93.2/120)**

## See Also

- `aggregation-pattern.md` - The Navigation Hub pattern explained
- `framework-skill-judge-dimensions.md` - Quality evaluation
- `duplication-detection-algorithm.md` - Finding candidates
