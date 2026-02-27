---
category: framework
priority: CRITICAL
source: evaluation criteria analysis
---

# Detailed Anti-Patterns for Skill Quality

Critical failure modes to avoid when evaluating and improving skill quality.

## Core Anti-Patterns

### NEVER skip baseline comparison in recurring audits

- **WHY**: score changes are meaningless without prior reference points.
- **BAD**: run ad hoc audits with no previous report linkage.
- **GOOD**: compare current results to previous dated audits.

### NEVER aggregate low-similarity skills

- **WHY**: merging unrelated skills harms discoverability and intent routing.
- **BAD**: aggregate different domains with weak overlap.
- **GOOD**: aggregate only when similarity and domain fit are clear.

### NEVER ship remediation plans without validation checks

- **WHY**: invalid or incomplete plans create execution drift.
- **BAD**: write plan and execute blindly.
- **GOOD**: validate schema/format and ensure deterministic success criteria.

### NEVER ignore Knowledge Delta scoring when it's below 15/20

- **WHY**: Knowledge Delta is the highest-weighted dimension and signals expert-only content gaps.
- **BAD**: accept scores of 10-14 without investigation.
- **GOOD**: prioritize Knowledge Delta improvements first, target ≥17/20 for A-grade skills.

### NEVER apply subjective scoring without deterministic checks

- **WHY**: human judgment varies and creates inconsistent audit results.
- **BAD**: rely on manual assessment for quality gates.
- **GOOD**: use automated scripts and measurable criteria for consistency.

### NEVER run evaluations without proper skill directory structure

- **WHY**: missing templates/, scripts/, or references/ directories indicate incomplete skill organization.
- **BAD**: evaluate skills with ad hoc file placement.
- **GOOD**: enforce conventional directory structure before quality assessment.

## Impact Analysis

Each anti-pattern leads to specific failure modes:

- **Baseline skipping**: Inability to measure progress over time
- **Low-similarity aggregation**: Reduced skill activation accuracy
- **Unvalidated remediation**: Implementation drift and wasted effort
- **Knowledge Delta neglect**: Generic content that doesn't add expert value
- **Subjective scoring**: Inconsistent quality gates across evaluations
- **Structure violations**: Poor maintainability and discoverability
