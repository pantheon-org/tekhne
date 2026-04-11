# Scenario 03: Triage cdk-nag Findings Before Acting

## User Prompt

A team ran `npx cdk synth` on their API stack after adding `AwsSolutionsChecks`. The synth output shows the following 5 findings:

```
[Error at /ApiStack/ApiGateway/Resource] AwsSolutions-APIG1: The REST API does not have access logging enabled.
[Error at /ApiStack/ApiGateway/Resource] AwsSolutions-APIG4: The REST API does not have authorization defined.
[Error at /ApiStack/LambdaRole/Resource] AwsSolutions-IAM4: The IAM entity contains wildcard permissions.
[Error at /ApiStack/Database/Resource] AwsSolutions-RDS10: The RDS instance or Aurora DB cluster does not have deletion protection enabled.
[Error at /ApiStack/Database/Resource] AwsSolutions-RDS11: The RDS instance or Aurora DB cluster uses the default endpoint port.
```

Context:
- **APIG1 (access logging):** The team uses centralized CloudWatch logging via a Lambda integration — they believe API Gateway access logging is redundant but haven't confirmed this with security.
- **APIG4 (authorization):** This API has a public health-check endpoint that must remain unauthenticated; all other routes use Cognito authorizers.
- **IAM4 (wildcard permissions):** The Lambda role was copied from a template and has `s3:*` on `*`. It only actually needs `s3:GetObject` on one specific bucket.
- **RDS10 (deletion protection):** This is a dev environment database. Deletion protection is intentionally off for cost reasons in dev, but must be on in prod.
- **RDS11 (default port):** The team uses the default port (`5432`) because the network team controls port mappings.

Produce a `TRIAGE.md` that categorizes each finding as **fix**, **suppress**, or **defer**, with a one-line rationale for each decision.

## Expected Behavior

1. Assign fix/suppress/defer to all 5 findings (APIG1, APIG4, IAM4, RDS10, RDS11)
2. Categorize IAM4 (wildcard `s3:*`) as **fix** — the correct scoped permission is known and this is a clear security defect
3. Note that RDS10 categorization depends on dev vs prod environment context
4. Scope the APIG4 rationale to the specific public health-check endpoint — not a blanket statement that authorization is unnecessary
5. Provide at least one line of rationale for every finding

## Success Criteria

- **All 5 findings categorized**: TRIAGE.md assigns fix/suppress/defer to all 5 findings (APIG1, APIG4, IAM4, RDS10, RDS11)
- **IAM4 categorized as fix**: IAM4 (wildcard s3:*) is categorized as 'fix' — the correct scoped permission is known and this is a clear security defect
- **RDS10 rationale references environment context**: RDS10 categorization notes that dev vs prod context matters (not a blanket suppress or fix without context)
- **APIG4 suppression rationale is scoped**: APIG4 rationale notes the public health-check endpoint specifically, not 'authorization is not needed'
- **Rationale provided for every decision**: Every finding in TRIAGE.md has at least one line of rationale explaining the decision

## Failure Conditions

- Any of the 5 findings is missing a fix/suppress/defer categorization
- IAM4 is categorized as suppress or defer instead of fix
- RDS10 is categorized without any reference to the dev vs prod distinction
- APIG4 rationale says authorization is simply not needed without scoping to the health-check endpoint
- Any finding has no rationale accompanying its categorization
