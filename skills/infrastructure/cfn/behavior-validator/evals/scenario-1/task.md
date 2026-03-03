# Design Test Infrastructure for Lambda Memory Configuration

## Problem Description

Your team needs to understand whether changing a Lambda function's memory configuration from 128MB to 512MB triggers a function replacement or an in-place update. This is critical because the function maintains state through environment variables that reference other resources, and a replacement could cause temporary service disruption.

Before testing this in your integration environment, you need to design a proper test infrastructure that isolates this specific change and allows for clear observation of CloudFormation's behavior.

## Output Specification

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
