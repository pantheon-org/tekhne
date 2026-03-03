# Anti-Pattern Template - Comprehensive

Use this template for skills aiming for **A-grade (≥108/120)** or those with complex domains requiring 5+ anti-patterns.

---

## Anti-Patterns

### NEVER [specific harmful action - pattern 1]

- **WHY**: [root cause - 1 sentence].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].
- **IMPACT**: [optional - quantify consequence, e.g., "increases build time 3x"].

### NEVER [specific harmful action - pattern 2]

- **WHY**: [root cause - 1 sentence].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].
- **IMPACT**: [optional - user-facing consequence].

### NEVER [specific harmful action - pattern 3]

- **WHY**: [root cause - 1 sentence].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].
- **IMPACT**: [optional - technical debt or risk].

### NEVER [specific harmful action - pattern 4]

- **WHY**: [root cause - 1 sentence].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].
- **IMPACT**: [optional - system impact].

### NEVER [specific harmful action - pattern 5]

- **WHY**: [root cause - 1 sentence].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].
- **IMPACT**: [optional - maintenance burden].

---

## Common Mistake Patterns

[Optional subsection grouping related anti-patterns by theme]

### Configuration Errors
- [List anti-patterns related to config/setup]

### Runtime Mistakes
- [List anti-patterns related to execution]

### Architecture Violations
- [List anti-patterns related to design]

---

## Usage Guidelines

### When to Use Comprehensive Template

Use this template for:
- **Complex skills** with multiple failure modes (e.g., framework integrations, CI/CD)
- **A-grade candidates** needing D3 scores ≥13/15 (87%+)
- **Mission-critical skills** where mistakes have high cost
- **Reference skills** meant to serve as exemplars

### Structure Requirements

Each anti-pattern MUST include:
1. **Title**: "NEVER [verb] [specific action]" - precise, actionable
2. **WHY**: Root cause (1 sentence max)
3. **BAD**: Concrete wrong example (real code/commands)
4. **GOOD**: Concrete right alternative (real code/commands)
5. **IMPACT** (optional): Quantified consequence or category (performance, security, maintainability)

### Content Guidelines

**DO:**
- Cover diverse failure categories (config, runtime, architecture, testing)
- Use real examples from actual skill usage patterns
- Quantify IMPACT when possible ("3x slower", "breaks CI", "leaks secrets")
- Order by severity (highest impact first)
- Group related patterns with subsections

**DON'T:**
- Repeat similar patterns with different examples
- Include IMPACT for obvious consequences
- Write more than 5-7 anti-patterns (diminishing returns)
- Use vague "considered harmful" language
- Skip concrete BAD/GOOD examples

### Tone

- Authoritative and specific (not advisory)
- Technical depth matching skill complexity
- Concise WHY + concrete BAD/GOOD + optional quantified IMPACT

---

## Example Anti-Patterns by Domain

### Executor/Task Runner Skills

**NEVER put business logic directly inside executor entrypoints**

- **WHY**: bloated executors are hard to test and reuse.
- **BAD**: 200 lines of transformation logic in `executor.ts`.
- **GOOD**: delegate to composable library functions.
- **IMPACT**: testing requires full executor context instead of isolated unit tests.

### Configuration/Setup Skills

**NEVER commit API keys directly in config files**

- **WHY**: secret leakage through source control history is irreversible.
- **BAD**: `"apiKey": "sk-abc123..."` in `config.json`.
- **GOOD**: `"baseEnv": "OPENAI_API_KEY"` referencing environment variable.
- **IMPACT**: security breach exposing production credentials publicly.

### Linting/Formatting Skills

**NEVER run competing tools on the same files**

- **WHY**: conflicting rules create contradictory output and noisy reviews.
- **BAD**: ESLint and Biome both lint `src/**/*.ts`.
- **GOOD**: route TypeScript linting through Biome only, disable ESLint for TS.
- **IMPACT**: duplicate diagnostics and unstable CI, constant formatting churn.

### Documentation Skills

**NEVER dump encyclopedic content into root documentation**

- **WHY**: oversized docs increase token cost and reduce usability.
- **BAD**: embed full framework manuals in AGENTS.md.
- **GOOD**: keep root concise, link to scoped files or external references.
- **IMPACT**: agents hit context limits, scan time increases 5-10x.

### Code Generation Skills

**NEVER modify files outside the framework's abstraction API**

- **WHY**: direct filesystem writes bypass validation and change tracking.
- **BAD**: `writeFileSync("libs/my-lib/src/index.ts", content)`.
- **GOOD**: `tree.write("libs/my-lib/src/index.ts", content)` via framework API.
- **IMPACT**: dry-run mode broken, change preview unavailable.

---

## Validation Checklist

Before committing, verify:
- [ ] 5-7 anti-patterns documented (not fewer, not too many)
- [ ] Each has WHY (1 sentence), BAD (code), GOOD (code)
- [ ] IMPACT included for high-severity patterns
- [ ] Patterns cover diverse failure categories
- [ ] Examples are skill-specific with real syntax
- [ ] Titles use "NEVER [verb] [specific action]"
- [ ] Optional: grouped by theme if ≥6 patterns
- [ ] Re-audit shows D3 score ≥13/15 (87%+)

---

## Estimated Impact

- **D3 Score**: +4-6 points (from 8-10/15 to 13-15/15)
- **Overall Score**: +4-6 points toward A-grade
- **Time**: 60-90 minutes per skill to research and customize
- **Use Cases**: Complex framework skills, reference exemplars, mission-critical guides
