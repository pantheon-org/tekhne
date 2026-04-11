# Scenario 03: Encrypted Data Storage with Lifecycle and prevent_destroy

## User Prompt

A healthcare startup is building a patient records platform that requires strict data governance. The platform architect has asked you to provision the AWS storage layer using Terraform. The required resources are:

- An S3 bucket (`patient-records-prod`) with versioning enabled, server-side encryption using a customer-managed KMS key, and a lifecycle policy to transition objects to STANDARD_IA storage class after 90 days and expire non-current versions after 365 days.
- A KMS key for envelope encryption of the S3 bucket contents.
- An RDS PostgreSQL instance (`db.t3.medium`) for structured patient data, encrypted at rest using the same KMS key.

The architect emphasizes that accidental deletion of any of these resources would constitute a serious compliance incident. The organization has been audited before and the auditor flagged an issue with how incomplete multipart uploads were handled in a previous project — the architect wants to make sure this is correctly addressed.

Generate the Terraform configuration for these resources. Include all standard files. Provide usage instructions after generating the files.

## Expected Behavior

1. The `aws_kms_key` resource has `lifecycle { prevent_destroy = true }`
2. The `aws_db_instance` resource has `lifecycle { prevent_destroy = true }`
3. The `aws_s3_bucket` resource has `lifecycle { prevent_destroy = true }`
4. The S3 lifecycle configuration includes a rule with `abort_incomplete_multipart_upload { days_after_initiation = 7 }`
5. The `aws_db_instance` resource has `storage_encrypted = true`
6. The S3 bucket has server-side encryption configured (`aws_s3_bucket_server_side_encryption_configuration` or equivalent)
7. Generated files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
8. Response includes next steps for `terraform init`, `plan`, `apply` and security reminders
9. No passwords, keys, or other sensitive strings are hardcoded — they use variables or are generated

## Success Criteria

- **KMS key prevent_destroy**: The `aws_kms_key` resource has `lifecycle { prevent_destroy = true }`
- **RDS instance prevent_destroy**: The `aws_db_instance` resource has `lifecycle { prevent_destroy = true }`
- **S3 bucket prevent_destroy**: The `aws_s3_bucket` resource has `lifecycle { prevent_destroy = true }`
- **abort_incomplete_multipart_upload rule**: The S3 lifecycle configuration includes a rule with `abort_incomplete_multipart_upload { days_after_initiation = 7 }`
- **RDS storage_encrypted**: The `aws_db_instance` resource has `storage_encrypted = true`
- **S3 encryption configured**: The S3 bucket has server-side encryption configured (`aws_s3_bucket_server_side_encryption_configuration` or equivalent)
- **File organization correct**: Generated files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
- **Usage instructions included**: Response includes next steps for `terraform init`, `plan`, `apply` and security reminders
- **No sensitive values hardcoded**: No passwords, keys, or other sensitive strings are hardcoded — they use variables or are generated

## Failure Conditions

- Any of the three critical resources (`aws_kms_key`, `aws_db_instance`, `aws_s3_bucket`) is missing `lifecycle { prevent_destroy = true }`
- `abort_incomplete_multipart_upload` rule is absent from the S3 lifecycle configuration
- `storage_encrypted = true` is absent from the RDS instance
- S3 server-side encryption is not configured
- Sensitive values (passwords, keys) are hardcoded in the configuration
- Required files (`main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`) are absent
