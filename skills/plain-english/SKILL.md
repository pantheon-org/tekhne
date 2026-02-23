---
name: plain-english
description: Write technical content in plain English for non-technical stakeholders by translating jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline.
version: 1.1.0
created: 2026-01-21
---

# Plain English Writing

Navigation hub for converting technical content into decision-ready communication.

## When to Use

- You need to communicate technical risks to executives or managers.
- You need non-technical readers to make a decision quickly.
- You need to rewrite technical updates into action-oriented business language.

## When Not to Use

- The audience is purely technical and expects low-level implementation detail.
- The task is legal drafting that must preserve specialized legal terminology verbatim.

## Workflow

1. Step 1: Identify audience.
Output: explicit audience label (executive, manager, compliance/legal, cross-functional).
2. Step 2: Define desired outcome.
Output: one sentence stating what the reader should decide or do.
3. Step 3: Draft key message first.
Output: opening paragraph with problem, impact, and required action.
4. Step 4: Add supporting context only as needed.
Output: concise background, options, tradeoffs, and timeline.
5. Step 5: Run anti-pattern and readability checks.
Output: final copy with clear ownership and no unexplained jargon.

## Quick Commands

```bash
# Rough jargon density check
grep -E "(API|HTTP|SQL|async|callback|microservice|container|k8s)" <file>.md | wc -l
```

```bash
# Acronym extraction for glossary review
grep -oE '\b[A-Z]{2,}\b' <file>.md | sort -u
```

```bash
# Sentence length sanity check (approximate)
awk -F'.' '{for(i=1;i<=NF;i++) print $i}' <file>.md | awk '{print NF}' | awk '{s+=$1; n+=1} END {if(n>0) print s/n}'
```

## Verification Checklist

- [ ] Acronyms are defined on first use.
- [ ] Opening paragraph contains key decision or recommendation.
- [ ] Technical terms are translated to plain language.
- [ ] Action items include clear owner and timeline.
- [ ] Paragraphs are concise and scannable.

## Constraints vs Flexibility

### Hard Constraints

- ALWAYS define acronyms on first use.
- ALWAYS lead with key message for executive audiences.
- NEVER hide decisions deep in long paragraphs.
- NEVER use passive voice for required actions when ownership is known.

### Flexible Guidance

- Sentence length target can vary by audience depth.
- Section ordering can adapt to document type.
- Detail level can increase for technical-adjacent stakeholders.

### Fallback Paths

- Unknown audience: default to manager-level clarity.
- Unknown terminology: add glossary terms in-line or at end.
- Conflicting priorities: lead with risk and decision deadline.

## Anti-Patterns

### NEVER use jargon without translation

- **WHY**: non-technical readers cannot act on unexplained specialist terms.
- **Consequence**: decision delay and misunderstandings.
- **BAD**: "The API returns 429 due to upstream throttling."
- **GOOD**: "The system is receiving too many requests; retry windows are being enforced."

### NEVER assume prior technical context

- **WHY**: stakeholders have varying domain familiarity.
- **Consequence**: gaps in understanding and misaligned actions.
- **BAD**: "Refactor monolith into microservices."
- **GOOD**: "Break the large application into smaller independent services to reduce release risk and speed delivery."

### NEVER bury the key decision

- **WHY**: time-constrained readers scan the first section only.
- **Consequence**: critical actions are missed.
- **BAD**: recommendation appears after multiple background sections.
- **GOOD**: first paragraph states recommendation and impact.

### NEVER use passive action language for owned tasks

- **WHY**: passive voice obscures accountability.
- **Consequence**: work stalls because ownership is unclear.
- **BAD**: "The database should be optimized."
- **GOOD**: "The database team should optimize query performance by Q2."

### NEVER skip audience identification

- **WHY**: tone and depth must match reader intent.
- **Consequence**: over-technical or over-simplified communication.
- **BAD**: draft starts before audience is known.
- **GOOD**: start with "Audience: compliance managers; goal: approve mitigation plan."

## Quick Reference

- [references/audience-types.md](references/audience-types.md) - audience-specific format and depth guidance
- [references/jargon-translations.md](references/jargon-translations.md) - common technical-to-plain-language mappings

## References

- [PlainLanguage.gov](https://www.plainlanguage.gov/)
