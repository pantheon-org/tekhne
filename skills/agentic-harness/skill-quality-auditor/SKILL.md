---
name: skill-quality-auditor
description: "Evaluate, score, and remediate agent skill collections using a 9-dimension quality framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation). Performs duplication detection, generates remediation plans with T-shirt sizing, enforces CI quality gates, validates artifact conventions, tracks score trends, and ensures tessl registry compliance. Use when evaluating skill quality, auditing SKILL.md files, scoring agent skills, generating remediation plans, detecting duplicate skills, validating skill format, enforcing quality gates, optimizing for A-grade publication, comparing audit baselines, batch skill assessments, or checking tessl compliance. Triggers: 'check my skills', 'skill audit', 'improve my SKILL.md', 'quality check', 'A-grade scoring', 'quality gates', 'eval validation', 'audit all skills', 'remediation plan', 'skill judge', 'dimension scoring'."
---

# Skill Quality Auditor

Navigation hub for evaluating, maintaining, and improving skill quality with 9-dimension framework scoring.

## Quick Start

Build once, then audit:

Build once:

```bash
bun run build:skill-auditor
```

Run audits:

```bash
# Single skill
skill-auditor evaluate <domain>/<skill-name> --json --store

# Batch with grade gate
skill-auditor batch <skill1> <skill2> --fail-below B --store
```

## When to Use

- Evaluate skills before merge or publication using 9-dimension scoring
- Generate remediation plans, detect duplication (>20% threshold), or enforce CI quality gates
- Validate eval scenario coverage and artifact conventions

## When Not to Use

- Write the skill first — do not audit an unfinished draft
- Avoid using this as a substitute for peer review of logic or domain accuracy

## Workflow

1. Run `skill-auditor evaluate <skill> --json --store`
2. Check artifacts and eval coverage using deterministic criteria
3. Generate a remediation plan with T-shirt sizing and score delta estimates
4. Run the auditor again to verify improvement; if score is below target, check `remediation-plan.md` and focus on the lowest-scoring dimension

## Mindset

- Use scores as directional signals, not absolute truth.
- Apply deterministic, reproducible checks over manual review.
- Use threshold-based evaluation rather than relative comparisons.
- Keep audit rules strict for safety and consistency; stay flexible elsewhere.

## Anti-Patterns (Summary)

- NEVER skip baseline comparison in recurring audits — WHY: score regressions go undetected without a prior audit.json
- NEVER ignore Knowledge Delta below 15/20 — WHY: low D1 means the skill adds no value over LLM baseline
- NEVER apply subjective scoring — WHY: scores drift between evaluators and cannot be automated in CI
- NEVER create kitchen-sink skills covering unrelated tasks — WHY: broad scope kills D7 and prevents correct triggering
- NEVER use harness-specific paths in skill content — WHY: absolute paths break when installed in a different repo
- NEVER list references without "When to Use" conditions — WHY: unconditional loading bloats context and penalises D5

Ensure you review [Detailed Anti-Patterns](references/detailed-anti-patterns.md) for all WHY/BAD/GOOD failure modes including agent name references and D4 heading rules.

## Examples

Remediation workflow:

```bash
skill-auditor evaluate documentation/markdown-authoring --json --store
# Score: 98/140 (C+) -> review remediation-plan.md -> fix -> re-audit -> 128/140 (A)
```

PR-scoped triage:

```bash
skills=$(git diff --name-only origin/main | grep "skills/.*/SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
skill-auditor batch $skills --fail-below B --store
```

Audit all skills:

```bash
skill-auditor batch $(find skills -name "SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
```

See [Audit Workflow Examples](references/examples-audit-workflows.md) for input/output pairs and CI quality gate examples.

## Self-Audit

```bash
skill-auditor evaluate agentic-harness/skill-quality-auditor --json
# Expected: A grade, total >= 126/140
```

## References

### Framework

| Topic | Reference | When to Use |
| --- | --- | --- |
| Per-dimension criteria and bonus rules | [Dimensions](references/framework-dimensions.md) | Evaluating any dimension or understanding the rubric |
| Score thresholds and grade bands | [Scoring Rubric](references/framework-scoring-rubric.md) | Calculating a total score or assigning a grade |
| A-grade checklist and red flags | [Quality Standards](references/framework-quality-standards.md) | Targeting A-grade or reviewing blockers |
| Trigger pattern density and keyword analysis | [Pattern Recognition](references/advanced-pattern-recognition.md) | Scoring D7 or improving description keywords |
| Canonical SKILL.md structure and References table standard | [SKILL Template](references/skill-template.md) | Authoring or refactoring a skill |

### Operations

| Topic | Reference | When to Use |
| --- | --- | --- |
| CI gate configuration and batch pass/fail logic | [Quality Thresholds](references/quality-thresholds-scoring.md) | Setting up CI quality gates |
| NEVER/WHY/BAD/GOOD failure modes per dimension | [Anti-Patterns](references/detailed-anti-patterns.md) | Explaining low scores or writing remediation guidance |
| T-shirt sizing and remediation roadmaps | [Remediation Planning](references/remediation-planning.md) | Writing a remediation plan for a C/D-grade skill |
| Deduplication workflow and aggregation guidance | [Duplication Detection](references/duplication-detection-algorithm.md) | Detecting skill overlap or planning aggregations |
| `skill-auditor evaluate/batch` usage and output formats | [Scripts Workflow](references/scripts-audit-workflow.md) | Running audits from the command line |
| Registry publication gates and tessl compliance checks | [Tessl Compliance](references/tessl-compliance-framework.md) | Preparing a skill for public registry submission |
