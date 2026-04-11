# Scenario 04: Categorize Template Differences by Risk Level

## User Prompt

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

Create a file `risk-assessment.md` that:
1. **Risk Categories**: Define or list the risk categories used for classification (Expected, Low, Medium, High, Critical)
2. **Change Classification**: Categorize each of the identified changes into the appropriate risk level with reasoning
3. **Risk Category Table**: Include a table that maps types of changes to risk levels
4. **Deployment Recommendation**: Based on the risk assessment, provide a clear deployment decision (Auto-approve, Approve with review, Require sign-off, Block deployment)
5. **Required Actions**: List specific actions required before deployment based on the risk levels found

## Expected Behavior

1. Define or list at least 4 risk categories (Expected, Low, Medium, High, Critical or similar) in `risk-assessment.md`
2. Categorize the `GitRef` tag change as Expected or Low risk (environment-specific metadata)
3. Categorize the alarm threshold change as Medium risk (requires review)
4. Categorize the IAM policy modification as High risk (requires explicit sign-off)
5. Categorize the new CDK Nag suppression as Critical or High risk (security override)
6. Categorize the added CloudWatch alarms and removed S3 lifecycle policy with risk levels
7. Include a clear deployment decision (approve, review, sign-off, or block)
8. Ensure the deployment decision is appropriate for the risk levels found (should require sign-off or block due to IAM/CDK Nag changes)
9. List specific actions required before deployment (e.g., InfoSec approval, stakeholder review)

## Success Criteria

- **Risk categories defined**: risk-assessment.md defines or lists at least 4 risk categories (Expected, Low, Medium, High, Critical or similar)
- **GitRef as expected**: The GitRef tag change is categorized as Expected or Low risk (environment-specific metadata)
- **Alarm threshold as medium**: The alarm threshold change is categorized as Medium risk (requires review)
- **IAM policy as high risk**: The IAM policy modification is categorized as High risk (requires explicit sign-off)
- **CDK Nag suppression as critical**: The new CDK Nag suppression is categorized as Critical or High risk (security override)
- **Resource changes assessed**: The added CloudWatch alarms and removed S3 lifecycle policy are categorized with risk levels
- **Deployment decision present**: Document includes a clear deployment decision (approve, review, sign-off, or block)
- **Decision matches risk**: The deployment decision is appropriate for the risk levels found (should require sign-off or block due to IAM/CDK Nag changes)
- **Required actions listed**: Document lists specific actions required before deployment (e.g., InfoSec approval, stakeholder review)

## Failure Conditions

- Fewer than 4 risk categories are defined or referenced
- GitRef tag change is categorized as Medium or higher risk
- Alarm threshold change is categorized as Low or Expected risk (under-assessed) or Critical (over-assessed)
- IAM policy modification is not categorized as High risk
- CDK Nag suppression is not categorized as Critical or High risk
- Added/removed resources are not given risk level assessments
- No deployment decision is provided
- Deployment decision is Auto-approve or simple Approve despite IAM and CDK Nag changes present
- No required actions are listed before deployment
