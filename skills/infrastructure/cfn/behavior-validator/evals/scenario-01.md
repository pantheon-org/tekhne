# Scenario 01: Investigate SNS Subscription Endpoint Change Behavior

## User Prompt

Your team is building a notification system using AWS SNS subscriptions with email endpoints. The DevOps lead is concerned about a potential production issue: if an SNS email subscription endpoint needs to be changed (e.g., changing from `alerts@oldcompany.com` to `alerts@newcompany.com` after a company acquisition), they're unsure whether CloudFormation will perform an in-place update or replace the subscription entirely.

The AWS CloudFormation documentation for `AWS::SNS::Subscription` needs to be verified, as there have been community reports of discrepancies between documented behavior and actual behavior for various subscription properties. Before making this change in production, the team needs empirical evidence of what actually happens.

Your task is to investigate this behavior scientifically and provide a recommendation on whether special handling or workarounds are needed. Do not actually deploy resources to AWS (since this is a planning exercise), but document the exact commands and process you would follow to execute this validation.

Create the following files:

1. **research-findings.md** - Document your research process including what the AWS documentation states, any community reports, and your hypothesis about expected behavior
2. **test-plan.md** - Describe how you would test this behavior (environment, resources, structure, observable outcomes)
3. **validation-report.md** - A formal report with test execution details, expected vs actual behavior analysis, recommendation, and any code changes needed

## Expected Behavior

1. Reference AWS CloudFormation documentation and identify the 'Update requires' field for the Endpoint property in `research-findings.md`
2. Mention searching GitHub (AWS CDK repo), Stack Overflow, or AWS re:Post for community reports
3. State an explicit hypothesis about expected behavior in `research-findings.md`
4. Specify using a non-production or disposable test environment in `test-plan.md`
5. Describe creating a minimal test stack with only the SNS subscription in `test-plan.md`
6. Define specific success criteria such as looking for DELETE/CREATE events vs UPDATE_IN_PLACE
7. Include the `aws cloudformation describe-stack-events` command in `test-plan.md` or `validation-report.md`
8. Include at least 5 of these fields in `validation-report.md`: Date/Region/CDK Version, Resource Type & Property Changed, AWS Docs Say, What Actually Happened, CFN Events, Matches Docs, Workaround Needed
9. Classify the behavior as UPDATE_IN_PLACE, REPLACEMENT, NO-OP, or error in `validation-report.md`
10. Include an explicit decision on whether a workaround is needed with reasoning in `validation-report.md`

## Success Criteria

- **CFN docs lookup**: research-findings.md references AWS CloudFormation documentation and identifies the 'Update requires' field for the Endpoint property
- **Community research**: research-findings.md mentions searching or checking GitHub (AWS CDK repo), Stack Overflow, or AWS re:Post
- **Hypothesis stated**: research-findings.md contains an explicit hypothesis statement about expected behavior
- **Non-prod environment**: test-plan.md specifies using a non-production or disposable test environment
- **Resource isolation**: test-plan.md describes creating a minimal test stack with only the SNS subscription (no unrelated resources)
- **Observable criteria**: test-plan.md defines specific success criteria such as looking for DELETE/CREATE events vs UPDATE_IN_PLACE
- **CFN events inspection**: test-plan.md or validation-report.md includes using 'aws cloudformation describe-stack-events' command
- **Required report fields**: validation-report.md contains at least 5 of these fields: Date/Region/CDK Version, Resource Type & Property Changed, AWS Docs Say, What Actually Happened, CFN Events, Matches Docs, Workaround Needed
- **Behavior classification**: validation-report.md classifies the behavior as one of: UPDATE_IN_PLACE, REPLACEMENT, NO-OP, or error
- **Workaround decision**: validation-report.md includes explicit decision on whether a workaround is needed with reasoning

## Failure Conditions

- Does not reference AWS CloudFormation documentation for the Endpoint property's update behavior
- Does not mention community research sources (GitHub, Stack Overflow, re:Post)
- Does not state a hypothesis about expected behavior
- Does not specify a non-production environment for testing
- Test plan includes unrelated resources instead of a minimal isolated stack
- Test plan has no specific observable criteria for determining replacement vs update
- `aws cloudformation describe-stack-events` command is not included anywhere
- `validation-report.md` contains fewer than 5 of the required fields
- Behavior is not classified as UPDATE_IN_PLACE, REPLACEMENT, NO-OP, or error
- No explicit workaround decision is provided in `validation-report.md`
