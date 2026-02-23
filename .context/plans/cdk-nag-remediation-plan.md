---
plan_date: 2026-02-23
skill_name: cdk-nag
source_audit: .context/audits/cdk-nag-audit-2026-02-22.md
---

# Remediation Plan: cdk-nag

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 84/120 | 100/120 |
| **Grade** | C | B |
| **Priority** | High | - |
| **Effort** | Medium (S) | - |

**Focus Areas**: Anti-pattern quality (D3), Pattern recognition (D7), Freedom calibration (D6)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern examples | High | D3 (8/15) | Security rules may be misapplied |
| Vague trigger phrases | High | D7 (6/10) | Skill may not activate for CDK security tasks |
| Unclear constraint balance | Medium | D6 (10/15) | Over/under-suppression of findings |
| Imprecise frontmatter | Medium | D4 (10/15) | Routing ambiguity |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add explicit security anti-patterns

**File**: `skills/cdk-nag/SKILL.md`

Add a dedicated Anti-Patterns section:

````markdown
## Anti-Patterns

### NEVER: Suppress cdk-nag findings without documentation

WHY: Creates audit gaps and security debt.

BAD:
```ts
// Just suppress it
Aspects.of(stack).add(new AwsSolutionsChecks());
NagSuppressions.addStackSuppressions(stack, [
  { id: 'AwsSolutions-IAM5', reason: 'TODO' } // Vague!
]);
```

GOOD:
```ts
// Suppress with explicit justification and expiry
NagSuppressions.addStackSuppressions(stack, [
  { 
    id: 'AwsSolutions-IAM5', 
    reason: 'Lambda requires S3 GetObject on bucket prefix. Review Q2 2026.',
    appliesTo: ['Resource::MyBucket/*']
  }
]);
```

### NEVER: Apply NagSuppressions at stack level for resource-specific issues

WHY: Masks legitimate findings for other resources.

BAD:
```ts
NagSuppressions.addStackSuppressions(stack, [
  { id: 'AwsSolutions-IAM5', reason: 'Broad wildcard needed' }
]); // Suppresses ALL IAM5 findings in stack!
```

GOOD:
```ts
NagSuppressions.addResourceSuppressions(myLambda, [
  { id: 'AwsSolutions-IAM5', reason: '...' }
], true); // Apply to specific resource only
```

### NEVER: Skip cdk-nag in CI/CD pipelines

WHY: Security regressions go undetected.

BAD:
```yaml
# .github/workflows/deploy.yml
- run: cdk deploy # No nag check!
```

GOOD:
```yaml
- run: cdk synth
- run: npx cdk-nag-synth # Fail on high/critical findings
- run: cdk deploy
```
````

#### Step 1.2: Add repository-specific risk tie-ins

Document common suppression patterns in this repository and when they're justified.

---

### Phase 2: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 2.1: Expand frontmatter description

**File**: `skills/cdk-nag/SKILL.md`

```yaml
---
name: cdk-nag
description: |
  CDK security linting with cdk-nag. Use when: setting up cdk-nag in CDK stacks,
  suppressing security findings, configuring AwsSolutions/NIST/PCI rule packs,
  troubleshooting NagSuppressions, or implementing security gates in CI/CD.
  
  Keywords: cdk-nag, NagSuppression, AwsSolutionsChecks, security linting,
  CDK security, NIST-800-53, PCI-DSS, HIPAA, NAG
---
```

#### Step 2.2: Add "Use When" section

```markdown
## Use When

- "Add cdk-nag to my CDK stack"
- "Suppress cdk-nag warning"
- "Configure AwsSolutionsChecks"
- "Fix NIST compliance findings"
- "cdk-nag CI/CD integration"
- NOT for: General CDK development without security focus
```

---

### Phase 3: Freedom Calibration (D6) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Clarify hard vs soft constraints

**File**: `skills/cdk-nag/SKILL.md`

Add constraint classification:

```markdown
## Constraint Levels

### Hard Constraints (NEVER violate)

- Never suppress `Critical` or `High` severity findings without CISO approval
- Never use `addStackSuppressions` for resource-specific issues
- Never skip cdk-nag in production deployment pipelines

### Soft Constraints (evaluate context)

- `Medium` severity findings may be suppressed with documented justification
- Use `appliesTo` to narrow suppression scope when possible
- Consider environment-specific rule packs (stricter for prod)
```

---

### Phase 4: Specification Compliance (D4) - Priority: Medium

**Target**: Increase from 10/15 to 12/15 (+2 points)

#### Step 4.1: Complete frontmatter

```yaml
---
name: cdk-nag
description: "[updated from Phase 2]"
version: 1.0.0
author: tekhne
tags: [cdk, security, linting, compliance, aws]
scope: CDK security validation and suppression management
---
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cdk-nag --json
bunx markdownlint-cli2 "skills/cdk-nag/SKILL.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| D6 Freedom Calibration | Score >= 13/15 |
| Overall Score | >= 100/120 (B) |
| No markdown lint errors | `bunx markdownlint-cli2` passes |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | S | 45 min |
| Phase 2: Triggers | S | 20 min |
| Phase 3: Constraints | S | 20 min |
| Phase 4: Frontmatter | S | 10 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cdk-nag/SKILL.md
```
