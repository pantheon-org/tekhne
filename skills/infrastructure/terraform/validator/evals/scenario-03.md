# Scenario 03: Validate a Terraform Config Using Multiple Utility Providers

## User Prompt

A DevOps engineer is onboarding a new service that uses Terraform to provision some supporting infrastructure. The configuration is relatively simple — it creates an S3 bucket with a randomly-generated suffix, sets up some local file outputs, and generates a TLS private key for internal use. The engineer wants a full validation pass run on this config and a provider documentation lookup report before submitting it for peer review.

Your job is to perform the validation and produce a documentation lookup report showing which providers were identified (both explicit and any implicit ones inferred from the resource types used), how you looked up their documentation, and what the key API/usage notes are for each.

The following files are provided as inputs:

```hcl
# terraform/main.tf
terraform {
  required_version = ">= 1.3"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

resource "random_id" "bucket_suffix" {
  byte_length = 4
}

resource "aws_s3_bucket" "service_data" {
  bucket = "svc-data-${random_id.bucket_suffix.hex}"
  tags = {
    Service = var.service_name
  }
}

resource "tls_private_key" "internal" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "local_file" "private_key_out" {
  content         = tls_private_key.internal.private_key_pem
  filename        = "${path.module}/output/private_key.pem"
  file_permission = "0600"
}

resource "null_resource" "post_deploy_hook" {
  triggers = {
    bucket_id = aws_s3_bucket.service_data.id
  }
  provisioner "local-exec" {
    command = "echo Bucket ${aws_s3_bucket.service_data.bucket} is ready"
  }
}
```

Produce two files:
- `validation_report.md`: The validation results (fmt, init, validate, lint, security)
- `provider_lookup_report.md`: A report listing every provider found (explicit and implicit), how its docs were retrieved, and key usage notes

## Expected Behavior

1. The `provider_lookup_report.md` identifies the `aws` provider as an explicit provider
2. The `provider_lookup_report.md` identifies at least 3 implicit providers from resource prefixes (`random`, `tls`, `local`, `null`)
3. The report shows the logic of extracting provider names from resource type prefixes (e.g., `random` from `random_id`, `tls` from `tls_private_key`)
4. The report documents using WebSearch (not Context7) for at least one utility provider (`random`, `tls`, `local`, or `null`)
5. The report documents attempting Context7 lookup for the `aws` provider
6. The `validation_report.md` documents running `bash scripts/extract_tf_info_wrapper.sh` as a step
7. The `provider_lookup_report.md` includes actual documentation content or usage notes (not just a list of provider names)
8. The `validation_report.md` shows `terraform fmt` was run
9. The `validation_report.md` shows `terraform validate` was run (after init)
10. The `validation_report.md` documents a security scan using the checkov wrapper script

## Success Criteria

- **Explicit provider identified**: The `provider_lookup_report.md` identifies the `aws` provider as an explicit provider
- **Implicit providers identified**: The `provider_lookup_report.md` identifies at least 3 implicit providers from resource prefixes (`random`, `tls`, `local`, `null`)
- **Prefix extraction documented**: The report shows the logic of extracting provider names from resource type prefixes (e.g., `random` from `random_id`, `tls` from `tls_private_key`)
- **WebSearch for utility providers**: The report documents using WebSearch (not Context7) for at least one utility provider (`random`, `tls`, `local`, or `null`)
- **Context7 attempted for aws**: The report documents attempting Context7 lookup for the `aws` provider
- **extract_tf_info_wrapper used**: The `validation_report.md` documents running `bash scripts/extract_tf_info_wrapper.sh` as a step
- **Provider docs content included**: The `provider_lookup_report.md` includes at least some actual documentation content or usage notes (not just a list of provider names)
- **terraform fmt run**: The `validation_report.md` shows `terraform fmt` was run
- **terraform validate run**: The `validation_report.md` shows `terraform validate` was run (after init)
- **Security scan run**: The `validation_report.md` documents a security scan using the checkov wrapper script

## Failure Conditions

- Implicit providers (`random`, `tls`, `local`, `null`) are not identified from resource type prefixes
- The provider prefix extraction logic is not documented
- WebSearch is not used for utility providers (Context7 used for all providers instead)
- `extract_tf_info_wrapper.sh` is not documented as a step in the validation report
- `provider_lookup_report.md` contains only provider names without documentation content
- `terraform fmt` or `terraform validate` is not documented
- Security scan using the checkov wrapper is not documented
