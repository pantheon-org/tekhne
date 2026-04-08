# ANALYSIS-tool template — section overview

Used when promoting a tool from REVIEWED.md to a standalone deep dive (`ANALYSIS-<slug>.md`).

## Frontmatter fields

```yaml
title: "Analysis — <Tool Name>"
date: YYYY-MM-DD
type: analysis
tool:
  name: "<Tool name>"
  repo: "<GitHub URL>"
  version: "<vX.Y.Z / commit SHA>"
  language: "<Python / TypeScript / Rust / etc>"
  license: "<license>"
source:
  - "references/<tool-reference-summary>.md"
  - "tools/<repo-name>/"
```

## Body structure (3 stages)

### Stage 1 — Descriptive
1.1 Problem it solves  
1.2 Architecture overview  
1.3 Context representation (storage primitives)  
1.4 Injection mechanism (what gets injected, priority logic, system prompt structure)  
1.5 Compression / summarisation (trigger → method → fidelity → reversibility)  
1.6 Eviction / overflow handling (detection → policy → archival)  
1.7 Session lifecycle (init / per-turn / checkpoint / cross-session continuity)  
1.8 What the tool explicitly does *not* cover  

### Stage 2 — Evaluative
2.1 Self-reported metrics (as reported)  
2.2 Independent verification (code audit / benchmark cross-check)  
2.3 Strengths  
2.4 Limitations / risks / missing details  
2.5 Operational complexity (deployment dependencies, failure modes, observability)  

### Stage 3 — Synthesis hooks
3.1 Comparison to adjacent tools  
3.2 Mapping to repo themes (compression / tiered loading / token budgeting / injection)  
3.3 Concrete takeaways (actionable)  

## Scope fit reminder

Always record scope fit in the analysis: in scope / borderline / out of scope — with reasoning.
