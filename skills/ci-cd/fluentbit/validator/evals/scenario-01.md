# Scenario 01: Tag Routing: Detecting Unmatched INPUT Tags That Silently Drop Logs

## User Prompt

An SRE team has been investigating a "missing logs" incident in production. Their Kubernetes logging pipeline appeared healthy — Fluent Bit was running, no errors in the Fluent Bit pod logs — but application logs from their payment service were simply not appearing in their log aggregation backend. After two hours of debugging, they suspect a tag routing misconfiguration.

Validate the Fluent Bit configuration below with a focus on tag routing. Trace every INPUT tag through all FILTER and OUTPUT Match patterns. Identify any INPUT tags that are not covered by an OUTPUT Match, any FILTER Match patterns that do not correspond to INPUT tags, and any orphaned sections. Produce a corrected configuration and a routing analysis report.

## Output Specification

Produce:
- A corrected `fluent-bit.conf` with tag routing issues fixed
- A `routing-report.md` that: (1) shows the complete tag flow for each INPUT, (2) identifies each routing gap, and (3) explains the real-world consequence of each gap (e.g., which service's logs would be dropped)

## Input Files

The following file is provided as input.

=============== FILE: inputs/fluent-bit.conf ===============
[SERVICE]
    Flush         5
    Daemon        Off
    Log_Level     info
    HTTP_Server   On
    HTTP_Listen   0.0.0.0
    HTTP_Port     2020

[INPUT]
    Name              tail
    Tag               kube.app.*
    Path              /var/log/containers/*_default_*.log
    Exclude_Path      /var/log/containers/*fluent-bit*.log
    Parser            docker
    Mem_Buf_Limit     50MB
    DB                /var/log/flb_kube.db

[INPUT]
    Name              tail
    Tag               kube.payment.*
    Path              /var/log/containers/*payment*.log
    Exclude_Path      /var/log/containers/*fluent-bit*.log
    Parser            docker
    Mem_Buf_Limit     50MB
    DB                /var/log/flb_payment.db

[INPUT]
    Name              systemd
    Tag               host.systemd
    Systemd_Filter    _SYSTEMD_UNIT=kubelet.service
    Read_From_Tail    On

[FILTER]
    Name              kubernetes
    Match             kube.app.*
    Kube_URL          https://kubernetes.default.svc:443
    Merge_Log         On

[FILTER]
    Name              grep
    Match             kube.payment.*
    Regex             log ERROR|WARN

[OUTPUT]
    Name              es
    Match             kube.app.*
    Host              elasticsearch.logging.svc
    Port              9200
    Index             app-logs
    tls               On
    tls.verify        On

[OUTPUT]
    Name              es
    Match             kube.infra.*
    Host              elasticsearch.logging.svc
    Port              9200
    Index             infra-logs
    tls               On
    tls.verify        On

## Expected Behavior

1. Trace all three INPUT tags (kube.app.*, kube.payment.*, host.systemd) through FILTER and OUTPUT Match patterns
2. Identify that kube.payment.* has a FILTER but no matching OUTPUT, meaning payment service logs are silently dropped
3. Identify that host.systemd has no FILTER or OUTPUT Match pattern, meaning systemd logs are silently dropped
4. Identify that the kube.infra.* OUTPUT has no corresponding INPUT that produces matching tags (orphaned OUTPUT)
5. Explain the real-world consequence of each routing gap (which service's logs are lost)
6. Produce a corrected configuration that adds appropriate OUTPUT sections for the unmatched INPUT tags

## Success Criteria

- **kube.payment.* unmatched OUTPUT detected**: routing-report.md (or equivalent) identifies that the kube.payment.* INPUT tag has a FILTER but no OUTPUT Match pattern covers it, meaning payment service logs are silently dropped
- **host.systemd unmatched OUTPUT detected**: routing-report.md identifies that the host.systemd INPUT tag has no FILTER or OUTPUT Match pattern, meaning systemd logs are silently dropped
- **kube.infra.* orphaned OUTPUT detected**: routing-report.md identifies that the kube.infra.* OUTPUT Match pattern has no corresponding INPUT that produces matching tags
- **Real-world consequence explained for each gap**: routing-report.md explains the consequence of each routing gap (e.g., payment service logs lost, systemd/kubelet events not forwarded)
- **Tag flow documented for each INPUT**: routing-report.md shows the full tag flow for each INPUT: tag produced -> FILTER(s) matched -> OUTPUT(s) matched
- **kube.payment.* routing fixed in output**: In the corrected fluent-bit.conf, an OUTPUT section with Match kube.payment.* (or a wildcard covering it) is added
- **host.systemd routing fixed in output**: In the corrected fluent-bit.conf, an OUTPUT section with Match host.systemd (or a wildcard covering it) is added
- **Orphaned OUTPUT remediated or explained**: In the corrected fluent-bit.conf, the kube.infra.* OUTPUT is either updated to match a real INPUT tag, given a corresponding INPUT, or removed with an explanation

## Failure Conditions

- The kube.payment.* tag routing gap (no OUTPUT) is not identified
- The host.systemd tag routing gap (no FILTER or OUTPUT) is not identified
- The orphaned kube.infra.* OUTPUT (no matching INPUT) is not flagged
- Real-world consequences of the routing gaps are not explained
- Tag flow is not traced for each INPUT through filters and outputs
- Corrected configuration does not add an OUTPUT for kube.payment.*
- Corrected configuration does not add an OUTPUT for host.systemd
