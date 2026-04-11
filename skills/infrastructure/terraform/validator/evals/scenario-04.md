# Scenario 04: Validate a Terraform Module Used by a Root Configuration

## User Prompt

A platform team has developed a reusable `network` module for setting up VPC infrastructure. A new project is consuming this module from its root configuration. Before this setup goes to code review, an engineer needs to run a thorough validation pass and document the results.

The engineer has noticed that another team recently ran `terraform validate` inside the module directory directly and got a clean result, but the root configuration later failed during a plan. Your task is to perform a proper validation and write a validation report documenting your approach and findings.

The following files are provided as inputs:

```hcl
# modules/network/main.tf
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

resource "aws_vpc" "main" {
  cidr_block           = var.vpc_cidr
  enable_dns_hostnames = true
  enable_dns_support   = true
  tags = {
    Name        = "${var.name}-vpc"
    Environment = var.environment
  }
}

resource "aws_subnet" "public" {
  count             = length(var.public_subnets)
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.public_subnets[count.index]
  availability_zone = var.availability_zones[count.index]
  tags = {
    Name = "${var.name}-public-${count.index + 1}"
  }
}

output "vpc_id" {
  value = aws_vpc.main.id
}
```

```hcl
# modules/network/variables.tf
variable "vpc_cidr" { type = string }
variable "name" { type = string }
variable "environment" { type = string }
variable "public_subnets" { type = list(string) }
variable "availability_zones" { type = list(string) }
```

```hcl
# root/main.tf
module "network" {
  source = "../modules/network"
  vpc_cidr           = var.vpc_cidr
  name               = var.project_name
  environment        = var.environment
  public_subnets     = var.public_subnets
  availability_zones = var.availability_zones
}
```

```
# root/terraform.tfvars
bastion_ami = "ami-0c55b159cbfafe1f0"
```

Produce `validation_report.md` documenting:
- Which configuration was used as the entry point for validation
- All validation steps performed with their outputs
- Any issues found
- Your assessment of the overall configuration health

## Expected Behavior

1. Document running terraform commands from the `root/` directory (not `modules/network/`)
2. Explicitly mention that validating inside the module directory in isolation is insufficient or risky
3. Show `terraform init` run from the root configuration directory
4. Show `terraform validate` run from the root configuration directory
5. Document using the `terraform.tfvars` file or providing variable values when validating/planning
6. Show `terraform fmt` was run
7. Document running `bash scripts/extract_tf_info_wrapper.sh` on the configuration
8. Document running the checkov wrapper script
9. Note whether all variables in both root and module have type declarations
10. Conclude with a pass/fail or summary assessment of the configuration's validation status

## Success Criteria

- **Root as entry point**: The `validation_report.md` documents running terraform commands from the `root/` directory (not `modules/network/`)
- **Module isolation anti-pattern noted**: The report explicitly mentions that validating inside the module directory in isolation is insufficient or risky
- **terraform init from root**: The report shows `terraform init` run from the root configuration directory
- **terraform validate from root**: The report shows `terraform validate` run from the root configuration directory
- **tfvars used**: The report documents using the `terraform.tfvars` file or providing variable values when validating/planning
- **terraform fmt run**: The report shows `terraform fmt` was run
- **extract_tf_info_wrapper run**: The report documents running `bash scripts/extract_tf_info_wrapper.sh` on the configuration
- **Security scan run**: The report documents running the checkov wrapper script
- **Variables declared with types**: The report notes whether all variables in both root and module have type declarations
- **Overall health assessment**: The report concludes with a pass/fail or summary assessment of the configuration's validation status

## Failure Conditions

- Terraform commands are run from inside the `modules/network/` directory instead of from `root/`
- The report does not warn against the module-isolation anti-pattern
- `terraform init` is not run from the root configuration directory
- `terraform validate` is not run from the root configuration directory
- The `terraform.tfvars` file is not referenced
- `extract_tf_info_wrapper.sh` is not documented
- No security scan using the checkov wrapper is documented
- No overall health assessment or pass/fail summary is provided
