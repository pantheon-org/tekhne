# Validate a Terraform Web Application Module

## Problem/Feature Description

A platform engineering team has written a new Terraform configuration for provisioning a web application stack on AWS. The configuration defines an EC2 instance, a security group, an S3 bucket for static assets, and an RDS instance. Before opening a pull request and running this against a real AWS account, the team lead has asked you to perform a thorough offline validation — catching any formatting problems, syntax issues, misconfigurations, and security concerns without actually deploying anything.

You should produce a written validation report covering every check you performed and its result. The team has the terraform CLI installed. You should document the commands you ran and their outputs in the report so the team can reproduce your findings.

## Output Specification

Produce a file called `validation_report.md` containing:
- A record of every validation step performed, in the order performed
- The command run for each step and a summary of its output
- Any formatting changes applied
- Any lint findings from tflint (or a note that it was skipped and why)
- Any security findings, with severity
- A summary of overall pass/fail status

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
  region = var.region
}

resource "aws_instance" "web" {
  ami           = var.ami_id
  instance_type = var.instance_type

  vpc_security_group_ids = [aws_security_group.web.id]

  tags = {
    Name        = "${var.app_name}-web"
    Environment = var.environment
  }
}

resource "aws_security_group" "web" {
  name        = "${var.app_name}-sg"
  description = "Security group for web application"

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "SSH"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_s3_bucket" "assets" {
  bucket = "${var.app_name}-assets-${var.environment}"
}

resource "aws_db_instance" "main" {
  identifier        = "${var.app_name}-db"
  engine            = "mysql"
  engine_version    = "8.0"
  instance_class    = "db.t3.micro"
  allocated_storage = 20
  username          = "admin"
  password          = "supersecret123"
  skip_final_snapshot = true
}

=============== FILE: terraform/variables.tf ===============
variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "app_name" {
  description = "Application name"
  type        = string
}

variable "environment" {
  description = "Deployment environment"
  type        = string
}

variable "ami_id" {
  description = "AMI ID for EC2 instance"
  type        = string
}

variable "instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "t3.micro"
}

=============== FILE: terraform/terraform.tfvars ===============
app_name     = "mywebapp"
environment  = "dev"
ami_id       = "ami-0c55b159cbfafe1f0"
