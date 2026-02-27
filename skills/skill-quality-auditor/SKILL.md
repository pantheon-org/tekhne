---
name: skill-quality-auditor
description: Audit and improve skill collections with an 8-dimension scoring framework, duplication detection, remediation planning, and CI quality gates. Use when evaluating skill quality, generating remediation plans, validating report format, enforcing repository-wide skill artifact conventions, or when users say 'check my skills', 'review skill files', 'skill audit', 'improve my SKILL.md files', 'find duplicate skills', 'validate skill format', or 'quality check my skills'.
---

# Skill Quality Auditor

Navigation hub for evaluating, maintaining, and improving skill quality.

## When to Use

- Running periodic quality audits of all skills.
- Evaluating a specific skill before merge.
- Creating remediation plans from audit results.
- Detecting duplication and planning aggregations.
- Enforcing artifact and report format conventions.

## When Not to Use

- The task is pure feature implementation unrelated to skill quality.
- The goal is ad hoc writing without measurable audit criteria.

## Mindset

- Treat scores as directional signals, not absolute truth.
- Prioritize deterministic, reproducible checks over subjective opinions.
- Apply strict rules where safety/consistency matters; stay flexible elsewhere.

## Workflow

1. Inventory target skills (single skill or full collection).
2. Run evaluator/audit scripts.
3. Generate and validate reports.
4. Create remediation plans for low-scoring skills.
5. Re-evaluate and track score deltas.

## Categories

| Priority | Area | Purpose |
| --- | --- | --- |
| Critical | Framework | 8-dimension quality scoring |
| High | Remediation | Plan and execute improvements |
| High | Duplication | Identify consolidation opportunities |
| Medium | Scripts | Automation and CI integration |
| Medium | Reporting | Report generation and format validation |
| Low | Advanced | Trends and custom metrics |

## Quick Start

### Evaluate One Skill

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json
```

### Audit All Skills

```bash
sh skills/skill-quality-auditor/scripts/audit-skills.sh
```

### Audit Only Changed Skills (PR Workflow)

```bash
sh skills/skill-quality-auditor/scripts/audit-skills.sh --pr-changes-only --latest-symlink
```

### Prune Old Audits

```bash
sh skills/skill-quality-auditor/scripts/prune-audits.sh --keep 5
```

### Validate Skill Artifacts

```bash
sh skills/skill-quality-auditor/scripts/validate-skill-artifacts.sh
```

### Validate Report Format

```bash
sh skills/skill-quality-auditor/scripts/validate-review-format.sh <report-path>
```

## Scoring

See [references/scoring-rubric.md](references/scoring-rubric.md) for full weights and grade scale.

Quick thresholds:

- Passing: `>= 84` (C)
- Recommended: `>= 96` (B)
- Target: `>= 108` (A)

## Remediation Planning

1. Run audit: `sh skills/skill-quality-auditor/scripts/audit-skills.sh`
2. Locate audit: `.context/audits/skill-audit/YYYY-MM-DD/audit.json`
3. Generate plan: `sh skills/skill-quality-auditor/scripts/generate-remediation-plan.sh --audit-dir .context/audits/skill-audit/latest/ <skill-name>`
4. Validate plan: `sh skills/skill-quality-auditor/scripts/validate-remediation-plan.sh <plan-path>`
5. Save plan: `.context/plans/<skill-name>-remediation-plan.md`

## Audit Directory Structure

```
.context/
└── audits/
    └── skill-audit/
        ├── 2026-02-21/
        │   ├── audit.json
        │   ├── analysis.md
        │   └── remediation-plan.md
        ├── 2026-02-22/
        │   ├── audit.json
        │   ├── analysis.md
        │   └── remediation-plan.md
        └── latest/        # symlink to most recent
            ├── audit.json
            ├── analysis.md
            └── remediation-plan.md
```

## Anti-Patterns

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

## Self-Audit

This skill is the quality baseline and must pass its own evaluator.

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json
```

Expected: score `>= 100`.

## Consistency Check

Use the helper script to verify basic structural consistency across skills:

```bash
sh skills/skill-quality-auditor/scripts/check-consistency.sh skills
```

## Reference Map

- Framework: `references/framework-skill-judge-dimensions.md`, `references/framework-skill-judge-canonical.md`
- Scoring: `references/scoring-rubric.md`, `references/framework-quality-standards.md`
- Remediation: `references/remediation-planning.md`, `references/dimension-analysis-template.md`
- Duplication: `references/duplication-detection-algorithm.md`, `references/duplication-remediation.md`
- Aggregation: `references/aggregation-pattern.md`, `references/aggregation-implementation.md`
- Scripts/CI: `references/scripts-audit-workflow.md`, `references/scripts-ci-integration.md`
- Reporting: `references/reporting-analysis.md`, `references/reporting-dashboards.md`

## References

- [Skill Judge Canonical Source](https://github.com/metaskills/skill-judge)
