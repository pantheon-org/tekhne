# Scenario 03: Detailed Security and IAM Analysis

## User Prompt

You have successfully retrieved both the deployed and local templates for your monitoring stack. Initial comparison shows they both have the same resource count (47 resources), but you need to perform a detailed hierarchical comparison focusing on security-sensitive changes.

Specifically, you need to check for:
- Changes to CDK Nag suppressions (security rule overrides)
- Changes to IAM roles and policies
- Any added or removed resources despite the same count

Your security team requires detailed analysis of these specific areas before approving any deployment.

Create a bash script `detailed-analysis.sh` that performs hierarchical comparison in this order:
1. Compares top-level template structure (keys)
2. Checks resource counts
3. Identifies any added/removed resources (even if counts match due to swaps)
4. Extracts and compares CDK Nag suppressions
5. Extracts and compares IAM-related resources

Also create `analysis-report.md` that explains:
- What each comparison step reveals
- Why the hierarchical order matters
- How to interpret CDK Nag suppression differences
- How to identify IAM policy changes
- What specific jq queries are most useful for each type of comparison

## Expected Behavior

1. Perform structure comparison using `jq 'keys'` before examining resource details
2. Use `jq '.Resources | length'` to check resource counts
3. Use `diff` or `comm` to find added/removed resources
4. Use process substitution `<()` syntax for resource comparison
5. Include `sort` in the resource key extraction pipeline
6. Extract CDK Nag metadata from Resources using `jq`
7. Filter resources where Type starts with `AWS::IAM` using `jq select`
8. Explain why structure → count → resources → security is the recommended order in `analysis-report.md`
9. Explain what CDK Nag suppressions are and why changes matter in `analysis-report.md`
10. Explain how to identify IAM policy changes and their significance in `analysis-report.md`

## Success Criteria

- **Structure comparison first**: detailed-analysis.sh performs structure comparison (jq 'keys') before resource details
- **Resource count check**: detailed-analysis.sh uses jq '.Resources | length' to check counts
- **Added/removed check**: detailed-analysis.sh uses diff or comm to find added/removed resources
- **Process substitution**: The resource comparison uses process substitution with <() syntax
- **Sorted resource lists**: Resource key extraction includes 'sort' in the pipeline
- **CDK Nag extraction**: detailed-analysis.sh extracts cdk_nag metadata from Resources using jq
- **IAM resource filter**: detailed-analysis.sh filters resources where Type starts with 'AWS::IAM' using jq select
- **Hierarchical order explained**: analysis-report.md explains why structure → count → resources → security is the recommended order
- **CDK Nag interpretation**: analysis-report.md explains what CDK Nag suppressions are and why changes matter
- **IAM policy analysis**: analysis-report.md explains how to identify IAM policy changes and their significance

## Failure Conditions

- Resource details are examined before structure comparison (`jq 'keys'`)
- `jq '.Resources | length'` is not used for resource count comparison
- `diff` or `comm` is not used to find added/removed resources
- Process substitution `<()` is not used in resource comparisons
- Resource key extraction pipeline does not include `sort`
- CDK Nag metadata is not extracted from Resources using jq
- IAM resources are not filtered using `jq select` with a Type check
- `analysis-report.md` does not explain the hierarchical comparison order
- `analysis-report.md` does not explain CDK Nag suppressions and their significance
- `analysis-report.md` does not explain how to identify and interpret IAM policy changes
