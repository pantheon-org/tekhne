# Write a Justified cdk-nag Suppression

## Problem Description

A developer is trying to silence two cdk-nag findings quickly. They wrote the following suppressions:

```typescript
// Attempt 1 — stack-level suppression
NagSuppressions.addStackSuppressions(storageStack, [
  { id: 'AwsSolutions-S1', reason: 'false positive' },
  { id: 'AwsSolutions-S2', reason: 'not applicable' },
]);

// Attempt 2 — resource-level but vague reason
NagSuppressions.addResourceSuppressions(
  logsBucket,
  [{ id: 'AwsSolutions-S1', reason: 'logging not needed' }],
);
```

The context for these findings:

- **AwsSolutions-S1** (server access logging): `logsBucket` is an S3 bucket that stores only ephemeral CI build logs. It contains no PII, is accessible only to the CI/CD IAM role, and access is audited via CloudTrail at the account level. The team has an open ADR (ADR-017) documenting this decision.
- **AwsSolutions-S2** (SSL enforcement): `logsBucket` already has a bucket policy that denies non-HTTPS requests via a condition on `aws:SecureTransport`.

Explain what is wrong with the original suppressions and produce a corrected `suppressions.ts` file with both suppressions rewritten correctly.
