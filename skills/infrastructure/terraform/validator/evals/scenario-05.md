# Scenario 05: Investigate and Document Undeclared Variable Warnings

## User Prompt

A junior engineer submitted a Terraform configuration that passes `terraform validate` but tflint is reporting several warnings about variables. The team lead is not sure whether these warnings are safe to ignore — the code "works" in the dev environment — and has asked you to investigate, document what these warnings mean and why they matter, and produce a remediated version of the configuration.

Run the full validation workflow on this configuration, pay special attention to any variable-related lint findings, and produce both a findings report and a corrected version of the configuration that resolves the variable declaration issues.

The following files are provided as inputs:

```hcl
# terraform/main.tf
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
```

```hcl
# terraform/variables.tf
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
```

```
# terraform/terraform.tfvars
ami_id        = "ami-0c55b159cbfafe1f0"
instance_type = "t3.small"
subnet_id     = "subnet-12345"
app_name      = "payments"
environment   = "staging"
owner_email   = "team@example.com"
cost_center   = "CC-1234"
```

Produce:
- `validation_report.md`: Full validation results documenting every step
- `fixed/variables.tf`: A corrected variables file that resolves any variable declaration issues found
- `fixed/main.tf`: A copy of main.tf if any changes are required there

## Expected Behavior

1. Document running tflint (or noting it was skipped with reason) in `validation_report.md`
2. Identify the specific undeclared variables: `subnet_id`, `app_name`, `environment`, `owner_email`, `cost_center`
3. Explain why undeclared variables are a problem (silently treated as null, masking misconfiguration, runtime errors at apply time)
4. `fixed/variables.tf` declares all missing variables with a `type` attribute
5. `fixed/variables.tf` declares all missing variables with a `description` attribute
6. `fixed/variables.tf` includes declarations for all 5 missing variables: `subnet_id`, `app_name`, `environment`, `owner_email`, `cost_center`
7. Show `terraform validate` was run (after init) in `validation_report.md`
8. Show `terraform fmt` was run in `validation_report.md`
9. Document running the `extract_tf_info_wrapper.sh` script in `validation_report.md`
10. The report does NOT characterize the undeclared variable warnings as safe to ignore or non-critical

## Success Criteria

- **tflint run**: The `validation_report.md` documents running tflint (or noting it was skipped with reason)
- **Missing vars identified**: The report identifies the specific undeclared variables: `subnet_id`, `app_name`, `environment`, `owner_email`, `cost_center`
- **Risk explanation**: The report explains why undeclared variables are a problem (silently treated as null, masking misconfiguration, runtime errors at apply time)
- **Variables declared with types**: The `fixed/variables.tf` file declares all missing variables with a `type` attribute
- **Variables declared with descriptions**: The `fixed/variables.tf` file declares all missing variables with a `description` attribute
- **All 5 missing vars fixed**: `fixed/variables.tf` includes declarations for all 5 missing variables: `subnet_id`, `app_name`, `environment`, `owner_email`, `cost_center`
- **terraform validate run**: The `validation_report.md` shows `terraform validate` was run (after init)
- **terraform fmt run**: The `validation_report.md` shows `terraform fmt` was run
- **extract_tf_info_wrapper run**: The `validation_report.md` documents running the `extract_tf_info_wrapper.sh` script
- **Warnings not dismissed**: The report does NOT characterize the undeclared variable warnings as safe to ignore or non-critical

## Failure Conditions

- tflint is not run or documented
- The 5 undeclared variables (`subnet_id`, `app_name`, `environment`, `owner_email`, `cost_center`) are not all identified
- The risk of undeclared variables is not explained (treated as null at runtime, masking errors)
- `fixed/variables.tf` does not include type or description declarations for the missing variables
- Fewer than all 5 missing variables are declared in `fixed/variables.tf`
- The report characterizes the undeclared variable warnings as non-critical or safe to ignore
