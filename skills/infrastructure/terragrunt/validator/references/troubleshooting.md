# Troubleshooting Guide

## Debug Mode

Enable debug output for troubleshooting:

```bash
# Terragrunt debug
TERRAGRUNT_DEBUG=1 terragrunt plan

# Terraform trace
TF_LOG=TRACE terragrunt plan
```

## Common Error Patterns

### "Error: Module not found"

- Clear cache: `rm -rf .terragrunt-cache`
- Re-initialize: `terragrunt init`

### "Error: Provider not found"

- Check provider configuration
- Run custom resource detection
- Use WebSearch to find correct provider source and version
- Verify required_providers block

### "Error: Invalid function call"

- Check Terragrunt version compatibility
- Review function syntax in documentation

### "Cycle detected in dependency graph"

- Review dependency chains
- Consider refactoring into single module
- Use data sources instead of dependencies

### "Error acquiring state lock"

- Check if another process is running
- Verify DynamoDB table (for S3 backend)
- Force unlock if safe: `terragrunt force-unlock <LOCK_ID>`

### "Error: unknown command" (Terragrunt 0.93+)

- Terragrunt 0.93+ has a new CLI with breaking changes
- Commands like `render-json`, `validate-inputs` are deprecated
- Use `terragrunt run -- <command>` for custom/unsupported commands
- Replace `graph-dependencies` with `dag graph`
- See: https://terragrunt.gruntwork.io/docs/migrate/cli-redesign/
