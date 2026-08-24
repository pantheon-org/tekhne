# Rule Format Reference

Every rule in the project rules file MUST follow this three-part structure:

```markdown
### Rule: <short imperative title>

**Directive:** <ALWAYS/NEVER instruction>

**Rationale:** <why this rule exists>
```

## Title conventions

- Imperative mood: "Always validate...", "Never commit...", "Prefer..."
- Max 10 words
- Must be unique across all rules

## Directive conventions

- Start with `ALWAYS` or `NEVER` for hard constraints
- Use `PREFER` or `AVOID` for soft guidance
- Include the triggering condition or scope

## Rationale conventions

- One to two sentences
- Explain the consequence of violating the rule
- Reference measurable outcomes where possible
