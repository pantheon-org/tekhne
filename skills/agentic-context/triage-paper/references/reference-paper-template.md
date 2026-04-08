---
title: "<Full paper title>"
author: "<First author> et al."
date: YYYY-MM-DD
type: reference
tags:
  - paper
  - context-management
  - <compression|tiered-loading|token-budgeting|injection|benchmark|survey>
source: "<primary landing page (arXiv abs / publisher DOI)>"
source_alt: "<PDF link>"
version: "<arXiv vN / camera-ready / etc>"
context: "<why we care; what synthesis bucket this supports>"
related:
  - "../ANALYSIS-<paper-analysis-file>.md"
files:
  - "papers/<paper-id>.pdf"
  - "papers/<paper-id>.md  # optional extracted text snapshot"
---

## TL;DR (3–8 bullets)

-
-
-

## What's novel / different

What does this do that adjacent work does not?

## Mechanism overview

### Context representation

How is the context window modeled? (token stream, structured slots, tiered segments, etc.)

### Compression / injection path

How is content reduced or selected before injection?

### Eviction / truncation policy

What happens when the budget is exceeded?

### Token budget model

Hard caps, soft priorities, dynamic adjustment?

## Evaluation (as reported)

- Benchmark(s):
- Baseline(s):
- Key metric(s):
- Result(s): (as reported)

## Implementation details worth stealing

-

## Open questions / risks / missing details

-

## Notes

Corrections, updates, follow-up pointers.
