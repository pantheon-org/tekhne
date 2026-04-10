# Scenario 01: Triage Paper — Prompt Compression

## User Prompt

Triage the following paper into the repo:

- **arxiv ID**: 2310.05736
- **Title**: LLMLingua: Compressing Prompts for Accelerated Inference of Large Language Models
- **Authors**: Huiqiang Jiang, Qianhui Wu, Chin-Yew Lin, Yuqing Yang, Lili Qiu

Add it to the research collection following the standard triage process. The paper is not currently in the repo.

## Expected Behavior

1. Search `REVIEWED.md` and/or `references/REFERENCE_INDEX.md` for the arxiv ID or a matching slug before creating any new file
2. Create `references/<slug>.md` (e.g. `references/jiang-llmlingua.md`) using the `<firstauthor-surname>-<topic>` slug convention
3. Include a TL;DR section with 3 to 8 bullet points summarising the paper
4. Mark all benchmark metrics and evaluation results with `(as reported)` — no metric stated as a verified fact
5. Populate the mechanism overview to describe the compression/injection path; mark inapplicable sections as N/A
6. Include at least one substantive open question — not a placeholder
7. Append a new row to the REVIEWED.md summary table with at minimum: date, slug, type (paper), disposition (pending), and a one-line description
8. Add a detailed section below the REVIEWED.md summary table with arxiv ID, authors, date, tags, a 2–3 sentence summary, and disposition set to pending
9. Add a new row to the appropriate topic category table in `references/REFERENCE_INDEX.md` (under Compression & summarisation or closest matching category)
10. Refrain from creating any `ANALYSIS-*.md` file during triage
11. Conclude by offering the user a promotion choice: promote to a standalone ANALYSIS file, keep in REVIEWED.md, or skip

## Success Criteria

- **Duplicate check performed before any file is created**: The agent searches REVIEWED.md and/or references/REFERENCE_INDEX.md for the arxiv ID or a matching slug before writing any new file.
- **references/<slug>.md is created**: A new file is created at references/<slug>.md (e.g. references/jiang-llmlingua.md or similar slug). The file must exist in the output.
- **Slug follows the <firstauthor-surname>-<topic> convention**: The slug starts with the first author's surname (jiang) and includes a short topic descriptor (e.g. jiang-llmlingua or jiang-prompt-compression).
- **TL;DR section contains 3 to 8 bullets**: The TL;DR section in references/<slug>.md has between 3 and 8 bullet points.
- **Evaluation results marked as reported**: Any benchmark metrics or evaluation results include '(as reported)' or equivalent qualifier. No metric is stated as a verified fact.
- **Mechanism overview covers compression path**: The mechanism overview describes how the tool compresses prompts. Inapplicable sections are marked N/A rather than left blank.
- **Open questions section has substantive content**: The open questions section contains at least one meaningful gap, risk, or unverified claim — not just a placeholder.
- **REVIEWED.md summary table row is added**: A new row is appended with at minimum: date, slug, type (paper), disposition (pending), and a one-line description.
- **REVIEWED.md detailed section is added**: A detailed section is added with arxiv ID, authors, date, tags, a 2–3 sentence summary, and disposition set to pending.
- **REFERENCE_INDEX.md is updated**: A new row is added under Compression & summarisation or the closest matching category.
- **No ANALYSIS-*.md file is created**: The agent does not create any file matching ANALYSIS-*.md during triage.
- **User is offered a promotion choice at the end**: The agent asks the user whether to promote the paper to a standalone ANALYSIS file, keep it in REVIEWED.md, or skip it.

## Failure Conditions

- Agent creates files without first checking for a duplicate
- Agent creates the reference file at the wrong path or with a non-conforming slug
- Agent writes fewer than 3 or more than 8 TL;DR bullets
- Agent states evaluation metrics as verified facts without `(as reported)` qualifiers
- Agent leaves mechanism overview sections blank instead of marking N/A
- Agent leaves the open questions section as a placeholder with no substantive content
- Agent omits the REVIEWED.md summary table row or required fields
- Agent omits the REVIEWED.md detailed section or required fields
- Agent omits the REFERENCE_INDEX.md update or places the entry in the wrong category
- Agent creates an `ANALYSIS-*.md` file automatically during triage
- Agent does not offer the user a promotion choice at the end
