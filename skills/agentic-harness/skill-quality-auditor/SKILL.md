---
name: skill-quality-auditor
description: Audit and improve skill collections with 8-dimension scoring framework (Knowledge Delta, Anti-Patterns, Progressive Disclosure), duplication detection, remediation planning, CI quality gates, tessl compliance, A-grade optimization, baseline comparison, trend analysis, artifact validation, consistency checking. Use when evaluating skill quality, generating remediation plans, validating report format, skill-judge framework, score thresholds, enforcing repository-wide skill artifact conventions, or when users say 'check my skills', 'review skill files', 'skill audit', 'improve my SKILL.md files', 'find duplicate skills', 'validate skill format', 'quality check my skills', 'A-grade scoring', 'tessl compliance', 'skill assessment', or 'quality gates'.
---

# Skill Quality Auditor

Navigation hub for evaluating, maintaining, and improving skill quality with 8-dimension framework scoring.

## Quick Start

```bash
# Evaluate single skill
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json

# Batch audit multiple skills
sh skills/agentic-harness/skill-quality-auditor/scripts/batch-audit.sh <skill1> <skill2> [skill3...]

# Audit all skills
sh skills/agentic-harness/skill-quality-auditor/scripts/audit-skills.sh

# Emergency triage (PR context)
sh skills/agentic-harness/skill-quality-auditor/scripts/audit-skills.sh --pr-changes-only
```

Results stored in `.context/audits/<skill-name>/latest/`.

## When to Use

- Running periodic quality audits with 8-dimension framework scoring
- Evaluating specific skills before merge using deterministic criteria
- Creating remediation plans with measurable success criteria
- Detecting duplication (>20% similarity threshold) and planning aggregations
- Enforcing artifact conventions across skill collections
- Implementing CI quality gates with score thresholds (see [Score Thresholds](references/quality-thresholds-scoring.md))

## Workflow

1. **Inventory** target skills with proper directory scanning
2. **Evaluate** using 8-dimension framework (Knowledge Delta priority)
3. **Validate** artifacts and consistency using deterministic script checks
4. **Generate** reports with JSON output and baseline comparison data
5. **Plan** remediation with measurable success criteria
6. **Re-evaluate** and track score deltas with audit trails

## Mindset

- Treat scores as directional signals, not absolute truth.
- Prioritize deterministic, reproducible checks over manual review.
- Apply strict rules where safety/consistency matters; stay flexible elsewhere.
- Use threshold-based evaluation rather than relative comparisons.

## Anti-Patterns (Summary)

- NEVER skip baseline comparison in recurring audits
- NEVER ignore Knowledge Delta scoring below 15/20
- NEVER apply subjective scoring without deterministic checks
- NEVER use harness-specific paths in skill content
- NEVER mention specific agent names in skill instructions
- NEVER create kitchen-sink skills that cover multiple unrelated tasks
- NEVER bypass skill-quality-auditor in favor of tessl review alone

See [Detailed Anti-Patterns](references/detailed-anti-patterns.md) for complete WHY/BAD/GOOD failure mode documentation.

## Common Pitfalls

- ALWAYS run `./scripts/evaluate.sh` before publishing; NEVER trust manual review alone
- ALWAYS check anti-pattern coverage (D3) in production skills; missing gotcha documentation is the most common pitfall
- NEVER aggregate skills with <20% similarity; see [Duplication Detection](references/duplication-detection-algorithm.md)

## Self-Audit

This skill ALWAYS passes its own evaluator with score >= 100:

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh agentic-harness/skill-quality-auditor --json
# Expected: grade "B+" or higher, total >= 100
```

Interpret output:

```bash
# Grade thresholds: A >= 108, B+ >= 102, B >= 96, C+ < 96
# Dimension range: 0-20 (D1), 0-15 (D2-D8)
bun cli/index.ts audit status  # Cross-repo compliance dashboard
```

## Reference Map

**Critical**: [Quality Thresholds](references/quality-thresholds-scoring.md) | [Anti-Patterns](references/detailed-anti-patterns.md)

**Framework**: [Dimensions](references/framework-skill-judge-dimensions.md) | [Scoring Rubric](references/framework-scoring-rubric.md) | [Quality Standards](references/framework-quality-standards.md) | [Pattern Recognition](references/advanced-pattern-recognition.md)

**Operations**: [Remediation Planning](references/remediation-planning.md) | [Duplication Detection](references/duplication-detection-algorithm.md) | [Scripts Workflow](references/scripts-audit-workflow.md) | [Tessl Compliance](references/tessl-compliance-framework.md)
