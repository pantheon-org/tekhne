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
- Implementing CI quality gates with score thresholds (≥84 passing, ≥108 A-grade)

## Workflow

1. **Inventory** target skills with proper directory scanning
2. **Evaluate** using 8-dimension framework (Knowledge Delta priority)
3. **Validate** artifacts and consistency using deterministic script checks
4. **Generate** reports with JSON output and baseline comparison data
5. **Plan** remediation with measurable success criteria
6. **Re-evaluate** and track score deltas with audit trails

## Self-Audit

This skill is the quality baseline and must pass its own evaluator with a score ≥100. The skill-quality-auditor passes its own 8-dimension framework evaluation, demonstrating that quality assessment tools should meet the standards they enforce.

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json
```

Expected: score `>= 100`. This self-audit requirement ensures the framework maintains credibility and serves as a working example of A-grade skill quality.

## Mindset

- Treat scores as directional signals, not absolute truth.
- Prioritize deterministic, reproducible checks over subjective opinions - automated validation beats manual review.
- Apply strict rules where safety/consistency matters; stay flexible elsewhere.
- Use threshold-based evaluation (≥84 passing, ≥108 A-grade) rather than relative comparisons.

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

## Categories

### Framework (Critical Foundation)

- Quality assessment methodology
- Scoring frameworks and thresholds

### Process (High Priority)

- Audit workflows and remediation
- CI integration and quality gates

### Tools (Medium Priority)

- Script automation and validation
- Report generation and compliance
