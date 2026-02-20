# Skill Quality Auditor - Navigation Guide

This skill provides automated quality evaluation and maintenance for your skill collection using the skill-judge framework.

## Overview

**Total Files**: 13 reference files + 2 scripts  
**Categories**: 6 (Framework, Duplication, Aggregation, Scripts, Reporting, Advanced)  
**Pattern**: Navigation Hub (SKILL.md) + Expert References  
**Origin**: Consolidates skill-quality-auditor + skill-aggregation-pattern

## Usage Instructions

1. **Start with SKILL.md** - Read the navigation hub (you just did)
2. **Identify your task** - Choose category based on priority
3. **Load specific references** - Read only what you need for current task
4. **Apply methodology** - Follow workflow in reference file
5. **Generate outputs** - Use scripts to automate evaluation

## Reference Categories

### Framework (CRITICAL Priority)

Core evaluation methodology based on skill-judge:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `framework-skill-judge-dimensions.md` | 8-dimension evaluation criteria | ~410 | Evaluating any skill |
| `framework-scoring-rubric.md` | Detailed scoring methodology | ~280 | Understanding scores |
| `framework-quality-standards.md` | A-grade requirements | ~220 | Setting quality goals |

**Load first**: framework-skill-judge-dimensions.md

### Duplication Detection (HIGH Priority)

Find and remediate redundant content:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `duplication-detection-algorithm.md` | Similarity detection (>20%) | ~320 | Finding duplicates |
| `duplication-remediation.md` | Fix strategies | ~240 | Consolidating content |

**Key metric**: >20% similarity = aggregation candidate

### Aggregation Planning (HIGH Priority)

Consolidate related skills using Navigation Hub pattern:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `aggregation-pattern.md` | Navigation Hub + References | ~500 | Learning pattern |
| `aggregation-implementation.md` | 6-step implementation | ~490 | Creating aggregations |

**Success rate**: 96%+ size reduction, <5% duplication

### Scripts (MEDIUM Priority)

Automation tools for quality enforcement:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `scripts-audit-workflow.md` | Complete audit process | ~280 | Running full audit |
| `scripts-ci-integration.md` | GitHub Actions, GitLab CI | ~310 | Setting up automation |

**Executable scripts**: See `scripts/` directory

### Reporting (MEDIUM Priority)

Analysis and visualization of audit results:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `reporting-analysis.md` | Interpreting audit results | ~260 | Understanding reports |
| `reporting-dashboards.md` | Visualizations | ~290 | Creating dashboards |

**Output location**: `.context/analysis/skill-audit-*.md`

### Advanced (LOW Priority)

Optional features for power users:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `advanced-trends-analysis.md` | Historical tracking | ~310 | Monitoring changes |
| `advanced-custom-metrics.md` | Domain-specific evaluation | ~270 | Custom frameworks |

**Use when**: Standard metrics insufficient

## Complete File Listing

```
.agents/skills/skill-quality-auditor/
├── SKILL.md                                    # Navigation hub
├── AGENTS.md                                   # This file
├── references/
│   ├── framework-skill-judge-dimensions.md     # CRITICAL
│   ├── framework-scoring-rubric.md             # CRITICAL
│   ├── framework-quality-standards.md          # CRITICAL
│   ├── duplication-detection-algorithm.md      # HIGH
│   ├── duplication-remediation.md              # HIGH
│   ├── aggregation-pattern.md                  # HIGH
│   ├── aggregation-implementation.md           # HIGH
│   ├── scripts-audit-workflow.md               # MEDIUM
│   ├── scripts-ci-integration.md               # MEDIUM
│   ├── reporting-analysis.md                   # MEDIUM
│   ├── reporting-dashboards.md                 # MEDIUM
│   ├── advanced-trends-analysis.md             # LOW
│   └── advanced-custom-metrics.md              # LOW
└── scripts/
    ├── audit-skills.sh                         # Full audit
    └── detect-duplication.sh                   # Duplication finder

Total: 15 files (13 references + 2 scripts)
```

## Navigation Workflow

### For Quality Evaluation
1. Load `framework-skill-judge-dimensions.md`
2. Load `framework-scoring-rubric.md`
3. Apply to skill, calculate score
4. Compare against `framework-quality-standards.md`

### For Duplication Detection
1. Run `scripts/detect-duplication.sh`
2. Review generated report
3. Load `duplication-remediation.md` for fixes
4. Load `aggregation-pattern.md` if >20% similarity

### For Creating Aggregations
1. Load `aggregation-pattern.md` (learn pattern)
2. Load `aggregation-implementation.md` (step-by-step)
3. Follow 6-step process
4. Verify with `framework-quality-standards.md`

### For Automation Setup
1. Load `scripts-audit-workflow.md`
2. Load `scripts-ci-integration.md`
3. Configure CI/CD pipeline
4. Test with manual audit first

## Success Criteria

After using this skill, you should have:
- ✅ All skills evaluated with 8-dimension scores
- ✅ Duplication report showing <5% redundancy
- ✅ Aggregation candidates identified with ROI
- ✅ Quality dashboard tracking metrics
- ✅ CI/CD quality gates enforcing A-grade minimum

## See Also

- **supabase-postgres-best-practices** - Pattern inspiration (A-grade, 108/120)
- **skill-judge** - Foundation framework this implements
- **skill-harvester** - Creates new skills (audit after creation)
- **reducing-entropy** - Minimalism philosophy applied to skill collections
