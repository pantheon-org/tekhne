---
category: framework
priority: CRITICAL
source: scoring rubric analysis
---

# Quality Gates and Scoring Thresholds

Complete scoring framework and threshold interpretation for skill quality assessment.

## Quality Gates and Thresholds

| Score Range | Grade | Status | Action Required |
|-------------|-------|--------|----------------|
| 126+ | A | Production Ready | Tessl registry eligible |
| 112-125 | B/B+ | Good Quality | Minor improvements |
| 98-111 | C/C+ | Passing | Remediation recommended |
| 91-97 | D | Below Standard | Major improvements required |
| <91 | F | Failing | Complete rework needed |

## Score Interpretation

- **Knowledge Delta drives overall score** - prioritize this dimension first
- **Anti-patterns prevent catastrophic failures** - second priority
- **Progressive disclosure improves usability** - third priority
- **Specification compliance ensures consistency** - foundational requirement

## Dimension Weighting

| Dimension | Max Points | Priority | Focus Area |
|-----------|------------|----------|------------|
| Knowledge Delta | 20 | HIGHEST | Expert-only content |
| Eval Validation | 20 | HIGHEST | Runtime validation via tessl evals |
| Mindset + Procedures | 15 | HIGH | Philosophy + workflows |
| Anti-Pattern Quality | 15 | HIGH | NEVER + WHY + consequences |
| Practical Usability | 15 | HIGH | Concrete examples |
| Specification Compliance | 15 | MEDIUM | Description field critical |
| Progressive Disclosure | 15 | MEDIUM | Hub + references |
| Freedom Calibration | 15 | MEDIUM | Appropriate rigidity |
| Pattern Recognition | 10 | LOW | Activation keywords |

## A-Grade Requirements (>=126)

### Critical Success Factors

1. **Knowledge Delta >=17/20**: Expert content with <10% redundancy
2. **Eval Validation >=17/20**: Complete evals with >=80% instruction coverage
3. **Anti-Patterns >=13/15**: Multiple NEVER statements with consequences
4. **Progressive Disclosure >=13/15**: Navigation hub + references structure
5. **Strong Activation**: Comprehensive description with trigger keywords

### Common A-Grade Patterns

- Scores consistently 13+ across most dimensions
- Knowledge Delta typically 16-19 points
- Perfect or near-perfect specification compliance
- Clear navigation with references directory

## Improvement Priority Matrix

### High Impact, Low Effort

- Add missing NEVER statements to anti-patterns
- Enhance description field with activation keywords
- Create proper references directory structure
- Add concrete code examples
- Create eval scenarios using `creating-eval-scenarios` skill

### High Impact, High Effort

- Restructure for progressive disclosure (hub approach)
- Add expert-only content to boost Knowledge Delta
- Create comprehensive workflow documentation
- Develop specialized troubleshooting guides
- Achieve >=80% instruction coverage in evals

### Medium Priority

- Improve freedom calibration balance
- Enhance practical usability examples
- Refine mindset and procedure clarity
- Optimize pattern recognition triggers

## Score Optimization Strategies

### For 105-125 Range (B/B+ -> A)

Focus on the "Big 4": Knowledge Delta, Eval Validation, Anti-Patterns, Progressive Disclosure

### For 98-104 Range (C/C+ -> B)

Address specification compliance, practical usability, and create eval scenarios first

### For <98 Range (Failing)

Complete restructure required - start with navigation hub, expert content, and eval scenarios
