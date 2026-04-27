# Task: Skills Portfolio Duplication Analysis

A skills collection contains the following 8 skills in `skills/`:

- `ci-cd/github-actions-generator`
- `ci-cd/github-actions-validator`
- `ci-cd/gitlab-ci-generator`
- `ci-cd/gitlab-ci-validator`
- `infrastructure/terraform-generator`
- `infrastructure/terraform-validator`
- `infrastructure/terragrunt-generator`
- `infrastructure/terragrunt-validator`

The team suspects that the generator/validator pairs within each tool have high content overlap, and that terraform and terragrunt skills may also overlap significantly.

Analyse the collection for duplication and produce a consolidation recommendation report.

## Output Specification

Produce:
1. **similarity-analysis.md** — pairwise similarity percentages for all relevant pairs
2. **consolidation-recommendations.md** — which skills to consolidate and how (Navigation Hub pattern)
3. **duplication-report.json** — structured data: pairs, similarity %, action (monitor/plan/immediate)

## Context

Similarity thresholds:
- <20%: monitor only
- 20–35%: plan aggregation
- >35%: immediate action required
