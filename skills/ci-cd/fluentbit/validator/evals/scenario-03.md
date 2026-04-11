# Scenario 03: Performance Analysis: Memory Limits, Flush Intervals, and Storage Configuration

## User Prompt

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

## Expected Behavior

1. Identify the `Flush 30` interval as problematic — a 30-second flush creates large backlogs and delivery delays under high load
2. Identify missing `Mem_Buf_Limit` on the `app.access` and `app.error` tail inputs as a risk for OOM-kills
3. Identify `Retry_Limit False` on the OUTPUT as a reliability risk (unbounded retries can cause log backpressure)
4. Identify missing storage configuration (storage.total_limit_size or equivalent) as a disk exhaustion risk
5. Identify missing DB paths on tail inputs as a position tracking gap
6. Produce a corrected config with a lower Flush value, Mem_Buf_Limit on all inputs, and a numeric Retry_Limit

## Success Criteria

- Performance issues related to the high Flush interval are identified and explained in the report
- Missing Mem_Buf_Limit on tail inputs is flagged as an OOM risk
- `Retry_Limit False` is flagged as a reliability concern
- Missing storage configuration is identified as a disk exhaustion risk
- The corrected config reduces the Flush interval to a more appropriate value
- The corrected config adds Mem_Buf_Limit to all tail inputs that are missing it
- The corrected config changes Retry_Limit to a numeric value
- Each finding includes: check type, affected section, risk description, and the fix applied

## Failure Conditions

- The high Flush interval (30 seconds) is not identified as a performance concern
- Missing Mem_Buf_Limit on the access and error log inputs is not flagged
- `Retry_Limit False` is not identified as a reliability risk
- Missing storage configuration is not mentioned as a disk exhaustion concern
- Corrected config retains the 30-second Flush interval
- Corrected config does not add Mem_Buf_Limit to inputs that were missing it
- Corrected config retains `Retry_Limit False`
