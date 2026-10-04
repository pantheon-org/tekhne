# Remediation Plan — agentic-harness/opencode-toolkit/configure

**Current Grade:** B (116/140)

## Priority Actions

### Eval Validation (14/20) — 6 pts available

⚠️ summary.json coverage is 38% (below 80% threshold)

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Progressive Disclosure (10/15) — 5 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Mindset + Procedures (11/15) — 4 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Anti-Pattern Quality (11/15) — 4 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Freedom Calibration (12/15) — 3 pts available

Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).

### Specification Compliance (13/15) — 2 pts available

⚠️ agent-specific reference found: opencode

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

