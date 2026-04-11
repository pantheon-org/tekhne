# Scenario 05: Automate ECS Task Definition Memory Change Validation

## User Prompt

Your DevOps team regularly needs to validate CloudFormation update behaviors for various resource property changes. To make this process more efficient, they want to leverage automation tools and scripts that can stream events in real-time during deployments and compare resource properties before and after changes.

For an upcoming test of ECS task definition memory limit changes (from 512 to 1024), you need to set up an automated monitoring and comparison workflow.

Create a file `automation-workflow.md` that documents:
1. **Real-time Monitoring Setup**: How to monitor CloudFormation events as they happen during the deployment (not just querying after the fact)
2. **Resource Comparison Strategy**: How to capture and compare resource properties before and after the deployment
3. **Tool Usage**: What scripts or tools would be most useful for this automation
4. **Workflow Steps**: The complete sequence of automated steps from initial deployment through final comparison

Also create a simple bash script `monitor-deployment.sh` that shows the general approach to monitoring a deployment in progress (use comments liberally to explain the approach even if you don't have access to actual helper scripts).

## Expected Behavior

1. Mention streaming or watching CloudFormation events in real-time during deployment (not just querying after)
2. Reference a script like `watch-cfn-events.sh` or similar tool for streaming CFN events
3. Describe comparing resource properties before and after deployment
4. Reference a script like `compare-resources.sh` or similar tool for diffing resource properties
5. Include capturing resource properties/state before making changes
6. Include capturing resource properties/state after deployment completes
7. Show a clear sequence in `automation-workflow.md`: capture before state → deploy → monitor → capture after state → compare

## Success Criteria

- **Real-time event streaming**: automation-workflow.md or monitor-deployment.sh mentions streaming/watching CloudFormation events in real-time during deployment (not just after)
- **watch-cfn-events reference**: Document mentions a script like 'watch-cfn-events.sh' or similar tool for streaming CFN events
- **Resource comparison mentioned**: automation-workflow.md describes comparing resource properties before and after deployment
- **compare-resources reference**: Document mentions a script like 'compare-resources.sh' or similar tool for diffing resource properties
- **Before state capture**: Workflow includes capturing resource properties/state before making changes
- **After state capture**: Workflow includes capturing resource properties/state after deployment completes
- **Sequential workflow**: automation-workflow.md shows a clear sequence: capture before state → deploy → monitor → capture after state → compare

## Failure Conditions

- Describes only polling events after deployment instead of real-time streaming during deployment
- Does not mention a `watch-cfn-events.sh`-style tool for real-time event streaming
- Does not describe comparing resource properties between before and after states
- Does not mention a `compare-resources.sh`-style tool for property comparison
- Workflow does not include capturing the before state
- Workflow does not include capturing the after state
- Workflow steps are out of order or the sequence is not clearly defined
