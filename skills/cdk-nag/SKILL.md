# CDK Nag Skill

## Overview

CDK Nag is a security and compliance linting tool for AWS CDK applications that validates infrastructure code against various security frameworks and best practices. It operates as a CDK Aspect to automatically scan and validate your infrastructure during synthesis.

## Available References

This skill includes detailed reference documentation that can be loaded on demand:

- **[implementation-guide.md](references/implementation-guide.md)** - Complete implementation patterns and setup instructions
- **[rule-packs.md](references/rule-packs.md)** - All available rule packs and their purposes
- **[suppression-guide.md](references/suppression-guide.md)** - Comprehensive suppression patterns and best practices
- **[troubleshooting.md](references/troubleshooting.md)** - Common issues and resolution strategies
- **[rule-evolution.md](references/rule-evolution.md)** - Historical context of rule changes and deprecations
- **[integration-patterns.md](references/integration-patterns.md)** - CI/CD integration and workflow patterns

## Quick Start

### Basic Implementation

```typescript
import { AwsSolutionsChecks } from 'cdk-nag';
import { App, Aspects } from 'aws-cdk-lib';

const app = new App();
const stack = new MyStack(app, 'MyStack');
Aspects.of(app).add(new AwsSolutionsChecks());
```

### Basic Suppression

```typescript
import { NagSuppressions } from 'cdk-nag';

NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-SNS3',
    reason: 'SSL enforcement auto-applied when topic uses KMS encryption',
  },
]);
```

## Key Concepts

### Rule Naming Convention

Rules follow the pattern: `{RulePack}-{Service}{Number}`

- Examples: `AwsSolutions-SNS3`, `AwsSolutions-IAM4`, `HIPAA.Security-S3PublicRead`

### Rule Packs Available

1. **AWS Solutions** (AwsSolutions) - AWS Foundational Security Best Practices
2. **HIPAA Security** - Healthcare compliance requirements
3. **NIST 800-53 Rev 4 & 5** - Government security standards
4. **PCI DSS 3.2.1** - Payment card industry standards
5. **Serverless** - Serverless-specific security patterns

### Suppression Types

- **Resource-level** - Target specific constructs
- **Stack-level** - Apply to entire stacks
- **Path-based** - Target by construct path
- **Granular** - Target specific rule findings with `appliesTo`

## When to Load References

**Load implementation-guide.md when:**

- Setting up CDK Nag for the first time
- Implementing multiple rule packs
- Configuring conditional application

**Load rule-packs.md when:**

- Understanding available compliance frameworks
- Choosing appropriate rule pack for requirements
- Comparing rule pack coverage

**Load suppression-guide.md when:**

- Writing suppressions for rule violations
- Understanding granular suppression patterns
- Following suppression best practices

**Load troubleshooting.md when:**

- Encountering import or syntax errors
- Debugging rule validation failures
- Resolving CI/CD integration issues

**Load rule-evolution.md when:**

- Understanding deprecated rules
- Investigating rule changes between versions
- Planning suppression cleanup

**Load integration-patterns.md when:**

- Setting up CI/CD pipelines
- Implementing environment-specific rules
- Configuring automated security scanning

## Common Patterns

### Environmental Application

```typescript
// Production gets full compliance
if (environment === 'production') {
  AwsSolutionsChecks.check(app);
  HipaaSecurityChecks.check(app);
}
```

### Multiple Rule Packs

```typescript
import { AwsSolutionsChecks, NIST80053R5Checks } from 'cdk-nag';

Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new NIST80053R5Checks());
```

### Granular Suppressions

```typescript
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-IAM5',
    reason: 'Wildcard required for dynamic resource patterns',
    appliesTo: ['Action::s3:*'],
  },
]);
```

## Resources

- **CDK Nag Repository**: https://github.com/cdklabs/cdk-nag
- **Rule Documentation**: https://github.com/cdklabs/cdk-nag/blob/main/RULES.md
- **API Reference**: https://github.com/cdklabs/cdk-nag/blob/main/API.md

---

_Use the reference documents above for detailed guidance on specific aspects of CDK Nag usage. Each reference is self-contained and focuses on a specific domain._
