# Triage cdk-nag Findings Before Acting

## Problem Description

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
