# Scenario 07: Validate a Terraform Module Used by a Root Configuration

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
