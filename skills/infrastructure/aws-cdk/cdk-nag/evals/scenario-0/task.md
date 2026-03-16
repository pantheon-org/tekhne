# Add cdk-nag to an Existing CDK Application

## Problem Description

A team has an existing AWS CDK TypeScript application. They want to add cdk-nag to enforce the AWS Solutions security checks on all stacks.

The current `bin/app.ts` entry point is:

```typescript
import { App } from 'aws-cdk-lib';
import { ApiStack } from '../lib/api-stack';
import { StorageStack } from '../lib/storage-stack';

const app = new App();
new ApiStack(app, 'ApiStack', { env: { region: 'us-east-1' } });
new StorageStack(app, 'StorageStack', { env: { region: 'us-east-1' } });

app.synth();
```

Produce an updated `bin/app.ts` that:
1. Imports and applies the `AwsSolutionsChecks` rule pack to the entire app
2. Uses the correct Aspects API
3. Enables verbose mode so all finding details are printed during synthesis

Also produce a `SETUP.md` with the install command and the synth command to trigger the checks.
