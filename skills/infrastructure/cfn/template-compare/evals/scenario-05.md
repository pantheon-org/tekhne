# Scenario 05: Handle Very Large Template Comparison

## User Prompt

Your organization is migrating a massive microservices infrastructure to a new CDK-based deployment. The CloudFormation template for the main application stack contains 437 resources spanning Lambda functions, DynamoDB tables, API Gateway endpoints, EventBridge rules, and extensive IAM policies.

When you attempted to run a standard line-by-line diff between deployed and local templates, the output was over 8,000 lines and completely unmanageable. Your terminal buffer was overwhelmed and the diff provided no useful insights.

You need a strategy for comparing very large templates that focuses on meaningful differences rather than overwhelming line-by-line diffs.

Create a file `large-template-strategy.md` that:
1. **Problem Identification**: Explains when standard diff approaches fail (what line count threshold indicates a problem)
2. **Alternative Approach**: Describes the recommended strategy for large templates (hierarchical comparison instead of line diff)
3. **Comparison Steps**: Lists the specific comparison steps to take for large templates
4. **Practical Commands**: Shows example commands for hierarchical comparison that avoid massive diffs
5. **When to Use**: Explains the decision criteria for when to use hierarchical vs line diff approaches

Also create a script `large-template-compare.sh` that demonstrates the hierarchical approach with commands that output summarized results rather than thousands of lines of diff output.

## Expected Behavior

1. Identify a specific line count threshold (e.g., >5000 lines) when diff becomes unmanageable
2. Explicitly recommend hierarchical comparison instead of line diff for large templates
3. Include checking top-level structure (keys) as the first comparison step
4. Include checking resource counts using `jq '.Resources | length'`
5. Include identifying which resources were added/removed
6. Use `jq`, `comm`, or `diff` with process substitution in `large-template-compare.sh` rather than raw `diff file1 file2`
7. Produce concise output (counts, lists) rather than full template diffs
8. Explain when to use hierarchical vs line diff (based on size/complexity)
9. Mention focusing on security-sensitive changes (IAM, CDK Nag) rather than all resources

## Success Criteria

- **Problem threshold identified**: large-template-strategy.md mentions a specific line count threshold (e.g., >5000 lines) when diff becomes unmanageable
- **Hierarchical approach recommended**: Document explicitly recommends hierarchical comparison instead of line diff for large templates
- **Structure comparison first**: Comparison steps include checking top-level structure (keys) first
- **Resource count comparison**: Comparison steps include checking resource counts (jq '.Resources | length')
- **Added/removed resources**: Comparison steps include identifying which resources were added/removed
- **Avoid line diff**: large-template-compare.sh uses jq, comm, or diff with process substitution rather than raw 'diff file1 file2'
- **Summarized output**: Script commands produce concise output (counts, lists) rather than full template diffs
- **Decision criteria clear**: Document explains when to use hierarchical vs line diff (based on size/complexity)
- **Security focused subset**: Strategy mentions focusing on security-sensitive changes (IAM, CDK Nag) rather than all resources

## Failure Conditions

- Does not mention a specific line count threshold for when diff becomes problematic
- Does not explicitly recommend hierarchical comparison over line diff
- Top-level structure comparison is not the first step
- `jq '.Resources | length'` is not used for resource count comparison
- Does not include identifying added/removed resources
- `large-template-compare.sh` uses raw `diff file1 file2` instead of structured jq/comm comparisons
- Script produces verbose full-diff output instead of summarized results
- No criteria given for when to use hierarchical vs line diff approaches
- Does not mention focusing on security-sensitive resources (IAM, CDK Nag)
