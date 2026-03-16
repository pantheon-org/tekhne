# Validate a Terraform Module Used by a Root Configuration

## Problem/Feature Description

A platform team has developed a reusable `network` module for setting up VPC infrastructure. A new project is consuming this module from its root configuration. Before this setup goes to code review, an engineer needs to run a thorough validation pass and document the results.

The engineer has noticed that another team recently ran `terraform validate` inside the module directory directly and got a clean result, but the root configuration later failed during a plan. Your task is to perform a proper validation and write a validation report documenting your approach and findings.

## Output Specification

Produce `validation_report.md` documenting:
- Which configuration was used as the entry point for validation
- All validation steps performed with their outputs
- Any issues found
- Your assessment of the overall configuration health

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: modules/network/main.tf ===============
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

output "public_subnet_ids" {
  value = aws_subnet.public[*].id
}

=============== FILE: modules/network/variables.tf ===============
variable "vpc_cidr" {
  type        = string
}

variable "name" {
  type        = string
}

variable "environment" {
  type        = string
}

variable "public_subnets" {
  type        = list(string)
}

variable "availability_zones" {
  type        = list(string)
}

=============== FILE: root/main.tf ===============
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
  region = "us-east-1"
}

module "network" {
  source = "../modules/network"

  vpc_cidr           = "10.0.0.0/16"
  name               = "prod"
  environment        = "production"
  public_subnets     = ["10.0.1.0/24", "10.0.2.0/24"]
  availability_zones = ["us-east-1a", "us-east-1b"]
}

resource "aws_instance" "bastion" {
  ami           = var.bastion_ami
  instance_type = "t3.nano"
  subnet_id     = module.network.public_subnet_ids[0]
}

=============== FILE: root/variables.tf ===============
variable "bastion_ami" {
  description = "AMI ID for the bastion host"
  type        = string
}

=============== FILE: root/terraform.tfvars ===============
bastion_ami = "ami-0c55b159cbfafe1f0"
