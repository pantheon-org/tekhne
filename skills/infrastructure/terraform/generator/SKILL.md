---
name: terraform-generator
description: Generate Terraform modules, configure providers, define variables and outputs, set up remote state backends, and write production-ready HCL (.tf files) following current standards. Use when creating new Terraform resources, building Terraform projects, writing infrastructure as code (IaC), scaffolding AWS/Azure/GCP cloud infrastructure, working with terraform plan/apply workflows, or structuring multi-environment configurations.
---

# Terraform Generator

## Overview

This skill enables the generation of production-ready Terraform configurations following best practices and current standards. Automatically integrates validation and documentation lookup for custom providers and modules.

## Core Workflow

Work through these steps in order. Do not skip any step.

### Step 1: Understand Requirements

Analyze the user's request to determine:
- Which Terraform providers are required (AWS, Azure, GCP, custom, etc.)
- Whether any modules are being used (official, community, or custom)
- Version constraints for providers and modules
- Variable inputs and outputs needed
- State backend configuration (local, S3, remote, etc.)

### Step 2: Check for Custom Providers/Modules

**Standard providers** (no lookup needed):
- hashicorp/aws, hashicorp/azurerm, hashicorp/google, hashicorp/kubernetes, other official HashiCorp providers

**Custom/third-party providers/modules** (require documentation lookup):
- Third-party providers (e.g., datadog/datadog, mongodb/mongodbatlas)
- Custom or community modules from Terraform Registry
- Private or company-specific modules

**When custom providers/modules are detected:**

1. Use WebSearch to find version-specific documentation:
   ```
   Search query format: "[provider/module name] terraform [version] documentation [specific resource]"
   Example: "datadog terraform provider v3.30 monitor resource documentation"
   Example: "terraform-aws-modules vpc version 5.0 documentation"
   ```

2. Focus searches on official documentation (registry.terraform.io, provider websites): required/optional arguments, attribute references, example usage, version compatibility notes.

3. If Context7 MCP is available and the provider/module is supported, use it as an alternative:
   ```
   mcp__context7__resolve-library-id → mcp__context7__get-library-docs
   ```

### Step 2.5: Consult Reference Files

Before generating configuration, read the relevant reference files:

```
Read(file_path: ".claude/skills/terraform-generator/references/terraform_best_practices.md")
Read(file_path: ".claude/skills/terraform-generator/references/provider_examples.md")
```

**When to consult each reference:**

| Reference | Read When |
|-----------|-----------|
| `terraform_best_practices.md` | Always - contains required patterns |
| `common_patterns.md` | Multi-environment, workspace, or complex setups |
| `provider_examples.md` | Generating AWS, Azure, GCP, or K8s resources |
| `modern_features.md` | Using Terraform 1.8+ features (ephemeral resources, write-only args, actions, etc.) |

### Step 3: Generate Terraform Configuration

**File Organization:**
```
terraform-project/
├── main.tf           # Primary resource definitions
├── variables.tf      # Input variable declarations
├── outputs.tf        # Output value declarations
├── versions.tf       # Provider version constraints
├── terraform.tfvars  # Variable values (optional, for examples)
└── backend.tf        # Backend configuration (optional)
```

**Best Practices to Follow:**

1. **Provider Configuration:**
   ```hcl
   terraform {
     required_version = ">= 1.10, < 2.0"

     required_providers {
       aws = {
         source  = "hashicorp/aws"
         version = "~> 6.0"  # Latest: v6.23.0 (Dec 2025)
       }
     }
   }

   provider "aws" {
     region = var.aws_region
   }
   ```

2. **Resource Naming:** Use descriptive snake_case names; include resource type in name when helpful.
   ```hcl
   resource "aws_instance" "web_server" { ... }
   ```

3. **Variable Declarations:**
   ```hcl
   variable "instance_type" {
     description = "EC2 instance type for web servers"
     type        = string
     default     = "t3.micro"

     validation {
       condition     = contains(["t3.micro", "t3.small", "t3.medium"], var.instance_type)
       error_message = "Instance type must be t3.micro, t3.small, or t3.medium."
     }
   }
   ```

4. **Output Values:**
   ```hcl
   output "instance_public_ip" {
     description = "Public IP address of the web server"
     value       = aws_instance.web_server.public_ip
   }
   ```

5. **Use Data Sources for References:**
   ```hcl
   data "aws_ami" "ubuntu" {
     most_recent = true
     owners      = ["099720109477"] # Canonical

     filter {
       name   = "name"
       values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
     }
   }
   ```

6. **Module Usage:**
   ```hcl
   module "vpc" {
     source  = "terraform-aws-modules/vpc/aws"
     version = "5.0.0"

     name = "my-vpc"
     cidr = "10.0.0.0/16"

     azs             = ["us-east-1a", "us-east-1b"]
     private_subnets = ["10.0.1.0/24", "10.0.2.0/24"]
     public_subnets  = ["10.0.101.0/24", "10.0.102.0/24"]
   }
   ```

7. **Use locals for Computed Values:**
   ```hcl
   locals {
     common_tags = {
       Environment = var.environment
       ManagedBy   = "Terraform"
       Project     = var.project_name
     }
   }
   ```

8. **Lifecycle Rules When Appropriate:**
   ```hcl
   resource "aws_instance" "example" {
     lifecycle {
       create_before_destroy = true
       prevent_destroy       = true
       ignore_changes        = [tags]
     }
   }
   ```

9. **Dynamic Blocks for Repeated Configuration:**
   ```hcl
   resource "aws_security_group" "example" {
     dynamic "ingress" {
       for_each = var.ingress_rules
       content {
         from_port   = ingress.value.from_port
         to_port     = ingress.value.to_port
         protocol    = ingress.value.protocol
         cidr_blocks = ingress.value.cidr_blocks
       }
     }
   }
   ```

10. **Comments and Documentation:** Add comments explaining complex logic, document why certain values are used, and include examples in variable descriptions.

**Security Best Practices:**
- Never hardcode sensitive values (use variables)
- Use data sources for AMIs and other dynamic values
- Implement least-privilege IAM policies
- Enable encryption by default
- Use secure backend configurations

### Data Sources for Dynamic Values

Always use data sources for dynamic infrastructure values instead of hardcoding:

| Use Case | Data Source | Example |
|----------|-------------|---------|
| Current region | `data "aws_region" "current" {}` | `data.aws_region.current.name` |
| Current account | `data "aws_caller_identity" "current" {}` | `data.aws_caller_identity.current.account_id` |
| Available AZs | `data "aws_availability_zones" "available" {}` | `data.aws_availability_zones.available.names` |
| Latest AMI | `data "aws_ami" "..."` | With `most_recent = true` and filters |
| Existing VPC | `data "aws_vpc" "..."` | Reference existing infrastructure |

### Lifecycle Rules on Critical Resources

Add `prevent_destroy = true` on resources that could cause data loss or service disruption if accidentally destroyed:

```hcl
# KMS Keys - protect from deletion
resource "aws_kms_key" "encryption" {
  lifecycle { prevent_destroy = true }
}

# Databases - protect from deletion
resource "aws_db_instance" "main" {
  lifecycle { prevent_destroy = true }
}

# S3 Buckets with data - protect from deletion
resource "aws_s3_bucket" "data" {
  lifecycle { prevent_destroy = true }
}
```

**Resources that must have `prevent_destroy = true`:**
- KMS keys (`aws_kms_key`)
- RDS databases (`aws_db_instance`, `aws_rds_cluster`)
- S3 buckets containing data
- DynamoDB tables with data
- ElastiCache clusters
- Secrets Manager secrets

### S3 Lifecycle Best Practices

When creating S3 buckets with lifecycle configurations, always include a rule to abort incomplete multipart uploads:

```hcl
resource "aws_s3_bucket_lifecycle_configuration" "main" {
  bucket = aws_s3_bucket.main.id

  # Abort incomplete multipart uploads to prevent storage costs (Checkov CKV_AWS_300)
  rule {
    id     = "abort-incomplete-uploads"
    status = "Enabled"

    filter {}

    abort_incomplete_multipart_upload {
      days_after_initiation = 7
    }
  }

  rule {
    id     = "transition-to-ia"
    status = "Enabled"

    filter {
      prefix = ""
    }

    transition {
      days          = 90
      storage_class = "STANDARD_IA"
    }

    noncurrent_version_transition {
      noncurrent_days = 30
      storage_class   = "STANDARD_IA"
    }

    noncurrent_version_expiration {
      noncurrent_days = 365
    }
  }
}
```

### Step 4: Validate Generated Configuration

After generating Terraform files, validate them using the devops-skills:terraform-validator skill:

```
Invoke: Skill(devops-skills:terraform-validator)
```

The devops-skills:terraform-validator skill will:
1. Check HCL syntax with `terraform fmt -check`
2. Initialize the configuration with `terraform init`
3. Validate the configuration with `terraform validate`
4. Run security scan with Checkov
5. Perform dry-run testing (if requested) with `terraform plan`

#### Fix-and-Revalidate Loop

If validation fails, fix the issues and re-run the validator. Do NOT proceed to Step 5 until all checks pass.

```
┌─────────────────────────────────────────────────────────┐
│  VALIDATION FAILED?                                      │
│                                                          │
│  ┌─────────┐    ┌─────────┐    ┌─────────────────────┐  │
│  │  Fix    │───▶│ Re-run  │───▶│ All checks pass?    │  │
│  │  Issue  │    │ Skill   │    │ YES → Step 5        │  │
│  └─────────┘    └─────────┘    │ NO  → Loop back     │  │
│       ▲                         └─────────────────────┘  │
│       │                                    │             │
│       └────────────────────────────────────┘             │
└─────────────────────────────────────────────────────────┘
```

**Common validation failures to fix:**

| Check | Issue | Fix |
|-------|-------|-----|
| `CKV_AWS_300` | Missing abort multipart upload | Add `abort_incomplete_multipart_upload` rule |
| `CKV_AWS_24` | SSH open to 0.0.0.0/0 | Restrict to specific CIDR |
| `CKV_AWS_16` | RDS encryption disabled | Add `storage_encrypted = true` |
| `terraform validate` | Invalid resource argument | Check provider documentation |

**If custom providers are detected during validation:**
- The devops-skills:terraform-validator skill will automatically fetch documentation
- Use the fetched documentation to fix any issues

### Step 5: Provide Usage Instructions

After successful generation and validation with all checks passing, provide the user with:

```markdown
## Generated Files

| File | Description |
|------|-------------|
| `path/to/main.tf` | Main resource definitions |
| `path/to/variables.tf` | Input variables |
| `path/to/outputs.tf` | Output values |
| `path/to/versions.tf` | Provider version constraints |

## Next Steps

1. Review and customize `terraform.tfvars` with your values
2. Initialize Terraform: `terraform init`
3. Review the execution plan: `terraform plan`
4. Apply the configuration: `terraform apply`

## Customization Notes

- [ ] Update `variable_name` in terraform.tfvars
- [ ] Configure backend in backend.tf for remote state
- [ ] Adjust resource names/tags as needed

## Security Reminders

⚠️ Before applying:
- Review IAM policies and permissions
- Ensure sensitive values are NOT committed to version control
- Configure state backend with encryption enabled
- Set up state locking for team collaboration
```

## Common Generation Patterns

### Pattern 1: Simple Resource Creation

User request: "Create an AWS S3 bucket with versioning"

Generated files:
- `main.tf` - S3 bucket resource with versioning enabled
- `variables.tf` - Bucket name, tags variables
- `outputs.tf` - Bucket ARN and name outputs
- `versions.tf` - AWS provider version constraints

### Pattern 2: Module-Based Infrastructure

User request: "Set up a VPC using the official AWS VPC module"

Actions:
1. Identify module: terraform-aws-modules/vpc/aws
2. Web search for latest version and documentation
3. Generate configuration using module with appropriate inputs
4. Validate with devops-skills:terraform-validator

### Pattern 3: Multi-Provider Configuration

User request: "Create infrastructure across AWS and Datadog"

Actions:
1. Identify standard provider (AWS) and custom provider (Datadog)
2. Web search for Datadog provider documentation with version
3. Generate configuration with both providers properly configured
4. Ensure provider aliases if needed
5. Validate with devops-skills:terraform-validator

### Pattern 4: Complex Resource with Dependencies

User request: "Create an ECS cluster with ALB and auto-scaling"

Generated structure:
- Multiple resource blocks with proper dependencies
- Data sources for AMIs, availability zones, etc.
- Local values for computed configurations
- Comprehensive variables and outputs
- Proper dependency management using implicit references

## Error Handling

**Terraform-specific gotchas:**

1. **Provider Not Found:** Verify source address format (`namespace/name`) and version constraint syntax.

2. **Circular Dependencies:** Use explicit `depends_on` or break into separate modules.

3. **Validation Failures:** Run devops-skills:terraform-validator for detailed errors, fix iteratively.

## Version Awareness

Always consider version compatibility:

1. **Terraform Version:**
   - Use `required_version` constraint with both lower and upper bounds
   - Default to `>= 1.10, < 2.0` for modern features (ephemeral resources, write-only)
   - Use `>= 1.14, < 2.0` for latest features (actions, query command)
   - Document any version-specific features used

2. **Provider Versions (as of December 2025):**
   - AWS: `~> 6.0` (latest: v6.23.0)
   - Azure: `~> 4.0` (latest: v4.54.0)
   - GCP: `~> 7.0` (latest: v7.12.0) - 7.0 includes ephemeral resources & write-only attributes
   - Kubernetes: `~> 2.23`
   - Use `~>` for minor version flexibility, pin major versions

3. **Module Versions:** Always pin module versions, review module documentation for compatibility, test updates in non-production first.

> **Version feature matrix and modern Terraform features (1.8+):** See `references/modern_features.md` for the full feature matrix, ephemeral resources, write-only arguments, actions blocks, import blocks with `for_each`, list resources, and query command examples.

## Notes

- Always run devops-skills:terraform-validator after generation
- Web search is essential for custom providers/modules
- Follow the principle of least surprise in configurations
- Make configurations readable and maintainable
- Include helpful comments and documentation
- Generate realistic examples in terraform.tfvars when helpful

## References

### references/

- `terraform_best_practices.md` - Comprehensive best practices guide
- `common_patterns.md` - Common Terraform patterns and examples
- `provider_examples.md` - Example configurations for popular providers
- `modern_features.md` - Terraform 1.8+ features: ephemeral resources, write-only args, actions, import `for_each`, version feature matrix

To load a reference, use the Read tool:
```
Read(file_path: ".claude/skills/terraform-generator/references/[filename].md")
```

### assets/

Template files for common setups:
- `minimal-project/` - Minimal Terraform project template
- `aws-web-app/` - AWS web application infrastructure template
- `multi-env/` - Multi-environment configuration template
