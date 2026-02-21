---
category: framework
priority: CRITICAL
source: skill-judge canonical + session experience
---

# Skill-Judge Evaluation Framework: 8 Dimensions

Complete evaluation methodology for assessing skill quality using the skill-judge framework. This is the foundation for all quality auditing.

Canonical source reference: `framework-skill-judge-canonical.md`

## Overview

The skill-judge framework evaluates skills across 8 dimensions totaling 120 points. **Dimension 1 (Knowledge Delta)** is most important - skills must contain expert-only knowledge, not concepts Claude already knows.

**Target Score:** ≥108 points (90%) = A-grade

## Dimension 1: Knowledge Delta (20 points) ⭐ MOST IMPORTANT

**Purpose:** Ensure skill contains expert-only knowledge, not redundant information.

**Scoring:**

- **18-20 points:** Pure expert knowledge, <5% redundancy
- **15-17 points:** Mostly expert, 5-15% redundancy  
- **12-14 points:** 15-30% redundancy (acceptable)
- **9-11 points:** 30-50% redundancy (needs improvement)
- **0-8 points:** >50% redundancy (failing)

**Core Principle:** Skill = Expert Knowledge - What Claude Already Knows

### Three Knowledge Types

1. **Expert (KEEP):**
   - Domain-specific patterns Claude doesn't know
   - Project-specific conventions
   - Lessons from production experience
   - Tool gotchas and non-obvious behavior
   - Decision frameworks (when to use X vs Y)
   - Anti-patterns with WHY they fail

2. **Activation (BRIEF REMINDERS OK):**
   - When to use this skill
   - Trigger keywords for pattern matching
   - Brief context setting (2-3 sentences)

3. **Redundant (DELETE):**
   - Basic syntax Claude knows
   - Installation instructions from official docs
   - API documentation copied verbatim
   - Generic best practices
   - Obvious examples

### Red Flags for Low Knowledge Delta

❌ Teaching basic syntax (Claude knows `if/else`, `function`, `class`)  
❌ Copying official documentation (schema definitions, rule lists)  
❌ Explaining fundamentals (what is REST, what is a database)  
❌ Generic advice (write tests, use version control)  
❌ Installation tutorials (npm install, pip install)

### Examples

**❌ Low Knowledge Delta (12/20):**

```markdown
# TypeScript Basics

## Variables
Use `let` for mutable, `const` for immutable:
let count = 0
const name = "Alice"

## Functions
Functions can be declared or arrow:
function add(a: number, b: number) { return a + b }
const add = (a: number, b: number) => a + b
```

*Problem: Claude already knows basic TypeScript syntax*

**✅ High Knowledge Delta (19/20):**

```markdown
# TypeScript: Making Illegal States Unrepresentable

## The Pattern
Use discriminated unions to eliminate impossible states:

❌ BAD: Multiple optional fields create 16 possible states
type Request = {
  loading?: boolean
  error?: string
  data?: User
}

✅ GOOD: Tagged union with 3 valid states only
type Request = 
  | { status: 'loading' }
  | { status: 'error', error: string }
  | { status: 'success', data: User }

## Why This Matters
Bad design allows bugs: `{ loading: true, data: user }` is impossible but TypeScript allows it.
Good design: TypeScript prevents impossible states at compile time.
```

*Expert pattern Claude doesn't know by default*

## Dimension 2: Mindset + Procedures (15 points)

**Purpose:** Provide philosophical framing and step-by-step workflows.

**Scoring:**

- **13-15 points:** Clear mindset + detailed procedures + when/when-not
- **10-12 points:** Has most elements, minor gaps
- **7-9 points:** Missing key element
- **0-6 points:** Generic or absent

### Components

1. **Clear Mindset/Philosophy (5 points)**
   - Core principle or philosophy
   - Why this approach over alternatives
   - Example: "Trust but verify" (proof-of-work), "Composition over inheritance" (structural-design)

2. **Step-by-Step Procedures (5 points)**
   - Numbered workflow
   - Clear entry/exit points
   - Validation steps
   - Example: TDD cycle (Red → Green → Refactor)

3. **When/When-Not Guidance (5 points)**
   - Clear activation criteria
   - Explicit non-applicable scenarios
   - Example: "Use for backend APIs, NOT for UI styling"

### Example

**✅ Strong Mindset + Procedures (15/15):**

```markdown
# Test-Driven Development

## Mindset
Write tests BEFORE implementation. The test defines the contract; implementation fulfills it.

## Workflow
1. Red: Write failing test (verify it fails)
2. Green: Minimum code to pass
3. Refactor: Improve without breaking tests

## When to Apply
✅ New functions, features, bug fixes (reproduce first)
❌ UI styling, configuration, documentation

## When NOT to Apply
- Throwaway prototypes
- Generated code
- Trivial getters/setters
```

## Dimension 3: Anti-Pattern Quality (15 points)

**Purpose:** Teach what NOT to do with clear explanations of WHY.

**Scoring:**

- **13-15 points:** NEVER lists + concrete examples + consequences
- **10-12 points:** Has most elements
- **7-9 points:** Generic warnings
- **0-6 points:** Missing or weak

### Components

1. **NEVER Lists with WHY (5 points)**
   - Explicit "NEVER do X because Y" statements
   - Not just "avoid" - use strong language
   - Example: "NEVER trust agent completion reports without verification"

2. **Concrete Examples (5 points)**
   - Show bad code, not just descriptions
   - Side-by-side ❌ BAD / ✅ GOOD comparisons
   - Real-world scenarios

3. **Consequences Explained (5 points)**
   - What breaks when anti-pattern used
   - Impact: security, performance, maintainability
   - Example: "Leads to SQL injection attacks"

### Example

**✅ Strong Anti-Patterns (14/15):**

```markdown
## Anti-Patterns

❌ **NEVER use string interpolation for SQL**
WHY: Opens SQL injection vulnerabilities

// BAD - Vulnerable to injection
db.query(`SELECT * FROM users WHERE id = ${userId}`)

// GOOD - Safe with prepared statements
db.query('SELECT * FROM users WHERE id = ?', [userId])

**Consequence:** Attacker can inject `1 OR 1=1` to dump entire table.

❌ **NEVER skip test failure verification**
WHY: False positives waste hours debugging phantom issues

**Consequence:** Test passes even with bugs, leading to production failures.
```

## Dimension 4: Specification Compliance (15 points)

**Purpose:** Ensure proper frontmatter, single-task focus, and activation keywords.

**Scoring:**

- **13-15 points:** Perfect spec compliance
- **10-12 points:** Minor issues
- **7-9 points:** Missing key elements
- **0-6 points:** Non-compliant

### Components

1. **Task Focus Declaration (5 points) ⭐ CRITICAL**
    - Skill indicates ONE type of task it helps complete
    - Description clearly scopes to single purpose
    - No ambiguity about what the skill does
    - Example: "Write BDD tests" (good) vs "Testing and development" (bad - two tasks)

2. **Description Field Quality (7 points)**
    - **Primary agents:** Exactly 3 words
    - **Other agents:** Comprehensive with trigger examples
    - Must include activation keywords
    - Determines if skill activates

3. **Proper Frontmatter (2 points)**
    - name, description present
    - Consolidation notes if applicable
    - Correct YAML syntax

4. **Activation Keywords (1 point)**
    - Domain terms that trigger skill
    - Example: "BDD, Gherkin, Given-When-Then, Cucumber"

### Example

**✅ Excellent Description (10/10):**

```yaml
---
name: bdd-testing
description: Behavior-Driven Development with Given-When-Then scenarios, Cucumber.js, Three Amigos collaboration, Example Mapping, living documentation, and acceptance criteria. Use when writing BDD tests, feature files, or planning discovery workshops.
---
```

*Comprehensive, includes triggers, explains when to use*

**❌ Weak Description (4/10):**

```yaml
---
name: bdd-testing  
description: BDD testing patterns
---
```

*Too generic, no activation keywords, no usage guidance*

## Dimension 5: Progressive Disclosure (15 points)

**Purpose:** Structure content for on-demand loading, not frontloading everything.

**Scoring:**

- **13-15 points:** Navigation hub + references/ + categories
- **10-12 points:** Some organization, could improve
- **7-9 points:** Everything frontloaded, >300 lines
- **0-6 points:** No structure, >500 lines

### Components

1. **Navigation Hub Approach (8 points)**
   - SKILL.md is <100 lines
   - Overview + when-to-use + reference guide
   - NOT full content
   - Example: supabase-postgres-best-practices (65 lines)

2. **References Directory (4 points)**
   - Detailed content in references/*.md
   - Each reference 100-500 lines
   - Focused on ONE topic

3. **Category Organization (3 points)**
   - Files organized by prefix (principles-, patterns-, etc.)
   - Priority labels (CRITICAL, HIGH, MEDIUM, LOW)

### Example

**✅ Excellent Progressive Disclosure (15/15):**

```
bdd-testing/
├── SKILL.md (64 lines - navigation hub)
├── AGENTS.md (reference guide)
└── references/
    ├── principles-three-amigos.md (CRITICAL, 250 lines)
    ├── gherkin-syntax.md (HIGH, 180 lines)
    └── practices-tags.md (MEDIUM, 120 lines)
```

**❌ Poor Progressive Disclosure (6/15):**

```
bdd-testing/
└── SKILL.md (1,800 lines - everything frontloaded)
```

## Dimension 6: Freedom Calibration (15 points)

**Purpose:** Balance prescription (rigid rules) vs flexibility (guidelines).

**Scoring:**

- **13-15 points:** Appropriate for skill type
- **10-12 points:** Slightly too rigid or loose
- **7-9 points:** Mismatched calibration
- **0-6 points:** Completely wrong

### Calibration Levels

1. **Rigid (Mindset skills):** Strong rules, must follow
   - Example: proof-of-work "NEVER trust agent reports without verification"
   - Use: Critical foundations, security, correctness

2. **Balanced (Process skills):** Clear steps with flexibility
   - Example: TDD "Red → Green → Refactor (adapt to context)"
   - Use: Workflows, methodologies

3. **Flexible (Tool skills):** Options and trade-offs
   - Example: typescript-type-system "Choose based on use case"
   - Use: Technical tools, patterns

### Example

**✅ Well-Calibrated (14/15):**

```markdown
# Proof of Work (Mindset skill)

## Zero-Tolerance Rules
NEVER trust agent completion reports without verification.
ALWAYS show command output as proof.
ZERO exceptions to verification protocol.
```

*Appropriately rigid for critical verification*

**❌ Miscalibrated (7/15):**

```markdown
# TypeScript Basics (Tool skill)

## Rules
ALWAYS use const for all variables.
NEVER use let or var under any circumstances.
```

*Too rigid - let has valid use cases*

## Dimension 7: Pattern Recognition (10 points)

**Purpose:** Ensure skill activates when needed via description keywords.

**Scoring:**

- **9-10 points:** Rich keywords, comprehensive triggers
- **7-8 points:** Good keywords, could expand
- **5-6 points:** Basic keywords
- **0-4 points:** Missing or poor

### Requirements

- Description must include domain keywords
- Trigger scenarios in description or "When to Apply"
- Example: "Use when writing BDD tests, feature files, Gherkin scenarios..."

**Remember:** Best description = exhaustive trigger list + examples

## Dimension 8: Practical Usability (15 points)

**Purpose:** Ensure skill is immediately useful with clear examples.

**Scoring:**

- **13-15 points:** Concrete + runnable + clear
- **10-12 points:** Most examples good
- **7-9 points:** Some weak examples
- **0-6 points:** Abstract or missing

### Components

1. **Concrete Examples (5 points)**
   - Real code, not pseudocode
   - Realistic scenarios
   - Actual file paths, commands

2. **Runnable Code (5 points)**
   - Can copy/paste and execute
   - Complete, not fragments
   - Correct syntax

3. **Clear Structure (5 points)**
   - Logical organization
   - Scannable headings
   - Code blocks properly formatted

## Summary: The 120-Point Scale

| Dimension | Max | Focus |
|-----------|-----|-------|
| D1: Knowledge Delta | 20 | Expert knowledge only |
| D2: Mindset + Procedures | 15 | Philosophy + workflows |
| D3: Anti-Pattern Quality | 15 | NEVER + WHY + consequences |
| D4: Specification | 15 | Description field critical |
| D5: Progressive Disclosure | 15 | Hub + references |
| D6: Freedom Calibration | 15 | Appropriate rigidity |
| D7: Pattern Recognition | 10 | Activation keywords |
| D8: Practical Usability | 15 | Concrete examples |
| **TOTAL** | **120** | **A-grade = 108+** |

## See Also

- `framework-scoring-rubric.md` - Detailed scoring methodology
- `framework-quality-standards.md` - A-grade requirements
- Original skill-judge in `.agents/skills/skill-judge/`
