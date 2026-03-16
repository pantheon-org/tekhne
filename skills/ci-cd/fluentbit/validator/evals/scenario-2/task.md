# Performance Analysis: Memory Limits, Flush Intervals, and Storage Configuration

## Problem/Feature Description

An infrastructure team running a high-throughput analytics platform has experienced repeated Fluent Bit OOM-kills and log delivery delays during peak hours. Their current configuration was set up by a contractor and has never been tuned for production load. The team wants a performance-focused analysis before their next product launch.

Analyze the Fluent Bit configuration below for performance issues. Identify missing memory limits, problematic flush intervals, missing storage configuration, and any other settings that would cause instability under high log volume. Produce a corrected configuration and a performance analysis report.

## Output Specification

Produce:
- A corrected `fluent-bit.conf` with all performance issues addressed
- A `performance-report.md` listing each finding with: check type, affected section, description of the risk, and the fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/fluent-bit.conf ===============
[SERVICE]
    Flush         30
    Daemon        Off
    Log_Level     info

[INPUT]
    Name    tail
    Tag     app.access
    Path    /var/log/nginx/access.log
    Parser  nginx

[INPUT]
    Name    tail
    Tag     app.error
    Path    /var/log/nginx/error.log
    Parser  nginx

[INPUT]
    Name    tail
    Tag     app.events
    Path    /var/log/app/events.log
    Parser  json
    Mem_Buf_Limit  10MB

[FILTER]
    Name    record_modifier
    Match   app.*
    Record  host ${HOSTNAME}

[OUTPUT]
    Name         forward
    Match        app.*
    Host         fluentd-aggregator.logging.svc
    Port         24224
    Retry_Limit  False
