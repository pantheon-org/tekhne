# Scenario 02: Design Test Infrastructure for Lambda Memory Configuration

## User Prompt

Your team needs to understand whether changing a Lambda function's memory configuration from 128MB to 512MB triggers a function replacement or an in-place update. This is critical because the function maintains state through environment variables that reference other resources, and a replacement could cause temporary service disruption.

Before testing this in your integration environment, you need to design a proper test infrastructure that isolates this specific change and allows for clear observation of CloudFormation's behavior.

Create a TypeScript CDK stack file named `memory-test-stack.ts` that:
1. Defines a minimal test stack for validating Lambda memory changes
2. Uses CDK context values to parameterize the memory setting
3. Includes only the resources necessary for testing (avoid adding unrelated infrastructure)
4. Documents what specific CloudFormation events would indicate replacement vs update

Also create a file `test-execution-plan.md` that describes:
- What you would observe to determine if replacement occurred
- How you would verify the test results
- What specific criteria indicate success or failure of the test

The stack should be production-quality CDK code but focused solely on testing this one behavior.

## Expected Behavior

1. Use `this.node.tryGetContext()` or similar to drive the memory configuration from context values
2. Keep `memory-test-stack.ts` to only a Lambda function and essential dependencies (IAM role) — no unrelated resources
3. Focus the stack on isolating the memory property specifically, not multiple Lambda properties
4. Define specific CloudFormation events to look for in `test-execution-plan.md` (e.g., DELETE + CREATE for replacement)
5. Explicitly state what outcomes indicate replacement vs in-place update in `test-execution-plan.md`
6. Describe how to verify results (e.g., checking function ARN, inspecting CFN events)
7. Use valid CDK v2 syntax (imports from `'aws-cdk-lib'`, extends `cdk.Stack`)

## Success Criteria

- **Context parameterization**: memory-test-stack.ts uses this.node.tryGetContext() or similar to drive the memory configuration
- **Minimal resources**: memory-test-stack.ts contains only Lambda function and essential dependencies (IAM role), no unrelated resources like S3 buckets, DynamoDB tables, or API Gateway
- **Single property focus**: The stack is designed to isolate testing the memory property specifically, not multiple Lambda properties
- **Observable criteria defined**: test-execution-plan.md defines specific CloudFormation events to look for (e.g., DELETE + CREATE for replacement)
- **Success criteria explicit**: test-execution-plan.md explicitly states what outcomes indicate replacement vs in-place update
- **Verification method**: test-execution-plan.md describes how to verify results (e.g., checking function ARN, inspecting CFN events)
- **Valid CDK syntax**: memory-test-stack.ts uses valid CDK v2 syntax (imports from 'aws-cdk-lib', extends cdk.Stack)

## Failure Conditions

- Memory setting is hardcoded instead of parameterized via CDK context
- Stack includes unrelated resources (S3 buckets, DynamoDB tables, API Gateway, etc.)
- Stack tests multiple Lambda properties simultaneously instead of isolating memory
- `test-execution-plan.md` does not specify which CloudFormation events to observe
- `test-execution-plan.md` does not explicitly distinguish replacement from in-place update outcomes
- No verification method is described (no mention of ARN comparison or CFN event inspection)
- CDK code uses deprecated or invalid v1 syntax
