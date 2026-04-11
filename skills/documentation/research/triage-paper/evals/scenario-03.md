# Scenario 03: Triage Paper — Survey

## User Prompt

You are working in the research repository.

Triage the following paper:

- **arxiv ID**: 2404.13501
- **Title**: A Survey on the Memory Mechanism of Large Language Model based Agents
- **Authors**: Zeyu Zhang, Xiaohe Bo, Chen Ma, Rui Li, et al.
- **Abstract excerpt**: "This paper surveys memory mechanisms for LLM-based agents, covering definitions, motivations, design axes (sources, forms, operations), evaluation approaches, and application domains."

The paper is not currently in the repo. Add it to the research collection.

## Expected Behavior

1. Assign the `survey` tag to the paper in the reference file frontmatter and/or the REVIEWED.md entry
2. Create `references/<slug>.md` using the `<firstauthor-surname>-<topic>` convention (e.g. `zhang-survey-memory-mechanism`)
3. Write a TL;DR with 3–8 bullets describing the survey's scope, axes covered, or key findings — not just "this is a survey of X"
4. Populate the "What's novel" section to explain what this survey covers that adjacent surveys do not, or what makes its taxonomy/framing distinctive
5. Mark inapplicable mechanism overview subsections (e.g. compression path, eviction policy) as N/A rather than omitting or leaving them blank
6. Include an evaluation section noting benchmarks referenced and systems compared, with coverage statistics marked `(as reported)`
7. Include at least one substantive open question identifying a gap, limitation, or dated aspect of the survey
8. Append a row to the REVIEWED.md summary table that includes `survey` in the notes or tags field
9. Add a detailed section to REVIEWED.md with a tags field listing `survey` among the assigned tags
10. Place the new REFERENCE_INDEX.md entry under the Surveys category — not a technique-specific category
11. Refrain from creating any `ANALYSIS-*.md` file during triage
12. Conclude by offering the user a promotion choice

## Success Criteria

- **Paper classified with the survey tag**: The agent assigns the 'survey' tag to this paper in the reference file frontmatter and/or in the REVIEWED.md entry.
- **references/<slug>.md is created**: A new file is created at references/<slug>.md following the <firstauthor-surname>-<topic> convention (e.g. zhang-survey-memory-mechanism).
- **TL;DR describes what the survey synthesises, not just that it exists**: The TL;DR bullets describe the survey's scope, axes covered, or key findings — not just 'this is a survey of X'. Contains 3–8 bullets.
- **What's novel section addresses scope relative to prior surveys**: The What's novel section explains what this survey covers that adjacent surveys do not, or what makes its taxonomy/framing distinctive.
- **Mechanism overview marks inapplicable subsections as N/A**: Subsections that do not apply to a survey paper are marked N/A rather than left blank or omitted entirely.
- **Evaluation section present with appropriately scoped claims**: The evaluation section notes what the survey covers (benchmarks referenced, systems compared) and marks any coverage statistics as '(as reported)'.
- **Open questions section has substantive content**: Open questions identifies at least one gap, limitation, or dated aspect of the survey — not just a placeholder.
- **REVIEWED.md summary table row includes survey tag**: The row added includes 'survey' in the notes or tags field, or the type column reflects the classification.
- **REVIEWED.md detailed section includes tags field with survey**: The detailed section includes a tags field that lists 'survey' among the assigned tags.
- **REFERENCE_INDEX.md entry placed under Surveys category**: The new entry is placed in the Surveys section, not under a technique-specific category like Compression or Token budgeting.
- **No ANALYSIS-*.md file created during triage**: The agent does not create any ANALYSIS-*.md file as part of this triage.
- **User offered promotion choice at the end**: The agent ends by asking the user whether to promote, keep, or skip the paper.

## Failure Conditions

- Agent omits the `survey` tag from the reference file frontmatter and REVIEWED.md entry
- Agent creates the reference file at the wrong path or with a non-conforming slug
- Agent writes TL;DR bullets that only describe "this is a survey of X" without substantive synthesis
- Agent omits or provides a generic "What's novel" section without addressing scope relative to prior surveys
- Agent leaves inapplicable mechanism overview subsections blank instead of marking N/A
- Agent omits the evaluation section or states coverage statistics as facts without `(as reported)`
- Agent leaves the open questions section as a placeholder
- Agent omits `survey` from the REVIEWED.md summary table row
- Agent omits `survey` from the tags field in the REVIEWED.md detailed section
- Agent places the REFERENCE_INDEX.md entry in a technique-specific category instead of Surveys
- Agent creates an `ANALYSIS-*.md` file during triage
- Agent does not offer a promotion choice at the end
