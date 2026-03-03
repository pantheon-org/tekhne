# Suppression Guide

## Overview

Suppressions allow you to disable specific CDK Nag rules for valid business or technical reasons. Proper suppression practices ensure security while accommodating legitimate use cases.

## Suppression Types

### 1. Resource-Level Suppressions

Target specific constructs in your CDK code:

```typescript
import { NagSuppressions } from 'cdk-nag';

NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-SNS3',
    reason: 'SSL enforcement auto-applied when topic uses KMS encryption',
  },
]);
```

### 2. Multiple Resource Suppressions

Apply the same suppression to multiple resources:

```typescript
const resources = [lambda1, lambda2, lambda3];

NagSuppressions.addResourceSuppressions(
  resources,
  [
    {
      id: 'AwsSolutions-L1',
      reason: 'Custom runtime layer requires specific image version',
    },
  ],
  true, // Apply to all resources in array
);
```

### 3. Stack-Level Suppressions

Apply suppressions to an entire stack:

```typescript
NagSuppressions.addStackSuppressions(stack, [
  {
    id: 'AwsSolutions-IAM5',
    reason: 'Wildcard permissions required for CloudWatch Logs integration',
  },
]);
```

### 4. Path-Based Suppressions

Target resources by their construct path:

```typescript
NagSuppressions.addResourceSuppressionsByPath(
  this,
  '/StackName/Custom::CDKBucketDeployment8675309/ServiceRole/Resource',
  [
    {
      id: 'AwsSolutions-IAM4',
      reason: 'AWS managed policy required for bucket deployment',
    },
  ],
);
```

### 5. Child Construct Suppressions

Apply suppressions to child constructs:

```typescript
const user = new User(this, 'User');
user.addToPolicy(
  new PolicyStatement({
    actions: ['s3:PutObject'],
    resources: ['arn:aws:s3:::bucket/*'],
  }),
);

NagSuppressions.addResourceSuppressions(
  user,
  [
    {
      id: 'AwsSolutions-IAM5',
      reason: 'Wildcard required for dynamic S3 object paths',
      appliesTo: ['Resource::arn:aws:s3:::bucket/*'],
    },
  ],
  true, // Enable child construct suppression
);
```

## Granular Suppressions

### Using appliesTo

Target specific findings within a rule violation:

```typescript
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-IAM5',
    reason: 'S3 wildcard actions required for object operations',
    appliesTo: [
      'Action::s3:*', // Suppress specific action
      'Resource::*', // Suppress specific resource
    ],
  },
]);
```

### Regex Patterns in appliesTo

Use regular expressions for dynamic matching:

```typescript
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-IAM5',
    reason: 'SQS queue pattern requires wildcard for account-specific ARNs',
    appliesTo: [
      {
        regex: '/^Resource::arn:aws:sqs:(.*):\\*$/g',
      },
    ],
  },
]);
```

### Common appliesTo Patterns

```typescript
// IAM Actions
appliesTo: ['Action::s3:*'];
appliesTo: ['Action::kms:*'];

// Resource ARNs
appliesTo: ['Resource::*'];
appliesTo: ['Resource::arn:aws:s3:::bucket/*'];

// Parameter references
appliesTo: ['Parameter::BucketArn'];

// Log exports (for services with granular logging)
appliesTo: ['LogExport::audit', 'LogExport::api'];
```

## Validation Failure Suppressions

### Suppressing All Validation Failures

```typescript
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'CdkNagValidationFailure',
    reason: 'Custom validation logic implemented in application layer',
  },
]);
```

### Granular Validation Failure Suppression

```typescript
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'CdkNagValidationFailure',
    reason: 'Lambda runtime validation handled by deployment pipeline',
    appliesTo: ['AwsSolutions-L1'],
  },
]);
```

## Suppression Best Practices

### 1. Meaningful Reasons

❌ **Bad Examples**:

```typescript
{ id: 'AwsSolutions-IAM5', reason: 'Not applicable' }
{ id: 'AwsSolutions-S1', reason: 'We know what we are doing' }
{ id: 'AwsSolutions-EC23', reason: 'Suppressed' }
```

✅ **Good Examples**:

```typescript
{
  id: 'AwsSolutions-IAM5',
  reason: 'Wildcard required for CloudWatch Logs API calls with dynamic log group names'
}
{
  id: 'AwsSolutions-S1',
  reason: 'Access logging not required for internal artifact storage bucket with 7-day lifecycle'
}
{
  id: 'AwsSolutions-EC23',
  reason: 'Security group allows HTTP from ALB security group only, not 0.0.0.0/0'
}
```

### 2. Granular Over Broad

❌ **Avoid broad suppressions**:

```typescript
// Suppresses ALL IAM5 violations on the resource
NagSuppressions.addResourceSuppressions(resource, [
  { id: 'AwsSolutions-IAM5', reason: 'Required for functionality' },
]);
```

✅ **Use granular suppressions**:

```typescript
// Suppresses only specific wildcard actions
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-IAM5',
    reason: 'CloudWatch metrics API requires wildcard for dynamic metric names',
    appliesTo: ['Action::cloudwatch:PutMetricData'],
  },
]);
```

### 3. Resource-Specific Over Stack-Wide

❌ **Avoid stack-level suppressions**:

```typescript
NagSuppressions.addStackSuppressions(stack, [
  { id: 'AwsSolutions-IAM4', reason: 'AWS managed policies needed' },
]);
```

✅ **Target specific resources**:

```typescript
NagSuppressions.addResourceSuppressions(lambdaRole, [
  {
    id: 'AwsSolutions-IAM4',
    reason: 'AWSLambdaVPCAccessExecutionRole required for VPC Lambda execution',
  },
]);
```

## Common Suppression Scenarios

### Lambda Functions

```typescript
// VPC execution role
NagSuppressions.addResourceSuppressions(lambdaRole, [
  {
    id: 'AwsSolutions-IAM4',
    reason:
      'AWSLambdaVPCAccessExecutionRole managed policy required for VPC execution',
  },
]);

// Custom runtime
NagSuppressions.addResourceSuppressions(lambda, [
  {
    id: 'AwsSolutions-L1',
    reason:
      'Custom runtime layer requires specific Python version for dependencies',
  },
]);
```

### S3 Buckets

```typescript
// Internal buckets without public access
NagSuppressions.addResourceSuppressions(bucket, [
  {
    id: 'AwsSolutions-S1',
    reason:
      'Internal artifact bucket with short lifecycle, access logging not required',
  },
]);

// Static website buckets
NagSuppressions.addResourceSuppressions(bucket, [
  {
    id: 'AwsSolutions-S2',
    reason: 'Public read access required for static website hosting',
  },
]);
```

### SNS Topics

```typescript
// KMS encrypted topics
NagSuppressions.addResourceSuppressions(topic, [
  {
    id: 'AwsSolutions-SNS3',
    reason:
      'SSL enforcement automatically applied when topic uses KMS encryption',
  },
]);
```

### Security Groups

```typescript
// ALB to target communication
NagSuppressions.addResourceSuppressions(securityGroup, [
  {
    id: 'AwsSolutions-EC23',
    reason:
      'HTTP traffic allowed from ALB security group only, not public internet',
  },
]);
```

## Suppression Ignore Conditions

### Ignoring Error-Level Suppressions

```typescript
import { SuppressionIgnoreErrors } from 'cdk-nag';

Aspects.of(app).add(
  new AwsSolutionsChecks({
    suppressionIgnoreCondition: new SuppressionIgnoreErrors(),
  }),
);
```

### Custom Ignore Conditions

```typescript
import { ISuppressionIgnoreCondition } from 'cdk-nag';

class ProductionOnlyIgnore implements ISuppressionIgnoreCondition {
  createMessage(input: SuppressionIgnoreInput): string {
    if (process.env.ENVIRONMENT === 'production') {
      return 'Suppression ignored in production environment';
    }
    return '';
  }
}

Aspects.of(app).add(
  new AwsSolutionsChecks({
    suppressionIgnoreCondition: new ProductionOnlyIgnore(),
  }),
);
```

## CloudFormation Template Suppressions

### Embedded in Template

```json
{
  "Resources": {
    "MyResource": {
      "Type": "AWS::S3::Bucket",
      "Properties": {},
      "Metadata": {
        "cdk_nag": {
          "rules_to_suppress": [
            {
              "id": "AwsSolutions-S1",
              "reason": "Access logging not required for internal bucket"
            }
          ]
        }
      }
    }
  }
}
```

### Granular Template Suppressions

```json
{
  "Metadata": {
    "cdk_nag": {
      "rules_to_suppress": [
        {
          "id": "AwsSolutions-IAM5",
          "reason": "KMS actions require wildcard for key operations",
          "applies_to": ["Action::kms:*"]
        }
      ]
    }
  }
}
```

## Suppression Review Process

### Regular Review Checklist

1. **Validity**: Is the suppression still applicable?
2. **Accuracy**: Does the reason accurately reflect the current state?
3. **Scope**: Can the suppression be made more granular?
4. **Alternatives**: Are there alternative implementations that avoid the need for suppression?

### Documentation Standards

- Link to architectural decisions or security reviews
- Reference specific compliance requirements
- Include review dates and responsible teams
- Document compensating controls where applicable

### Automated Review

```typescript
// Include suppression metadata for tracking
NagSuppressions.addResourceSuppressions(resource, [
  {
    id: 'AwsSolutions-IAM5',
    reason:
      'CloudWatch Logs wildcard required for dynamic log groups. Reviewed: 2024-01-15. Next review: 2024-07-15',
  },
]);
```
