# Validate a Terraform Config Using Multiple Utility Providers

## Problem/Feature Description

A DevOps engineer is onboarding a new service that uses Terraform to provision some supporting infrastructure. The configuration is relatively simple — it creates an S3 bucket with a randomly-generated suffix, sets up some local file outputs, and generates a TLS private key for internal use. The engineer wants a full validation pass run on this config and a provider documentation lookup report before submitting it for peer review.

Your job is to perform the validation and produce a documentation lookup report showing which providers were identified (both explicit and any implicit ones inferred from the resource types used), how you looked up their documentation, and what the key API/usage notes are for each.

## Output Specification

Produce two files:
- `validation_report.md`: The validation results (fmt, init, validate, lint, security)
- `provider_lookup_report.md`: A report listing every provider found (explicit and implicit), how its docs were retrieved, and key usage notes

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: terraform/main.tf ===============
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

=============== FILE: terraform/variables.tf ===============
variable "aws_region" {
  description = "AWS region to deploy into"
  type        = string
  default     = "us-west-2"
}

variable "service_name" {
  description = "Name of the service"
  type        = string
}
