# Scenario 05: Fix a cdk-nag Finding Rather Than Suppress It

## User Prompt

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

## Expected Behavior

1. Explain that suppressing is wrong because the fix is simple and the vague reason 'will fix later' leaves real risk open
2. Add `enforceSSL: true` to the Bucket constructor props in `lib/app-stack.ts`
3. Retain the existing `blockPublicAccess: BlockPublicAccess.BLOCK_ALL` and `versioned: true` properties
4. Reference the principle of preferring to fix insecure resources over suppressing findings

## Success Criteria

- **Suppression rejected as wrong approach**: The response explains that suppressing is wrong because the fix is simple and the vague reason 'will fix later' leaves real risk open
- **enforceSSL: true added to the Bucket**: lib/app-stack.ts sets enforceSSL: true in the Bucket constructor props
- **Existing Bucket properties preserved**: lib/app-stack.ts retains blockPublicAccess: BlockPublicAccess.BLOCK_ALL and versioned: true from the original
- **Fix-first principle stated**: The explanation references the principle of preferring to fix insecure resources over suppressing findings

## Failure Conditions

- Does not explain why using a suppression is wrong in this case
- Does not add `enforceSSL: true` to the Bucket props
- Removes `blockPublicAccess: BlockPublicAccess.BLOCK_ALL` or `versioned: true` from the corrected file
- Does not mention the fix-first principle or the preference for remediation over suppression
