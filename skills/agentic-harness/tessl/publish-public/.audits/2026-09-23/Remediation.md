# Remediation Plan — agentic-harness/tessl/publish-public

**Current Grade:** C (101/140)

## Priority Actions

### Eval Validation (4/20) — 16 pts available

⚠️ 9 flat scenario-NN.md file(s) found; migrate to scenario-N/ subdirectory format to score on D9

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Anti-Pattern Quality (9/15) — 6 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Specification Compliance (10/15) — 5 pts available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Practical Usability (11/15) — 4 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).

### Knowledge Delta (18/20) — 2 pts available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Mindset + Procedures (13/15) — 2 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Progressive Disclosure (13/15) — 2 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Freedom Calibration (13/15) — 2 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

