---
name: fluentbit-validator
description: Validates syntax, checks pipeline tag connections, detects security misconfigurations, audits best practices, and performs dry-run testing for Fluent Bit configurations. Use this skill when working with Fluent Bit config files, validating syntax, checking for best practices, identifying security issues, performing dry-run testing, or troubleshooting configuration-related errors.
---

# Fluent Bit Config Validator

## Overview

This skill provides a comprehensive validation workflow for Fluent Bit configurations, combining syntax validation, semantic checks, security auditing, best practice enforcement, and dry-run testing. Validate Fluent Bit configs with confidence before deploying to production.

## Validation Workflow

Follow this sequential validation workflow. Each stage catches different types of issues.

> **Recommended:** For comprehensive validation, use `--check all` which runs all validation stages in sequence:
> ```bash
> python3 scripts/validate_config.py --file <config-file> --check all
> ```
> Individual check modes are available for targeted validation when debugging specific issues.

### Stage 1: Configuration File Structure

```bash
python3 scripts/validate_config.py --file <config-file> --check structure
```

**Common issues caught:** missing section headers, malformed key-value pairs, invalid section names, unclosed brackets, mixed tabs/spaces, UTF-8 encoding issues.

### Stage 2: Section Validation

```bash
python3 scripts/validate_config.py --file <config-file> --check sections
```

#### SERVICE Section
- Required: `Flush`
- Valid `Log_Level`: off, error, warn, info, debug, trace
- `Parsers_File` references must exist
- **Best practices:** Flush 1–5s; `HTTP_Server On`; `storage.metrics on`

#### INPUT Section
- Required: `Name`, valid plugin names, Tag format, file paths exist (tail), port range valid
- **Best practices:** Always set `Mem_Buf_Limit` (50–100MB); use `DB` for tail; set `Skip_Long_Lines On`

#### FILTER Section
- Required: `Name`, `Match` (or `Match_Regex`)
- Match pattern must correspond to at least one INPUT tag
- **Best practices:** Avoid bare `*` Match unless intentional; order parsers before modifiers

#### OUTPUT Section
- Required: `Name`, `Match`
- Valid plugins: elasticsearch, kafka, loki, s3, cloudwatch, http, forward, file, opentelemetry
- OpenTelemetry-specific: URI endpoints (metrics_uri, logs_uri, traces_uri)
- **Best practices:** Set `Retry_Limit 3–5`; configure `storage.total_limit_size`; enable TLS; use env vars for credentials

#### PARSER Section
- Required: `Name`, `Format`
- Valid formats: json, regex, logfmt, ltsv
- Regex syntax must be valid; `Time_Key` required when `Time_Format` is set
- **Best practices:** Use built-in parsers where possible; use `MULTILINE_PARSER` for stack traces

### Stage 3: Tag Consistency Check

```bash
python3 scripts/validate_config.py --file <config-file> --check tags
```

**Checks:** INPUT tags match FILTER Match patterns; FILTER tags match OUTPUT Match patterns; no orphaned filters or outputs; wildcard usage is correct.

**Example:**
```ini
[INPUT]
    Tag    kube.*     # Produces: kube.var.log.containers.pod.log

[FILTER]
    Match  kube.*     # Matches: ✅

[OUTPUT]
    Match  app.*      # Matches: ❌ No logs will reach this output
```

### Stage 4: Security Audit

```bash
python3 scripts/validate_config.py --file <config-file> --check security
```

**Checks:**
1. **Hardcoded credentials:** HTTP_User/Passwd, AWS keys, API tokens in plain text
2. **TLS configuration:** TLS disabled; `tls.verify Off`; missing certificate files
3. **File permissions:** DB and parser files readable/writable
4. **Network exposure:** INPUTs listening on 0.0.0.0 without auth; HTTP_Server exposed without auth

**Auto-fix pattern:**
```ini
# Before (insecure)
[OUTPUT]
    HTTP_User     admin
    HTTP_Passwd   password123

# After (secure)
[OUTPUT]
    HTTP_User     ${ES_USER}
    HTTP_Passwd   ${ES_PASSWORD}
```

### Stage 5: Performance Analysis

```bash
python3 scripts/validate_config.py --file <config-file> --check performance
```

**Checks:**
1. `Mem_Buf_Limit` set on all tail inputs; `storage.total_limit_size` set on outputs
2. Flush interval appropriate (1–5s)
3. `Skip_Long_Lines On`; `Refresh_Interval` set; compression on network outputs
4. Kubernetes: `Buffer_Size 0` for kubernetes filter recommended

**Example of well-tuned config:**
```ini
[SERVICE]
    Flush        1

[INPUT]
    Mem_Buf_Limit     50MB
    Skip_Long_Lines   On
    Refresh_Interval  10

[OUTPUT]
    storage.total_limit_size 5G
    Retry_Limit       3
    Compress          gzip
```

### Stage 6: Best Practice Validation

```bash
python3 scripts/validate_config.py --file <config-file> --check best-practices
```

**Checklist:**
- ✅ SERVICE section with Flush parameter
- ✅ HTTP_Server enabled for health checks
- ✅ Mem_Buf_Limit on all tail inputs
- ✅ DB file for tail inputs (position tracking)
- ✅ Retry_Limit on all outputs
- ✅ storage.total_limit_size on outputs
- ✅ TLS enabled for production
- ✅ Environment variables for credentials
- ✅ kubernetes filter for K8s environments
- ✅ Exclude_Path to prevent log loops

### Stage 7: Dry-Run Testing

```bash
fluent-bit -c <config-file> --dry-run
```

**Catches:** config parsing errors, plugin loading errors, parser syntax errors, file permission issues, missing dependencies.

**If fluent-bit binary is not available:** skip this stage, document that dry-run was skipped, and recommend testing in a development environment.

**Common errors:**
- `[error] [config] parser file 'parsers.conf' not found` → verify Parsers_File path
- `[error] [plugins] invalid plugin 'unknownplugin'` → check plugin name spelling
- `[error] [input:tail] invalid property 'InvalidParam'` → remove invalid parameter
- `[error] cannot open /var/log/containers/*.log` → check file permissions

### Stage 8: Documentation Lookup (if needed)

**Try context7 MCP first:**
```
Use mcp__context7__resolve-library-id with "fluent-bit"
Then use mcp__context7__get-library-docs with:
- context7CompatibleLibraryID: /fluent/fluent-bit-docs
- topic: "<plugin-type> <plugin-name> configuration"
- page: 1
```

**Fallback to WebSearch:**
```
Search query: "fluent-bit <plugin-type> <plugin-name> configuration parameters site:docs.fluentbit.io"
```

### Stage 9: Report and Fix Issues

**1. Summarize all issues:**
```
Validation Report for fluent-bit.conf
=====================================

Errors (3):
  - [Line 15] OUTPUT elasticsearch missing required parameter 'Host'
  - [Line 25] FILTER Match pattern 'app.*' doesn't match any INPUT tags
  - [Line 8] INPUT tail missing Mem_Buf_Limit (OOM risk)

Warnings (2):
  - [Line 30] OUTPUT elasticsearch has hardcoded password (security risk)
  - [Line 12] INPUT tail missing DB file (no crash recovery)

Info (1):
  - [Line 3] SERVICE Flush interval is 10s (consider reducing for lower latency)

Best Practices (2):
  - Consider enabling HTTP_Server for health checks
  - Consider enabling compression on OUTPUT elasticsearch
```

**2. Categorize by severity:**
- **Errors (must fix):** Configuration won't work, Fluent Bit won't start
- **Warnings (should fix):** Configuration works but has issues
- **Info (consider):** Optimization opportunities
- **Best Practices:** Recommended improvements

**3. Propose specific fixes:**
```ini
# Fix 1: Add missing Host parameter
[OUTPUT]
    Name  es
    Match *
    Host  elasticsearch.logging.svc  # Added
    Port  9200

# Fix 2: Add Mem_Buf_Limit to prevent OOM
[INPUT]
    Name              tail
    Tag               kube.*
    Path              /var/log/containers/*.log
    Mem_Buf_Limit     50MB  # Added

# Fix 3: Use environment variable for password
[OUTPUT]
    Name        es
    HTTP_User   admin
    HTTP_Passwd ${ES_PASSWORD}  # Changed from hardcoded
```

**4. Get user approval** via AskUserQuestion

**5. Apply approved fixes** using Edit tool

**6. Re-run validation** to confirm

**7. Provide completion summary** (fixed issues, per-check pass/fail status, and overall validation result)

**8. Report-only summary (when user declines fixes):**
```
📋 Validation Report Complete - No fixes applied

Summary:
  - Errors: 2 (must fix before deployment)
  - Warnings: 16 (should fix)
  - Info: 15 (optimization suggestions)

Critical Issues Requiring Attention:
  - [Line 5] Invalid Log_Level 'invalid_level'
  - [Line 52] [OUTPUT opentelemetry] missing required parameter 'Host'

Recommendations:
  - Review the errors above before deploying this configuration
  - Consider addressing warnings to improve reliability and security
  - Run validation again after manual fixes: python3 scripts/validate_config.py --file <config> --check all
```

## Integration with fluentbit-generator

This validator is automatically invoked by the fluentbit-generator skill after generating configurations. It can also be used standalone to validate existing configurations.

**Generator workflow:**
1. Generate configuration using fluentbit-generator
2. Automatically validate using fluentbit-validator
3. Fix any issues found
4. Re-validate until all checks pass
5. Deploy with confidence

## Resources

### scripts/

#### validate_config.py

- Main validation script with all checks integrated in a single file
- Usage: `python3 scripts/validate_config.py --file <config> --check <type>`
- Available check types: `all`, `structure`, `syntax`, `sections`, `tags`, `security`, `performance`, `best-practices`, `dry-run`
- Comprehensive 1000+ line validator covering all validation stages
- Returns detailed error messages with line numbers
- Supports JSON output format: `--json`

#### validate.sh

- Convenience wrapper script for easier invocation
- Usage: `bash scripts/validate.sh <config-file>`
- Automatically calls validate_config.py with proper Python interpreter

### Test Fixtures

The skill includes test configuration files in `references/test-fixtures/` for validating the validator itself. See `references/test-fixtures.md` for details on running tests.

### Documentation Sources

- [Fluent Bit Official Documentation](https://docs.fluentbit.io/manual)
- [Fluent Bit Operations and Best Practices](https://fluentbit.net/fluent-bit-operations-and-best-practices/)
- [Configuration File Format](https://docs.fluentbit.io/manual/administration/configuring-fluent-bit/classic-mode/configuration-file)
- Context7 Fluent Bit documentation (/fluent/fluent-bit-docs)
