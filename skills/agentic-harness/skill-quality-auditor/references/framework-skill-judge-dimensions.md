---
category: framework
priority: CRITICAL
source: skill-judge canonical + session experience
---

# Skill-Judge Evaluation Framework: 9 Dimensions

Complete evaluation methodology for assessing skill quality using the skill-judge framework. This is the foundation for all quality auditing.

Canonical source reference: `framework-skill-judge-canonical.md`

## Overview

The skill-judge framework evaluates skills across 9 dimensions totaling 140 points. **Dimension 1 (Knowledge Delta)** and **Dimension 9 (Eval Validation)** carry the highest weight at 20 points each - skills must contain expert-only knowledge AND be validated at runtime via tessl eval scenarios.

**Target Score:** ≥126 points (90%) = A-grade

## Dimension 1: Knowledge Delta (20 points) ⭐ MOST IMPORTANT

**Purpose:** Ensure skill contains expert-only knowledge, not redundant information.

**Scoring:**

- **18-20 points:** Pure expert knowledge, <5% redundancy
- **15-17 points:** Mostly expert, 5-15% redundancy  
- **12-14 points:** 15-30% redundancy (acceptable)
- **9-11 points:** 30-50% redundancy (needs improvement)
- **0-8 points:** >50% redundancy (failing)

**Core Principle:** Skill = Expert Knowledge - What AI Assistants Already Know

### Three Knowledge Types

1. **Expert (KEEP):**
   - Domain-specific patterns AI assistants don't know
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
   - Basic syntax AI assistants know
   - Installation instructions from official docs
   - API documentation copied verbatim
   - Generic best practices
   - Obvious examples

### Red Flags for Low Knowledge Delta

❌ Teaching basic syntax (AI assistants know `if/else`, `function`, `class`)  
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

Problem: AI assistants already know basic TypeScript syntax.

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

Expert pattern AI assistants don't know by default.

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

**Purpose:** Ensure proper frontmatter, single-task focus, activation keywords, and cross-harness portability.

**Scoring:**

- **13-15 points:** Perfect spec compliance
- **10-12 points:** Minor issues
- **7-9 points:** Missing key elements
- **0-6 points:** Non-compliant

### Components

1. **Task Focus Declaration (4 points) ⭐ CRITICAL**
    - Skill indicates ONE type of task it helps complete
    - Description clearly scopes to single purpose
    - No ambiguity about what the skill does
    - Example: "Write BDD tests" (good) vs "Testing and development" (bad - two tasks)

2. **Description Field Quality (6 points)**
    - **Primary agents:** Exactly 3 words
    - **Other agents:** Comprehensive with trigger examples
    - Must include activation keywords
    - Determines if skill activates

3. **Cross-Harness Portability (3 points) ⭐ CRITICAL**
    - **No harness-specific paths (1 point):** Avoid `.opencode/`, `.claude/`, `.cursor/`, `.aider/`, `.continue/`
    - **No agent-specific references (1 point):** Don't mention "Claude Code", "Cursor Agent", "GitHub Copilot", etc. in instructions
    - **Relative path usage (1 point):** Reference files relative to skill directory (`scripts/`, `references/`, `templates/`)
    - **WHY:** Skills must work across 40+ agentic harnesses without modification
    - **IMPACT:** Harness-specific paths break skill discovery when synced to other agents

4. **Self-Containment (penalties: up to -4 points) ⭐ CRITICAL**
    - **No parent-escaping paths (-2 points):** SKILL.md must not use `../` references outside fenced code blocks. Skills are installed to arbitrary locations; parent paths break when the skill is not in its original repo.
    - **No absolute repo paths (-1 point):** SKILL.md must not reference `skills/X/Y/Z` or other hardcoded repository paths outside fenced code blocks. Cross-skill dependencies should use skill names, not file paths.
    - **No repo-root directory references (-1 point):** SKILL.md must not reference `.context/`, `.agents/`, or other repo-root directories outside fenced code blocks.
    - **WHY:** Skills must be fully self-contained. When installed via `tessl install` or `npx skills add`, they land in arbitrary directories. Any reference to files outside the skill's own directory tree will break.
    - **IMPACT:** Non-self-contained skills fail silently when installed outside their authoring repo.

5. **Script Language Portability (bonus: +1 point)**
    - Skills with `scripts/` containing Python (`.py`), TypeScript (`.ts`), or JavaScript (`.js`) files earn a portability bonus.
    - These languages provide better cross-platform string manipulation, JSON handling, and error handling compared to shell for complex logic.
    - Shell scripts (`.sh`) remain the accepted default and receive no penalty.
    - **Accepted shebangs:** `#!/usr/bin/env python3` (Python), `#!/usr/bin/env bun` (TypeScript), `#!/usr/bin/env node` (JavaScript)
    - **WHY:** Complex scripts that parse JSON, manipulate strings, or make HTTP calls are more portable and robust in Python/TS/JS than in POSIX shell (which depends on external tools like `jq`, and has GNU-vs-BSD divergence for `grep`/`sed`/`awk`).

6. **Proper Frontmatter (1 point)**
    - name, description present
    - Consolidation notes if applicable
    - Correct YAML syntax

7. **Activation Keywords (1 point)**
    - Domain terms that trigger skill
    - Example: "BDD, Gherkin, Given-When-Then, Cucumber"

### Examples

**✅ Excellent Specification Compliance (15/15):**

```yaml
---
name: bdd-testing
description: Behavior-Driven Development with Given-When-Then scenarios, Cucumber.js, Three Amigos collaboration, Example Mapping, living documentation, and acceptance criteria. Use when writing BDD tests, feature files, or planning discovery workshops.
---

# BDD Testing

Execute test runner with portable path:
```bash
bun run scripts/run-tests.sh
```

Reference files use relative paths: `references/file.md`
```

*Perfect: comprehensive description, portable paths (scripts/, references/), no agent mentions*

**❌ Poor Specification Compliance (7/15):**

```yaml
---
name: bdd-testing  
description: BDD testing patterns
---

# BDD Testing

For Claude Code users, run:
```bash
.opencode/scripts/run-tests.sh
```

For Cursor users, see `.claude/docs/file.md`
```

*Problems: weak description, harness-specific paths (.opencode/, .claude/), agent-specific references*

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

Appropriately rigid for critical verification.

**❌ Miscalibrated (7/15):**

```markdown
# TypeScript Basics (Tool skill)

## Rules
ALWAYS use const for all variables.
NEVER use let or var under any circumstances.
```

Too rigid - let has valid use cases.

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

## Dimension 9: Eval Validation (20 points) -- HIGHEST PRIORITY

**Purpose:** Verify the skill has been validated at runtime through tessl eval scenarios, proving agents actually follow its instructions.

**Scoring:**

- **17-20 points:** Complete evals with >=80% instruction coverage, >=3 valid scenarios
- **13-16 points:** Evals present with partial coverage or incomplete scenarios
- **7-12 points:** Evals directory exists but missing key files
- **1-6 points:** Minimal eval structure, no coverage data
- **0 points:** No evals directory

**Core Principle:** Static quality (D1-D8) is necessary but not sufficient. Runtime validation proves the skill actually changes agent behavior.

### Components

1. **Eval Directory Structure (4 points)**
   - `evals/` directory exists with proper layout
   - Follows tessl eval harness conventions

2. **Instruction Inventory (3 points)**
   - `instructions.json` present and non-empty
   - Every instruction extracted from SKILL.md
   - Classified by `why_given`: reminder, new knowledge, preference

3. **Coverage Statistics (6 points)**
   - `summary.json` with `instructions_coverage` data (3 points)
   - Coverage percentage >= 80% (3 points)

4. **Valid Scenarios (4 points)**
   - >= 3 scenarios with complete structure (task.md + criteria.json + capability.txt)
   - Each criteria.json sums to exactly 100

5. **Criteria Quality (3 points)**
   - 10+ checklist items per scenario
   - Binary yes/no criteria traceable to specific instructions
   - No instruction leakage in task.md

### Relationship to D1 and D3

When `instructions.json` exists, its data enriches other dimensions:

- **D1 (Knowledge Delta):** The `why_given` distribution (new knowledge + preference vs reminders) provides a more accurate expert content ratio than shell heuristics alone.
- **D3 (Anti-Pattern Quality):** Instructions containing NEVER/ALWAYS/anti-pattern keywords are cross-referenced with scenario coverage for a stronger signal.

### Creating Evals

Use the `creating-eval-scenarios` skill to generate evaluation scenarios:

```bash
# Ensure skill is packaged as a tessl tile first
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
```

### Examples

**High Eval Validation (19/20):**

```
skill-name/evals/
  instructions.json      # 28 instructions extracted
  summary.json           # 100% coverage, 5 scenarios
  summary_infeasible.json
  scenario-0/            # task.md + criteria.json (sum=100) + capability.txt
  scenario-1/
  scenario-2/
  scenario-3/
  scenario-4/
```

**Low Eval Validation (4/20):**

```
skill-name/evals/
  instructions.json      # Present but only 5 instructions
  # No summary.json, no scenarios
```

**Zero Eval Validation (0/20):**

```
skill-name/
  SKILL.md               # No evals/ directory at all
```

## Summary: The 140-Point Scale

| Dimension | Max | Priority | Focus |
|-----------|-----|----------|-------|
| D1: Knowledge Delta | 20 | HIGHEST | Expert knowledge only |
| D2: Mindset + Procedures | 15 | HIGH | Philosophy + workflows |
| D3: Anti-Pattern Quality | 15 | HIGH | NEVER + WHY + consequences |
| D4: Specification | 15 | MEDIUM | Description field critical |
| D5: Progressive Disclosure | 15 | MEDIUM | Hub + references |
| D6: Freedom Calibration | 15 | MEDIUM | Appropriate rigidity |
| D7: Pattern Recognition | 10 | LOW | Activation keywords |
| D8: Practical Usability | 15 | HIGH | Concrete examples |
| D9: Eval Validation | 20 | HIGHEST | Runtime validation via tessl evals |
| **TOTAL** | **140** | | **A-grade = 126+** |

## See Also

- `framework-scoring-rubric.md` - Detailed scoring methodology
- `framework-quality-standards.md` - A-grade requirements
- `creating-eval-scenarios` skill - Tessl eval scenario generation
