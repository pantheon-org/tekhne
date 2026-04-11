# Challenge — Reference

## Pattern Catalog (v1: 8 patterns)

| # | Pattern | Subcommand | Error Type | Source | Mechanism |
|---|---------|-----------|------------|--------|-----------|
| 1 | Gatekeeper | anchor | Premature commitment | Florian WP01 §4.5 | Demand explicit pass/fail criteria for the decision BEFORE accepting it. Ask: "What must be true for this to be correct?" then check which criteria remain unverified. Blocks premature acceptance. |
| 2 | Reset | anchor | Premature commitment | Florian WP01 §4.5 | Discard the current solution entirely. Re-read ONLY the original problem. Generate a fresh answer from first principles. Compare with original — divergences reveal anchoring. |
| 3 | Alternative Approaches | anchor | Premature commitment | White et al. 2023 (Vanderbilt) | Force generation of ≥2 genuine alternatives (different mechanisms, not strawmen). For each: "Why was this NOT chosen?" — if no reason exists, the original wasn't properly evaluated. |
| 4 | Pre-mortem | anchor | Premature commitment | Klein 2007; EMNLP 2024 (Wang) | Assume the decision was implemented and FAILED. Generate 3 failure scenarios (most likely, second-order, black swan). For each: identify early warning signs and mitigation actions available NOW. |
| 5 | Proof Demand | verify | Factual errors | Florian WP01 §4.5 | Extract every factual claim. For each, classify: ✅ Sourced (citation exists), ⚠️ Unsourced (stated as fact without evidence), ❌ Contradicted (evidence disagrees). Flag ⚠️/❌ as untrustworthy until verified. |
| 6 | CoVe | verify | Factual errors | Dhuliawala et al. 2023 (Meta) | 4-step Chain-of-Verification: (1) state the claim, (2) write specific verification questions, (3) answer each question INDEPENDENTLY without looking at the original claim, (4) compare independent answers to original — discrepancies = errors. |
| 7 | Fact Check List | verify | Factual errors | White et al. 2023 (Vanderbilt) | Decompose response into atomic checkable assertions (one fact per line). Rate confidence: High/Medium/Low/Unknown. For Low/Unknown: write a concrete verification action (search query, doc to read, test to run). Priority-order by impact × uncertainty. |
| 8 | Socratic | framing | Framing errors | Chang 2023 | 6-stage questioning: (1) Definition — what do key terms actually mean? (2) Elenchus — is the definition consistent with usage? (3) Dialectic — what's the opposite position? (4) Maieutics — what's the real goal, stripped of framing? (5) Generalization — local issue or systemic pattern? (6) Counterfactual — if the problem didn't exist, what changes? |
| 9 | Steelman | framing | Framing errors | Rationalist tradition | Build the STRONGEST possible counter-argument to the current framing (opposite of strawman). Use best available evidence for the counter-position. Assume it's correct — what would that imply? Stress-test: "What must be true for the steelman to be wrong?" If no clear answer, the framing is weak. |

## When-To-Use Guide

| Symptom | Error Type | Subcommand |
|---------|-----------|-----------|
| AI picked one solution without exploring options | Anchoring bias | `/challenge anchor` |
| AI committed to approach too quickly | Premature commitment | `/challenge anchor` |
| Specific facts, numbers, or claims feel uncertain | Factual error | `/challenge verify` |
| AI cited something that can't be verified | Hallucination | `/challenge verify` |
| The answer is correct but might be solving the wrong problem | Framing error | `/challenge framing` |
| The question itself seems wrong | Wrong problem | `/challenge framing` |
| High-stakes decision, want genuine debiasing in fresh context | All error types | `/challenge deep` |

## Challenge Report Format

All subcommand outputs MUST follow this structure:

```markdown
## Challenge Report: [Pattern(s) Applied]

**Target**: [what was challenged]
**Error type**: [anchoring | factual | framing | high-stakes]

### Technique Selection

- **Family**: [anchor (premature commitment) | verify (factual errors) | framing (wrong problem)]
- **Patterns applied**: [named patterns, e.g., Gatekeeper, Pre-mortem, CoVe]
- **Why these patterns**: [what about the target triggered this selection — specific observations]
- **Patterns considered but skipped**: [and why, or "none — full protocol applied"]

### Findings

[Pattern-specific structured output — see protocol files.
For each finding include: Observation → Technique (family + pattern) → Reasoning → Confidence]

### Verdict

- **Assessment**: [Decision holds / Needs revision / Needs rejection]
- **Confidence**: [High / Medium / Low]
- **What would flip this**: [specific evidence or condition that would change the verdict]
- **Strongest counter to this verdict**: [steelman the opposite conclusion]

### Recommended Action

[Proceed as-is | Proceed with modifications: X | Reconsider: Y]
```

## Sources

- Florian WP01 §4.5 — Gatekeeper, Proof Demand, Reset (practitioner patterns)
- White et al. 2023 — Vanderbilt Prompt Pattern Catalog ([arxiv.org/abs/2302.11382](https://arxiv.org/abs/2302.11382))
- Dhuliawala et al. 2023 — Chain-of-Verification (CoVe), Meta AI ([arxiv.org/abs/2309.11495](https://arxiv.org/abs/2309.11495))
- Wang et al. 2024 — Devil's Advocate, EMNLP ([arxiv.org/abs/2405.16334](https://arxiv.org/abs/2405.16334))
- Chang 2023 — Socratic Method prompting ([arxiv.org/abs/2303.08769](https://arxiv.org/abs/2303.08769))
- Klein 2007 — Pre-mortem analysis (Harvard Business Review)
