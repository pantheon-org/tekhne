---
name: skill-quality-auditor
description: Audit and improve skill collections with 8-dimension scoring framework (Knowledge Delta, Anti-Patterns, Progressive Disclosure), duplication detection, remediation planning, CI quality gates, tessl compliance, A-grade optimization, baseline comparison, trend analysis, artifact validation, consistency checking. Use when evaluating skill quality, generating remediation plans, validating report format, skill-judge framework, score thresholds, enforcing repository-wide skill artifact conventions, or when users say 'check my skills', 'review skill files', 'skill audit', 'improve my SKILL.md files', 'find duplicate skills', 'validate skill format', 'quality check my skills', 'A-grade scoring', 'tessl compliance', 'skill assessment', or 'quality gates'.
---

# Skill Quality Auditor

Navigation hub for evaluating, maintaining, and improving skill quality with 8-dimension framework scoring.

## Quick Start

### Evaluate Single Skill

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json
```

### Audit All Skills  

```bash
sh skills/skill-quality-auditor/scripts/audit-skills.sh
```

Generates comprehensive reports in `.context/audits/` with timestamp-based baseline comparisons.

### Emergency Triage (PR Context)

```bash  
sh skills/skill-quality-auditor/scripts/audit-skills.sh --pr-changes-only
```

## When to Use

- Running periodic quality audits with 8-dimension framework scoring
- Evaluating specific skills before merge using deterministic criteria
- Creating remediation plans with measurable success criteria
- Detecting duplication (>20% similarity threshold) and planning aggregations
- Enforcing artifact conventions across skill collections
- Implementing CI quality gates with score thresholds (see [Score Thresholds](#score-thresholds))

## Workflow

1. **Inventory** target skills with proper directory scanning
2. **Evaluate** using 8-dimension framework (Knowledge Delta priority)
3. **Validate** artifacts and consistency using deterministic script checks
4. **Generate** reports with JSON output and baseline comparison data
5. **Plan** remediation with measurable success criteria
6. **Re-evaluate** and track score deltas with audit trails

## Self-Audit

This skill must pass its own evaluator with score >= 100:

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json
```

## Mindset

- Treat scores as directional signals, not absolute truth.
- Prioritize deterministic, reproducible checks — automated validation beats manual review.
- Apply strict rules where safety/consistency matters; stay flexible elsewhere.
- Use threshold-based evaluation rather than relative comparisons (see [Score Thresholds](#score-thresholds)).

## Anti-Patterns

### NEVER skip baseline comparison in recurring audits

- **WHY**: score changes are meaningless without prior reference points.
- **BAD**: run ad hoc audits with no previous report linkage.
- **GOOD**: compare current results to previous dated audits.

### NEVER ignore Knowledge Delta scoring below 15/20

- **WHY**: Knowledge Delta is the highest-weighted dimension and signals expert-only content gaps.
- **BAD**: accept scores of 10-14 without investigation.
- **GOOD**: prioritize Knowledge Delta improvements first, target ≥17/20 for A-grade skills.

### NEVER apply subjective scoring without deterministic checks

- **WHY**: human judgment varies and creates inconsistent audit results.
- **BAD**: rely on manual assessment for quality gates.
- **GOOD**: use automated scripts and measurable criteria for consistency.

See [Detailed Anti-Patterns](references/detailed-anti-patterns.md) for complete failure mode documentation.

## Reference Map

### Critical References (CRITICAL priority)

- [**Quality Thresholds & Scoring**](references/quality-thresholds-scoring.md) - A-grade requirements, score interpretation
- [**Detailed Anti-Patterns**](references/detailed-anti-patterns.md) - Critical failure modes with WHY/BAD/GOOD structure

### High Priority References (HIGH priority)

- [**Advanced Pattern Recognition**](references/advanced-pattern-recognition.md) - Quality patterns, trigger optimization
- [**Framework Dimensions**](references/framework-skill-judge-dimensions.md) - Complete 8-dimension evaluation criteria
- [**Scoring Rubric**](references/framework-scoring-rubric.md) - Detailed scoring methodology

### Supporting Documentation

- [**Remediation Planning**](references/remediation-planning.md) - Fix low-scoring skills systematically
- [**Duplication Detection**](references/duplication-detection-algorithm.md) - Similarity algorithms and aggregation
- [**Scripts Workflow**](references/scripts-audit-workflow.md) - Advanced script usage patterns
- [**Tessl Compliance**](references/tessl-compliance-framework.md) - Registry submission requirements

## Consistency Check

Use the helper script to verify basic structural consistency across skills:

```bash
sh skills/skill-quality-auditor/scripts/check-consistency.sh skills
```

This script validates frontmatter format, directory structures, required sections, and naming conventions. Critical for maintaining uniform skill quality standards across the entire collection.

Run this check:

- Before publishing skills to ensure compliance
- After bulk edits to catch structural issues
- As part of CI quality gates

## Artifact Validation

Validate skill artifacts before submission:

```bash
sh skills/skill-quality-auditor/scripts/validate-skill-artifacts.sh
```

This validates:

- **tile.json existence and format**: Required for registry submission — every skill needs valid tile.json with name, summary, version
- **SKILL.md frontmatter**: name, description fields present and valid
- **Directory conventions**: Proper structure under `skills/<skill-name>/`
- **Template format**: YAML files in templates/ use correct `.yaml`/`.yml` extension
- **Schema format**: JSON Schema files include `"$schema"` declaration
- **Script portability**: Shell scripts have proper shebang `#!/usr/bin/env sh`

Always validate artifacts before running audits to ensure results are meaningful.

## Progressive Disclosure Evaluation

Skills should use Navigation Hub architecture with progressive disclosure:

1. **SKILL.md as Hub (8 points)**: Keep SKILL.md under 100 lines — overview, when-to-use, and reference guide only.
2. **References Directory (4 points)**: Detailed content goes in `references/*.md` files for deeper dives.
3. **Artifact Separation**: Scripts in `scripts/`, templates in `templates/`, schemas in `schemas/` — not mixed into main docs.

## Score Thresholds

See [Quality Thresholds & Scoring](references/quality-thresholds-scoring.md) for grade boundaries, score interpretation, and detailed A-grade requirements.
