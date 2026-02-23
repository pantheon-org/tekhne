---
plan_date: 2026-02-23
skill_name: cfn-template-compare
source_audit: .context/audits/cfn-template-compare-audit-2026-02-22.md
---

# Remediation Plan: cfn-template-compare

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 99/120 | 108/120 |
| **Grade** | B | A |
| **Priority** | Low | - |
| **Effort** | Small (S) | - |

**Focus Areas**: Anti-pattern quality (D3), Progressive disclosure (D5)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Large SKILL.md file (458 lines, 2 refs) | Medium | D5 (10/15) | Navigation complexity |
| Moderate anti-pattern quality | Medium | D3 (9/15) | Some failure modes not documented |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 9/15 to 14/15 (+5 points)

#### Step 1.1: Add explicit anti-patterns

**File**: `skills/cfn-template-compare/SKILL.md`

Add to existing anti-patterns section:

````markdown
## Anti-Patterns

### NEVER: Compare templates from different CDK versions

WHY: Version differences in synthesized output cause false drift detection.

BAD:
```bash
# Local CDK v2.80 vs deployed from v2.75
cdk synth && compare-templates local.json deployed.json
```

GOOD:
```bash
# Ensure CDK version matches
npx cdk@2.75 synth && compare-templates ...
```

### NEVER: Skip normalization before comparison

WHY: Intrinsic functions and logical IDs may differ without semantic changes.

BAD:
```bash
diff template1.json template2.json # Raw diff!
```

GOOD:
```bash
# Normalize templates first
normalize-template template1.json > norm1.json
normalize-template template2.json > norm2.json
compare-templates norm1.json norm2.json
```

### NEVER: Ignore property ordering differences

WHY: CloudFormation does not guarantee property order.

BAD:
```ts
expect(template1).toEqual(template2); // Fails on ordering
```

GOOD:
```ts
expect(normalize(template1)).toEqual(normalize(template2));
// Or use deep equality with ordering ignored
```

### NEVER: Compare without accounting for dynamic values

WHY: Auto-generated names, timestamps, and random IDs always differ.

BAD:
```bash
compare-templates --strict local.json deployed.json
# Reports differences for dynamic values!
```

GOOD:
```bash
compare-templates --ignore-patterns '.*PhysicalId.*,.*Timestamp.*' local.json deployed.json
```
````

---

### Phase 2: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Create additional reference files

**File**: `skills/cfn-template-compare/references/`

```
skills/cfn-template-compare/
├── SKILL.md (hub, ~150 lines)
├── references/
│   ├── normalization-rules.md (new)
│   ├── drift-detection.md (new)
│   └── [existing references]
```

#### Step 2.2: Extract normalization details

**File**: `skills/cfn-template-compare/references/normalization-rules.md`

Move detailed normalization logic and edge cases from SKILL.md:

```markdown
# Template Normalization Rules

## Logical ID Normalization

CDK generates logical IDs from construct paths. Comparison must handle:

- Path-based ID generation
- Hash suffixes for uniqueness
- Cross-stack references

## Property Ordering

Standard ordering for comparison:

1. Type
2. Properties (alphabetical)
3. DependsOn
4. Metadata
5. Condition
```

#### Step 2.3: Extract drift detection details

**File**: `skills/cfn-template-compare/references/drift-detection.md`

Move drift detection patterns and CloudFormation-specific considerations.

#### Step 2.4: Update SKILL.md hub

Replace extracted content with:

```markdown
## Normalization

Templates are normalized before comparison to handle:

- Logical ID differences
- Property ordering
- Dynamic values

See [Normalization Rules](references/normalization-rules.md) for details.

## Drift Detection

Compare deployed resources against expected configuration.

See [Drift Detection](references/drift-detection.md) for implementation.
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-template-compare --json
bunx markdownlint-cli2 "skills/cfn-template-compare/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D5 Progressive Disclosure | Score >= 14/15 |
| SKILL.md line count | <= 180 lines |
| References count | >= 4 files |
| Overall Score | >= 108/120 (A) |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | S | 30 min |
| Phase 2: Disclosure | S | 45 min |
| **Total** | **S** | **1.25 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cfn-template-compare/
```
