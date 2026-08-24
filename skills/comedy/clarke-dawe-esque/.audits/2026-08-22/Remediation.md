# Remediation Plan — comedy/clarke-dawe-esque

**Current Grade:** B (117/140)

## Priority Actions

### Anti-Pattern Quality (10/15) — 5 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Progressive Disclosure (10/15) — 5 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Specification Compliance (12/15) — 3 pts available

⚠️ agent-specific reference found: cline

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Eval Validation (17/20) — 3 pts available

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Knowledge Delta (18/20) — 2 pts available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Mindset + Procedures (13/15) — 2 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Practical Usability (13/15) — 2 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash,```typescript).

### Freedom Calibration (14/15) — 1 pt available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).
