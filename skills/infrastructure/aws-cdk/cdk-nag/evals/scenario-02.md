# Scenario 02: Write a Justified cdk-nag Suppression

## User Prompt

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

## Expected Behavior

1. Identify that `addStackSuppressions` is too broad and suppresses all resources in the stack
2. Identify 'false positive' and 'not applicable' as inadequate reasons that hide real risk
3. Use `NagSuppressions.addResourceSuppressions` for both findings, scoped to `logsBucket`
4. Write the AwsSolutions-S1 suppression reason referencing the ephemeral data nature, restricted access, CloudTrail audit, or ADR-017
5. Write the AwsSolutions-S2 suppression reason referencing SSL enforcement via the bucket policy and `aws:SecureTransport` condition

## Success Criteria

- **Stack-level suppression rejected**: The explanation (inline comment or note) identifies that addStackSuppressions is too broad and suppresses all resources in the stack
- **Vague reason rejected**: The explanation identifies 'false positive' and 'not applicable' as inadequate reasons that hide real risk
- **Both suppressions use addResourceSuppressions**: suppressions.ts uses NagSuppressions.addResourceSuppressions for both findings, scoped to logsBucket
- **S1 reason references ADR or compensating controls**: The AwsSolutions-S1 suppression reason mentions ephemeral data, restricted access, CloudTrail audit, or ADR-017
- **S2 reason references SecureTransport enforcement**: The AwsSolutions-S2 suppression reason explains that SSL is enforced via bucket policy / aws:SecureTransport condition

## Failure Conditions

- Does not explain why `addStackSuppressions` is too broad
- Does not flag 'false positive' and 'not applicable' as inadequate reasons
- Uses `addStackSuppressions` instead of `addResourceSuppressions` in the corrected file
- AwsSolutions-S1 suppression reason is generic and does not reference ephemeral data, CloudTrail, or ADR-017
- AwsSolutions-S2 suppression reason does not reference the bucket policy or `aws:SecureTransport`
