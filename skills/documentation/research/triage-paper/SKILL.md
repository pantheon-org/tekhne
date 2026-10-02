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

When available, prefer the `semantic-scholar` MCP (primary) and then the `google-scholar` MCP over `WebFetch` for paper discovery and metadata resolution. Fall back to `WebFetch` on the arxiv abstract page only when neither is configured or returns results. The server configuration and the order of preference are in [mcp-servers.md](references/mcp-servers.md).

## Philosophy

Triage is a quality gate, not a data-entry task. The goal is a scannable, honest record.

- **Evidence first**: quote what the paper reports; never infer or embellish claims.
- **Triage-then-promote**: every paper enters via `REVIEWED.md`; promotion to `ANALYSIS-*.md` requires a deliberate user decision, never automatic.
- **Scope over completeness**: a well-reasoned rejection is as valuable as a full summary. If the paper is tangentially related, triage it and flag it; do not silently skip it.

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

Assign one or more tags appropriate to the research domain: `survey`, `benchmark`, `empirical`, `theoretical` or `system`. Their meanings are in [reviewed-and-index-formats.md](references/reviewed-and-index-formats.md).

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

Add a row to the summary table at the top (reverse-chronological), then a detailed section below the table with the arxiv ID, authors, date, tags, a 2–3 sentence summary and the disposition `pending`. The exact formats are in [reviewed-and-index-formats.md](references/reviewed-and-index-formats.md).

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

**BAD:** Start the file with `# ANALYSIS: <slug>` followed by bold-text fields.
**GOOD:** Open with a `---` YAML frontmatter block containing all required fields before any prose.

### NEVER invent benchmark numbers

**WHY:** Fabricated metrics corrupt the research record.

**BAD:** `"Achieves 87% recall on LongMemEval."`
**GOOD:** `"Reports 87% recall on LongMemEval (as reported, Table 3)."`

### NEVER skip the duplicate check

**WHY:** Re-triaging the same paper wastes effort and creates conflicting entries.

**BAD:** Create a new file without checking REVIEWED.md.
**GOOD:** Run `grep -i "<slug>" REVIEWED.md references/REFERENCE_INDEX.md` first.

### NEVER promote without user confirmation

**WHY:** Promotion to ANALYSIS-*.md is a quality gate, not automatic.

**BAD:** Create ANALYSIS-*.md as part of triage.
**GOOD:** Triage to REVIEWED.md, then ask the user.

### NEVER infer or embellish claims the paper does not make

**WHY:** The record must show what the paper reports, so later readers can trust each statement.

**BAD:** `"The method generalises to all model sizes."` when only two sizes were tested.
**GOOD:** `"Tested on two model sizes (as reported)."`

### NEVER silently skip a tangential paper

**WHY:** A well-reasoned rejection is as valuable as a full summary, and a silent skip leaves no record.

**BAD:** Drop an adjacent paper without telling the user.
**GOOD:** Triage it, flag the scope question, and let the user decide.

### NEVER leave a template section blank

**WHY:** A blank section reads as an oversight; an explicit marker shows the section was considered.

**BAD:** An empty "Mechanism overview" for a survey.
**GOOD:** `N/A` with a one-line reason for sections that do not apply.

### NEVER state an evaluation number without marking it as reported

**WHY:** Numbers come from the authors and have not been reproduced; the marker keeps that visible.

**BAD:** `"Recall is 87%."`
**GOOD:** `"Recall is 87% (as reported)."`

### NEVER fall back to WebFetch while a configured MCP can answer

**WHY:** The MCP servers return structured metadata, whereas scraping an HTML page is brittle.

**BAD:** `WebFetch` on the arxiv abstract page with `semantic-scholar` configured.
**GOOD:** Resolve through `semantic-scholar`, then `google-scholar`, and use `WebFetch` last.

## References

- **Reference artifacts**: [YAML template](assets/templates/REFERENCE-paper.yaml) · [schema](assets/schemas/reference-paper.schema.json) · [validator](scripts/validate-reference-paper.sh)
- **Analysis artifacts**: [YAML template](assets/templates/ANALYSIS-paper.yaml) · [schema](assets/schemas/analysis-paper.schema.json) · [validator](scripts/validate-analysis-paper.sh)

## Reference Material

| Topic | Reference | When to Use |
|---|---|---|
| MCP server setup and source order | [mcp-servers.md](references/mcp-servers.md) | Resolving arxiv IDs, DOIs and metadata |
| REVIEWED.md and index formats, tags, promotion | [reviewed-and-index-formats.md](references/reviewed-and-index-formats.md) | Writing steps 3, 5, 6 and 8 |
