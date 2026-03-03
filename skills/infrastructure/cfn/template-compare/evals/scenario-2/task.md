# Detailed Security and IAM Analysis

## Problem Description

You have successfully retrieved both the deployed and local templates for your monitoring stack. Initial comparison shows they both have the same resource count (47 resources), but you need to perform a detailed hierarchical comparison focusing on security-sensitive changes.

Specifically, you need to check for:
- Changes to CDK Nag suppressions (security rule overrides)
- Changes to IAM roles and policies
- Any added or removed resources despite the same count

Your security team requires detailed analysis of these specific areas before approving any deployment.

## Output Specification

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

The script should use specific jq queries and diff commands appropriate for each comparison type.
