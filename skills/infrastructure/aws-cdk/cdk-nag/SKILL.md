---
name: cdk-nag
description: Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks.
---

# CDK Nag

## When to Use

- Validating CDK infrastructure against security and compliance guardrails.
- Adding a rule pack such as `AwsSolutionsChecks` to a CDK app.
- Triaging findings reported during `npx cdk synth`.
- Writing justified, scoped suppressions for accepted findings.
- Wiring nag checks into CI/CD so insecure patterns fail the build.

## When Not to Use

- Terraform-only repositories without AWS CDK constructs.
- Runtime or account-level posture checks, which cdk-nag does not perform.
- Reviews of application code that never reach a CDK stack.

## Philosophy

- Run compliance checks early in development, not at release.
- Prefer fixing insecure resources over suppressing findings.
- Use suppressions only with explicit, reviewable rationale.
- Keep CI enforcement consistent with the risk profile.

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
pantheon-skill-auditor evaluate infrastructure/aws-cdk/cdk-nag --json
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

### NEVER suppress a finding when a fix is feasible

**WHY:** Fixing the resource removes the risk; a suppression only hides it.

**BAD:** Suppress `AwsSolutions-S10` instead of adding `enforceSSL: true`.
**GOOD:** Change the construct so the finding no longer fires.

**Consequence:** Fixable gaps stay in production behind a reason string.

### NEVER leave a remaining finding without an owner and a decision

**WHY:** An undecided finding is neither fixed nor accepted, so nobody is accountable.

**BAD:** Move on to suppressions with three findings still unclassified.
**GOOD:** Categorize each as fix / suppress / defer before advancing.

**Consequence:** Findings are lost between triage and merge.

### NEVER merge without re-running synth and the CI checks

**WHY:** A fix or suppression can change the findings that remain.

**BAD:** Merge straight after editing a suppression.
**GOOD:** Re-run `npx cdk synth` and the CI pipeline first.

**Consequence:** The merged change fails the pipeline or ships a new violation.

### NEVER apply a rule pack to one stack and assume the app is covered

**WHY:** Stacks without the aspect are never checked.

**BAD:** `Aspects.of(stack).add(new AwsSolutionsChecks())` on a single stack.
**GOOD:** `Aspects.of(app).add(new AwsSolutionsChecks({ verbose: true }))` once on the app.

**Consequence:** Unchecked stacks drift out of compliance silently.

### NEVER use this skill for Terraform-only repositories

**WHY:** cdk-nag inspects AWS CDK constructs and has nothing to analyse elsewhere.

**BAD:** Add cdk-nag guidance to a pure Terraform module.
**GOOD:** Use a Terraform-appropriate checker instead.

**Consequence:** Effort is wasted and the repository gains no real guardrail.

## References

- [Implementation Guide](references/implementation-guide.md) — full setup walkthrough, rule pack selection, and stack-level vs construct-level application
- [Rule Packs](references/rule-packs.md) — AwsSolutionsChecks, NIST 800-53, HIPAA, PCI-DSS: what each enforces and when to use it
- [Suppression Guide](references/suppression-guide.md) — addResourceSuppressions vs addStackSuppressions, rationale templates, and audit-friendly patterns
- [Troubleshooting](references/troubleshooting.md) — common synth failures, false positives, and rule ID lookup
- [Rule Evolution](references/rule-evolution.md) — tracking deprecated rules, new rules in version upgrades, and migration paths
- [Integration Patterns](references/integration-patterns.md) — CI/CD wiring, pre-commit hooks, and multi-account enforcement strategies
