# Scenario 05: Document Security Findings for an Insecure Terraform Config

## User Prompt

A security engineer at a fintech company is preparing a pre-deployment security review for a Terraform configuration that provisions AWS infrastructure. Checkov has already been run against the configuration and produced raw output, but the security engineer needs a properly structured findings report that can be handed off to the development team and used to track remediation.

Take the raw Checkov output below and produce a properly formatted security findings document. Each finding should be documented clearly enough that a developer who has never used Checkov can understand what is wrong, where to find it, how severe it is, and exactly what change to make. The report should reference established best practices rather than just describing the raw tool output.

The following files are provided as inputs:

```
# checkov_output.txt
Check: CKV_AWS_24: "Ensure no security groups allow ingress from 0.0.0.0:0 to port 22"
	FAILED for resource: aws_security_group.bastion
	File: /terraform/main.tf:12-45

Check: CKV_AWS_8: "Ensure all data stored in the Launch configuration EBS is securely encrypted"
	FAILED for resource: aws_launch_configuration.app
	File: /terraform/main.tf:50-70

Check: CKV_AWS_20: "Ensure the S3 bucket has access control list (ACL) defined and is not public"
	FAILED for resource: aws_s3_bucket.uploads
	File: /terraform/storage.tf:5-15

Check: CKV_AWS_18: "Ensure the S3 bucket has access logging enabled"
	FAILED for resource: aws_s3_bucket.uploads
	File: /terraform/storage.tf:5-15

Check: CKV_AWS_53: "Ensure S3 bucket has block public ACLS enabled"
	FAILED for resource: aws_s3_bucket.uploads
	File: /terraform/storage.tf:5-15

Passed checks: 12, Failed checks: 5, Skipped checks: 0
```

```hcl
# terraform/main.tf (excerpt)
resource "aws_security_group" "bastion" {
  name        = "bastion-sg"
  description = "Bastion host security group"
  vpc_id      = var.vpc_id

  ingress {
    description = "SSH access"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

Produce a file called `security_findings.md` with one section per finding. Each finding should contain all information needed for a developer to understand and fix the issue.
