# Skills Format

Skills are procedural workflows that guide agents through complex tasks.

When creating skills:

1. Consider installing and using the `skill-creator` skill. You can install as follows:

```bash
tessl i github:anthropics/skills --skill skill-creator
```

2. After creating a skill and ensuring proper tile packaging, make sure you review and improve quality if needed. You can review by running:

```bash
tessl skill review < path-to-folder-containing-SKILL.md >
```

## Structure

```
skills/
└── my-skill/
    ├── SKILL.md       # Required
    ├── scripts/       # Optional executables
    ├── references/    # Optional detailed docs
    └── assets/        # Optional output files
```

## SKILL.md Format

Every skill requires a `SKILL.md` with YAML frontmatter:

```markdown
---
name: my-skill-name
description: What this skill does. Use when [specific triggers].
---

# Skill Title

Instructions and guidance...
```

## Frontmatter Fields

| Field         | Required | Description                   |
| ------------- | -------- | ----------------------------- |
| `name`        | Yes      | Lowercase, hyphens only       |
| `description` | Yes      | What it does + when to use it |

## Writing Good Descriptions

The description triggers skill activation. Be specific:

**Good:**

```yaml
description: Database migration helper for creating and managing schema changes. Use when creating migrations, altering tables, or managing database versions.
```

**Bad:**

```yaml
description: Helps with databases.
```

## Bundled Resources

### scripts/

Executable code for deterministic operations:

```
scripts/
├── validate.py
├── generate.sh
└── deploy.js
```

### references/

Detailed documentation loaded on-demand:

```
references/
├── api-spec.md
├── schema.md
└── examples.md
```

### assets/

Files used in output (templates, images):

```
assets/
├── template.html
├── logo.png
└── boilerplate/
```

## tile.json Configuration

```json
{
  "skills": {
    "my-skill": { "path": "skills/my-skill/SKILL.md" }
  }
}
```

## Example SKILL.md

```markdown
---
name: deploy-staging
description: Deploy application to staging environment. Use when deploying to staging, testing deployments, or preparing releases.
---

# Deploy to Staging

## Prerequisites

- AWS CLI configured
- Docker installed
- Access to staging cluster

## Deployment Steps

1. Build the Docker image:
   \`\`\`bash
   docker build -t app:staging .
   \`\`\`

2. Push to registry:
   \`\`\`bash
   docker push registry.example.com/app:staging
   \`\`\`

3. Update deployment:
   \`\`\`bash
   kubectl apply -f k8s/staging/
   \`\`\`

4. Verify deployment:
   \`\`\`bash
   kubectl rollout status deployment/app -n staging
   \`\`\`

## Rollback

If issues occur:
\`\`\`bash
kubectl rollout undo deployment/app -n staging
\`\`\`
```

## CLI Commands

**Create a new skill:**

```bash
tessl skill new --name my-skill --description "Description" --path ./my-skill
```

**Validate a skill:**

```bash
tessl skill lint ./my-skill
```

**Review skill quality:**

```bash
tessl skill review ./my-skill
```
