# Tag Routing: Detecting Unmatched INPUT Tags That Silently Drop Logs

## Problem/Feature Description

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
