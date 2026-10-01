# Scenario 05: Security Audit: Hardcoded Credentials and Disabled TLS Verification

## User Prompt

A compliance engineer is reviewing a Fluent Bit configuration before a SOC 2 audit. The configuration forwards logs to an external SIEM and a cloud storage bucket. The engineer has been told the configuration was originally written for a local development environment and then deployed to production without security review.

Audit the configuration below for security issues. Identify every hardcoded credential, every TLS misconfiguration, and any network exposure concern. For each finding, explain the risk and propose a specific fix. Then produce a corrected configuration.
