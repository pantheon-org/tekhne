---
name: journal-entry-creator
description:
  "Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]',
  'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article
  summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation."
---

# Journal Entry Creator

Automate creation of structured journal entries with template schemas, frontmatter validation, and compliance checking.

## Entry Type Decision Framework

**Before selecting entry type, ask yourself:**

- **Is this documenting a problem and its resolution?** → Troubleshooting
- **Is this capturing new knowledge or skills acquired?** → Learning
- **Is this summarizing external content (article/video/podcast)?** → Article Summary
- **Otherwise (general documentation, session notes)?** → Journal Entry

## Entry Type Selection Matrix

| User Intent Signals                                        | Entry Type      | Template Schema        | Required Tag                             |
| ---------------------------------------------------------- | --------------- | ---------------------- | ---------------------------------------- |
| "error", "fix", "broke", "resolved", "incident"            | Troubleshooting | `troubleshooting.yaml` | `troubleshooting`                        |
| "learned", "studied", "discovered", "explored", "tutorial" | Learning        | `learning.yaml`        | `learning`                               |
| Has URL/source, "read", "watched", "listened", "summarize" | Article Summary | `article-summary.yaml` | `article`, `video`, `podcast`, or `talk` |
| General documentation, session notes, investigation        | Journal Entry   | `journal-entry.yaml`   | (flexible)                               |

**Trade-off:** When intent is ambiguous, prefer more specific type (Troubleshooting > Learning > General). Specific templates provide better structure and future searchability.

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

1. Filename: `2025-02-24-topic.md` (slug must be lowercase-only)
2. Frontmatter: `date: 2025-02-24`
3. H1 title: `# Topic - February 24, 2025`

**Critical:**

- The H1 date format is `Month D, YYYY` (not `Month DD`). Example: `February 3, 2025` not `February 03, 2025`.
- The filename slug (after date) must be **lowercase-only** with hyphens. NO uppercase letters allowed.

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

## NEVER Do

**Content and formatting violations:**

- NEVER use emojis in journal entries (status, sections, or content)
- NEVER create metadata block without bold formatting (`**Key:** Value`)
- NEVER use bullet points in metadata blocks (use `**Key:** Value` format only)
- NEVER skip heading levels (H1 → H3 without H2)

**Process violations:**

- NEVER skip reading the template schema before generating entry
- NEVER create entries without running validation afterward
- NEVER commit without running prettier and markdownlint first
- NEVER proceed if validation fails — fix issues first
- NEVER overwrite existing files without asking user

## Four-Phase Workflow

### Phase 1: Interactive Gathering

**Principles (high freedom):**

- Be conversational and adaptive
- Extract meaningful keywords for slug generation
- Identify entry type from context clues
- Default to current date unless user specifies otherwise

**Key questions to ask:**

1. Topic/issue being documented
2. Entry type (or infer from topic)
3. Date (default: today)
4. Type-specific context (problem details, learning source, article URL, etc.)

**Edge case:** If user provides topic like "OpenCode killed process fix" - identify as Troubleshooting based on "fix" keyword.

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
2. Generate filename: `YYYY-MM-DD-slug.md` (slug must be lowercase with hyphens only)
3. Populate YAML frontmatter per schema requirements
4. Create metadata block with bold keys
5. Fill all required sections from schema in correct order
6. Add code blocks with language specifiers
7. Create Tags section matching frontmatter
8. Write file to correct location

**Slug generation principles:**

- Extract 3-6 meaningful keywords from topic
- Remove common words ("the", "a", "and", "for")
- **MUST be lowercase-only with hyphens** (NO uppercase letters, NO underscores)
- Target 30-50 characters
- Examples: `opencode-killed-process-fix`, `aws-bedrock-inventory`, `foss-enterprise-evaluation`

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

**Resolution:**

1. Ask user: `Use date [yesterday] or [today] for this entry?`
2. Update ALL three locations (filename, frontmatter, H1) to match
3. Ensure file location directory matches date (YYYY/MM/)

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
| Missing code language  | Add `bash` or `text` specifier based on content |
| Multiple blank lines   | Remove extras with prettier                     |
| Heading hierarchy skip | Adjust heading levels to follow H1→H2→H3        |
| Tag mismatch           | Sync Tags section with frontmatter array        |
| Date format wrong      | Reformat H1 date to `Month D, YYYY`             |

**Cannot auto-fix (ask user):**

- Missing required sections (need content)
- Incorrect entry type (need confirmation to change)
- Ambiguous slug (need better keywords)

### User Wants Custom Structure

**Problem:** User asks to deviate from template (add/remove sections, change order)

**Resolution:**

1. Explain: `This journal system has validation that enforces template structure. Deviation will fail CI checks.`
2. Offer options:
   - **Option A:** Add custom content within existing sections (flexible)
   - **Option B:** Add custom sections AFTER all required sections (acceptable)
   - **Option C:** Use different entry type that fits better
3. Warn: `Removing required sections will fail validation and block commits`

**Trade-off:** Consistency and searchability vs flexibility. This system prioritizes consistency.

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

## Success Criteria

Entry is complete when ALL of the following are true:

- ✅ Template schema was read before generation
- ✅ File created in correct location: `YYYY/MM/YYYY-MM-DD-slug.md`
- ✅ YAML frontmatter populated with all required fields
- ✅ All required sections present in schema order
- ✅ Triple date sync: filename = frontmatter = H1
- ✅ Tags consistent: frontmatter array = Tags section
- ✅ All code blocks have language specifiers
- ✅ Validation script passes with zero errors
- ✅ Prettier formatting applied
- ✅ Markdownlint passes with zero errors
- ✅ User confirms entry is ready

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
