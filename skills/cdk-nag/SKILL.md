---
name: cdk-nag
description: Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks. Keywords: cdk-nag, aws cdk, security linting, compliance, suppressions, aws solutions checks, nist, pci, hipaa.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
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
4. Add minimal, scoped suppressions when justified.
5. Re-run synth and CI checks before merge.

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
sh skills/skill-quality-auditor/scripts/evaluate.sh cdk-nag --json
```

Expected result: updated skill score and grade.

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

- `references/implementation-guide.md`
- `references/rule-packs.md`
- `references/suppression-guide.md`
- `references/troubleshooting.md`
- `references/rule-evolution.md`
- `references/integration-patterns.md`
