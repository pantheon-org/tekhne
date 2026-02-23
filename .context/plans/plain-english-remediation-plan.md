---
plan_date: 2026-02-23
skill_name: plain-english
source_audit: .context/audits/plain-english-audit-2026-02-22.md
---

# Remediation Plan: plain-english

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 85/120 (70%) |
| **Current Grade** | C |
| **Target Score** | 100+/120 (B+ or higher) |
| **Priority** | High - Priority improvements required |
| **Estimated Effort** | Medium (3-4 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Weak anti-pattern precision | High | D3 | 8/15 | 13/15 |
| Low practical usability | High | D8 | 8/15 | 13/15 |
| Moderate procedural clarity | Medium | D2 | 10/15 | 13/15 |
| Moderate progressive disclosure | Medium | D5 | 10/15 | 13/15 |
| Moderate freedom calibration | Medium | D6 | 10/15 | 13/15 |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - High Priority

**Goal**: Increase D3 score from 8/15 to 13/15

**File**: `skills/plain-english/SKILL.md`

**Step 1.1**: Add explicit NEVER rules with consequences

Add a new section after the core guidelines:

```markdown
## Anti-Patterns

### NEVER Use Technical Jargon Without Explanation

- **WHY**: Non-technical stakeholders cannot decode domain-specific terms.
- **Consequence**: Misunderstanding, decisions made on incorrect assumptions.
- **BAD**: "The API returns a 429 rate limit error."
- **GOOD**: "The system is receiving too many requests. Please wait a moment and try again."

### NEVER Assume Prior Knowledge

- **WHY**: Executives and compliance officers have different expertise than engineers.
- **Consequence**: Exclusion, frustration, incomplete comprehension.
- **BAD**: "We need to refactor the monolith into microservices."
- **GOOD**: "We need to break the large application into smaller, independent pieces. This lets teams work faster and reduces risk."

### NEVER Use Passive Voice for Recommendations

- **WHY**: Passive voice obscures accountability and action requirements.
- **Consequence**: Unclear ownership, delayed decisions.
- **BAD**: "The database should be optimized."
- **GOOD**: "The database team should optimize query performance by Q2."

### NEVER Bury Key Decisions in Paragraphs

- **WHY**: Scannability is critical for time-constrained readers.
- **Consequence**: Important points missed, repeated questions.
- **BAD**: A 5-paragraph explanation with the decision in paragraph 4.
- **GOOD**: Lead with the decision, then provide context.
```

**Step 1.2**: Tie anti-patterns to repository-specific risk

Add context-specific anti-patterns:

```markdown
### NEVER Skip Audience Identification

- **WHY**: Tone and depth vary dramatically across audience types.
- **Consequence**: Wrong level of detail, misaligned expectations.
- **BAD**: Starting to write without confirming: Is this for executives? Engineers? Legal?
- **GOOD**: First, identify the audience: "This document is for [role] who need [outcome]."
```

---

### Phase 2: Practical Usability (D8) - High Priority

**Goal**: Increase D8 score from 8/15 to 13/15

**File**: `skills/plain-english/SKILL.md`

**Step 2.1**: Add executable commands section

Add after anti-patterns:

````markdown
## Quick Commands

### Audience Analysis

```bash
# Check for jargon density (requires grep)
grep -E "(API|HTTP|SQL|async|callback|microservice|container|k8s)" <file>.md | wc -l
```

### Readability Check

```bash
# Install readability tool
npm install -g readability-cli

# Check document grade level
readability <file>.md
```

### Glossary Generation

```bash
# Extract potential jargon terms
grep -oE '\b[A-Z]{2,}\b' <file>.md | sort -u
```
````

**Step 2.2**: Add verification checklist

```markdown
## Verification Checklist

Before finalizing any document for non-technical audiences:

- [ ] All acronyms defined on first use
- [ ] Sentences average under 20 words
- [ ] No paragraph exceeds 5 sentences
- [ ] Key decisions appear in first paragraph
- [ ] No technical jargon without plain-English translation
- [ ] Action items have clear owners and deadlines
```

---

### Phase 3: Procedural Clarity (D2) - Medium Priority

**Goal**: Increase D2 score from 10/15 to 13/15

**File**: `skills/plain-english/SKILL.md`

**Step 3.1**: Rewrite workflow as deterministic sequence

Replace generic workflow guidance with:

```markdown
## Workflow

### Step 1: Identify Audience (Required)

Determine the primary reader:
- Executive: Needs decisions, risks, business impact
- Manager: Needs timeline, resources, tradeoffs
- Compliance/Legal: Needs evidence, controls, audit trail
- Cross-functional: Needs context, dependencies, impact

**Output**: Document header specifying audience and purpose.

### Step 2: Determine Outcome (Required)

What should the reader DO after reading?
- Make a decision? State the recommendation first.
- Understand a change? Lead with impact.
- Take action? List steps with owners.

**Output**: One-sentence goal statement.

### Step 3: Draft Core Message (Required)

Write the single most important point in 2-3 sentences.
Place this in the first paragraph.

**Output**: Opening paragraph with key message.

### Step 4: Add Supporting Context (Conditional)

Only if audience needs depth:
- Background: 2-3 sentences maximum
- Options considered: Bullet list
- Tradeoffs: Table format if 3+ options

**Output**: Structured context section.

### Step 5: Review Against Anti-Patterns (Required)

Check each anti-pattern in this skill.
Remove or rewrite any violations.

**Output**: Anti-pattern compliance confirmation.

### Step 6: Readability Verification (Required)

Run readability check. Target grade level:
- Executive: 8th grade
- Manager: 10th grade
- Technical: 12th grade

**Output**: Readability score within target.
```

---

### Phase 4: Progressive Disclosure (D5) - Medium Priority

**Goal**: Increase D5 score from 10/15 to 13/15

**File**: `skills/plain-english/SKILL.md`

**Step 4.1**: Create references directory structure

```bash
mkdir -p skills/plain-english/references
```

**Step 4.2**: Extract deep content to reference files

Create `skills/plain-english/references/audience-types.md`:

```markdown
# Audience Type Reference

## Executive Audience

### Characteristics
- Time-constrained
- Decision-focused
- Risk-aware
- Business-outcome oriented

### Preferred Format
- Lead with recommendation
- One-page maximum
- Bullet points for options
- Dollar/percentage impact quantified

### Example Structure
1. Recommendation (2 sentences)
2. Business impact (2-3 bullets)
3. Risks and mitigations (2-3 bullets)
4. Decision required (yes/no or choose A/B)
```

Create `skills/plain-english/references/jargon-translations.md`:

```markdown
# Common Jargon Translations

| Technical Term | Plain English |
| --- | --- |
| API | Connection point between systems |
| Database | Centralized data storage |
| Microservice | Small, independent program |
| Container | Self-contained software package |
| Latency | Response delay |
| Throughput | Processing capacity |
| Refactor | Improve internal structure |
| Technical debt | Deferred maintenance work |
```

**Step 4.3**: Update SKILL.md with reference links

Replace detailed content with:

```markdown
## Quick Reference

- [Audience Types](references/audience-types.md) - Detailed guidance per audience
- [Jargon Translations](references/jargon-translations.md) - Common terms and plain-English equivalents
```

---

### Phase 5: Freedom Calibration (D6) - Medium Priority

**Goal**: Increase D6 score from 10/15 to 13/15

**File**: `skills/plain-english/SKILL.md`

**Step 5.1**: Clarify hard constraints vs. flexible guidance

Add section:

```markdown
## Constraints vs. Flexibility

### Hard Constraints (Never Violate)

1. Always define acronyms on first use
2. Always lead with the key message for executive audiences
3. Never exceed one page for executive summaries
4. Never use passive voice for action items

### Flexible Guidance (Adapt to Context)

- Sentence length targets (adjust for technical depth)
- Paragraph length (shorter for email, longer for documents)
- Section ordering (match document type requirements)
- Detail level (scale with audience expertise)

### Fallback Paths

If audience is unknown:
1. Default to 10th-grade readability
2. Include brief glossary for any technical terms
3. Provide executive summary at the top
```

---

## Verification Commands

```bash
# Run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh plain-english --json

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Validate no duplication
skills/skill-quality-auditor/scripts/detect-duplication.sh skills

# Check markdown linting
bunx markdownlint-cli2 "skills/plain-english/**/*.md"
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 100/120 |
| Grade | B+ or higher |
| D3: Anti-Pattern Quality | >= 13/15 |
| D8: Practical Usability | >= 13/15 |
| D2: Mindset + Procedures | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |
| References created | >= 2 files |
| Anti-patterns with BAD/GOOD | >= 4 |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Anti-Patterns | 1 hour | None |
| Phase 2: Practical Usability | 1 hour | Phase 1 |
| Phase 3: Procedural Clarity | 45 min | None |
| Phase 4: Progressive Disclosure | 1 hour | Phase 3 |
| Phase 5: Freedom Calibration | 30 min | None |
| Verification | 30 min | All phases |

**Total Estimated Time**: 4-4.5 hours

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with 5 phases and Timeline table
- Has Estimated Effort in Executive Summary
- Code examples in remediation steps are specific and actionable
- Addresses audience-specific writing (executives, managers, compliance)
