---
name: fluentbit-validator
description: Validates syntax, checks pipeline tag connections, detects security misconfigurations, audits best practices, and performs dry-run testing for Fluent Bit configurations. Use this skill when working with Fluent Bit config files, validating syntax, checking for best practices, identifying security issues, performing dry-run testing, or troubleshooting configuration-related errors.
---

# Fluent Bit Config Validator

## Overview

This skill provides a comprehensive validation workflow for Fluent Bit configurations, combining syntax validation, semantic checks, security auditing, best practice enforcement, and dry-run testing. Validate Fluent Bit configs with confidence before deploying to production.

## Philosophy

- Validate in stages, because each stage catches a different class of fault and a pass at one stage says nothing about the next.
- Treat tag routing as part of correctness: a config that parses can still drop every record.
- Keep secrets out of config files; reference environment variables instead.
- Propose fixes and wait for approval; never change a config the user has not agreed to change.
- Prefer the single `--check all` run, and drop to individual checks only to debug one stage.

## When to Use

- Validating an existing Fluent Bit configuration file before deployment
- Checking that INPUT tags reach FILTER and OUTPUT `Match` patterns
- Auditing a config for hardcoded credentials, disabled TLS or exposed network listeners
- Reviewing memory limits, flush intervals and storage settings
- Running a `fluent-bit --dry-run` parse check
- Verifying output from the fluentbit-generator skill

## When Not to Use

- Generating a new Fluent Bit configuration from requirements (use fluentbit-generator)
- Validating Kubernetes manifests, Helm charts or Dockerfiles
- Debugging a running Fluent Bit process from its runtime logs
- Checking configs for other log shippers such as Fluentd, Logstash or Vector

## Validation Workflow

Follow this sequential validation workflow. Each stage catches different types of issues.

> **Recommended:** For comprehensive validation, use `--check all` which runs all validation stages in sequence:
> ```bash
> python3 scripts/validate_config.py --file <config-file> --check all
> ```

### Validation Stages Summary

| Stage | Check Type | What It Validates |
|-------|-----------|-------------------|
| 1 | `structure` | Section headers, key-value format, brackets, indentation, encoding |
| 2 | `sections` | Required fields, valid plugins, field values per section type |
| 3 | `tags` | INPUT tags match FILTER/OUTPUT patterns, no orphaned sections |
| 4 | `security` | Hardcoded credentials, TLS config, file permissions, network exposure |
| 5 | `performance` | Memory limits, flush intervals, compression, buffer sizes |
| 6 | `best-practices` | HTTP server, retry limits, storage config, environment variables |
| 7 | `dry-run` | Config parsing, plugin loading, file permissions (requires fluent-bit binary) |

**Individual check usage** (for debugging specific issues):
```bash
python3 scripts/validate_config.py --file <config-file> --check <stage-type>
```

**Detailed section validation rules:** See `references/SECTION-RULES.md` for comprehensive requirements, valid plugins, field specifications, and best practices for SERVICE, INPUT, FILTER, OUTPUT, and PARSER sections

### Tag Consistency Check

**Validates:** INPUT tags match FILTER Match patterns; FILTER tags match OUTPUT Match patterns; no orphaned filters or outputs; wildcard usage is correct.

**Example:**
```ini
[INPUT]
    Tag    kube.*     # Produces: kube.var.log.containers.pod.log

[FILTER]
    Match  kube.*     # Matches: ✅

[OUTPUT]
    Match  app.*      # Matches: ❌ No logs will reach this output
```

### Security Audit

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

### Performance Analysis

**Key checks:**
- `Mem_Buf_Limit` set on all tail inputs
- `storage.total_limit_size` set on outputs
- Flush interval appropriate (1–5s)
- `Skip_Long_Lines On`; compression on network outputs
- Kubernetes: `Buffer_Size 0` for kubernetes filter recommended

### Dry-Run Testing

```bash
fluent-bit -c <config-file> --dry-run
```

**Catches:** config parsing errors, plugin loading errors, parser syntax errors, file permission issues, missing dependencies.

**If fluent-bit binary is not available:** skip this stage, document that dry-run was skipped, and recommend testing in a development environment.

### Documentation Lookup

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

### Report and Fix Issues

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

**8. Report-only summary (when the user chooses not to apply fixes):**
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

## Anti-Patterns

### NEVER validate config syntax without checking tag routing

**WHY:** A configuration that parses without errors can still drop all logs silently if no OUTPUT `Match` pattern covers the tags produced by the INPUTs. Syntax validation alone gives false confidence.

**BAD:** Confirm `fluent-bit --dry-run` passes and ship the configuration to production.

**GOOD:** Trace every INPUT tag through all FILTER and OUTPUT `Match` patterns, for example by running `python3 scripts/validate_config.py --file <config-file> --check tags`.

### NEVER skip TLS certificate validation in output plugins

**WHY:** `tls.verify Off` is convenient for local testing but is frequently forgotten when promoting a config to production, leaving log data in transit exposed to interception or man-in-the-middle attacks.

**BAD:** `tls.verify Off` present in a production output plugin targeting an external log aggregator.

**GOOD:** `tls.verify On` with `tls.ca_file /etc/ssl/certs/ca-certificates.crt` (or the appropriate CA bundle for your environment).

### NEVER use the same buffer path for multiple Fluent Bit instances

**WHY:** Overlapping `storage.path` directories corrupt the backpressure state database, causing one instance to consume or delete the other's buffered records and resulting in duplicate or lost log delivery.

**BAD:** Two Fluent Bit daemonsets sharing `/var/log/flb-storage/` as their storage path.

**GOOD:** Assign a distinct `storage.path` value to each Fluent Bit instance (e.g., `/var/log/flb-storage-app/` and `/var/log/flb-storage-infra/`).

### NEVER ignore pipeline tag connection warnings

**WHY:** An INPUT tag that no OUTPUT `Match` pattern covers causes Fluent Bit to silently drop those records. This is the most common root cause of "missing logs" production incidents and is invisible without explicit tag validation.

**BAD:** Dismiss unmatched tag warnings from the validator as noise and proceed with deployment.

**GOOD:** Treat any unmatched tag as a P1 configuration error; every INPUT tag must be covered by at least one OUTPUT `Match` pattern before the config is considered valid.

### NEVER leave hardcoded credentials in a configuration file

**WHY:** Plain-text `HTTP_User`, `HTTP_Passwd`, AWS keys and API tokens end up in version control, container images and config maps where anyone with read access can recover them.

**BAD:** `HTTP_Passwd   password123` in an `[OUTPUT]` section.

**GOOD:** `HTTP_Passwd   ${ES_PASSWORD}`, with the value supplied through the environment.

### NEVER omit Mem_Buf_Limit on tail inputs

**WHY:** Without a memory buffer limit, a slow or unavailable output lets a tail input buffer records until the process runs out of memory.

**BAD:** An `[INPUT]` with `Name tail` and no `Mem_Buf_Limit`.

**GOOD:** `Mem_Buf_Limit     50MB` on every tail input.

### NEVER apply fixes without user approval

**WHY:** Validation findings are proposals. Rewriting a config the user did not agree to change can alter routing or credentials in ways they did not review.

**BAD:** Edit the config file as soon as an error is found.

**GOOD:** Present the proposed fixes, ask for approval, apply only the approved ones, then re-run the validation.

### NEVER skip the dry-run stage without recording that it was skipped

**WHY:** The dry-run is the only stage that loads plugins and parses the config with the real binary. Silently omitting it makes a partial validation look complete.

**BAD:** Report the configuration as valid after stages 1 to 6 on a host with no `fluent-bit` binary, without saying so.

**GOOD:** State that the dry-run was skipped because the binary is unavailable, and recommend testing in a development environment.

### NEVER stop at the first error when reporting

**WHY:** Fixing one issue and re-running one at a time wastes cycles and hides related problems; users need the full list to plan the fix.

**BAD:** Report only the first failing check and abort.

**GOOD:** Run `python3 scripts/validate_config.py --file <config-file> --check all`, then group all findings into errors, warnings, info and best practices.

### NEVER expose the HTTP server or network inputs without authentication

**WHY:** INPUTs listening on 0.0.0.0 and an exposed `HTTP_Server` let anyone on the network inject records or read internal metrics.

**BAD:** An input bound to `0.0.0.0` with no authentication in front of it.

**GOOD:** Bind to a specific interface where possible and flag any unauthenticated listener in the security audit.

## References

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

## Quick Start

```bash
bash scripts/validate.sh <config-file>
python3 scripts/validate_config.py --file <config-file> --check all --json
fluent-bit -c <config-file> --dry-run
```
