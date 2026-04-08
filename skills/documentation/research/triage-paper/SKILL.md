---
name: triage-paper
description: "Triage an academic paper into a structured reference summary for the research repo. Use when given an arxiv ID, arxiv URL, DOI, or paper PDF to assess context management relevance. Creates references/{slug}.md; adds REVIEWED.md entry and REFERENCE_INDEX.md row. Triggers: triage paper, add paper, analyse paper, arxiv, context window, prompt compression, token budget, token reduction, context injection, context summarisation, RAG injection, eviction policy, long context, LLM context, context overflow, tiered context, context pruning, sliding window, KV cache, attention sink, survey paper, benchmark paper, deep dive paper."
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
- The paper is clearly out of scope (not related to context window management)
- User wants a full deep-dive analysis — use `triage-paper` first, then promote to `ANALYSIS-*.md`

## Mindset

Triage is a quality gate, not a data-entry task. The goal is a scannable, honest record.

1. **Evidence first**: quote what the paper reports; never infer or embellish claims.
2. **Triage-then-promote**: every paper enters via `REVIEWED.md`; promotion to `ANALYSIS-*.md` requires a deliberate user decision — never automatic.
3. **Scope over completeness**: a well-reasoned rejection is as valuable as a full summary. If the paper is tangentially related, triage it and flag it; don't silently skip it.

## Workflow

### 1. Resolve the source

- If given an arxiv ID or URL, fetch the abstract page to get title, authors, date, and abstract.
- If given a PDF path, read it to extract the same fields.
- Derive a stable slug: `<firstauthor-surname>-<2-3-word-topic>` (e.g. `jiang-llmlingua`, `press-longchat`).

### 2. Check for duplicates

- Search `REVIEWED.md` and `references/REFERENCE_INDEX.md` for the slug or arxiv ID.
- If already present, report it and stop.

### 3. Classify the paper

Assign one or more tags from this controlled list:

| Tag | Meaning |
|---|---|
| `compression` | Reduces token count (summarisation, pruning, distillation) |
| `tiered-loading` | Priority-based injection (L0/L1/L2, lazy vs eager) |
| `token-budgeting` | Hard caps, soft priorities, eviction policies |
| `injection` | How content enters the context window |
| `benchmark` | Evaluation dataset or framework |
| `survey` | Overview paper covering the topic broadly |
| `retrieval` | RAG / retrieval-augmented context |

### 4. Fill in the reference summary

Create `references/<slug>.md` using `templates/REFERENCE-paper.md`. Populate every section:

- **TL;DR**: 3–8 bullets capturing what the paper does and why it matters for context management.
- **What's novel**: one paragraph — what does this do that adjacent work does not?
- **Mechanism overview**: describe context representation, compression/injection path, eviction policy, token budget model. Skip sections that are not applicable (mark `N/A`).
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

- **Guides**: [REFERENCE-paper](references/reference-paper-guide.md) — body structure and conventions · [ANALYSIS-paper](references/analysis-paper-guide.md) — 3-stage deep-dive structure
- **Reference artifacts**: [YAML template](assets/templates/REFERENCE-paper.yaml) · [schema](assets/schemas/reference-paper.schema.json) · [validator](scripts/validate-reference-paper.sh)
- **Analysis artifacts**: [YAML template](assets/templates/ANALYSIS-paper.yaml) · [schema](assets/schemas/analysis-paper.schema.json) · [validator](scripts/validate-analysis-paper.sh)
