# Scenario 01: Compare Deployed Stack with Local CDK Changes

## User Prompt

Your team is preparing to deploy changes to the production monitoring stack (`lct-monitoring-pr-stack`). Before deploying, you need to compare the currently deployed CloudFormation template in the `us-east-1` region with your local CDK changes to ensure you understand all differences and can assess deployment risk.

The local project uses `make synth` to synthesize templates. After comparison, you need to generate a formal comparison report that categorizes the changes and provides a deployment recommendation.

Create the following files:

1. **comparison-script.sh** — A bash script that:
   - Retrieves the deployed template from AWS
   - Synthesizes the local template
   - Performs structure comparison (top-level keys)
   - Checks resource count differences
   - Identifies added/removed resources
   - Saves all artifacts with timestamps

2. **comparison-report.md** — A markdown report using this structure:
   ```markdown
   # CloudFormation Template Comparison
   
   ## Summary
   - Deployed: <stack-name> (X resources)
   - Local: <stack-name> (Y resources)
   - Status: [Safe to deploy | Review required | Critical issues]
   
   ## Differences
   [List key differences found]
   
   ## Recommendations
   [Actions required before deployment]
   
   ## Deployment Decision
   [Approve | Reject | Conditional — reasoning]
   ```

Use placeholder values for stack names and profiles where needed (e.g., `<stack-name>`, `<profile>`).

## Expected Behavior

1. Use `aws cloudformation get-template` with `--stack-name` and `--query TemplateBody` flags to retrieve the deployed template
2. Redirect `get-template` output to a file like `deployed.json`
3. Use `make synth` to synthesize the local template
4. Copy the synthesized template from `cdk.out/` to a local comparison file
5. Use `jq 'keys'` on both templates to compare top-level structure
6. Use `jq '.Resources | length'` on both templates to check resource counts
7. Use `diff` or `comm` with process substitution to find added/removed resources
8. Create a directory with timestamp and/or branch name for saving comparison artifacts
9. Include all 4 required sections in `comparison-report.md`: Summary, Differences, Recommendations, Deployment Decision
10. Include resource counts for both deployed and local in the Summary section
11. Include a status indicator (Safe to deploy / Review required / Critical issues)

## Success Criteria

- **aws get-template command**: comparison-script.sh uses 'aws cloudformation get-template' with --stack-name and --query TemplateBody flags
- **Output to JSON file**: comparison-script.sh redirects get-template output to a file like deployed.json
- **make synth command**: comparison-script.sh uses 'make synth' to synthesize local template
- **Copy synthesized template**: comparison-script.sh copies the synthesized template from cdk.out/ to a local comparison file
- **Structure comparison**: comparison-script.sh uses jq 'keys' on both templates to compare top-level structure
- **Resource count check**: comparison-script.sh uses jq '.Resources | length' on both templates
- **Added/removed resources**: comparison-script.sh uses diff or comm with process substitution to find resource changes
- **Timestamped artifacts**: comparison-script.sh creates a directory with timestamp and/or branch name for saving artifacts
- **Report template structure**: comparison-report.md contains all 4 required sections: Summary, Differences, Recommendations, Deployment Decision
- **Resource counts in summary**: comparison-report.md Summary section mentions resource counts for both deployed and local
- **Status indicator**: comparison-report.md includes a status indicator (Safe to deploy / Review required / Critical issues)

## Failure Conditions

- Does not use `aws cloudformation get-template` with `--stack-name` and `--query TemplateBody`
- Does not save the retrieved template to a JSON file
- Does not use `make synth` for local template synthesis
- Does not copy the synthesized template from `cdk.out/`
- Does not use `jq 'keys'` for top-level structure comparison
- Does not use `jq '.Resources | length'` for resource count comparison
- Does not use `diff` or `comm` for added/removed resource detection
- Does not create a timestamped artifact directory
- `comparison-report.md` is missing any of the 4 required sections
- Summary section does not include resource counts
- No status indicator in the report
