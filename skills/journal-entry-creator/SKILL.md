---
name: journal-entry-creator
description:
  "Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]',
  'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article
  summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation."
---

# Journal Entry Creator

Automate creation of structured journal entries with template schemas, frontmatter validation, and compliance checking.

## Entry Type Selection

**Decision criteria:**
- **Problem resolution?** → Troubleshooting
- **New knowledge/skills?** → Learning  
- **External content summary?** → Article Summary
- **Otherwise** → Journal Entry

| User Intent Signals | Type | Template | Required Tag |
| -------------------- | ---- | -------- | ------------ |
| "error", "fix", "resolved", "incident" | Troubleshooting | `troubleshooting.yaml` | `troubleshooting` |
| "learned", "tutorial", "discovered" | Learning | `learning.yaml` | `learning` |
| URL/source, "read", "watched", "summarize" | Article Summary | `article-summary.yaml` | `article`/`video`/`podcast`/`talk` |
| General documentation, investigation | Journal Entry | `journal-entry.yaml` | (flexible) |

## Template Schema System

**MANDATORY - READ BEFORE PROCEEDING:**

Before generating any entry, you MUST read the complete template schema file:

```bash
# Based on entry type selected, read ENTIRE file:
skills/journal-entry-creator/template/troubleshooting.yaml
skills/journal-entry-creator/template/learning.yaml
skills/journal-entry-creator/template/article-summary.yaml
skills/journal-entry-creator/template/journal-entry.yaml
```

**Do NOT generate entries without loading the schema first.** The schema defines required sections, frontmatter fields, heading hierarchy, and validation rules.

**When to load references:**

- Load schema: ALWAYS before creating entry (mandatory)
- Load `compliance.md`: Only if validation fails and you need detailed rules
- Load `example-*.md`: Only if user asks for examples or you need clarification on structure
- Do NOT load: `journal-command.md` (superseded by this skill)

## Domain-Specific Compliance Rules

Beyond standard markdown, this journal system enforces:

### Date Consistency (Triple Sync)

All three must match exactly:

1. Filename: `2025-02-24-topic.md` (slug lowercase-only)
2. Frontmatter: `date: 2025-02-24`
3. H1 title: `# Topic - February 24, 2025` (Month D, YYYY format)

### Location Hierarchy

File must be in `YYYY/MM/` directory matching its date:

- `2025-02-24-*.md` → Must be in `2025/02/`
- `2025-11-05-*.md` → Must be in `2025/11/`

### Single H1 Format

Exactly ONE H1 in the entire document with precise format:

```markdown
# [Title] - [Month D, YYYY]
```

**Not allowed:**

- Multiple H1 headings
- H1 without date
- Wrong date format (YYYY-MM-DD in H1)

### Tag Consistency

Tags must:

1. Match between frontmatter array and final `## Tags` section
2. Be lowercase with hyphens (not underscores, not camelCase)
3. Include entry type tag when required (troubleshooting, learning, article/video/podcast/talk)

**Example:**

```yaml
tags:
  - troubleshooting
  - api-gateway
  - aws-lambda
```

Must match:

```markdown
## Tags

`troubleshooting` | `api-gateway` | `aws-lambda`
```

### Code Block Language Specifiers

ALL code blocks MUST have language identifiers. No bare triple backticks allowed.

**Valid:**

````markdown
```bash
git status
```
````

**Invalid:**

````markdown
```
git status
```
````

## Success Criteria & Validation Rules

Entry is complete when ALL criteria are met:

**Critical violations (NEVER):**
- Using emojis, bare code blocks (without language), or skipping heading levels
- Creating entries without reading template schema first
- Proceeding with failed validation or overwriting files without confirmation

**Triple sync validation:**
- ✅ File location: `YYYY/MM/YYYY-MM-DD-slug.md`
- ✅ YAML frontmatter with all required fields  
- ✅ Date consistency: filename = frontmatter = H1 title
- ✅ Tag consistency: frontmatter array = Tags section
- ✅ All required sections present per schema
- ✅ Validation script passes with zero errors
- ✅ Prettier formatting and markdownlint pass

## Four-Phase Workflow

### Phase 1: Interactive Gathering

**Principles (high freedom):**

- Be conversational and adaptive
- Extract meaningful keywords for slug generation
- Identify entry type from context clues
- Default to current date unless user specifies otherwise

**Key questions to ask:**

1. Topic/issue being documented
2. Entry type (or infer from context)
3. Date (default: today)
4. Type-specific context per table above

**Slug generation:** Extract 3-6 meaningful keywords, remove common words, use lowercase-only with hyphens (30-50 chars). Examples: `opencode-killed-process-fix`, `aws-bedrock-inventory`

### Phase 2: Schema Loading (MANDATORY)

**Low freedom - exact steps:**

1. Determine entry type from Phase 1
2. Read complete template schema file (troubleshooting.yaml, learning.yaml, article-summary.yaml, or journal-entry.yaml)
3. Review required sections, frontmatter fields, and structure order
4. Do NOT proceed without schema loaded

**Fallback:** If schema file missing or unreadable, STOP and report error. Do not guess structure.

### Phase 3: Generation

**Medium freedom - guided by schema:**

1. Create directory if needed: `mkdir -p YYYY/MM`
2. Generate filename: `YYYY-MM-DD-slug.md` (see slug principles above)
3. Populate YAML frontmatter per schema requirements
4. Create metadata block with bold keys
5. Fill all required sections from schema in correct order
6. Add code blocks with language specifiers
7. Create Tags section matching frontmatter
8. Write file to correct location

**Slug generation principles:**

- Extract 3-6 meaningful keywords, remove common words ("the", "a", "and", "for")
- **MUST be lowercase-only with hyphens** (NO uppercase/underscores)
- Target 30-50 characters
- Examples: `opencode-killed-process-fix`, `aws-bedrock-inventory`

### Phase 4: Validation & Formatting (LOW FREEDOM)

**Exact commands in sequence:**

```bash
# 1. Validate structure
bash skills/journal-entry-creator/scripts/validate-journal-entry.sh YYYY/MM/YYYY-MM-DD-slug.md

# 2. Format (only if validation passes)
npx prettier --write YYYY/MM/YYYY-MM-DD-slug.md

# 3. Lint and auto-fix
npx markdownlint-cli2 YYYY/MM/YYYY-MM-DD-slug.md --fix

# 4. Re-validate to confirm
bash skills/journal-entry-creator/scripts/validate-journal-entry.sh YYYY/MM/YYYY-MM-DD-slug.md
```

**If validation fails:**

1. Show specific errors to user
2. Fix automatically where safe (formatting, code block languages, heading hierarchy)
3. Ask user for clarification on content issues (missing sections, unclear context)
4. Re-run validation after fixes
5. Do NOT proceed to git commit if validation fails

## Edge Case Handling

### File Already Exists

**Detection:** Check before writing file

**Resolution:**

1. Ask user: `Entry already exists at [path]. Overwrite? (y/N)`
2. If no: Suggest alternative filename with suffix `-v2`, `-alt`, or `-revised`
3. If yes: Backup original (rename to `.bak`) then write new file

### Date Mismatch Detected

**Problem:** User says "document yesterday's work" but filename uses today's date

**Resolution:** Ask user which date to use, then update ALL locations (filename, frontmatter, H1) to match and ensure correct directory placement.

### Schema Not Found

**Problem:** Template YAML file missing or unreadable

**Resolution:**

1. STOP immediately - do not proceed
2. Error message: `Template schema [type].yaml not found. Cannot generate entry without schema definition.`
3. List available schemas in template/ directory
4. Ask user to select available type or fix schema location

**Do NOT:** Generate entry with guessed structure - this violates compliance

### Validation Fails After Generation

**Common failures and auto-fixes:**

| Error                  | Auto-fix Strategy                               |
| ---------------------- | ----------------------------------------------- |
| Missing code language  | Add `bash` or `text` based on content          |
| Multiple blank lines   | Remove with prettier                            |
| Heading hierarchy skip | Adjust levels H1→H2→H3                          |
| Tag mismatch           | Sync Tags section with frontmatter             |
| Date format wrong      | Use `Month D, YYYY` format                      |

**Manual fixes required:** Missing sections (need content), incorrect entry type, ambiguous slug.

### User Wants Custom Structure

**Problem:** User asks to deviate from template structure

**Resolution:** Explain validation requirements and offer: (A) custom content within existing sections, (B) additional sections after required ones, or (C) different entry type. Warn that removing required sections fails validation.

## Git Integration (Optional)

After successful validation, offer to commit:

```bash
git add YYYY/MM/YYYY-MM-DD-slug.md
git commit -m "Add journal entry: [Brief Description] (YYYY-MM-DD)"
```

**Commit message format:**

- Prefix: `Add journal entry:`
- Brief description (30-50 chars)
- Date in parentheses (YYYY-MM-DD)
- Example: `Add journal entry: OpenCode process fix (2025-02-24)`



## Bundled Resources

### Template Schemas (template/)

- `journal-entry.yaml` - General purpose entries
- `troubleshooting.yaml` - Problem resolution sessions
- `learning.yaml` - Knowledge acquisition documentation
- `article-summary.yaml` - External content summaries

Load with relative paths: `skills/journal-entry-creator/template/[file]`

### Scripts (scripts/)

- `validate-journal-entry.sh` - Compliance validation (run before commit)

### Checklists (checklist/)

- `compliance.md` - Detailed validation rules (load only if validation fails)

### References (references/)

- `example-journal-entry.md` - Real entry example (load only if user asks)
- `example-with-frontmatter.md` - Frontmatter example (load only if needed)
- `journal-command.md` - Legacy workflow (superseded, do not use)
