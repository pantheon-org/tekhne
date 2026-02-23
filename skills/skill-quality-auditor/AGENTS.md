# Skill Quality Auditor - Navigation Guide

This skill provides automated quality evaluation and maintenance for your skill collection using the skill-judge framework.

## Overview

**Total Files**: 15 reference files + 3 scripts + 2 templates  
**Categories**: 7 (Framework, Duplication, Aggregation, Remediation, Scripts, Reporting, Advanced)  
**Pattern**: Navigation Hub (SKILL.md) + Expert References  
**Origin**: Consolidates skill-quality-auditor + skill-aggregation-pattern + skill-judge

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
| `framework-skill-judge-canonical.md` | Canonical upstream skill-judge mapping | ~60 | Validating framework intent/source |
| `framework-skill-judge-dimensions.md` | 8-dimension evaluation criteria | ~410 | Evaluating any skill |
| `framework-scoring-rubric.md` | Detailed scoring methodology | ~280 | Understanding scores |
| `framework-quality-standards.md` | A-grade requirements | ~220 | Setting quality goals |

**Load first**: framework-skill-judge-dimensions.md (load canonical source when checking framework fidelity)

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

### Remediation Planning (HIGH Priority)

Create actionable improvement plans from audit results:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `remediation-planning.md` | Plan creation guide | ~300 | Creating remediation plans |
| `templates/remediation-plan-template.yaml` | Plan template | ~110 | Formatting plans |
| `schemas/remediation-plan.schema.json` | JSON Schema | ~250 | Validating plans |

**Usage**:

```bash
# Generate plan from audit
sh scripts/generate-remediation-plan.sh <skill-name> [target-score]

# Validate plan against schema
sh scripts/validate-remediation-plan.sh .context/plans/<skill>-remediation-plan.md
```

**Output**: `.context/plans/<skill-name>-remediation-plan.md`

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

```text
.agents/skills/skill-quality-auditor/
├── SKILL.md                                    # Navigation hub
├── AGENTS.md                                   # This file
├── templates/
│   ├── review-report-template.yaml             # Audit report format
│   └── remediation-plan-template.yaml           # Remediation plan format
├── schemas/
│   └── remediation-plan.schema.json            # JSON Schema for plan validation
├── references/
│   ├── framework-skill-judge-dimensions.md     # CRITICAL
│   ├── framework-skill-judge-canonical.md      # CRITICAL
│   ├── framework-scoring-rubric.md             # CRITICAL
│   ├── framework-quality-standards.md           # CRITICAL
│   ├── duplication-detection-algorithm.md       # HIGH
│   ├── duplication-remediation.md               # HIGH
│   ├── aggregation-pattern.md                   # HIGH
│   ├── aggregation-implementation.md            # HIGH
│   ├── remediation-planning.md                  # HIGH
│   ├── scripts-audit-workflow.md               # MEDIUM
│   ├── scripts-ci-integration.md               # MEDIUM
│   ├── reporting-analysis.md                    # MEDIUM
│   ├── reporting-dashboards.md                  # MEDIUM
│   ├── advanced-trends-analysis.md              # LOW
│   └── advanced-custom-metrics.md               # LOW
└── scripts/
    ├── audit-skills.sh                         # Full audit
    ├── detect-duplication.sh                   # Duplication finder
    ├── generate-remediation-plan.sh            # Remediation plan generator
    └── validate-remediation-plan.sh            # Plan schema validator

Total: 22 files (15 references + 4 scripts + 2 templates + 1 schema)
```

## Navigation Workflow

### For Quality Evaluation

1. Load `framework-skill-judge-dimensions.md`
2. Load `framework-skill-judge-canonical.md` when checking intent or edge cases
3. Load `framework-scoring-rubric.md`
4. Apply to skill, calculate score
5. Compare against `framework-quality-standards.md`

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

### For Remediation Planning

1. Run `sh scripts/evaluate.sh <skill-name> --json`
2. Load `remediation-planning.md` for guidance
3. Use `templates/remediation-plan-template.yaml`
4. Save plan to `.context/plans/<skill-name>-remediation-plan.md`

### For Automation Setup

1. Load `scripts-audit-workflow.md`
2. Load `scripts-ci-integration.md`
3. Configure CI/CD pipeline
4. Test with manual audit first

## Success Criteria

After using this skill, you should have:

- ✅ All skills evaluated with 8-dimension scores
- ✅ Duplication report showing <5% redundancy
- ✅ Remediation plans for all skills below target grade
- ✅ Aggregation candidates identified with ROI
- ✅ Quality dashboard tracking metrics
- ✅ CI/CD quality gates enforcing A-grade minimum

## See Also

- **supabase-postgres-best-practices** - Pattern inspiration (A-grade, 108/120)
- **skill-judge** - Foundation framework this implements
- **skill-harvester** - Creates new skills (audit after creation)
- **reducing-entropy** - Minimalism philosophy applied to skill collections
