---
name: skill-quality-auditor
description: "Evaluate, score, and remediate agent skill collections using a 9-dimension quality framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation). Performs duplication detection, generates remediation plans with T-shirt sizing, enforces CI quality gates, validates artifact conventions, tracks score trends, and ensures tessl registry compliance. Use when evaluating skill quality, auditing SKILL.md files, scoring agent skills, generating remediation plans, detecting duplicate skills, validating skill format, enforcing quality gates, optimizing for A-grade publication, comparing audit baselines, batch skill assessments, or checking tessl compliance. Triggers: 'check my skills', 'skill audit', 'improve my SKILL.md', 'quality check', 'A-grade scoring', 'quality gates', 'eval validation', 'audit all skills', 'remediation plan', 'skill judge', 'dimension scoring'."
---

# Skill Quality Auditor

Navigation hub for evaluating, maintaining, and improving skill quality with 9-dimension framework scoring.

## Quick Start

```bash
# Evaluate single skill — pass full path from skills/ root, preserving all directory levels
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain/skill-name> --json
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain/subdomain/skill-name> --json

# Batch audit multiple skills
sh skills/agentic-harness/skill-quality-auditor/scripts/batch-audit.sh <skill1> <skill2> [skill3...]

# Audit all skills
sh skills/agentic-harness/skill-quality-auditor/scripts/audit-skills.sh

# Emergency triage (PR context)
sh skills/agentic-harness/skill-quality-auditor/scripts/audit-skills.sh --pr-changes-only
```

Results stored in `.context/audits/<full-skill-path>/latest/` — the path mirrors `skills/<full-skill-path>/SKILL.md` exactly, including every directory level.

## When to Use

- Running periodic quality audits with 9-dimension framework scoring
- Evaluating specific skills before merge using deterministic criteria
- Validating runtime effectiveness via tessl eval scenarios (D9)
- Creating remediation plans with measurable success criteria
- Detecting duplication (>20% similarity threshold) and planning aggregations
- Enforcing artifact conventions across skill collections
- Implementing CI quality gates with score thresholds (see [Score Thresholds](references/quality-thresholds-scoring.md))

## Workflow

1. **Inventory** target skills with proper directory scanning
2. **Evaluate** using 9-dimension framework (Knowledge Delta + Eval Validation priority)
3. **Validate** artifacts, consistency, and eval coverage using deterministic script checks
4. **Generate** reports with JSON output and baseline comparison data
5. **Plan** remediation with measurable success criteria (including eval creation)
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

## Examples

```bash
# 2-level skill (domain/skill-name)
./scripts/evaluate.sh infrastructure/terraform-generator --json --store
# Output: {"grade":"B+","total":122,"dimensions":{...}}
# Stored: .context/audits/infrastructure/terraform-generator/<date>/

# 3-level skill (domain/subdomain/skill-name)
./scripts/evaluate.sh software-engineering/design-principles/clean-architecture --json --store
# Output: {"grade":"C","total":101,"dimensions":{...}}
# Stored: .context/audits/software-engineering/design-principles/clean-architecture/<date>/

./scripts/batch-audit.sh infrastructure/terraform-generator ci-cd/github-actions-generator
# Audits multiple skills, stores results in .context/audits/

./scripts/evaluate.sh documentation/markdown-authoring --json --store
# Score: 98/140 (C+) -> review remediation-plan.md -> fix -> re-audit -> 128/140 (A)
```

See [Audit Workflow Examples](references/examples-audit-workflows.md) for detailed input/output pairs, baseline comparison, remediation workflow, and CI quality gate configuration.

## Self-Audit

```bash
./scripts/evaluate.sh agentic-harness/skill-quality-auditor --json
# Expected: grade "A" or higher, total >= 126/140
# Grade thresholds: A >= 126, B+ >= 119, B >= 112, C+ < 112
```

## Reference Map

**Critical**: [Quality Thresholds](references/quality-thresholds-scoring.md) | [Anti-Patterns](references/detailed-anti-patterns.md)

**Framework**: [Dimensions](references/framework-skill-judge-dimensions.md) | [Scoring Rubric](references/framework-scoring-rubric.md) | [Quality Standards](references/framework-quality-standards.md) | [Pattern Recognition](references/advanced-pattern-recognition.md)

**Operations**: [Remediation Planning](references/remediation-planning.md) | [Duplication Detection](references/duplication-detection-algorithm.md) | [Scripts Workflow](references/scripts-audit-workflow.md) | [Tessl Compliance](references/tessl-compliance-framework.md)

**Related Skills**: creating-eval-scenarios (D9 eval generation)
