# Test Configuration Files

This directory contains test fixtures for validating the Fluent Bit configuration validator.

## Test Files

**Valid Configurations:**
- `valid-basic.conf` - Valid basic Kubernetes logging setup
- `valid-multioutput.conf` - Valid configuration with multiple outputs
- `valid-opentelemetry.conf` - Valid OpenTelemetry output configuration (Fluent Bit 2.x+)

**Invalid Configurations:**
- `invalid-missing-required.conf` - Missing required parameters
- `invalid-security-issues.conf` - Security vulnerabilities (hardcoded credentials, disabled TLS)
- `invalid-opentelemetry.conf` - OpenTelemetry configuration errors
- `invalid-tag-mismatch.conf` - Tag routing issues

## Running Tests

### Test Individual Config

```bash
# Test on valid config
python3 scripts/validate_config.py --file references/test-fixtures/valid-basic.conf

# Test on invalid config (should report errors)
python3 scripts/validate_config.py --file references/test-fixtures/invalid-security-issues.conf
```

### Test All Configs

```bash
for config in references/test-fixtures/*.conf; do
    echo "Testing $config"
    python3 scripts/validate_config.py --file "$config"
done
```

## Expected Results

Valid configurations should pass all validation checks. Invalid configurations should report specific errors matching their intended failure scenarios.
