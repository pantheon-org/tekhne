# Remediation Plan — documentation/research/sci-hub-search

**Current Grade:** C+ (111/140)

## Priority Actions

### Eval Validation (13/20) — 7 pts available

⚠️ 3 flat scenario-NN.md file(s) found; migrate to scenario-N/ subdirectory format to score on D9

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Anti-Pattern Quality (9/15) — 6 pts available

🔴 instructions.json exists but cannot be parsed

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Freedom Calibration (10/15) — 5 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

### Mindset + Procedures (11/15) — 4 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Progressive Disclosure (11/15) — 4 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Specification Compliance (12/15) — 3 pts available

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

