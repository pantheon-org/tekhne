# Task: Anti-Pattern Documentation Enhancement

A skill for a CI/CD pipeline generator currently scores D3: 5/15. Its SKILL.md has a brief "Common Mistakes" section with three one-liner bullets but no NEVER/WHY/BAD/GOOD structure.

Current content of the anti-patterns section:

```markdown
## Common Mistakes

- Don't hardcode secrets in workflow files
- Avoid running all jobs in parallel when they have dependencies
- Don't use latest tags for actions
```

Rewrite and expand this section to score D3: >=13/15. Follow the anti-pattern format used by this skill-quality-auditor.

## Output Specification

Produce:
1. **anti-patterns-section.md** — the rewritten section ready to paste into SKILL.md
2. **before-after-diff.md** — a summary showing old vs new structure and expected score impact
