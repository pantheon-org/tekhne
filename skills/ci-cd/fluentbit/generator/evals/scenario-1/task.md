# Log Forwarding to Authenticated HTTP Endpoint

## Problem/Feature Description

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
