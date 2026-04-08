# ANALYSIS-paper template — section overview

Used when promoting a paper from REVIEWED.md to a standalone deep dive (`ANALYSIS-arxiv-<id>-<slug>.md`).

## Frontmatter fields

```yaml
title: "Analysis — <Paper Short Name> (<FirstAuthorYYYY>)"
date: YYYY-MM-DD
type: analysis
paper:
  id: "arxiv:XXXX.XXXXX"
  title: "<Full paper title>"
  authors: [...]
  year: YYYY
  venue: "<arXiv / conference / journal>"
links: [...]
artifacts:
  pdf: "references/papers/<paper-id>.pdf"
  text: "references/papers/<paper-id>.md"
source:
  - "references/<paper-reference-summary>.md"
```

## Body structure (3 stages)

### Stage 1 — Descriptive
1.1 Problem statement  
1.2 Core approach  
1.3 Context representation and data model  
1.4 Injection path (inputs → selection → gating → assembly)  
1.5 Compression / summarisation path (trigger → method → fidelity → output)  
1.6 Eviction / truncation policy (overflow → eviction order → recovery)  
1.7 What the paper explicitly does *not* cover  

### Stage 2 — Evaluative
2.1 Evaluation setup (as reported)  
2.2 Main results (as reported)  
2.3 Strengths  
2.4 Limitations / open questions  
2.5 Reproducibility  

### Stage 3 — Synthesis hooks
3.1 Comparison to adjacent work  
3.2 Mapping to repo themes (compression / tiered loading / token budgeting / injection / benchmarking)  
3.3 Concrete takeaways (actionable)  
