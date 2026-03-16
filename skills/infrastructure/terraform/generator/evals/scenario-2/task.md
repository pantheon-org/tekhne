# Encrypted Data Storage for a Healthcare Application

## Problem Description

A healthcare startup is building a patient records platform that requires strict data governance. The platform architect has asked you to provision the AWS storage layer using Terraform. The required resources are:

- An S3 bucket (`patient-records-prod`) with versioning enabled, server-side encryption using a customer-managed KMS key, and a lifecycle policy to transition objects to STANDARD_IA storage class after 90 days and expire non-current versions after 365 days.
- A KMS key for envelope encryption of the S3 bucket contents.
- An RDS PostgreSQL instance (db.t3.medium) for structured patient data, encrypted at rest using the same KMS key.

The architect emphasizes that accidental deletion of any of these resources would constitute a serious compliance incident. The organization has been audited before and the auditor flagged an issue with how incomplete multipart uploads were handled in a previous project — the architect wants to make sure this is correctly addressed.

## Output Specification

Generate the Terraform configuration for these resources. Include all standard files. Provide usage instructions after generating the files.
