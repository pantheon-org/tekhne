# Investigate and Document Undeclared Variable Warnings

## Problem/Feature Description

A junior engineer submitted a Terraform configuration that passes `terraform validate` but tflint is reporting several warnings about variables. The team lead is not sure whether these warnings are safe to ignore — the code "works" in the dev environment — and has asked you to investigate, document what these warnings mean and why they matter, and produce a remediated version of the configuration.

Run the full validation workflow on this configuration, pay special attention to any variable-related lint findings, and produce both a findings report and a corrected version of the configuration that resolves the variable declaration issues.

## Output Specification

Produce:
- `validation_report.md`: Full validation results documenting every step
- `fixed/variables.tf`: A corrected variables file that resolves any variable declaration issues found
- `fixed/main.tf`: A copy of main.tf if any changes are required there

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: terraform/main.tf ===============
terraform {
  required_version = ">= 1.0"
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

resource "aws_instance" "app" {
  ami           = var.ami_id
  instance_type = var.instance_type
  subnet_id     = var.subnet_id

  tags = {
    Name        = "${var.app_name}-${var.environment}"
    Owner       = var.owner_email
    CostCenter  = var.cost_center
  }
}

resource "aws_eip" "app" {
  instance = aws_instance.app.id
  domain   = "vpc"
}

=============== FILE: terraform/variables.tf ===============
variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "ami_id" {
  description = "AMI ID"
  type        = string
}

variable "instance_type" {
  description = "Instance type"
  type        = string
  default     = "t3.micro"
}

=============== FILE: terraform/terraform.tfvars ===============
ami_id        = "ami-0c55b159cbfafe1f0"
instance_type = "t3.small"
subnet_id     = "subnet-12345"
app_name      = "payments"
environment   = "staging"
owner_email   = "team@example.com"
cost_center   = "CC-1234"
