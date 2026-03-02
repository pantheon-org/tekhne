# Compare Deployed Stack with Local CDK Changes

## Problem Description

Your team is preparing to deploy changes to the production monitoring stack (`lct-monitoring-pr-stack`). Before deploying, you need to compare the currently deployed CloudFormation template in the `us-east-1` region with your local CDK changes to ensure you understand all differences and can assess deployment risk.

The local project uses `make synth` to synthesize templates. After comparison, you need to generate a formal comparison report that categorizes the changes and provides a deployment recommendation.

## Output Specification

Create the following files:

1. **comparison-script.sh** - A bash script that:
   - Retrieves the deployed template from AWS
   - Synthesizes the local template
   - Performs structure comparison (top-level keys)
   - Checks resource count differences
   - Identifies added/removed resources
   - Saves all artifacts with timestamps

2. **comparison-report.md** - A markdown report using this structure:
   ```markdown
   # CloudFormation Template Comparison
   
   ## Summary
   - Deployed: <stack-name> (X resources)
   - Local: <stack-name> (Y resources)
   - Status: [✅ Safe to deploy | ⚠️ Review required | ❌ Critical issues]
   
   ## Differences
   [List key differences found]
   
   ## Recommendations
   [Actions required before deployment]
   
   ## Deployment Decision
   [Approve | Reject | Conditional — reasoning]
   ```

The script should use placeholder values for stack names and profiles where needed (e.g., `<stack-name>`, `<profile>`).
