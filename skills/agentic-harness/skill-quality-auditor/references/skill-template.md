---
category: template
priority: HIGH
source: skill-quality-auditor standards
---

# Canonical SKILL.md Template

Use this as the structural blueprint when authoring or refactoring any SKILL.md file.
Copy the sections that apply; omit optional sections only when genuinely not needed.

## Minimal Compliant SKILL.md

````markdown
---
name: {kebab-case-name}
description: "{150-300 char description with domain keywords and trigger phrases. Use when...}"
---

# {Title Case Skill Name}

## When to Use

- {Trigger scenario 1}
- {Trigger scenario 2}

## When Not to Use

- {Anti-trigger scenario 1}

## Principles

1. {Core principle or philosophy — the mental model the agent must adopt}
2. {Second principle}

## Workflow

1. {Step 1} — **Verify:** {how to confirm this succeeded}
2. {Step 2}
3. {Step 3} — **Stop if:** {blockers that require human input}

## Quick Commands

```bash
# {Command description}
{command}
```

Expected result: {what success looks like}

## Anti-Patterns

### NEVER {do X}

**WHY:** {reason this fails}

**BAD:** {concrete example of the anti-pattern}
**GOOD:** {concrete example of the correct approach}

**Consequence:** {what breaks when this anti-pattern is used}

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| {What this reference covers} | [{Display Name}](references/{file}.md) | {Scenario when agent should load this} |
| {External resource description} | [{Title}](https://example.com) | {When to consult official docs} |
````

---

## Section-by-Section Rules

### Frontmatter

```yaml
---
name: skill-name          # Required. Must match directory name. kebab-case only.
description: "..."        # Required. 150-300 chars. Include domain keywords + trigger phrases.
---
```

### Heading

- H1 (`#`) is the skill title — title case, human-readable
- Do not repeat the frontmatter `name` verbatim; expand it

### When to Use / When Not to Use

- List format, 2-5 bullets each
- "When Not to Use" prevents misfire on adjacent skills
- Required for D2 score

### Principles

- 2-5 numbered items
- Each is a short declarative statement the agent should internalize
- These are the philosophical framing (D2 Mindset component)

### Workflow

- Numbered steps (not bullets)
- Each step includes a **Verify:** checkpoint OR a **Stop if:** gate
- Maximum 8 steps before the workflow should be split into sub-sections

### Quick Commands

- One fenced code block per command group
- Every block ends with `Expected result:` on its own line
- Use `bunx`, `npx`, or language-agnostic paths — no harness-specific prefixes

### Anti-Patterns

- H3 heading: `### NEVER <imperative phrase>`
- **WHY:** explains the underlying risk or failure mode
- **BAD:** / **GOOD:** shows concrete code or config examples side-by-side
- **Consequence:** states what breaks — security, reliability, maintainability
- Minimum 3 anti-patterns for D3 score; 5+ for A-grade

### References (Table Standard)

The `## References` section MUST:

1. Be the **last H2** in the file
2. Use a **Markdown table** with exactly these column headers: `Topic`, `Reference`, `When to Use`
3. Have every `Reference` cell contain a markdown link `[text](url)` — no bare paths, no bare URLs
4. Have `Topic` describe what the reference covers
5. Have `When to Use` state the concrete scenario that tells an agent to load it

Optional H3 sub-headings are allowed to group rows by theme (e.g. `### Generators`, `### Executors`).

**Full example:**

```markdown
## References

### Core

| Topic | Reference | When to Use |
| --- | --- | --- |
| Rule pack options, Aspects API, and suppression patterns | [Implementation Guide](references/implementation-guide.md) | First-time setup or rule pack selection |
| Complete list of AwsSolutions rules and severity levels | [Rule Packs](references/rule-packs.md) | Triaging an unfamiliar finding |

### External

| Topic | Reference | When to Use |
| --- | --- | --- |
| Official cdk-nag rule catalog and release notes | [cdk-nag GitHub](https://github.com/cdklabs/cdk-nag) | Checking rule IDs or upgrade behavior |
```

---

## Common Mistakes to Avoid

| Mistake | Fix |
| --- | --- |
| Bullet list in `## References` instead of table | Convert to `Topic \| Reference \| When to Use` table |
| Heading `## Resources` or `## See Also` | Rename to exactly `## References` |
| Bare URL in Reference column: `https://...` | Wrap as `[Title](https://...)` |
| Missing `When to Use` column | Add column — "When to consult" forces actionable descriptions |
| `## References` is not the last H2 | Move it to the end of the file |
| References section appears mid-file before Anti-Patterns | Reorder: Anti-Patterns → References |
