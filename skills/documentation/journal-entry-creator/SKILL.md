---
name: journal-entry-creator
description:
  "Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]',
  'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article
  summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation."
---

# Journal Entry Creator

Automate creation of structured journal entries with template schemas, frontmatter validation, and compliance checking.

## When to Use This Skill

Use journal-entry-creator when:
- **Documentation requirement**: User explicitly asks to "create journal entry", "document this", or "write about [topic]"
- **Structured output needed**: Standard journal workflows require YAML frontmatter, triple-sync dates, and template compliance
- **Multiple entry types**: Need to select between troubleshooting, learning, article summary, or general journal
- **Validation critical**: Entry must pass compliance checks before commit (automated validation available)

**Do NOT use for:**
- Quick markdown notes without frontmatter (use simple file creation instead)
- External documentation systems (Confluence, Notion) — this skill is for local .md files only
- Retrospective backdating of dozens of old entries — use batch import scripts instead

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
- Load `edge-cases.md`: Only for complex or unusual edge cases
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

**Slug generation:** Extract 3-6 meaningful keywords (see Phase 3 for detailed rules)

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
2. Generate filename: `YYYY-MM-DD-slug.md` using slug principles below
3. Populate YAML frontmatter per schema requirements
4. Create metadata block with bold keys
5. Fill all required sections from schema in correct order
6. Add code blocks with language specifiers
7. Create Tags section matching frontmatter
8. Write file to correct location

**Slug generation principles:**

- Extract 3-6 meaningful keywords from topic/title
- Remove common words ("the", "a", "and", "for", "with", "to")
- **MUST be lowercase-only with hyphens** (NO uppercase/underscores)
- Target 30-50 characters for readability
- Examples: `opencode-killed-process-fix`, `aws-bedrock-inventory`, `react-state-patterns`

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

**Common scenarios:** File exists, date mismatch, schema missing, validation failures, custom structure requests.

**Quick reference:**

- **File exists:** Ask to overwrite or suggest alternative filename
- **Date mismatch:** Confirm intended date, update all three locations
- **Schema missing:** STOP immediately, list available schemas
- **Validation fails:** Auto-fix formatting issues, ask user for content clarifications

**For detailed resolution strategies:** Load `skills/journal-entry-creator/references/edge-cases.md` only when encountering an unusual or complex edge case.

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

**Commit message format:**

- Prefix: `Add journal entry:`
- Brief description (30-50 chars)
- Date in parentheses (YYYY-MM-DD)
- Example: `Add journal entry: OpenCode process fix (2025-02-24)`

## Anti-Patterns

### NEVER create entries without reading the template schema first

- **WHY**: guessing structure leads to validation failures and missing required sections.
- **BAD**: immediately write journal entry based on assumptions about structure.
- **GOOD**: `cat skills/journal-entry-creator/template/troubleshooting.yaml` first, review required fields, then generate.

### NEVER proceed with failed validation

- **WHY**: invalid entries break parsing tools and violate compliance rules.
- **BAD**: validation script shows 3 errors → ignore and commit anyway.
- **GOOD**: fix all validation errors (or ask user for clarification), re-run validation until passing, then commit.

### NEVER use bare code blocks without language specifiers

- **WHY**: bare triple backticks fail markdownlint and reduce syntax highlighting readability.
- **BAD**: ` ```\ngit status\n``` ` (no language).
- **GOOD**: ` ```bash\ngit status\n``` ` (explicit language).

### NEVER create date mismatches between filename, frontmatter, and H1

- **WHY**: triple sync requirement ensures consistency; mismatches cause directory placement errors and broken date queries.
- **BAD**: filename `2025-02-24-*.md`, frontmatter `date: 2025-02-25`, H1 `March 1, 2025`.
- **GOOD**: all three match exactly - `2025-02-24` in filename, `date: 2025-02-24` in frontmatter, `February 24, 2025` in H1.

## References

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

- `edge-cases.md` - Detailed edge case resolution strategies (load only for complex scenarios)
- `example-journal-entry.md` - Real entry example (load only if user asks)
- `example-with-frontmatter.md` - Frontmatter example (load only if needed)
- `journal-command.md` - Legacy workflow (superseded, do not use)
