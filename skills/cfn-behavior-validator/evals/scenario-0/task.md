# Investigate SNS Subscription Endpoint Change Behavior

## Problem Description

Your team is building a notification system using AWS SNS subscriptions with email endpoints. The DevOps lead is concerned about a potential production issue: if an SNS email subscription endpoint needs to be changed (e.g., changing from `alerts@oldcompany.com` to `alerts@newcompany.com` after a company acquisition), they're unsure whether CloudFormation will perform an in-place update or replace the subscription entirely.

The AWS CloudFormation documentation for `AWS::SNS::Subscription` needs to be verified, as there have been community reports of discrepancies between documented behavior and actual behavior for various subscription properties. Before making this change in production, the team needs empirical evidence of what actually happens.

Your task is to investigate this behavior scientifically and provide a recommendation on whether special handling or workarounds are needed.

## Output Specification

Create the following files:

1. **research-findings.md** - Document your research process including:
   - What the AWS documentation states about the property
   - Any community reports or issues you found
   - Your hypothesis about expected behavior

2. **test-plan.md** - Describe how you would test this behavior:
   - What environment and resources you would use
   - How you would structure the test
   - What specific outcomes you would look for to determine behavior

3. **validation-report.md** - A formal report documenting:
   - Test execution details (even if simulated)
   - Expected vs actual behavior analysis
   - Recommendation on whether workarounds are needed
   - Any code changes that should be made

Do not actually deploy resources to AWS (since this is a planning exercise), but document the exact commands and process you would follow to execute this validation.
