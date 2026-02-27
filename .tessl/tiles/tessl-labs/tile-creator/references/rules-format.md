# Rules Format

Rules tell agents how to behave. They're loaded into the agent's instruction context.

## When to Use Rules

**Rules are for critical behavioral guidance only.** Most content belongs in docs or skills instead.

| Content Type | Use For                     | Example                                   |
| ------------ | --------------------------- | ----------------------------------------- |
| **Docs**     | Facts, knowledge, reference | API documentation, schemas                |
| **Skills**   | Procedures, workflows       | Deployment process, code generation       |
| **Rules**    | Must-follow constraints     | Security requirements, naming conventions |

**Ask yourself:** Is this a _constraint_ the agent must always follow, or _information_ it should know? Constraints → rules. Information → docs.

## Minimalism Principles

Rules consume context on every request. Keep them minimal:

1. **Few rules** - Only the most critical behavioral guidance
2. **Short rules** - Each rule should be scannable, not a document
3. **Actionable rules** - Direct instructions, not explanations

**Good rule content:**

```markdown
# API Error Responses

Return errors as:
\`\`\`json
{"error": {"code": "ERROR_CODE", "message": "Description"}}
\`\`\`

Status codes: 400 (validation), 401 (auth), 403 (forbidden), 404 (not found), 500 (server)
```

**Bad rule content:** Multi-page explanations of why these patterns matter, history of error handling approaches, comparisons with alternatives.

## Structure

```
rules/
├── security.md      # Critical security constraints
└── code-style.md    # Core conventions (if truly essential)
```

Each rule is a separate file referenced in `tile.json` under `steering`.

## YAML Frontmatter

Every rule file requires frontmatter:

| Field         | Required                | Description                                             |
| ------------- | ----------------------- | ------------------------------------------------------- |
| `alwaysApply` | Yes                     | `true` = always loaded, `false` = conditional           |
| `description` | If `alwaysApply: false` | Trigger description (2-3 sentences, include "Use when") |

## Always-Apply Rules

Use sparingly - these load on every request:

```markdown
---
alwaysApply: true
---

# Security

- Never log secrets or credentials
- Sanitize all user input before database queries
- Use parameterized queries, never string concatenation
```

## Conditional Rules

Loaded only when the trigger matches:

```markdown
---
description: Database migration patterns. Use when creating or modifying database schemas.
alwaysApply: false
---

# Migrations

1. Include both up and down migrations
2. Use transactions for data modifications
3. Test on copy of production data first
```

## Writing Good Descriptions

The `description` acts as a trigger. Be specific:

**Good:**

```yaml
description: TypeScript error handling patterns. Use when implementing try/catch or error types.
```

**Bad:**

```yaml
description: Error handling guide.
```

## tile.json Configuration

```json
{
  "steering": {
    "security": { "rules": "rules/security.md" },
    "migrations": { "rules": "rules/migrations.md" }
  }
}
```
