---
category: reporting
priority: MEDIUM
source: skill audit visualization
---

# Quality Dashboards

Visualization approaches for skill quality metrics. Provides templates for tracking and presenting audit results.

## Dashboard Overview

**Purpose**: Visual representation of skill collection health  
**Audience**: Team leads, developers, stakeholders  
**Update Frequency**: Weekly automated, on-demand manual

## Metrics Dashboard

### Core Metrics Panel

```
┌─────────────────────────────────────────────────────────────┐
│                   SKILL COLLECTION HEALTH                   │
├──────────────────┬──────────────────┬───────────────────────┤
│  Total Skills    │  A-Grade Rate    │  Avg Duplication      │
│      50          │      72%         │       12%             │
│   (target: N/A)  │ (target: 90%)    │  (target: <5%)        │
├──────────────────┴──────────────────┴───────────────────────┤
│  STATUS: NEEDS ATTENTION                                    │
│  • 8 skills below A-grade                                   │
│  • 3 aggregation candidates                                 │
└─────────────────────────────────────────────────────────────┘
```

### Grade Distribution Chart

```
Grade Distribution
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
A (108+) ████████████████████████████████████ 36 (72%)
B (96-107) ████████████ 12 (24%)
C (84-95) ███ 2 (4%)
D/F (<84) 0 (0%)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Top/Bottom Skills

```
Top Performers          │  Needs Improvement
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
bdd-testing      98/120 │  old-patterns     72/120
typescript-adv   96/120 │  legacy-utils     78/120
bun-development  94/120 │  temp-skills      82/120
```

## Trend Dashboard

### Quality Trend (12 weeks)

```
Average Score
120 ┤
110 ┤       ╭───╮    ╭────╮
100 ┤   ╭───╯   ╰────╯    ╰───
 90 ┤───╯
 80 ┤
    └──────────────────────────
      W1  W3  W5  W7  W9  W11

Trend: Improving (+8 points over 12 weeks)
```

### Duplication Trend

```
Duplication %
35 ┤
30 ┤╮
25 ┤╰╮
20 ┤ ╰╮
15 ┤  ╰─╮
10 ┤    ╰──╮
 5 ┤       ╰──────
 0 ┤
    └──────────────
      Jan Feb Mar

Trend: Decreasing (35% → 8%)
```

## Duplication Heatmap

```
             bdd-  type-  bun-  mise- biome-
             test  script dev   plete plete
bdd-test     ───   ███    ░░    ░░    ░░
typescript   ███   ───    ██    ░░    ░░
bun-dev      ░░    ██     ───   ░░    ░░
mise-comp    ░░    ░░     ░░    ───   ███
biome-comp   ░░    ░░     ░░    ███   ───

Legend: ███ High (>30%)  ██ Medium (20-30%)  ░░ Low (<20%)  ─── Self
```

## HTML Dashboard Template

```html
<!DOCTYPE html>
<html>
<head>
  <title>Skill Quality Dashboard</title>
  <style>
    .metric-card {
      border: 1px solid #ddd;
      padding: 1rem;
      margin: 0.5rem;
      border-radius: 8px;
    }
    .grade-a { background: #d4edda; }
    .grade-b { background: #fff3cd; }
    .grade-c { background: #f8d7da; }
    .progress-bar {
      height: 20px;
      background: #eee;
      border-radius: 4px;
    }
    .progress-fill {
      height: 100%;
      border-radius: 4px;
    }
  </style>
</head>
<body>
  <h1>Skill Quality Dashboard</h1>
  <p>Last updated: <span id="timestamp"></span></p>
  
  <div class="metrics-row">
    <div class="metric-card">
      <h3>Total Skills</h3>
      <div class="value">50</div>
    </div>
    <div class="metric-card">
      <h3>A-Grade Rate</h3>
      <div class="value">72%</div>
      <div class="progress-bar">
        <div class="progress-fill" style="width: 72%; background: #ffc107;"></div>
      </div>
      <small>Target: 90%</small>
    </div>
    <div class="metric-card">
      <h3>Duplication</h3>
      <div class="value">12%</div>
      <div class="progress-bar">
        <div class="progress-fill" style="width: 12%; background: #28a745;"></div>
      </div>
      <small>Target: &lt;5%</small>
    </div>
  </div>
  
  <h2>Grade Distribution</h2>
  <div id="grade-chart"></div>
  
  <h2>Skills Needing Attention</h2>
  <table>
    <tr><th>Skill</th><th>Score</th><th>Grade</th><th>Issue</th></tr>
    <tr class="grade-c">
      <td>old-patterns</td><td>72/120</td><td>C</td>
      <td>Missing anti-patterns, no examples</td>
    </tr>
  </table>
  
  <script>
    // Load data from audit report
    document.getElementById('timestamp').textContent = new Date().toISOString();
  </script>
</body>
</html>
```

## Markdown Dashboard Template

```markdown
# Skill Quality Dashboard

**Last Updated**: 2026-02-20
**Status**: ⚠️ Needs Attention

## Summary Metrics

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Total Skills | 50 | - | - |
| A-Grade Rate | 72% | 90% | ⚠️ Below |
| Avg Score | 98/120 | 108/120 | ⚠️ Below |
| Duplication | 12% | <5% | ⚠️ Above |
| Avg File Size | 180 lines | <200 lines | ✅ On Target |

## Grade Distribution

- A (108+): 36 skills (72%)
- B (96-107): 12 skills (24%)
- C (84-95): 2 skills (4%)

## Top Priority Actions

1. **Consolidate BDD family** - 6 skills → 1 aggregation
2. **Improve old-patterns** - Score 72 → 90+
3. **Reduce duplication** - 12% → <5%

## Trend (Last 4 Weeks)

| Week | A-Rate | Avg Score | Duplication |
|------|--------|-----------|-------------|
| W1 | 68% | 94 | 15% |
| W2 | 70% | 96 | 14% |
| W3 | 71% | 97 | 13% |
| W4 | 72% | 98 | 12% |

↑ Improving
```

## Real-time Monitoring

### Watch Script

```bash
#!/bin/bash
# watch-quality.sh - Real-time quality monitoring

watch -n 60 '
echo "=== Skill Quality Monitor ==="
echo "Time: $(date)"
echo ""
echo "Skills: $(find skills -name "SKILL.md" -not -path "*/.deprecated/*" | wc -l)"
echo "A-Grade: $(grep -l "grade: A" skills/*/SKILL.md 2>/dev/null | wc -l)"
echo "Pending: $(ls .context/analysis/*.md 2>/dev/null | wc -l) reports"
echo ""
echo "Recent Changes:"
git log --oneline -5 -- "skills/"
'
```

### Alert Thresholds

| Metric | Warning | Critical | Alert |
|--------|---------|----------|-------|
| A-grade rate | <80% | <70% | Slack + Email |
| Duplication | >15% | >25% | Slack |
| C-grade count | >3 | >5 | Email |
| File size | >400 | >600 | Slack |

## Export Formats

### JSON Export

```bash
# Export metrics as JSON
cat > .context/analysis/metrics.json <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "skills": {
    "total": $(find skills -name "SKILL.md" | wc -l),
    "aGrade": $(grep -l "grade: A" skills/*/SKILL.md | wc -l),
    "bGrade": $(grep -l "grade: B" skills/*/SKILL.md | wc -l),
    "cGrade": $(grep -l "grade: C" skills/*/SKILL.md | wc -l)
  },
  "quality": {
    "avgScore": 98,
    "duplication": 12
  }
}
EOF
```

### CSV Export

```bash
# Export skill scores as CSV
echo "skill,score,grade,lines" > skills-report.csv
for skill in skills/*/SKILL.md; do
  name=$(basename $(dirname $skill))
  lines=$(wc -l < $skill)
  echo "$name,,$lines" >> skills-report.csv
done
```

## See Also

- `reporting-analysis.md` - Interpreting reports
- `advanced-trends-analysis.md` - Historical tracking
- `scripts-ci-integration.md` - Automated reporting
