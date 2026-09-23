# Remediation Plan — agentic-harness/pick-model

**Current Grade:** C (100/140)

## Priority Actions

### Eval Validation (7/20) — 13 pts available

⚠️ 5 flat scenario-NN.md file(s) found; migrate to scenario-N/ subdirectory format to score on D9

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Anti-Pattern Quality (10/15) — 5 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Progressive Disclosure (10/15) — 5 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Freedom Calibration (10/15) — 5 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

### Mindset + Procedures (11/15) — 4 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Practical Usability (11/15) — 4 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).

### Specification Compliance (13/15) — 2 pts available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Knowledge Delta (19/20) — 1 pt available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Pattern Recognition (9/10) — 1 pt available

Expand the `description` frontmatter field to more than 15 qualifying words (words longer than 3 characters).

