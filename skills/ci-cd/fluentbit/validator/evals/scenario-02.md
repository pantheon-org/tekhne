# Scenario 02: Security Audit: Hardcoded Credentials and Disabled TLS Verification

## User Prompt

A compliance engineer is reviewing a Fluent Bit configuration before a SOC 2 audit. The configuration forwards logs to an external SIEM and a cloud storage bucket. The engineer has been told the configuration was originally written for a local development environment and then deployed to production without security review.

Audit the configuration below for security issues. Identify every hardcoded credential, every TLS misconfiguration, and any network exposure concern. For each finding, explain the risk and propose a specific fix. Then produce a corrected configuration.

## Output Specification

Produce:
- A corrected `fluent-bit.conf` with all security issues fixed
- A `security-report.md` listing each finding with: severity (HIGH/MEDIUM/INFO), section/plugin type, description of the issue, and the fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/fluent-bit.conf ===============
[SERVICE]
    Flush         1
    Daemon        Off
    Log_Level     info
    HTTP_Server   On
    HTTP_Listen   0.0.0.0
    HTTP_Port     2020

[INPUT]
    Name    tail
    Tag     app.logs
    Path    /var/log/app/*.log
    Mem_Buf_Limit  50MB
    DB      /var/log/flb_app.db

[FILTER]
    Name    record_modifier
    Match   app.logs
    Record  environment production
    Record  host ${HOSTNAME}

[OUTPUT]
    Name         es
    Match        app.logs
    Host         siem.example.com
    Port         9243
    HTTP_User    admin
    HTTP_Passwd  Log$tream2024!
    tls          On
    tls.verify   Off
    Index        prod-app-logs

[OUTPUT]
    Name              s3
    Match             app.logs
    region            us-east-1
    bucket            prod-logs-archive
    AWS_Auth          On
    AWS_Access_Key_ID AKIAIOSFODNN7EXMAPLZ
    AWS_Secret_Access_Key wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
    total_file_size   100M
    upload_timeout    10m

## Expected Behavior

1. Detect the hardcoded `HTTP_Passwd` value in the Elasticsearch OUTPUT and flag it as a credential exposure finding
2. Detect the hardcoded `AWS_Access_Key_ID` and `AWS_Secret_Access_Key` values in the S3 OUTPUT and flag both as credential exposure findings
3. Detect `tls.verify Off` in the Elasticsearch OUTPUT and flag it as a TLS verification bypass
4. Replace `HTTP_Passwd` with a `${ENV_VAR}` reference in the corrected config
5. Replace `AWS_Access_Key_ID` and `AWS_Secret_Access_Key` with `${ENV_VAR}` references in the corrected config
6. Set `tls.verify On` in the corrected Elasticsearch OUTPUT
7. Assign severity levels (HIGH/MEDIUM/INFO) and explain the risk for each finding

## Expected Behavior

1. Flag the hardcoded HTTP_Passwd credential in the Elasticsearch OUTPUT
2. Flag both hardcoded AWS credentials (Access Key ID and Secret Access Key) in the S3 OUTPUT
3. Flag `tls.verify Off` as a TLS certificate verification bypass
4. Replace all hardcoded credentials with `${ENV_VAR}` references in the corrected config
5. Set `tls.verify On` in the corrected Elasticsearch OUTPUT
6. Assign severity to each finding and explain the associated risk

## Success Criteria

- **Hardcoded HTTP_Passwd detected**: security-report.md (or equivalent) flags the plaintext password in the Elasticsearch OUTPUT HTTP_Passwd field as a hardcoded credential
- **Hardcoded AWS credentials detected**: security-report.md flags both AWS_Access_Key_ID and AWS_Secret_Access_Key in the S3 OUTPUT as hardcoded credentials
- **tls.verify Off detected**: security-report.md flags `tls.verify Off` in the Elasticsearch OUTPUT as a TLS certificate verification bypass
- **HTTP_Passwd replaced with env var reference**: In the corrected fluent-bit.conf, HTTP_Passwd uses a ${ENV_VAR} reference (e.g., ${ES_PASSWORD}) instead of the plaintext value
- **AWS credentials replaced with env var references**: In the corrected fluent-bit.conf, AWS_Access_Key_ID and AWS_Secret_Access_Key use ${ENV_VAR} references instead of literal values
- **tls.verify corrected**: In the corrected fluent-bit.conf, tls.verify is set to On in the Elasticsearch OUTPUT
- **Severity assigned to each finding**: security-report.md assigns a severity (HIGH, MEDIUM, or INFO) to each finding
- **Risk explanation for each finding**: security-report.md provides a brief explanation of the risk for each finding (e.g., credential exposure in version control, MITM vulnerability)

## Failure Conditions

- The hardcoded HTTP_Passwd is not identified as a security finding
- The hardcoded AWS_Access_Key_ID or AWS_Secret_Access_Key is not flagged as a credential exposure finding
- `tls.verify Off` is not identified as a TLS misconfiguration
- Corrected config still contains the literal password in HTTP_Passwd
- Corrected config still contains literal AWS credential values
- `tls.verify` remains Off in the corrected Elasticsearch OUTPUT
- No severity levels are assigned to findings in the report
