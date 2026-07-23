---
name: journal-entry-creator
description:
  "Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]', 'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports five entry types: general journal entries, troubleshooting sessions, learning notes, article summaries, and ticket-refinement sessions. Keywords: journal, documentation, troubleshooting, learning, article-summary, ticket-refinement, refine ticket, YAML frontmatter, template schemas, validation."
---

# Journal Entry Creator

Automate creation of structured journal entries with template schemas, frontmatter validation, and compliance checking.

## Mindset

Every entry is a durable, queryable record, not a scratch note: get the frontmatter, triple-synced dates, and structure right the first time so future search and tooling can rely on them. Know **when not to** use this skill: a throwaway note with no frontmatter needs no ceremony.

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
- **Fleshing out or amending a ticket (refinement prep, backlog grooming)?** → Ticket Refinement
- **Otherwise** → Journal Entry

| User Intent Signals | Type | Template | Required Tag |
| -------------------- | ---- | -------- | ------------ |
| "error", "fix", "resolved", "incident" | Troubleshooting | `troubleshooting.yaml` | `troubleshooting` |
| "learned", "tutorial", "discovered" | Learning | `learning.yaml` | `learning` |
| URL/source, "read", "watched", "summarize" | Article Summary | `article-summary.yaml` | `article`/`video`/`podcast`/`talk` |
| "refine", "flesh out", "groom", "amend ticket" | Ticket Refinement | `ticket-refinement.yaml` | `ticket-refinement` |
| General documentation, investigation | Journal Entry | `journal-entry.yaml` | (flexible) |

**Trade-off:** When intent is ambiguous, prefer the more specific type (Troubleshooting > Ticket Refinement > Learning > General).

## Template Schema System

**MANDATORY - READ BEFORE PROCEEDING:**

Before generating any entry, you MUST read the complete template schema file:

```bash
# Based on entry type selected, read ENTIRE file:
skills/journal-entry-creator/assets/templates/troubleshooting.yaml
skills/journal-entry-creator/assets/templates/learning.yaml
skills/journal-entry-creator/assets/templates/article-summary.yaml
skills/journal-entry-creator/assets/templates/ticket-refinement.yaml
skills/journal-entry-creator/assets/templates/journal-entry.yaml
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
4. Prefer the canonical vocabulary defined in the taxonomy (see below)

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

### Tag Taxonomy (Controlled Vocabulary)

Beyond tag *shape*, this system maintains a controlled tag *vocabulary* to keep
the corpus queryable and prevent tag sprawl. The vocabulary lives in a
`taxonomy.json` at the journal root (a generic default ships with the skill under
`assets/taxonomy.default.json`; its shape is documented in
`assets/schemas/taxonomy.schema.json`).

The taxonomy defines:

- `facets`: named groups of canonical tags (for example `type`, `tech`, `topic`).
- `aliases`: non-canonical spellings that collapse onto a canonical one (for
  example `teams` becomes `ms-teams`).
- `threshold`: how many times an unfaceted tag may appear before it is flagged.
- `ticketPattern`: the regex that marks issue-tracker keys, which are exempt.

When choosing tags for an entry, prefer a tag already listed in a facet, and
prefer a canonical spelling over a near-duplicate. Corpus-wide tag hygiene is
checked separately from single-entry validation by the CLI:

```bash
journal lint            # advisory: alias suggestions and unfaceted tags
journal lint --strict   # non-zero exit on findings (for CI)
```

`lint` is advisory and never rewrites the taxonomy or an entry; it reports
candidates for a human to fold into `taxonomy.json`.

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

### Assets / Screenshots Convention

Each entry that includes screenshots or attachments MUST use an **entry-specific sibling directory**:

```text
YYYY/MM/YYYY-MM-DD-slug.md          ← entry file
YYYY/MM/YYYY-MM-DD-slug/assets/     ← entry assets (gitignored, local-only)
```

Reference assets in markdown with a relative path from the entry file:

```markdown
![Alt text](YYYY-MM-DD-slug/assets/NN-description.png)
```

**Why:** A shared `screenshots/` or `assets/` directory at the month level causes filename collisions
when multiple entries use the same numbering scheme (e.g. `01-cloudwatch-alarm.png`). Scoping assets
under the entry slug directory makes every path unique.

**Note:** The `assets/` directories are gitignored — screenshots are local-only. The markdown image
references are tracked in git as documentation of what evidence was captured.

### Proposed Ticket Description (Ticket-Refinement Entries)

Applies to ticket-refinement sessions: fleshing out or amending an issue-tracker ticket (refinement prep, backlog grooming, turning a one-line ticket into a refinement-ready one). Use the `ticket-refinement.yaml` type.

**HARD RULE: this skill NEVER edits the ticket directly.** The amended ticket content lives inside the journal entry as a ready-to-paste markdown block. Applying it to the tracker is a separate step the user explicitly confirms, performed outside this skill.

When an entry refines a ticket, you MUST:

1. Set `refinement_ticket: <KEY>` in the frontmatter (e.g. `refinement_ticket: TICKET-123`). Use this field, not `jira_ticket` — the deliverable is a ticket description, not a comment.
2. Add a `## Proposed Ticket Description` section holding the full amended description inside a fenced markdown block:

````markdown
## Proposed Ticket Description

Draft for TICKET-123 - review before applying; not yet applied to the ticket.

```markdown
**Summary:** <one-line summary>

**Background**

<full, self-contained amended ticket description>
```
````

Rules for the proposed description:

- It is a DRAFT and has NOT been applied. Lead with a `Draft for [TICKET] - review before applying; not yet applied.` line, and never write it to the tracker from this skill.
- It MUST sit inside a fenced code block with a language specifier so it copies verbatim and its inner headings do not become document headings (this keeps the single-H1 rule intact). Use a 4-backtick outer fence when the ticket content itself contains 3-backtick code blocks.
- It must be self-contained: a ticket reader has not seen the journal entry, so the description stands on its own.
- Follow your team's ticket-writing standard if one exists.
- If the work touches regulated or personal data, state the constraint and route final sign-off to the appropriate function.

The validator (`validate-journal-entry.sh`) enforces this: when `refinement_ticket` is present in frontmatter, a `## Proposed Ticket Description` section is required. It is a no-op when the field is absent, so other entries are unaffected.

## Success Criteria & Validation Rules

Entry is complete when ALL criteria are met:

**Critical violations (NEVER):**
- Using emojis, bare code blocks (without language), or skipping heading levels
- Creating entries without reading template schema first
- Proceeding with failed validation or overwriting files without confirmation

**Triple sync validation:**
- ✅ File location: `YYYY/MM/YYYY-MM-DD-slug.md` (or `YYYY/MM/YYYY-MM-DD-JIRA-TICKET-slug.md` for troubleshooting with ticket)
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
2. Generate filename using slug principles below:
   - Troubleshooting with Jira ticket: `YYYY-MM-DD-JIRA-TICKET-slug.md` (e.g. `2026-04-07-proj-1234-verify-details-alarm.md`)
   - All other entries: `YYYY-MM-DD-slug.md`
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
- **Troubleshooting + Jira ticket:** prefix slug with the ticket ID in lowercase: `YYYY-MM-DD-proj-1234-slug.md`
- **Do NOT include the Jira ticket again in the slug** — it appears once as the prefix only
- Examples: `2026-04-07-proj-1234-verify-details-alarm`, `opencode-killed-process-fix`, `aws-bedrock-inventory`

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

## Anti-Patterns

### NEVER create entries without reading the template schema first

- **WHY**: guessing structure leads to validation failures and missing required sections.
- **BAD**: immediately write journal entry based on assumptions about structure.
- **GOOD**: `cat skills/journal-entry-creator/assets/templates/troubleshooting.yaml` first, review required fields, then generate.

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

### NEVER edit the ticket directly when refining it

- **WHY**: this skill produces a reviewable draft; applying changes to the tracker is a separate, user-confirmed step outside the skill.
- **BAD**: refine a ticket and push the edit straight to the issue tracker.
- **GOOD**: set `refinement_ticket` and put the amended content in a `## Proposed Ticket Description` fenced block; the user applies it separately.

## References

### Template Schemas (assets/templates/)

- `journal-entry.yaml` - General purpose entries
- `troubleshooting.yaml` - Problem resolution sessions
- `learning.yaml` - Knowledge acquisition documentation
- `article-summary.yaml` - External content summaries
- `ticket-refinement.yaml` - Issue-tracker ticket refinement (amended ticket captured in-entry, never written to the tracker)

Load with relative paths: `skills/journal-entry-creator/assets/templates/[file]`

### Scripts (scripts/)

- `validate-journal-entry.sh` - Compliance validation (run before commit)

### References (references/)

- `compliance.md` - Detailed validation rules (load only if validation fails)
- `edge-cases.md` - Detailed edge case resolution strategies (load only for complex scenarios)
- `example-journal-entry.md` - Real entry example (load only if user asks)
- `example-with-frontmatter.md` - Frontmatter example (load only if needed)
- `journal-command.md` - Legacy workflow (superseded, do not use)
