# Scenario 01: Add cdk-nag to an Existing CDK Application

## User Prompt

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

## Expected Behavior

1. Use `Aspects.of(app).add(...)` to apply the rule pack at the app level — not per-stack
2. Instantiate `AwsSolutionsChecks` with `{ verbose: true }` for full finding output
3. Place the `Aspects.of(app).add(...)` call before `app.synth()`
4. Include the npm install command in `SETUP.md` (`npm install --save-dev cdk-nag` or equivalent)
5. Include `npx cdk synth` as the command to surface findings in `SETUP.md`

## Success Criteria

- **Aspects.of(app).add used**: bin/app.ts uses Aspects.of(app).add(...) to apply the rule pack at the app level
- **AwsSolutionsChecks instantiated with verbose: true**: The rule pack is instantiated as new AwsSolutionsChecks({ verbose: true })
- **Applied before app.synth()**: The Aspects.of(app).add call appears before app.synth() in the file
- **Install command in SETUP.md**: SETUP.md includes npm install --save-dev cdk-nag or equivalent install command
- **Synth command in SETUP.md**: SETUP.md includes npx cdk synth as the command to surface findings

## Failure Conditions

- Applies `AwsSolutionsChecks` per-stack instead of using `Aspects.of(app).add`
- Instantiates `AwsSolutionsChecks` without `{ verbose: true }`
- Places the Aspects call after `app.synth()`
- SETUP.md is missing the npm install command
- SETUP.md is missing `npx cdk synth` as the trigger command
