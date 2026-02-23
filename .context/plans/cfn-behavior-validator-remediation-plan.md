---
plan_date: 2026-02-23
skill_name: cfn-behavior-validator
source_audit: .context/audits/cfn-behavior-validator-audit-2026-02-22.md
---

# Remediation Plan: cfn-behavior-validator

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 93/120 | 105/120 |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Medium (S-M) | - |

**Focus Areas**: Progressive disclosure (D5), Pattern recognition (D7), Anti-pattern quality (D3)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Large SKILL.md file (292 lines, 0 refs) | High | D5 (10/15) | Hard to maintain and navigate |
| Vague trigger phrases | Medium | D7 (6/10) | Skill may not activate correctly |
| Moderate anti-pattern quality | Medium | D3 (11/15) | Some failure modes not documented |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: High

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 1.1: Create references directory structure

**File**: `skills/cfn-behavior-validator/references/`

Create the following reference files:

```
skills/cfn-behavior-validator/
├── SKILL.md (hub, ~100 lines)
├── references/
│   ├── behavior-tests.md
│   ├── test-templates.md
│   └── troubleshooting.md
```

#### Step 1.2: Extract deep content to references

**File**: `skills/cfn-behavior-validator/references/behavior-tests.md`

Move detailed test case examples and complex scenarios from SKILL.md to this reference.

**File**: `skills/cfn-behavior-validator/references/test-templates.md`

Move test templates and fixture patterns.

**File**: `skills/cfn-behavior-validator/references/troubleshooting.md`

Move error resolution and debugging guidance.

#### Step 1.3: Update SKILL.md as navigation hub

**File**: `skills/cfn-behavior-validator/SKILL.md`

Replace extracted content with concise summaries and links:

````markdown
## Behavior Testing

Quick validation of CloudFormation change sets before deployment.

### Quick Start
```bash
npx cfn-behavior-validator --template cdk.out/MyStack.template.json
```

For detailed test configuration, see [Behavior Tests](references/behavior-tests.md).

## Test Templates

Common assertion patterns for resource changes:

- Resource creation detection
- Property change validation
- Conditional update behavior

See [Test Templates](references/test-templates.md) for full examples.

## Troubleshooting

Common issues and resolutions:

- "Template not found" → Check CDK synth output path
- "Invalid change set" → Verify stack exists

See [Troubleshooting](references/troubleshooting.md) for detailed guidance.
````

---

### Phase 2: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 2.1: Expand frontmatter description

**File**: `skills/cfn-behavior-validator/SKILL.md`

```yaml
---
name: cfn-behavior-validator
description: |
  Validate CloudFormation change sets and template behaviors before deployment.
  Use when: testing CDK/CFN template changes, validating update behaviors,
  checking for resource replacement, verifying conditional logic,
  or setting up pre-deployment validation pipelines.
  
  Keywords: cfn-behavior-validator, CloudFormation validation, change set,
  template testing, pre-deployment check, resource replacement, update behavior
---
```

#### Step 2.2: Add "Use When" section

```markdown
## Use When

- "Test CloudFormation changes before deploy"
- "Validate CDK template behavior"
- "Check if resource will be replaced"
- "Set up CFN validation pipeline"
- "Debug CloudFormation update failures"
- NOT for: Runtime validation, deployed resource inspection
```

---

### Phase 3: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 11/15 to 14/15 (+3 points)

#### Step 3.1: Add missing anti-patterns

**File**: `skills/cfn-behavior-validator/SKILL.md`

````markdown
## Anti-Patterns

### NEVER: Skip behavior validation for production stacks

WHY: Undetected resource replacements cause downtime.

BAD:
```bash
cdk deploy --no-rollback # Deploy without validation
```

GOOD:
```bash
cdk synth
npx cfn-behavior-validator --template cdk.out/*.template.json
cdk deploy
```

### NEVER: Test only happy path scenarios

WHY: Edge cases often cause production incidents.

BAD:
```ts
// Only test creation
test('creates resource', () => { ... });
```

GOOD:
```ts
test('creates resource', () => { ... });
test('handles property update without replacement', () => { ... });
test('handles property update with replacement', () => { ... });
test('handles deletion', () => { ... });
```
````

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-behavior-validator --json
bunx markdownlint-cli2 "skills/cfn-behavior-validator/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 14/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| SKILL.md line count | <= 120 lines |
| References created | >= 2 files |
| Overall Score | >= 105/120 (B+) |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | M | 1 hour |
| Phase 2: Triggers | S | 20 min |
| Phase 3: Anti-patterns | S | 30 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cfn-behavior-validator/
```
