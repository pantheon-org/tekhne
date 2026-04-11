# Protocol: verify

Challenge factual claims, hallucinations, and unverified assertions.

**Patterns**: Proof Demand · CoVe · Fact Check List

---

## Execution

Apply ALL 3 patterns in sequence.

### Pattern 1: Proof Demand

*Require citations or verifiable evidence for each factual claim.*

1. Extract every factual claim from the target response (numbers, dates, attributions, capabilities, guarantees)
2. For each claim, ask: "What is the SOURCE for this?" (paper, doc, official release, empirical measurement)
3. Classify each:
   - ✅ **Sourced**: citation provided and plausible
   - ⚠️ **Unsourced**: stated as fact without evidence
   - ❌ **Contradicted**: evidence suggests it's wrong
4. Flag ⚠️ and ❌ claims as requiring verification before trusting

Record: claim inventory with source status.

### Pattern 2: CoVe (Chain-of-Verification)

*Generate verification questions, answer independently, then revise.*

4-step process (Dhuliawala et al. 2023):

**Step 1 — Baseline response**: What does the current answer claim?

**Step 2 — Plan verifications**: For each ⚠️/❌ claim from Proof Demand, write a specific verification question:
- "Is it true that [claim]?"
- "What is the actual [number/date/name/behavior]?"
- Keep questions atomic — one fact per question

**Step 3 — Answer independently**: Answer each verification question WITHOUT looking at the original response. Force independent recall or acknowledge uncertainty.

**Step 4 — Revise**: Compare independent answers to original claims. Identify discrepancies → revise the original response accordingly.

Record: verification questions, independent answers, discrepancies found.

### Pattern 3: Fact Check List

*Extract atomic claims and verify each independently.*

1. Decompose the target response into atomic, checkable assertions (one claim per line)
2. For each assertion, rate confidence: High / Medium / Low / Unknown
3. For Low/Unknown: write a concrete action to verify (search query, doc to read, test to run)
4. Priority-order the verification list by: impact × uncertainty

Record: atomic claim list with confidence ratings and verification actions.

---

## Output

```markdown
## Challenge Report: verify (Proof Demand · CoVe · Fact Check List)

**Target**: [claim or response challenged]
**Error type**: factual / hallucination

### Technique Selection

- **Family**: Verify — factual errors / hallucination
- **Patterns applied**: Proof Demand, CoVe, Fact Check List
- **Why these patterns**: [what about the target triggered factual scrutiny — e.g., specific claims, numbers cited without source, confident assertions]
- **Patterns considered but skipped**: none — full verify protocol applied

### Findings

**Proof Demand** *(verify family — classifies claims by evidence status)*
| Claim | Source Status | Reasoning | Notes |
|-------|--------------|-----------|-------|
| [claim] | ✅/⚠️/❌ | [why this classification — what evidence was checked or missing] | [source or gap] |

**CoVe Verification** *(verify family — independent re-derivation exposes confirmation bias)*
| Verification Question | Independent Answer | Discrepancy? | Reasoning |
|-----------------------|-------------------|--------------|-----------|
| [question] | [answer] | [yes/no — detail] | [what the discrepancy reveals about the original claim] |

**Fact Check List** *(verify family — priority-ranks claims by impact × uncertainty)*
| Assertion | Confidence | Reasoning | Verification Action |
|-----------|-----------|-----------|---------------------|
| [claim] | High/Med/Low | [why this confidence level] | [how to check] |

### Verdict

- **Assessment**: [Claims verified / Some claims need checking / Claims likely wrong]
- **Confidence**: [High / Medium / Low]
- **What would flip this**: [specific evidence that would change the verdict]
- **Strongest counter to this verdict**: [steelman the opposite conclusion]

### Recommended Action

[Proceed as-is | Verify before proceeding: [list] | Do not use without verification]
```
