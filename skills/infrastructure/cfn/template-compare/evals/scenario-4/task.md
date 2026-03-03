# Handle Very Large Template Comparison

## Problem Description

Your organization is migrating a massive microservices infrastructure to a new CDK-based deployment. The CloudFormation template for the main application stack contains 437 resources spanning Lambda functions, DynamoDB tables, API Gateway endpoints, EventBridge rules, and extensive IAM policies.

When you attempted to run a standard line-by-line diff between deployed and local templates, the output was over 8,000 lines and completely unmanageable. Your terminal buffer was overwhelmed and the diff provided no useful insights.

You need a strategy for comparing very large templates that focuses on meaningful differences rather than overwhelming line-by-line diffs.

## Output Specification

Create a file `large-template-strategy.md` that:

1. **Problem Identification**: Explains when standard diff approaches fail (what line count threshold indicates a problem)

2. **Alternative Approach**: Describes the recommended strategy for large templates (hierarchical comparison instead of line diff)

3. **Comparison Steps**: Lists the specific comparison steps to take for large templates:
   - What to compare first
   - What to compare second
   - What to compare last
   - Which comparisons can be skipped

4. **Practical Commands**: Shows example commands for hierarchical comparison that avoid massive diffs

5. **When to Use**: Explains the decision criteria for when to use hierarchical vs line diff approaches

Also create a script `large-template-compare.sh` that demonstrates the hierarchical approach with commands that output summarized results rather than thousands of lines of diff output.
