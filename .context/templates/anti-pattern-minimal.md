# Anti-Pattern Template - Minimal

Use this template for skills that need to meet the B-grade threshold (96/120). This provides **3 anti-patterns minimum** to address D3 (Anti-Pattern Quality) weakness.

---

## Anti-Patterns

### NEVER [specific harmful action]

- **WHY**: [explain the root cause of failure - what breaks, what risks emerge].
- **BAD**: [concrete example showing the anti-pattern].
- **GOOD**: [concrete example showing correct alternative].

### NEVER [second specific harmful action]

- **WHY**: [explain consequences and system impact].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].

### NEVER [third specific harmful action]

- **WHY**: [explain long-term or hidden costs].
- **BAD**: [concrete wrong example].
- **GOOD**: [concrete right example].

---

## Usage Guidelines

### Structure Requirements

Each anti-pattern MUST include:
1. **Title**: "NEVER [verb] [specific action]" - be specific, not vague
2. **WHY**: Root cause explanation (1 sentence)
3. **BAD**: Concrete wrong example (code snippet or command)
4. **GOOD**: Concrete right alternative (code snippet or command)

### Content Guidelines

**DO:**
- Use domain-specific examples relevant to the skill
- Keep WHY explanations to 1 sentence (concise root cause)
- Show actual code/commands in BAD/GOOD (not descriptions)
- Focus on high-impact mistakes that agents commonly make
- Use imperative "NEVER" not "Don't" or "Avoid"

**DON'T:**
- Use generic "best practices" language
- Write essay-length explanations in WHY
- Show pseudocode in examples (use real syntax)
- Include obvious anti-patterns everyone knows
- Mix multiple problems in one anti-pattern

### Tone

- Direct and authoritative (NEVER, not "you should avoid")
- Concise (BAD and GOOD are code-first, not prose)
- Technical depth appropriate to skill complexity

---

## Example Anti-Patterns by Domain

### Configuration Skills

**NEVER commit sensitive values directly in config files**

- **WHY**: secret leakage through source control history is irreversible.
- **BAD**: `"apiKey": "sk-abc123..."` in checked-in JSON.
- **GOOD**: `"baseEnv": "API_KEY"` with `.env` file excluded from git.

### CLI/Tool Skills

**NEVER run commands without validating prerequisites exist**

- **WHY**: cryptic errors from missing dependencies block users.
- **BAD**: `npm run build` when Node.js version mismatched.
- **GOOD**: check `node -v` first and warn if <18.0.0 required.

### Framework Skills

**NEVER bypass framework conventions for "quick fixes"**

- **WHY**: convention violations compound maintenance debt.
- **BAD**: direct DOM manipulation in React component.
- **GOOD**: state management via `useState` and reconciliation.

### Testing Skills

**NEVER test implementation details instead of behavior**

- **WHY**: brittle tests break when refactoring working code.
- **BAD**: assert internal state variable name.
- **GOOD**: assert observable output or side effects.

---

## Validation Checklist

Before committing, verify:
- [ ] 3+ anti-patterns documented
- [ ] Each has WHY (1 sentence), BAD (code), GOOD (code)
- [ ] Examples are skill-specific, not generic
- [ ] Titles use "NEVER [verb] [specific thing]"
- [ ] Re-audit shows D3 improvement (+2pts minimum)

---

## Estimated Impact

- **D3 Score**: +2-4 points (from 8-10/15 to 10-12/15)
- **Overall Score**: +2-4 points toward B-grade threshold
- **Time**: 30-45 minutes per skill to customize and apply
