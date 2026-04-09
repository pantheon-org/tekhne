---
name: triage-paper
description: "Triage an academic paper into a structured reference summary for the research repo. Use when given an arxiv ID, arxiv URL, DOI, or paper PDF. Creates references/{slug}.md; adds REVIEWED.md entry and REFERENCE_INDEX.md row. Triggers: triage paper, add paper, analyse paper, review paper, literature review, survey paper, benchmark paper, deep dive paper, arxiv, DOI."
allowed-tools: Read, Write, Edit, Bash, WebFetch
---

# Triage Paper

Add a new academic paper to the research repo as a structured reference summary.

## When to Use

- User provides an arxiv ID (e.g. `2310.08560`), arxiv URL, or paper PDF path
- User says "triage this paper", "add this paper", or "analyse this paper"
- Evaluating whether a paper belongs in the repo

## When Not to Use

- The paper has already been triaged (check `REVIEWED.md` first)
- The paper is clearly out of scope (not related to the research domain)
- User wants a full deep-dive analysis — use `triage-paper` first, then promote to `ANALYSIS-*.md`

## Recommended MCP Servers

When available, prefer these MCPs over `WebFetch` for paper discovery and metadata resolution — they return structured data and avoid HTML scraping.

```json
{
  "mcpServers": {
    "semantic-scholar": {
      "type": "stdio",
      "command": "uvx",
      "args": ["semantic-scholar-fastmcp"]
    },
    "google-scholar": {
      "type": "stdio",
      "command": "uvx",
      "args": ["google_scholar_mcp_server"]
    }
  }
}
```

Use `semantic-scholar` as the primary source (open, structured, covers most CS/ML papers). Fall back to `google-scholar` for papers not indexed there. Fall back to `WebFetch` (arxiv abstract page) only when neither MCP is configured or returns results.

## Mindset

Triage is a quality gate, not a data-entry task. The goal is a scannable, honest record.

1. **Evidence first**: quote what the paper reports; never infer or embellish claims.
2. **Triage-then-promote**: every paper enters via `REVIEWED.md`; promotion to `ANALYSIS-*.md` requires a deliberate user decision — never automatic.
3. **Scope over completeness**: a well-reasoned rejection is as valuable as a full summary. If the paper is tangentially related, triage it and flag it; don't silently skip it.

## Workflow

### 1. Resolve the source

- If given an arxiv ID or URL: use the `semantic-scholar` MCP to resolve metadata (title, authors, date, abstract, DOI). If not configured, fall back to `WebFetch` on the arxiv abstract page.
- If given a DOI: prefer `semantic-scholar` or `google-scholar` MCP over a raw HTTP fetch.
- If given a PDF path, read it to extract the same fields.
- Derive a stable slug: `<firstauthor-surname>-<2-3-word-topic>` (e.g. `jiang-llmlingua`, `press-longchat`).

### 2. Check for duplicates

- Search `REVIEWED.md` and `references/REFERENCE_INDEX.md` for the slug or arxiv ID.
- If already present, report it and stop.

### 3. Classify the paper

Assign one or more tags appropriate to the research domain. Universal tags:

| Tag | Meaning |
|---|---|
| `survey` | Overview paper covering the topic broadly |
| `benchmark` | Evaluation dataset or framework |
| `empirical` | Study with experimental evaluation |
| `theoretical` | Formal or theoretical contribution |
| `system` | System or implementation paper |

### 4. Fill in the reference summary

Read `assets/templates/REFERENCE-paper.yaml` to get the required frontmatter fields and section structure. Create `references/<slug>.md` with a YAML frontmatter block (all `required_fields` from the template) followed by the required sections.

Populate every section:

- **TL;DR**: 3–8 bullets capturing what the paper does and why it matters.
- **What's novel**: one paragraph — what does this do that adjacent work does not?
- **Mechanism overview**: describe the core approach and methodology. Skip sections that are not applicable (mark `N/A`).
- **Evaluation**: quote benchmark name, baselines, key metric, and result — always mark `(as reported)`.
- **Implementation details worth stealing**: concrete takeaways only.
- **Open questions**: gaps, risks, unverified claims.

Keep language precise. Do not pad. Quote all numbers with their source.

### 5. Update REVIEWED.md

Add a row to the summary table at the top (reverse-chronological):

```
| <today's date> | <slug> | paper | pending | <one-line description> |
```

Add a detailed section below the table:

```markdown
## <slug> — <Full paper title>

- **arxiv**: <ID>
- **Authors**: <list>
- **Date**: <YYYY-MM-DD>
- **Tags**: <tags>
- **Summary**: <2–3 sentences>
- **Disposition**: pending — awaiting user decision on promotion
```

### 6. Update REFERENCE_INDEX.md

Add a row under the most relevant category table. If no category fits, add it to the closest one and note it for the user.

### 7. Report and offer next step

Summarise what was created. Then ask:

> This paper is now in `REVIEWED.md` as **pending**. Would you like to:
> - **Promote** it to a standalone `ANALYSIS-arxiv-<id>-<slug>.md` deep dive?
> - **Keep** it in REVIEWED.md for now?
> - **Skip** it (mark as not promoted with reasoning)?

### 8. If user confirms promotion

Read `assets/templates/ANALYSIS-paper.yaml` to get the required frontmatter fields and section structure. Create `analysis/ANALYSIS-arxiv-<id>-<slug>.md` with a YAML frontmatter block (all `required_fields` from the template) followed by the required stage sections. Update the disposition in `REVIEWED.md` from `pending` to `analysis`.

## Quick Commands

```bash
# Check for duplicate before starting
grep -i "<arxiv-id-or-slug>" REVIEWED.md references/REFERENCE_INDEX.md

# Fetch arxiv abstract (title, authors, date)
curl -s "https://arxiv.org/abs/<id>"

# Create reference file from template
cp templates/REFERENCE-paper.md references/<slug>.md

# Validate the completed file
./scripts/validate-reference-paper.sh references/<slug>.md
./scripts/validate-analysis-paper.sh ANALYSIS-arxiv-<id>-<slug>.md

# | YYYY-MM-DD | <slug> | paper | pending | <one-line description> |
```

## Anti-Patterns

### NEVER omit YAML frontmatter

**WHY:** Files without frontmatter fail schema validation and break indexing tools that rely on structured metadata.

**BAD** Start the file with `# ANALYSIS: <slug>` followed by bold-text fields. → **GOOD** Open with `---` YAML frontmatter block containing all required fields before any prose.

### NEVER invent benchmark numbers

**WHY:** Fabricated metrics corrupt the research record.

**BAD** `"Achieves 87% recall on LongMemEval."` → **GOOD** `"Reports 87% recall on LongMemEval (as reported, Table 3)."`

### NEVER skip the duplicate check

**WHY:** Re-triaging the same paper wastes effort and creates conflicting entries.

**BAD** Create new file without checking REVIEWED.md. → **GOOD** Run `grep -i "<slug>" REVIEWED.md references/REFERENCE_INDEX.md` first.

### NEVER promote without user confirmation

**WHY:** Promotion to ANALYSIS-*.md is a quality gate, not automatic.

**BAD** Create ANALYSIS-*.md as part of triage. → **GOOD** Triage to REVIEWED.md, then ask the user.

## References

- **Reference artifacts**: [YAML template](assets/templates/REFERENCE-paper.yaml) · [schema](assets/schemas/reference-paper.schema.json) · [validator](scripts/validate-reference-paper.sh)
- **Analysis artifacts**: [YAML template](assets/templates/ANALYSIS-paper.yaml) · [schema](assets/schemas/analysis-paper.schema.json) · [validator](scripts/validate-analysis-paper.sh)
