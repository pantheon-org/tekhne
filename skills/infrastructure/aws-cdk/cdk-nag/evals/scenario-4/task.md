# Fix a cdk-nag Finding Rather Than Suppress It

## Problem Description

After running `npx cdk synth`, the following finding appears:

```
[Error at /AppStack/DataBucket/Resource] AwsSolutions-S2: The S3 Bucket does not have SSL requests only enabled.
```

The current CDK construct definition:

```typescript
import { Stack, StackProps } from 'aws-cdk-lib';
import { Bucket, BlockPublicAccess } from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';

export class AppStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const dataBucket = new Bucket(this, 'DataBucket', {
      blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
      versioned: true,
    });
  }
}
```

A teammate suggested suppressing the finding:

```typescript
NagSuppressions.addResourceSuppressions(dataBucket, [
  { id: 'AwsSolutions-S2', reason: 'will fix later' },
]);
```

Explain why the suppression approach is wrong for this case, and produce a corrected `lib/app-stack.ts` that fixes the finding by enforcing SSL on the bucket instead.
