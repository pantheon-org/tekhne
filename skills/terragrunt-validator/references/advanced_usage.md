# Advanced Usage

## Custom Validation Rules

Create custom tflint rules by adding `.tflint.hcl`:

```hcl
plugin "terraform" {
  enabled = true
  preset  = "recommended"
}

plugin "aws" {
  enabled = true
  version = "0.27.0"
  source  = "github.com/terraform-linters/tflint-ruleset-aws"
}

rule "terraform_naming_convention" {
  enabled = true
}
```

## Custom Security Policies

Create custom tfsec policies by adding `.tfsec/config.yml`:

```yaml
minimum_severity: MEDIUM
exclude:
  - AWS001  # Example: exclude specific rules
```

## Dependency Graph Analysis

Analyze complex dependency chains:

```bash
# Generate detailed graph (Terragrunt 0.93+ syntax)
terragrunt dag graph > graph.dot

# Convert to visual format
dot -Tpng graph.dot > graph.png
dot -Tsvg graph.dot > graph.svg

# Analyze for circular dependencies
grep -A5 "cycle" <(terragrunt dag graph 2>&1)
```
