---
name: terraform-validator
description: Comprehensive toolkit for validating, linting, testing, and automating Terraform configurations and HCL files. Use this skill when working with Terraform files (.tf, .tfvars), validating infrastructure-as-code, debugging Terraform configurations, performing dry-run testing with terraform plan, or working with custom providers and modules.
---

# Terraform Validator

Comprehensive toolkit for validating, linting, and testing Terraform configurations with automated workflows for syntax validation, security scanning, and intelligent documentation lookup.

## Validation Workflow

Execute these steps in order. Steps marked **Required** must not be skipped.

| Step | Action | Required |
|------|--------|----------|
| 1 | Run `bash scripts/extract_tf_info_wrapper.sh <path>` | Required |
| 2 | Context7 lookup for all providers (explicit and implicit); WebSearch fallback if not found | Required |
| 3 | Read `references/security_checklist.md` | Required |
| 4 | Read `references/best_practices.md` | Required |
| 5 | Run `terraform fmt` | Required |
| 6 | Run `tflint` (or note as skipped if unavailable) | Recommended |
| 7 | Run `terraform init` (if not initialized) | Required |
| 8 | Run `terraform validate` | Required |
| 9 | Run `bash scripts/run_checkov.sh <path>` | Required |
| 10 | Cross-reference findings with `security_checklist.md` sections | Required |
| 11 | Generate report citing reference files | Required |

> Steps 3–4 (reading reference files) must be completed **before** running security scans. The reference files contain remediation patterns that must be cited in the report.

## External Documentation

| Tool | Documentation |
|------|---------------|
| **Terraform** | [developer.hashicorp.com/terraform](https://developer.hashicorp.com/terraform/docs) |
| **TFLint** | [github.com/terraform-linters/tflint](https://github.com/terraform-linters/tflint) |
| **Checkov** | [checkov.io](https://www.checkov.io/1.Welcome/Quick%20Start.html) |
| **Trivy** | [aquasecurity.github.io/trivy](https://aquasecurity.github.io/trivy) |

## Scripts Reference

Use these wrapper scripts instead of calling tools directly:

| Script | Purpose | Command |
|--------|---------|---------|
| `extract_tf_info_wrapper.sh` | Parse Terraform files for providers/modules (auto-handles python-hcl2 via temporary venv) | `bash scripts/extract_tf_info_wrapper.sh <path>` |
| `extract_tf_info.py` | Core parser (requires python-hcl2) | Use wrapper instead |
| `run_checkov.sh` | Wrapper for Checkov scans with enhanced output | `bash scripts/run_checkov.sh <path>` |
| `install_checkov.sh` | Install Checkov in isolated venv | `bash scripts/install_checkov.sh install` |

## Provider Documentation Lookup

**Detection and lookup workflow:**

```
1. Run extract_tf_info_wrapper.sh to get provider list
2. Collect explicit providers from "providers" array
3. Detect implicit providers from resource type prefixes:
   - Extract prefix (e.g., "random" from "random_id")
   - Common implicit: random, null, local, tls, time, archive, http, external
4. For each provider (explicit + implicit):
   a. Call: mcp__context7__resolve-library-id with "terraform-provider-{name}"
   b. Call: mcp__context7__get-library-docs with the resolved ID
   c. If Context7 fails: WebSearch("terraform-provider-{name} hashicorp documentation site:registry.terraform.io")
5. Include relevant provider guidance in validation report
```

**Note:** HashiCorp utility providers (random, null, local, time, tls, archive, external, http) are often not indexed in Context7 — use WebSearch directly for these.

## Required Reference Files

Read these files at the specified points in the workflow:

| When | Reference File | Content |
|------|----------------|---------|
| Before security scan | `references/security_checklist.md` | Security checks, Checkov/Trivy usage, remediation patterns |
| During validation | `references/best_practices.md` | Project structure, naming conventions, module design, state management |
| When errors occur | `references/common_errors.md` | Error database with causes and solutions |
| If Terraform >= 1.10 | `references/advanced_features.md` | Ephemeral values (1.10+), Actions (1.14+), List Resources (1.14+) |

## Security Finding Reports

When reporting security findings from Checkov/Trivy scans, cross-reference specific sections from `security_checklist.md`. The security checklist contains:
- Checkov check ID to section mappings
- Remediation patterns with code examples
- Severity guidelines

### Report Template for Security Findings

```markdown
### Security Issue: [Check ID]

**Finding:** [Description from checkov]
**Resource:** [Resource name and file:line]
**Severity:** [HIGH/MEDIUM/LOW]

**Reference:** security_checklist.md - "[Section Name]" (see relevant section for check ID)

**Remediation Pattern:**
[Copy relevant code example from security_checklist.md]

**Recommended Fix:**
[Specific fix for this configuration]
```

### Example Cross-Referenced Report

```markdown
### Security Issue: CKV_AWS_24

**Finding:** Security group allows SSH from 0.0.0.0/0
**Resource:** aws_security_group.web (main.tf:47-79)
**Severity:** HIGH

**Reference:** security_checklist.md - "Overly Permissive Security Groups"

**Remediation Pattern (from reference):**
```hcl
variable "admin_cidr" {
  description = "CIDR block for admin access"
  type        = string
}

resource "aws_security_group" "app" {
  ingress {
    description = "SSH from admin network only"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [var.admin_cidr]
  }
}
```

**Recommended Fix:** Replace `cidr_blocks = ["0.0.0.0/0"]` with a variable or specific CIDR range.
```

## Handling Missing Tools

When a validation tool is not installed:

```
1. Inform user what is missing and why it's needed
2. Provide the installation command
3. Ask: "Would you like me to install [tool] and continue?"
4. If yes: run installation and rerun the validation step
5. If no: note as skipped in report, continue with available tools
```

**If checkov is missing:** Ask to install via `bash scripts/install_checkov.sh install`, then rerun security scan.

**If tflint is missing:** Ask to install (`brew install tflint` on macOS or equivalent), note as skipped if declined.

**If python-hcl2 is missing:** `extract_tf_info_wrapper.sh` handles this automatically via a temporary venv — no user action required.

**Required tools:** `terraform fmt`, `terraform validate`  
**Optional but recommended:** `tflint`, `checkov`

## Advanced Features

Terraform 1.10+ introduces ephemeral values for secure secrets management. Terraform 1.14+ adds Actions for imperative operations and List Resources for querying infrastructure.

Read `references/advanced_features.md` when:
- Terraform version >= 1.10 is detected
- Configuration uses `ephemeral` blocks
- Configuration uses `action` blocks
- Configuration uses `.tfquery.hcl` files

## Integration with Other Skills

- **k8s-yaml-validator** — For Terraform Kubernetes provider validation
- **helm-validator** — When Terraform manages Helm releases
- **k8s-debug** — For debugging infrastructure provisioned by Terraform

## References

### Format and Lint

```bash
# Check formatting (dry-run)
terraform fmt -check -recursive .

# Apply formatting
terraform fmt -recursive .

# Run tflint
tflint --init              # Install plugins
tflint --recursive         # Lint all modules
tflint --format compact    # Compact output
```

### Validate Configuration

```bash
terraform init             # Downloads providers and modules
terraform validate         # Validate syntax
terraform validate -json   # JSON output
```

### Security Scanning

```bash
# Use the wrapper script
bash scripts/run_checkov.sh ./terraform

# With specific options
bash scripts/run_checkov.sh -f json ./terraform
bash scripts/run_checkov.sh --compact ./terraform
```

### Dry-Run Testing

```bash
terraform plan                               # Generate execution plan
terraform plan -out=tfplan                   # Save plan to file
terraform plan -var-file="production.tfvars" # Plan with var file
terraform plan -target=aws_instance.example  # Plan specific resource
```

**Plan output symbols:** `+` create · `-` destroy · `~` modify · `-/+` replace
