# Remediation Plan — documentation/obsidian/obsidian-cli

**Current Grade:** C+ (108/140)

## Priority Actions

### Anti-Pattern Quality (9/15) — 6 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Eval Validation (14/20) — 6 pts available

⚠️ summary.json coverage is 73% (below 80% threshold)

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Progressive Disclosure (10/15) — 5 pts available

⚠️ no references/ directory (progressive disclosure missing)

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Freedom Calibration (11/15) — 4 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

### Practical Usability (11/15) — 4 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).

### Specification Compliance (12/15) — 3 pts available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Knowledge Delta (18/20) — 2 pts available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Mindset + Procedures (13/15) — 2 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

