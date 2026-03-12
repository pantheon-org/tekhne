---
name: plain-english
description: Translates technical content into plain English for non-technical stakeholders by converting jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline. Use when the user asks to simplify technical content, write an executive summary, explain something in layman's terms, rewrite for non-technical audiences, or translate jargon for management or stakeholders. Applies to document types such as technical reports, architecture decision records, incident summaries, risk assessments, and status updates. Distinct from general writing or documentation skills by its focus on audience identification, decision-first structure, and jargon elimination for non-technical readers.
metadata:
  version: "1.7.0"
  created: "2026-01-21"
---

# Plain English Writing

Translates technical content into decision-ready communication for non-technical stakeholders.

## When to Use

Apply when writing for: executives, business managers, compliance/legal, or cross-functional stakeholders without the requesting team's domain knowledge.

## When Not to Use

Skip when writing for engineers or technical specialists. You may optionally apply selective translation for mixed audiences; consider keeping technical depth for peer reviews or developer-facing docs.

## Mindset

Technical writing optimises for **completeness**. Plain-English writing optimises for **decisions**.

Before writing a single word, ask:

> _"What does this person need to decide or do — and what is the minimum information required for that?"_

**The single most reliable fix:** move the recommendation to sentence one. Everything else is secondary.

### Decision Framework

```
┌─────────────────────────────────────────────┐
│  OPENING (required)                          │
│  Problem + Business impact + Action needed   │
├─────────────────────────────────────────────┤
│  SUPPORTING CONTEXT (only if needed)         │
│  Background / Options / Tradeoffs / Timeline │
├─────────────────────────────────────────────┤
│  APPENDIX (optional)                         │
│  Technical detail / Metrics / Implementation │
└─────────────────────────────────────────────┘
```

If a reader only reads the opening, they must still know what to do.

---

## Workflow

### Step 1: Identify Audience

**executive** (zero jargon) | **manager** (minimal jargon) | **compliance/legal** (tech translated) | **cross-functional** (inline definitions).
See [references/constraints-and-fallbacks.md](references/constraints-and-fallbacks.md) for the audience depth guide.

If unknown: `Audience: unknown. Applying manager-level clarity (fallback).`

### Step 2: Define the Outcome

One sentence — what must the reader decide or do? Write it before drafting.

### Step 3: Draft Key Message First

Opening must contain: **problem + business impact + required action**.

```
BAD:  [3 paragraphs of background] "...we recommend approving the budget."
GOOD: "We need your approval for a $50k security fix by Thursday. [Impact follows.]"
```

### Step 4: Add Supporting Context

Every action item MUST follow:

```
[Owner] must [specific action] by [concrete deadline].
```

```
BAD:  "The database should be optimized."
GOOD: "Database team (Alex) must optimize query performance by March 15."
```

For acronym-heavy documents, extract all acronyms first:

```bash
grep -oE '\b[A-Z]{2,}\b' file.md | sort -u
```

Define each, then draft — rewrite still leads with key message, not the glossary.

### Step 5: Verify

```
- [ ] Every acronym defined on first use.
- [ ] Opening paragraph contains the key decision or recommendation.
- [ ] All technical terms translated — no unexplained jargon.
- [ ] Every action: [Owner] must [action] by [deadline].
- [ ] Paragraphs are concise and scannable.
```

---

## Constraints

**Hard rules:** ALWAYS state audience · ALWAYS define acronyms · ALWAYS lead with key message for executives · NEVER hide decisions · NEVER use passive voice for owned actions · NEVER leave deadlines undefined · NEVER present options without a recommendation.

**Fallback paths** (state explicitly at the top of your output before writing):

| Situation | Prefix |
|-----------|--------|
| Unknown audience | `Audience: unknown. Applying manager-level clarity (fallback).` |
| Unknown terminology | Define inline: `"TLS (the encryption protocol that keeps data private)"` |
| Conflicting priorities | `Multiple critical issues — leading with risk and deadlines.` |

Full formats: [references/constraints-and-fallbacks.md](references/constraints-and-fallbacks.md).

---

## Anti-Patterns

Full catalogue: [references/anti-patterns.md](references/anti-patterns.md).

| # | Anti-pattern | Fix |
|---|-------------|-----|
| AP-1 | Jargon without translation | ALWAYS translate or remove every specialist term |
| AP-2 | Buried key decision | ALWAYS put recommendation in sentence one |
| AP-3 | Passive action language | NEVER use passive voice; use `[Owner] must [action] by [deadline]` |
| AP-4 | Assumed technical context | NEVER assume prior knowledge; explain from first principles |
| AP-5 | Wall of acronyms | ALWAYS define each acronym on first use |
| AP-6 | Recommendations without deadlines | NEVER leave deadlines undefined; undated = ignored |
| AP-7 | Options without a recommendation | NEVER present options without stating which you recommend |
| AP-8 | Skipping audience identification | ALWAYS start with audience and goal |

---

## References

| File | Purpose |
|------|---------|
| [references/audience-types.md](references/audience-types.md) | Audience format and depth guidance |
| [references/jargon-translations.md](references/jargon-translations.md) | Technical-to-plain-language mappings |
| [references/anti-patterns.md](references/anti-patterns.md) | Full anti-pattern catalogue with BAD/GOOD examples |
| [references/before-after-examples.md](references/before-after-examples.md) | Complete before/after rewrites for 5 document types |
| [references/constraints-and-fallbacks.md](references/constraints-and-fallbacks.md) | Full constraint rules, fallback formats, audience depth guide |

External: [PlainLanguage.gov](https://www.plainlanguage.gov/)