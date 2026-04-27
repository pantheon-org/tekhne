# Audit Workflow Examples

## Single Skill Audit

```bash
skill-auditor evaluate infrastructure/terraform-generator --json --store
```

Output:

```json
{
  "skill": "infrastructure/terraform-generator",
  "grade": "B+",
  "total": 122,
  "dimensions": {
    "D1_knowledge_delta": 18,
    "D2_mindset_procedures": 13,
    "D3_anti_pattern_quality": 12,
    "D4_specification_compliance": 14,
    "D5_progressive_disclosure": 13,
    "D6_freedom_calibration": 12,
    "D7_pattern_recognition": 8,
    "D8_practical_usability": 14,
    "D9_eval_validation": 18
  }
}
```

Next steps based on output:

- D7 at 8/10 -> add more trigger keywords and domain-specific examples to SKILL.md description
- D3 at 12/15 -> add "Common Mistakes" section with 3-5 anti-patterns

## Batch Audit with Baseline Comparison

```bash
skill-auditor batch \
  infrastructure/terraform-generator \
  ci-cd/github-actions-generator \
  development/bash-script-generator \
  --store

# Compare against previous baseline
diff <(cat .context/audits/infrastructure/terraform-generator/2025-12-01/audit.json) \
     <(cat .context/audits/infrastructure/terraform-generator/$(date +%Y-%m-%d)/audit.json)
```

## Remediation Workflow

```bash
skill-auditor evaluate documentation/markdown-authoring --json --store
# Score: 98/140 (C+) -> blocked from publishing

cat .context/audits/documentation/markdown-authoring/$(date +%Y-%m-%d)/remediation-plan.md
# Outputs prioritized fixes:
#   1. [HIGH] D3 Anti-Patterns: Add 5 anti-patterns with BAD/GOOD examples (S effort)
#   2. [HIGH] D5 Progressive Disclosure: Restructure Quick Start -> Guide -> Advanced (M effort)
#   3. [MED]  D7 Pattern Recognition: Expand description keywords (S effort)

# After applying fixes:
skill-auditor evaluate documentation/markdown-authoring --json --store
# Score: 128/140 (A) -> publication-ready
```

## CI Quality Gate

```yaml
# .github/workflows/skill-quality.yml
- name: Build skill-auditor
  run: bun run build:skill-auditor
- name: Audit changed skills
  run: |
    skills=$(git diff --name-only origin/main | grep "skills/.*/SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
    skill-auditor batch $skills --fail-below B --store
    # Exits 1 if any skill scores below B-grade (112/140)
```
