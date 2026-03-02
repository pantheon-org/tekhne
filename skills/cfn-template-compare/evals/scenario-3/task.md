# Categorize Template Differences by Risk Level

## Problem Description

Your comparison script has identified the following differences between deployed and local templates:

**Changes Found:**
- 2 resources added (new CloudWatch alarms)
- 1 resource removed (old S3 bucket lifecycle policy)
- 5 resources modified:
  - Updated `GitRef` tag on multiple resources
  - Changed alarm threshold from 80 to 90
  - Modified IAM policy to add new S3 bucket permissions
  - Added new CDK Nag suppression for AwsSolutions-IAM5
  - Updated Lambda function memory from 128 to 256

Your deployment lead needs these changes categorized by risk level to make an informed deployment decision.

## Output Specification

Create a file `risk-assessment.md` that:

1. **Risk Categories**: Define or list the risk categories used for classification (Expected, Low, Medium, High, Critical)

2. **Change Classification**: Categorize each of the identified changes into the appropriate risk level with reasoning

3. **Risk Category Table**: Include a table that maps types of changes to risk levels (similar to the skill's reference table)

4. **Deployment Recommendation**: Based on the risk assessment, provide a clear deployment decision:
   - Auto-approve (all expected/low risk)
   - Approve with review (medium risk present)
   - Require sign-off (high risk)
   - Block deployment (critical issues)

5. **Required Actions**: List specific actions required before deployment based on the risk levels found

The assessment should demonstrate understanding of what constitutes different risk levels and appropriate responses.
