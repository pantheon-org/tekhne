---
category: scripts
priority: MEDIUM
source: skill audit automation
---

# Audit Workflow Automation

Complete workflow for running automated skill quality audits. Covers setup, execution, and maintenance of audit processes.

## Workflow Overview

**Purpose**: Automate skill quality evaluation on schedule and on-demand  
**Outputs**: Quality reports, duplication analysis, aggregation recommendations  
**Schedule**: Weekly automated + on-demand for new skills

## Prerequisites

- Bash shell environment
- Bun runtime for TypeScript scripts
- Write access to `.context/analysis/` directory

## Audit Scripts

### audit-skills.sh

Full skill collection audit:

```bash
./scripts/audit-skills.sh [--output path]
```

**Options:**

- `--output` - Custom output directory (default: `.context/analysis/`)

**Output:** `skill-audit-YYYY-MM-DD.md`

### detect-duplication.sh

Duplication detection:

```bash
./scripts/detect-duplication.sh [skills-dir]
```

**Arguments:**

- `skills-dir` - Skills directory (default: `skills`)

**Output:** `duplication-report-YYYY-MM-DD.md`

### evaluate.sh

Single skill evaluation:

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json
```

**Arguments:**

- `skill-name` - Name of skill to evaluate

**Output:** JSON score report

### plan-aggregation.ts

Aggregation planning:

```bash
bun run scripts/plan-aggregation.ts --family <prefix>
```

**Options:**

- `--family` - Skill family prefix (e.g., `bdd`, `typescript`)

**Output:** Aggregation plan with recommendations

## Complete Audit Workflow

### Phase 1: Inventory

```bash
# Count skills
find skills -name "SKILL.md" -not -path "*/.deprecated/*" | wc -l

# List by category
ls -1 skills/ | cut -d- -f1 | sort | uniq -c | sort -rn

# Identify families
ls -1 skills/ | grep -E "^[a-z]+-" | cut -d- -f1 | sort -u
```

### Phase 2: Quality Evaluation

```bash
# Run full audit
./scripts/audit-skills.sh

# Review report
cat .context/analysis/skill-audit-*.md | head -50
```

### Phase 3: Duplication Analysis

```bash
# Detect duplication
./scripts/detect-duplication.sh

# Review high-priority pairs
grep -A5 "High-Priority" .context/analysis/duplication-report-*.md
```

### Phase 4: Aggregation Planning

```bash
# Plan aggregation for each family
bun run scripts/plan-aggregation.ts --family bdd
bun run scripts/plan-aggregation.ts --family typescript
bun run scripts/plan-aggregation.ts --family bun
```

### Phase 5: Generate Summary

```bash
# Create summary report
cat > .context/analysis/audit-summary-$(date +%Y-%m-%d).md <<EOF
# Audit Summary - $(date +%Y-%m-%d)

## Statistics
- Total skills: $(find skills -name "SKILL.md" -not -path "*/.deprecated/*" | wc -l)
- A-grade: X
- B-grade: Y
- C-grade: Z

## Duplication
- High-priority pairs: X
- Critical (>35%): Y

## Recommendations
1. [From aggregation plans]
2. [From duplication report]
EOF
```

## Scheduled Audits

### Weekly Automated Audit

Add to CI/CD pipeline:

```yaml
# .github/workflows/skill-audit.yml
name: Skill Audit
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - run: chmod +x skills/skill-quality-auditor/scripts/*.sh
      - run: skills/skill-quality-auditor/scripts/audit-skills.sh
      - run: skills/skill-quality-auditor/scripts/detect-duplication.sh
      - uses: actions/upload-artifact@v4
        with:
          name: audit-reports
          path: .context/analysis/
```

### Pre-Merge Quality Gate

Block PRs with C-grade skills:

```yaml
# .github/workflows/skill-quality-gate.yml
name: Skill Quality Gate
on: pull_request

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - name: Check changed skills
        run: |
          for skill in $(git diff --name-only origin/main | grep "skills/.*/SKILL.md"); do
            skill_name=$(basename $(dirname $skill))
            score=$(sh skills/skill-quality-auditor/scripts/evaluate.sh "$skill_name" --json | jq '.total')
            if [ "$score" -lt 90 ]; then
              echo "Skill $skill_name scored $score (below 90)"
              exit 1
            fi
          done
```

## Single-Skill Report Format

Audit reports in `.context/audits/*.md` must follow the frontmatter-first format:

```yaml
review_date: YYYY-MM-DD
reviewer: automated audit
skill_location: `skills/<skill-name>/SKILL.md`
```

Use `skills/skill-quality-auditor/templates/review-report-template.yaml` as the canonical format source, then validate generated reports with:

```bash
./skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/<skill>-YYYY-MM-DD.md
```

## Report Interpretation

### Quality Report

```markdown
| Skill | Score | Grade | Lines |
|-------|-------|-------|-------|
| bdd-testing | 98/120 | A | 64 |
```

**Actions:**

- Score <90: Review and improve
- Score <80: Critical issues
- Lines >300: Consider progressive disclosure

### Duplication Report

```markdown
| Skill Pair | Similarity | Action |
|------------|------------|--------|
| bdd-gherkin ↔ cucumber | 42% | Aggregate |
```

**Actions:**
>
- >35%: Immediate aggregation
- 20-35%: Plan aggregation
- <20%: Monitor

## Maintenance

### Weekly Tasks

- [ ] Review automated audit reports
- [ ] Check for new duplication
- [ ] Update baseline metrics
- [ ] Address C-grade skills

### Monthly Tasks

- [ ] Full collection evaluation
- [ ] Update aggregation plans
- [ ] Review deprecated skills
- [ ] Archive old reports

### Quarterly Tasks

- [ ] Complete audit cycle
- [ ] Major consolidation if needed
- [ ] Update documentation
- [ ] Team review of metrics

## See Also

- `scripts-ci-integration.md` - CI/CD setup
- `reporting-analysis.md` - Interpreting results
- `reporting-dashboards.md` - Visualization
