# Scenario 02: Triage Paper — Duplicate Detection

## User Prompt

You are working in the research repository.

Current state of REVIEWED.md (excerpt):

```markdown
## Summary table

| Date | Name | Type | Disposition | Notes |
|---|---|---|---|---|
| 2026-03-15 | jiang-llmlingua | paper | promoted | Prompt compression via token importance scoring |
| 2026-03-10 | press-longchat | paper | pending | Long-context evaluation framework |

## jiang-llmlingua — LLMLingua: Compressing Prompts for Accelerated Inference of Large Language Models

- **arxiv**: 2310.05736
- **Authors**: Huiqiang Jiang et al.
- **Date**: 2023-10-09
- **Tags**: compression
- **Summary**: LLMLingua uses a small LLM to score token importance and compress prompts by up to 20x while preserving task performance. Evaluated on GSM8K, BBH, and ShareGPT.
- **Disposition**: promoted — standalone ANALYSIS-arxiv-2310.05736-jiang-llmlingua.md created
```

A colleague has asked you to add this paper to the repo:

- **arxiv ID**: 2310.05736
- **Title**: LLMLingua: Compressing Prompts for Accelerated Inference of Large Language Models

## Expected Behavior

1. Check `REVIEWED.md` for arxiv ID `2310.05736` or slug `jiang-llmlingua` before attempting to write any new file
2. Check `references/REFERENCE_INDEX.md` as part of the duplicate detection step
3. Identify that arxiv ID `2310.05736` already exists in REVIEWED.md and reference the existing entry (`jiang-llmlingua`)
4. Halt the workflow — do not create a new `references/<slug>.md` file
5. Do not append a new row to the REVIEWED.md summary table
6. Do not add a new detailed section to REVIEWED.md
7. Do not add a new row to `references/REFERENCE_INDEX.md`
8. Report the duplicate clearly to the user, including the slug or section name and its location

## Success Criteria

- **REVIEWED.md is searched before any file is created**: The agent checks REVIEWED.md for the arxiv ID (2310.05736) or slug (jiang-llmlingua) before attempting to write any new file.
- **references/REFERENCE_INDEX.md is also checked**: The agent checks REFERENCE_INDEX.md in addition to REVIEWED.md as part of the duplicate detection step.
- **Duplicate match is identified and the existing entry is cited**: The agent explicitly identifies that arxiv ID 2310.05736 already exists in REVIEWED.md and references the existing entry (jiang-llmlingua or the section heading).
- **No new references/<slug>.md file is created**: The agent does not create any new file under references/ for this paper.
- **No new row is added to the REVIEWED.md summary table**: The REVIEWED.md summary table is not modified. No new row is appended.
- **No new detailed section is added to REVIEWED.md**: The REVIEWED.md body is not modified. No new detailed section is written for this paper.
- **No new row is added to REFERENCE_INDEX.md**: references/REFERENCE_INDEX.md is not modified. No new entry is added for this paper.
- **The duplicate is reported clearly to the user**: The agent communicates that this paper has already been triaged, including at minimum the slug or section name where it can be found.
- **Path or location of the existing entry is provided**: The agent tells the user where the existing entry is (e.g. REVIEWED.md section 'jiang-llmlingua' or references/jiang-llmlingua.md).
- **Workflow halts after duplicate detection without requesting further action**: The agent does not continue with the triage workflow steps after finding the duplicate.

## Failure Conditions

- Agent proceeds to create files without checking for a duplicate first
- Agent checks only one of REVIEWED.md or REFERENCE_INDEX.md
- Agent fails to identify the existing entry or does not cite it
- Agent creates a new `references/<slug>.md` file despite the duplicate existing
- Agent appends a new row to the REVIEWED.md summary table
- Agent adds a new detailed section to REVIEWED.md
- Agent adds a new entry to REFERENCE_INDEX.md
- Agent reports the duplicate without specifying the slug or location of the existing entry
- Agent continues the triage workflow after finding the duplicate
