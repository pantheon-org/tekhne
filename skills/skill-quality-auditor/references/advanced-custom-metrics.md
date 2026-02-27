---
category: advanced
priority: LOW
source: custom quality frameworks
---

# Custom Metrics

Domain-specific evaluation frameworks for specialized skill quality assessment.

## Overview

**Purpose**: Extend skill-judge framework with custom metrics  
**Use Cases**: Domain-specific requirements, organizational standards  
**Priority**: LOW - Use when standard metrics insufficient

## When to Add Custom Metrics

Consider custom metrics when:

- Standard dimensions don't capture domain needs
- Organization has specific compliance requirements
- Skills serve specialized audiences
- Industry-specific quality standards apply

## Custom Metric Design

### Metric Structure

Each custom metric should define:

1. **Name**: Clear, descriptive identifier
2. **Description**: What the metric measures
3. **Max Points**: Contribution to total score
4. **Evaluation Method**: How to assess
5. **Scoring Rubric**: Point thresholds

### Example Custom Metrics

#### D9: Security Compliance (15 points)

For skills handling security-sensitive operations:

```markdown
## D9: Security Compliance (15 points)

### Purpose
Ensure skills follow security best practices.

### Evaluation Method
1. Check for security warnings
2. Verify no hardcoded secrets
3. Confirm input validation guidance
4. Review authentication patterns

### Scoring Rubric
| Score | Criteria |
|-------|----------|
| 13-15 | All security checks passed |
| 10-12 | Minor issues (warnings) |
| 7-9 | Moderate issues |
| 0-6 | Critical security flaws |
```

#### D10: Accessibility (10 points)

For skills involving UI/UX:

```markdown
## D10: Accessibility Compliance (10 points)

### Purpose
Ensure skills include accessibility guidance.

### Evaluation Method
1. Check for WCAG references
2. Verify screen reader guidance
3. Confirm keyboard navigation
4. Review color contrast guidance

### Scoring Rubric
| Score | Criteria |
|-------|----------|
| 9-10 | Comprehensive accessibility |
| 7-8 | Basic accessibility |
| 5-6 | Minimal accessibility |
| 0-4 | No accessibility guidance |
```

#### D11: Performance (10 points)

For skills involving performance-critical code:

```markdown
## D11: Performance Guidance (10 points)

### Purpose
Ensure skills include performance considerations.

### Evaluation Method
1. Check for performance warnings
2. Verify benchmark examples
3. Confirm optimization guidance
4. Review anti-patterns

### Scoring Rubric
| Score | Criteria |
|-------|----------|
| 9-10 | Comprehensive performance |
| 7-8 | Good performance guidance |
| 5-6 | Basic performance notes |
| 0-4 | No performance guidance |
```

## Implementation

### Custom Evaluation Script

```typescript
// evaluate-custom.ts

interface CustomMetric {
  name: string;
  maxPoints: number;
  evaluate(skillPath: string): number;
}

const customMetrics: CustomMetric[] = [
  {
    name: "Security Compliance",
    maxPoints: 15,
    evaluate: (skillPath) => {
      // Custom evaluation logic
      const content = readFile(skillPath);
      let score = 0;
      
      if (content.includes("NEVER hardcode")) score += 5;
      if (content.includes("input validation")) score += 5;
      if (content.includes("authentication")) score += 5;
      
      return score;
    }
  },
  {
    name: "Accessibility",
    maxPoints: 10,
    evaluate: (skillPath) => {
      const content = readFile(skillPath);
      let score = 0;
      
      if (content.includes("WCAG")) score += 3;
      if (content.includes("screen reader")) score += 3;
      if (content.includes("keyboard")) score += 2;
      if (content.includes("contrast")) score += 2;
      
      return score;
    }
  }
];

function evaluateWithCustom(skillPath: string): ScoreReport {
  // Standard evaluation
  const standardScore = evaluateStandard(skillPath);
  
  // Custom metrics
  const customScores = customMetrics.map(m => ({
    name: m.name,
    score: m.evaluate(skillPath),
    max: m.maxPoints
  }));
  
  return {
    standard: standardScore,
    custom: customScores,
    total: standardScore + customScores.reduce((a, b) => a + b.score, 0),
    maxTotal: 120 + customScores.reduce((a, b) => a + b.max, 0)
  };
}
```

### Custom Configuration

```yaml
# custom-metrics.yaml
metrics:
  - name: security-compliance
    enabled: true
    maxPoints: 15
    appliesTo:
      - "security-*"
      - "auth-*"
      - "*-api"
    
  - name: accessibility
    enabled: true
    maxPoints: 10
    appliesTo:
      - "ui-*"
      - "frontend-*"
      - "design-*"
    
  - name: performance
    enabled: true
    maxPoints: 10
    appliesTo:
      - "*-optimization"
      - "*-performance"
      - "database-*"
```

## Organization-Specific Standards

### Enterprise Compliance

For organizations with regulatory requirements:

```markdown
## Enterprise Compliance Metrics

### D12: Documentation Standards (10 points)

**Applies to**: All skills

**Requirements**:
- [ ] Version history maintained
- [ ] Owner/contact documented
- [ ] Review date specified
- [ ] Approval workflow followed

**Scoring**:
- 10: All compliance requirements met
- 7: Minor documentation gaps
- 4: Significant gaps
- 0: Non-compliant
```

### Industry-Specific

#### Healthcare (HIPAA)

```markdown
## D13: HIPAA Compliance (15 points)

**Applies to**: Skills handling PHI

**Requirements**:
- PHI handling guidance
- Encryption requirements
- Audit logging patterns
- Access control guidance
```

#### Finance (PCI-DSS)

```markdown
## D14: PCI-DSS Compliance (15 points)

**Applies to**: Skills handling payment data

**Requirements**:
- Card data handling
- Encryption standards
- Secure transmission
- Storage requirements
```

## Metric Integration

### Modified Grade Scale

With custom metrics, adjust grade scale:

| Custom Points | New Total | A-Grade Threshold |
|---------------|-----------|-------------------|
| +15 | 135 | 122 (90%) |
| +25 | 145 | 131 (90%) |
| +35 | 155 | 140 (90%) |

### Combined Report

```markdown
## Skill Evaluation Report

### Standard Dimensions (120 points)
| Dimension | Score | Max |
|-----------|-------|-----|
| D1: Knowledge Delta | 18 | 20 |
| D2: Mindset | 14 | 15 |
| ... | ... | ... |
| D8: Usability | 14 | 15 |
| **Subtotal** | **102** | **120** |

### Custom Dimensions (25 points)
| Dimension | Score | Max |
|-----------|-------|-----|
| D9: Security | 13 | 15 |
| D10: Accessibility | 8 | 10 |
| **Subtotal** | **21** | **25** |

### Total
| Metric | Value |
|--------|-------|
| Total Score | 123/145 (85%) |
| Grade | B+ |
| Standard Grade | A- |
```

## Best Practices

### DO

- Start with standard metrics
- Add custom metrics only when necessary
- Document rationale for each metric
- Keep custom metrics focused
- Re-evaluate custom metrics quarterly

### DON'T

- Create metrics without clear purpose
- Add metrics that duplicate standard dimensions
- Make metrics too complex
- Ignore custom metric maintenance
- Apply custom metrics universally

## Metric Maintenance

### Quarterly Review

```markdown
## Custom Metrics Review - Q1 2026

### Active Metrics
| Metric | Skills Applied | Avg Score | Keep? |
|--------|----------------|-----------|-------|
| Security | 12 | 12/15 | Yes |
| Accessibility | 8 | 7/10 | Yes |
| Performance | 5 | 6/10 | Review |

### Recommendations
- Performance metric needs refinement
- Consider adding API documentation metric
- Remove rarely-used metrics
```

### Metric Deprecation

When a metric is no longer needed:

1. Mark as deprecated in config
2. Stop applying to new skills
3. Remove from reports after 1 quarter
4. Archive metric definition

## See Also

- `framework-skill-judge-dimensions.md` - Standard dimensions
- `framework-scoring-rubric.md` - Scoring methodology
- `advanced-trends-analysis.md` - Tracking custom metrics
