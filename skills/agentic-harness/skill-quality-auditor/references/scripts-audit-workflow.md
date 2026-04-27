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

- Go 1.21+ (for building the binary)
- Bun runtime (for TypeScript aggregation scripts)
- Write access to `.context/audits/` directory

Build the binary once from the repo root:

```bash
bun run build:skill-auditor
```

## Audit Commands

### Single skill evaluation

```bash
skill-auditor evaluate <domain/skill-name> --json
```

**Options:**

- `--json` - Machine-readable JSON output
- `--store` - Write results to `.context/audits/<skill>/YYYY-MM-DD/`
- `--repo-root` - Explicit repo root (auto-detected if omitted)

**Output:** JSON score report

### Batch audit

```bash
skill-auditor batch <skill1> <skill2> [skill3...]
```

**Options:**

- `--json` - Machine-readable JSON array output
- `--store` - Write each result to `.context/audits/<skill>/YYYY-MM-DD/`
- `--fail-below <grade>` - Exit 1 if any skill scores below this grade (e.g. `B+`)

**Features:**

- Sequential evaluation with per-skill error isolation
- Failure tracking and summary table
- Audit results stored in `.context/audits/<skill-name>/YYYY-MM-DD/`
- Exit code reflects failures (0 = all passed, 1 = some failed)

**Output:** Summary table or JSON array

**Use when:** Auditing multiple skills in phases or batches with consolidated tracking

### Duplication detection (shell helper)

```bash
./scripts/detect-duplication.sh [skills-dir]
```

**Arguments:**

- `skills-dir` - Skills directory (default: `skills`)

**Output:** `duplication-report-YYYY-MM-DD.md`

### Aggregation planning

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
# Run full batch audit
skill-auditor batch $(find skills -name "SKILL.md" | sed 's|skills/||;s|/SKILL.md||') --store

# Review stored results
cat .context/audits/*/$(date +%Y-%m-%d)/audit.json | jq '.grade'
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
      - name: Build skill-auditor
        run: bun run build:skill-auditor
      - name: Run full batch audit
        run: |
          skills=$(find skills -name "SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
          skill-auditor batch $skills --store
      - run: ./scripts/detect-duplication.sh
      - uses: actions/upload-artifact@v4
        with:
          name: audit-reports
          path: .context/audits/
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
      - name: Build skill-auditor
        run: bun run build:skill-auditor
      - name: Check changed skills
        run: |
          for skill in $(git diff --name-only origin/main | grep "skills/.*/SKILL.md" | sed 's|skills/||;s|/SKILL.md||'); do
            skill-auditor evaluate "$skill" --json --store
          done
          skill-auditor batch $(git diff --name-only origin/main | grep "skills/.*/SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ') --fail-below B
```

## Single-Skill Report Format

Audit reports in `.context/audits/*.md` must follow the frontmatter-first format:

```yaml
review_date: YYYY-MM-DD
reviewer: automated audit
skill_location: `skills/<skill-name>/SKILL.md`
```

Use `templates/review-report-template.yaml` as the canonical format source, then validate generated reports with:

```bash
././scripts/validate-review-format.sh .context/audits/<skill>-YYYY-MM-DD.md
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
