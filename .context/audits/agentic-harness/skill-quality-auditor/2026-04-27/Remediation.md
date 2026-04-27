# Remediation Plan — agentic-harness/skill-quality-auditor

**Current Grade:** B+ (123/140)

## Priority Actions

### Specification Compliance (9/15) — 6 pts available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Anti-Pattern Quality (11/15) — 4 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Eval Validation (17/20) — 3 pts available

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Progressive Disclosure (13/15) — 2 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Knowledge Delta (19/20) — 1 pt available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Mindset + Procedures (14/15) — 1 pt available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

