# Remediation Plan — project-mgmt/create-context-file

**Current Grade:** B+ (124/140)

## Priority Actions

### Progressive Disclosure (10/15) — 5 pts available

- **What happened:** scored 10 of 15, 5 pts short.
- **Likely cause:** Detail sits inline in `SKILL.md` instead of `references/`, `SKILL.md` is long, or the References section is not the last section with link bullets.
- **Fix:** Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.

### Anti-Pattern Quality (11/15) — 4 pts available

- **What happened:** scored 11 of 15, 4 pts short.
- **Likely cause:** Anti-patterns are missing, or they are stated without a WHY or a BAD/GOOD contrast.
- **Fix:** Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.

### Eval Validation (17/20) — 3 pts available

⚠️ 8 of 8 scenario(s) have no failure check; add a criteria.json item with "failure_check": true for a behaviour the response must not show (not scored)

- **What happened:** scored 17 of 20, 3 pts short.
- **Likely cause:** `evals/` is missing or incomplete: no `instructions.json` or `summary.json`, coverage under 80%, or fewer than 3 valid scenarios.
- **Fix:** Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.

### Mindset + Procedures (13/15) — 2 pts available

- **What happened:** scored 13 of 15, 2 pts short.
- **Likely cause:** There is no Mindset or Philosophy section, few numbered procedures, or the When to Use and When NOT to Use sections are missing.
- **Fix:** Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.

### Specification Compliance (13/15) — 2 pts available

- **What happened:** scored 13 of 15, 2 pts short.
- **Likely cause:** The `description` is short or keyword-stuffed, or the skill names harness-specific paths, agents, or `../` links outside code blocks.
- **Fix:** Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.

