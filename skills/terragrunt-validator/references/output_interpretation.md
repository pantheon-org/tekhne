# Output Interpretation

## Success Indicators

✅ **All checks passing:**

- All HCL files properly formatted
- Inputs are valid
- Terraform configuration is valid
- No linting issues
- No critical security issues
- Valid dependency graph
- Plan generated successfully

## Warning Indicators

⚠️ **Review needed:**

- Security warnings from tfsec (non-critical)
- Linting suggestions (best practices)
- Deprecated provider features
- Missing recommended configurations

## Error Indicators

✗ **Must fix:**

- Format errors
- Invalid inputs
- Terraform validation failures
- Circular dependencies
- Provider authentication failures
- State locking errors
