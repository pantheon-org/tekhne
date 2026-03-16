# Security Audit: Hardcoded Credentials and Disabled TLS Verification

## Problem/Feature Description

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
