# Scenario 02: Log Forwarding to Authenticated HTTP Endpoint

## User Prompt

A financial services company collects application logs from multiple servers and needs to forward them to a centralized log aggregation service over HTTPS. The destination accepts logs at `https://logs.acme-corp.internal/api/ingest` and requires HTTP Basic Authentication (username: `fluentbit-writer`, password provided at runtime). The security team has flagged a previous configuration where the password was written directly into the config file that ended up committed to the company's internal Git repository, triggering a secrets rotation exercise. They now require that no credentials appear in any configuration file.

The agent is given a broken starting configuration below. The team wants the config corrected and ready for production use, forwarding structured JSON logs from `/var/app/logs/*.log`.

## Output Specification

Produce a corrected `fluent-bit.conf` that addresses the security concern and is production-ready. Also produce `notes.md` listing the environment variables the deployment team needs to inject at container startup.

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/fluent-bit.conf ===============
[SERVICE]
    Flush       5
    Daemon      Off
    Log_Level   info

[INPUT]
    Name   tail
    Tag    app.*
    Path   /var/app/logs/*.log

[OUTPUT]
    Name    http
    Match   *
    Host    logs.acme-corp.internal
    Port    443
    URI     /api/ingest
    Format  json
    tls     On
    HTTP_User  fluentbit-writer
    HTTP_Passwd secretpassword123
    Retry_Limit 3

## Expected Behavior

1. Replace the hardcoded `HTTP_Passwd` value with an `${ENV_VAR}` reference so no literal password appears in the config
2. Verify no other plaintext secrets remain anywhere in the configuration
3. Document the required environment variable(s) in `notes.md` for the deployment team
4. Add `Mem_Buf_Limit` and a `DB` path to the tail INPUT for reliability
5. Add or confirm `tls.verify` is set appropriately on the OUTPUT
6. Retain the numeric `Retry_Limit` on the OUTPUT

## Success Criteria

- **Password via env var**: HTTP_Passwd uses ${ENV_VAR} syntax (e.g., ${HTTP_PASSWD} or ${FLUENTBIT_PASSWORD}) — the literal string 'secretpassword123' does NOT appear in the config
- **No plaintext secret**: No password, token, or key appears as a literal string value anywhere in the config file
- **Env var documented**: The notes.md (or equivalent output) names the environment variable(s) that the deployment team must inject
- **Mem_Buf_Limit added**: The tail INPUT plugin now includes a Mem_Buf_Limit setting
- **DB path added**: The tail INPUT plugin now includes a DB setting for position tracking
- **tls.verify present**: The OUTPUT plugin includes tls.verify On, or tls.verify Off with an explanatory comment
- **Retry_Limit present**: The OUTPUT plugin retains a numeric Retry_Limit (not False)
- **storage.total_limit_size**: The OUTPUT plugin or SERVICE section includes a storage.total_limit_size setting
- **compression gzip**: The HTTP OUTPUT plugin includes Compress gzip or compression gzip

## Failure Conditions

- The literal password 'secretpassword123' still appears in the corrected config
- Any other plaintext credential or token appears as a literal value in the config
- `notes.md` does not name the environment variable the deployment team must inject
- `Mem_Buf_Limit` is not added to the tail INPUT
- No `DB` path is added to the tail INPUT
- `tls.verify` is absent from the OUTPUT plugin
- `Retry_Limit` is changed to `False` or removed entirely
