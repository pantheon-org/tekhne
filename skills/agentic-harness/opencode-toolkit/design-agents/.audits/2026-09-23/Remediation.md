# Remediation Plan — agentic-harness/opencode-toolkit/design-agents

**Current Grade:** C+ (110/140)

## Priority Actions

### Eval Validation (8/20) — 12 pts available

Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Anti-Pattern Quality (7/15) — 8 pts available

Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Knowledge Delta (18/20) — 2 pts available

Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).

### Mindset + Procedures (13/15) — 2 pts available

Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Specification Compliance (13/15) — 2 pts available

⚠️ agent-specific reference found: opencode

Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

### Progressive Disclosure (13/15) — 2 pts available

Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Practical Usability (13/15) — 2 pts available

Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).

