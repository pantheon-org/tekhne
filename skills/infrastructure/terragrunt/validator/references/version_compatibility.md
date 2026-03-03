# Terragrunt Version Compatibility

This skill is designed for **Terragrunt 0.93+** which includes the new CLI redesign.

## CLI Command Migration Reference

| Deprecated Command | New Command |
|-------------------|-------------|
| `run-all` | `run --all` |
| `hclfmt` | `hcl fmt` |
| `hclvalidate` | `hcl validate` |
| `validate-inputs` | `hcl validate --inputs` |
| `graph-dependencies` | `dag graph` |
| `render-json` | `render --json -w` |
| `terragrunt-info` | `info print` |
| `plan-all`, `apply-all` | `run --all plan`, `run --all apply` |

## Key Changes in 0.93+

- `terragrunt run --all` replaces `terragrunt run-all` for multi-module operations
- `terragrunt dag graph` replaces `terragrunt graph-dependencies` for dependency visualization
- `terragrunt hcl validate --inputs` replaces `validate-inputs` for input validation
- HCL syntax validation via `terragrunt hcl fmt --check` or `terragrunt hcl validate`
- Full validation requires `terragrunt init && terragrunt validate`

If using an older Terragrunt version, some commands may need adjustment.
