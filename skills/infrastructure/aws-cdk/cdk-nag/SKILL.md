---
name: cdk-nag
description: Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks.
---

# CDK Nag

## When to Use

Use this skill when CDK infrastructure must be validated against security/compliance guardrails.

## When Not to Use

Do not use this skill for Terraform-only repositories without AWS CDK constructs.

## Core Principles

1. Run compliance checks early in development.
2. Prefer fixing insecure resources over suppressing findings.
3. Use suppressions only with explicit, reviewable rationale.
4. Keep CI enforcement consistent with risk profile.

## Deterministic Workflow

1. Add relevant cdk-nag rule packs to the app.
2. Synthesize stacks and collect findings.
3. Fix failing resources where feasible.
4. **Checkpoint:** If findings remain after step 3, categorize each as fix / suppress / defer before proceeding. Do not advance until every finding has an owner and decision.
5. Add minimal, scoped suppressions when justified.
6. Re-run synth and CI checks before merge.

## Quick Commands

### Install cdk-nag

```bash
npm install --save-dev cdk-nag
```

Expected result: cdk-nag dependency available for CDK app.

### Run synth to surface findings

```bash
npx cdk synth
```

Expected result: nag findings shown during synthesis.

### Run synth for one stack

```bash
npx cdk synth MyStack
```

Expected result: targeted findings for the selected stack.

### Run tests and synth in CI style

```bash
npm test && npx cdk synth
```

Expected result: failing status when tests or nag checks fail.

### Evaluate this skill quality

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh cdk-nag --json
```

Expected result: updated skill score and grade.

## Code Examples

### Add a rule pack to a CDK app

```typescript
import { App, Aspects } from 'aws-cdk-lib';
import { AwsSolutionsChecks } from 'cdk-nag';

const app = new App();

// Apply the AWS Solutions rule pack to every stack in the app
Aspects.of(app).add(new AwsSolutionsChecks({ verbose: true }));
```

Expected result: all stacks synthesized with the `AwsSolutionsChecks` pack applied; findings printed to stdout during `cdk synth`.

### Add a scoped suppression on a specific resource

```typescript
import { NagSuppressions } from 'cdk-nag';

// Suppress a single rule on a specific construct, not the whole stack
NagSuppressions.addResourceSuppressions(
  myBucket,
  [
    {
      id: 'AwsSolutions-S1',
      reason:
        'Server access logging disabled intentionally: bucket stores only ephemeral build artifacts ' +
        'with no PII; access is restricted to the CI role via bucket policy. ' +
        'Risk accepted and documented in ADR-042.',
    },
  ],
);
```

Expected result: only `myBucket` is exempted from `AwsSolutions-S1`; all other resources and rules remain enforced.

## Anti-Patterns

### NEVER suppress findings without a concrete security rationale

**WHY:** Unjustified suppressions hide unresolved risk.

**BAD:** `reason: "false positive"` with no evidence.
**GOOD:** `reason: "resource isolated in private subnet; compensating controls documented"`.

**Consequence:** Audit posture weakens and real issues stay unresolved.

### NEVER apply broad stack-level suppressions for convenience

**WHY:** Broad suppressions mask unrelated violations.

**BAD:** Suppress an entire rule for every resource in a stack.
**GOOD:** Scope suppression to exact resource and finding.

**Consequence:** New regressions pass undetected.

### NEVER enable production rule packs only at release time

**WHY:** Late checks create expensive rework.

**BAD:** Add strict rule packs only before deployment.
**GOOD:** Run target rule packs continuously in feature branches.

**Consequence:** Security defects are found too late.

### NEVER ignore recurring nag violations in CI

**WHY:** Repeated failures indicate systemic misconfiguration.

**BAD:** Re-run pipeline until flake passes without remediation.
**GOOD:** Fix root cause or add justified suppression once.

**Consequence:** Compliance debt accumulates quickly.

## References

- [Implementation Guide](references/implementation-guide.md) — full setup walkthrough, rule pack selection, and stack-level vs construct-level application
- [Rule Packs](references/rule-packs.md) — AwsSolutionsChecks, NIST 800-53, HIPAA, PCI-DSS: what each enforces and when to use it
- [Suppression Guide](references/suppression-guide.md) — addResourceSuppressions vs addStackSuppressions, rationale templates, and audit-friendly patterns
- [Troubleshooting](references/troubleshooting.md) — common synth failures, false positives, and rule ID lookup
- [Rule Evolution](references/rule-evolution.md) — tracking deprecated rules, new rules in version upgrades, and migration paths
- [Integration Patterns](references/integration-patterns.md) — CI/CD wiring, pre-commit hooks, and multi-account enforcement strategies
