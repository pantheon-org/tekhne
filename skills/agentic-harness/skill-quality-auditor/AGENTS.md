# Skill Quality Auditor - Navigation Guide

This skill provides automated quality evaluation and maintenance for your skill collection using the 9-dimension quality framework.

## Overview

**Total Files**: 25 reference files + 17 scripts + 2 templates + 2 schemas + 1 requirements spec
**Categories**: 7 (Framework, Duplication, Aggregation, Remediation, Scripts, Reporting, Advanced)
**Pattern**: Navigation Hub (SKILL.md) + Expert References
**Origin**: Consolidates skill-quality-auditor + skill-aggregation-pattern + skill-quality-auditor

## Usage Instructions

1. **Start with SKILL.md** - Read the navigation hub (you just did)
2. **Identify your task** - Choose category based on priority
3. **Load specific references** - Read only what you need for current task
4. **Apply methodology** - Follow workflow in reference file
5. **Generate outputs** - Use scripts to automate evaluation

## Reference Categories

### Framework (CRITICAL Priority)

Core evaluation methodology based on skill-quality-auditor:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `framework-dimensions.md` | Canonical upstream skill-quality-auditor mapping | ~60 | Validating framework intent/source |
| `framework-dimensions.md` | 9-dimension evaluation criteria | ~510 | Evaluating any skill |
| `framework-scoring-rubric.md` | Detailed scoring methodology | ~280 | Understanding scores |
| `framework-quality-standards.md` | A-grade requirements | ~220 | Setting quality goals |
| `scoring-rubric.md` | Concise scoring reference | ~120 | Quick score lookup |
| `skill-template.md` | Canonical SKILL.md structure template | ~80 | Authoring or refactoring a skill |
| `quality-thresholds-scoring.md` | CI gate thresholds and grade bands | ~150 | Configuring CI quality gates |
| `detailed-anti-patterns.md` | NEVER/WHY/BAD/GOOD failure modes | ~300 | Explaining low scores or writing remediation guidance |
| `dimension-analysis-template.md` | Per-dimension analysis scaffold | ~100 | Structuring a dimension-by-dimension audit |
| `skill-taxonomy.md` | Domain classification criteria | ~120 | Categorising skills or detecting misclassification |

**Load first**: framework-dimensions.md

### Duplication Detection (HIGH Priority)

Find and remediate redundant content:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `duplication-detection-algorithm.md` | Similarity detection (>20%) | ~320 | Finding duplicates |
| `duplication-remediation.md` | Fix strategies | ~240 | Consolidating content |
| `advanced-pattern-recognition.md` | Trigger pattern density and keyword analysis | ~200 | Scoring D7 or improving description keywords |

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
| `assets/templates/remediation-plan-template.yaml` | Plan template | ~110 | Formatting plans |
| `assets/schemas/remediation-plan.schema.json` | JSON Schema for plan validation | ~250 | Validating plans |

**Usage**:

```bash
# Generate plan from audit
./scripts/generate-remediation-plan.sh <skill-name> [target-score]

# Validate plan against schema
./scripts/validate-remediation-plan.sh .context/plans/<skill>-remediation-plan.md
```

**Output**: `.context/plans/<skill-name>-remediation-plan.md`

### Scripts (MEDIUM Priority)

Automation tools for quality enforcement:

| File | Purpose | Lines | When to Read |
|------|---------|-------|--------------|
| `scripts-audit-workflow.md` | Complete audit process | ~280 | Running full audit |
| `scripts-ci-integration.md` | GitHub Actions, GitLab CI | ~310 | Setting up automation |
| `examples-audit-workflows.md` | Input/output pairs and workflow examples | ~200 | Learning by example |

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
| `tessl-compliance-framework.md` | Tessl registry compliance checks | ~280 | Preparing for public registry submission |

**Use when**: Standard metrics insufficient

## Complete File Listing

```text
skills/agentic-harness/skill-quality-auditor/
├── SKILL.md                                          # Navigation hub
├── AGENTS.md                                         # This file
├── tile.json                                         # Tessl tile manifest
├── evals/                                            # Tessl eval scenarios
├── assets/
│   ├── templates/
│   │   ├── review-report-template.yaml               # Audit report format
│   │   └── remediation-plan-template.yaml            # Remediation plan format
│   ├── schemas/
│   │   ├── review-report.schema.json                 # JSON Schema for review reports
│   │   └── remediation-plan.schema.json              # JSON Schema for plan validation
│   └── requirements/
│       └── review-report.requirements.json           # Custom validation spec for reports
├── references/
│   ├── framework-dimensions.md           # CRITICAL
│   ├── framework-dimensions.md            # CRITICAL
│   ├── framework-scoring-rubric.md                   # CRITICAL
│   ├── framework-quality-standards.md                # CRITICAL
│   ├── scoring-rubric.md                             # CRITICAL
│   ├── skill-template.md                             # CRITICAL
│   ├── quality-thresholds-scoring.md                 # HIGH
│   ├── detailed-anti-patterns.md                     # HIGH
│   ├── dimension-analysis-template.md                # HIGH
│   ├── skill-taxonomy.md                             # HIGH
│   ├── duplication-detection-algorithm.md            # HIGH
│   ├── duplication-remediation.md                    # HIGH
│   ├── aggregation-pattern.md                        # HIGH
│   ├── aggregation-implementation.md                 # HIGH
│   ├── remediation-planning.md                       # HIGH
│   ├── advanced-pattern-recognition.md               # MEDIUM
│   ├── scripts-audit-workflow.md                     # MEDIUM
│   ├── scripts-ci-integration.md                     # MEDIUM
│   ├── examples-audit-workflows.md                   # MEDIUM
│   ├── reporting-analysis.md                         # MEDIUM
│   ├── reporting-dashboards.md                       # MEDIUM
│   ├── tessl-compliance-framework.md                 # LOW
│   ├── advanced-trends-analysis.md                   # LOW
│   └── advanced-custom-metrics.md                    # LOW
└── scripts/
    ├── evaluate.sh                                   # Primary evaluator
    ├── audit-skills.sh                               # Full collection audit
    ├── audit-per-skill.sh                            # Per-skill audit runner
    ├── batch-audit.sh                                # Batch multiple skills
    ├── detect-duplication.sh                         # Duplication finder
    ├── detect-duplication-enhanced.sh                # Enhanced duplication detection
    ├── generate-remediation-plan.sh                  # Remediation plan generator
    ├── validate-remediation-plan.sh                  # Plan schema validator
    ├── validate-review-format.sh                     # Review report validator
    ├── validate-skill-artifacts.sh                   # Artifact convention validator
    ├── check-consistency.sh                          # Consistency checker
    ├── tessl-compliance-check.sh                     # Tessl registry compliance
    ├── pattern-recognition-pipeline.sh              # Pattern analysis pipeline
    ├── ml-pattern-detection.sh                       # ML-based pattern detection
    ├── semantic-analysis.sh                          # Semantic content analysis
    ├── plan-aggregation.sh                           # Aggregation planner
    └── prune-audits.sh                               # Audit output cleanup

Total: 25 references + 17 scripts + 2 templates + 2 schemas + 1 requirements spec
```

## Navigation Workflow

### For Quality Evaluation

1. Load `framework-dimensions.md`
2. Load `framework-dimensions.md` when checking intent or edge cases
3. Load `framework-scoring-rubric.md`
4. Apply to skill, calculate score
5. Compare against `framework-quality-standards.md`

### For Duplication Detection

1. Run `./scripts/detect-duplication.sh`
2. Review generated report
3. Load `duplication-remediation.md` for fixes
4. Load `aggregation-pattern.md` if >20% similarity

### For Creating Aggregations

1. Load `aggregation-pattern.md` (learn pattern)
2. Load `aggregation-implementation.md` (step-by-step)
3. Follow 6-step process
4. Verify with `framework-quality-standards.md`

### For Remediation Planning

1. Run `./scripts/evaluate.sh <skill-name> --json`
2. Load `remediation-planning.md` for guidance
3. Use `assets/templates/remediation-plan-template.yaml`
4. Save plan to `.context/plans/<skill-name>-remediation-plan.md`

### For Eval Validation (D9)

1. Check if skill has `evals/` directory
2. If missing, use `creating-eval-scenarios` skill to generate scenarios
3. Run `tessl eval run <tile-path>` to execute evals
4. Verify `summary.json` shows `coverage_percentage >= 80`
5. Re-run `./scripts/evaluate.sh <skill-name> --json` to confirm D9 score

### For Automation Setup

1. Load `scripts-audit-workflow.md`
2. Load `scripts-ci-integration.md`
3. Configure CI/CD pipeline
4. Test with manual audit first

## Success Criteria

After using this skill, you should have:

- All skills evaluated with 9-dimension scores
- Duplication report showing <5% redundancy
- Remediation plans for all skills below target grade
- Aggregation candidates identified with ROI
- Quality dashboard tracking metrics
- CI/CD quality gates enforcing A-grade minimum

## See Also

- **supabase-postgres-best-practices** - Pattern inspiration (A-grade)
- **skill-quality-auditor** - Foundation framework this implements
- **skill-harvester** - Creates new skills (audit after creation)
- **reducing-entropy** - Minimalism philosophy applied to skill collections
- **creating-eval-scenarios** - Tessl eval scenario generation (D9 Eval Validation)
