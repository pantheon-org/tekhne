---
plan_date: 2026-02-23
skill_name: markdown-authoring
source_audit: .context/audits/markdown-authoring-audit-2026-02-22.md
---

# Remediation Plan: markdown-authoring

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 82/120 (68%) | 100/120 (83%) |
| **Grade** | D | B |
| **Priority** | Critical | - |

**Verdict**: Major rewrite recommended. Low usability and weak anti-patterns.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern precision | D3 (8/15) | High | Common markdown mistakes repeated |
| Low practical usability | D8 (8/15) | **Critical** | Commands/examples missing |
| Poor trigger discoverability | D7 (6/10) | High | Skill not activated |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |
| Over/under constraint | D6 (10/15) | Medium | Flexibility issues |

## Detailed Remediation Steps

### Phase 1: Practical Usability (D8) - CRITICAL PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

**Problem**: D8 score of 8/15 indicates missing commands and examples.

1. **Add Quick Commands section**:

````markdown
## Quick Commands

### Lint Markdown Files
```bash
bunx markdownlint-cli2 "**/*.md"
```

### Fix Markdown Issues
```bash
bunx markdownlint-cli2 "**/*.md" --fix
```

### Check Specific File
```bash
bunx markdownlint-cli2 path/to/file.md
```

### Validate Configuration
```bash
cat .markdownlint.json
```

### Generate Table of Contents
```bash
npx markdown-toc README.md --no-firsth1
```

### Convert to Other Formats
```bash
npx marked README.md > README.html
```
````

2. **Add expected output examples**:

````markdown
## Expected Outputs

### Lint Output (Clean)
```
[No issues found]
```

### Lint Output (Issues Found)
```
README.md:3:81 MD013/line-length Line length [Expected: 80, Actual: 95]
README.md:15:1 MD022/blanks-around-headings Headings should be surrounded by blank lines
```

### Fixed File Structure
```
document.md
├── Heading 1 (line 1)
├── Blank line (line 2)
├── Paragraph (lines 3-5)
├── Blank line (line 6)
├── Heading 2 (line 7)
└── ...
```
````

3. **Add concrete examples for common tasks**:

````markdown
## Common Tasks

### Create a README
1. Create file with standard structure:
```markdown
# Project Name

Brief description of what this project does.

## Installation

\`\`\`bash
npm install project-name
\`\`\`

## Usage

\`\`\`javascript
import { feature } from 'project-name';
feature();
\`\`\`

## License

MIT
```

### Add a Code Block
- Use triple backticks with language tag
- Always specify language for syntax highlighting
- BAD: ```
- GOOD: \`\`\`typescript

### Create a Table
```markdown
| Column 1 | Column 2 |
|----------|----------|
| Value 1  | Value 2  |
```
````

### Phase 2: Anti-Pattern Quality (D3) - HIGH PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

1. **Add explicit anti-patterns section**:

````markdown
## Anti-Patterns

### NEVER use inline HTML for formatting
- **WHY**: Reduces portability and linting effectiveness
- **BAD**: `<strong>bold text</strong>`
- **GOOD**: `**bold text**`
- **CONSEQUENCE**: Inconsistent rendering, linting failures

### NEVER skip blank lines around headings
- **WHY**: Markdown parsers require blank lines for proper heading detection
- **BAD**:
  ```markdown
  # Heading
  Some text
  ```
- **GOOD**:
  ```markdown
  # Heading

  Some text
  ```
- **CONSEQUENCE**: Headings may not render, TOC generation fails

### NEVER use hard tabs
- **WHY**: Inconsistent rendering across editors
- **BAD**: `→→code indent`
- **GOOD**: `  code indent` (2 spaces)
- **CONSEQUENCE**: Mixed indentation, formatting issues

### NEVER exceed line length limits
- **WHY**: Readability and diff clarity
- **BAD**: Single line of 150+ characters
- **GOOD**: Break at 80-100 characters
- **CONSEQUENCE**: Horizontal scrolling, poor diffs

### NEVER nest code blocks improperly
- **WHY**: Parser ambiguity
- **BAD**: Triple backticks inside triple backticks without escaping
- **GOOD**: Use different fence characters or indentation
- **CONSEQUENCE**: Rendering breaks

### NEVER use emoji in technical documentation
- **WHY**: Plain text compatibility, accessibility
- **BAD**: "Great feature!"
- **GOOD**: "Feature: implemented authentication module"
- **CONSEQUENCE**: Display issues, accessibility problems

### Repository-Specific Rules
- This repository enforces MD013 (line-length) at 100 characters
- All markdown files must pass markdownlint-cli2
- Configuration in `.markdownlint.json` at repository root
````

### Phase 3: Pattern Recognition (D7) - HIGH PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

1. **Expand frontmatter description**:

```markdown
---
name: markdown-authoring
description: Complete markdown authoring and documentation guidance covering syntax, documentation structure, markdownlint configuration, and CI/CD integration. Use when: "write README", "create documentation", "fix markdown lint errors", "configure markdownlint", "markdown best practices", "format markdown", "add code blocks", "create tables".
---
```

2. **Add explicit trigger phrases**:

```markdown
## Activation Triggers

This skill activates when users ask:

### Documentation Creation
- "Write a README for this project"
- "Create documentation for [feature]"
- "Add a contributing guide"
- "Generate API documentation"

### Linting Issues
- "Fix markdown lint errors"
- "Why is markdownlint failing?"
- "Configure markdownlint rules"
- "Suppress markdown lint warning"

### Formatting Questions
- "How do I format [element] in markdown?"
- "Create a table in markdown"
- "Add a code block"
- "Link to another file"

### Best Practices
- "Markdown best practices"
- "Improve documentation structure"
- "Documentation style guide"
```

### Phase 4: Specification Compliance (D4) - MEDIUM PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

1. **Add output specification**:

````markdown
## Output Specification

### README Structure
```markdown
# Project Name (required)
Brief description (required)

## Installation (required)
## Usage (required)
## Configuration (optional)
## API Reference (optional)
## Contributing (optional)
## License (required)
```

### Documentation File Structure
- All documentation files in `docs/` directory
- Use kebab-case filenames
- Include frontmatter with `title`, `date`, `author` (optional)
````

### Phase 5: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

1. **Add explicit workflow**:

````markdown
## Workflow

### Step 1: Identify Document Type
- **Input**: User request
- **Decision**: README | API docs | Contributing guide | General docs
- **Output**: Document type confirmed

### Step 2: Apply Template
- **Action**: Select appropriate template
- **Output**: Document skeleton with required sections

### Step 3: Write Content
- **Action**: Fill in sections following style guide
- **Output**: Complete document content

### Step 4: Validate
```bash
bunx markdownlint-cli2 "path/to/document.md"
```
- **Exit condition**: No lint errors

### Step 5: Review Structure
- Check heading hierarchy (H1 > H2 > H3)
- Verify all links resolve
- Confirm code blocks have language tags
````

### Phase 6: Freedom Calibration (D6) - MEDIUM PRIORITY

**File**: `skills/markdown-authoring/SKILL.md`

1. **Add flexibility guidelines**:

```markdown
## Flexibility Guidelines

### Hard Constraints (NEVER violate)
- Files must pass markdownlint
- Line length max 100 characters
- No inline HTML for formatting
- Blank lines around headings

### Flexible Decisions (adapt to context)
- Document section order (adapt to project needs)
- Level of detail (match audience)
- Additional sections (add as needed)

### Fallback Paths
- If lint config missing: Use defaults from `.markdownlint.json`
- If template unknown: Use standard README template
- If link unresolved: Mark as TODO for review
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh markdown-authoring --json

# Verify markdownlint works
bunx markdownlint-cli2 "**/*.md"

# Check skill structure
ls -la skills/markdown-authoring/

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D8: Practical Usability | Score increase | >= 13/15 |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D7: Pattern Recognition | Score increase | >= 9/10 |
| D4: Specification Compliance | Score increase | >= 13/15 |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| D6: Freedom Calibration | Score increase | >= 12/15 |
| Overall Score | Total points | >= 100/120 |
| Grade | Letter grade | >= B |

## Effort Estimate

- **T-shirt size**: M (4-6 hours)
- **Complexity**: Medium
- **Risk**: Low (additive changes)

## Dependencies

- markdownlint-cli2 installed (already in project)

## Notes

- D5 (Progressive Disclosure) is already strong at 15/15 - maintain current structure
- Consider moving detailed lint rule explanations to references/
- 16 reference files already exist - leverage them in SKILL.md navigation
