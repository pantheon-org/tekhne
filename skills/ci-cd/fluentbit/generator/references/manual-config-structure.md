# Manual Configuration Structure

A full `fluent-bit.conf` skeleton for a Kubernetes tail pipeline, with an optional `parsers.conf`. Use it when `scripts/generate_config.py` does not cover the use case.

```ini
# ── SERVICE ─────────────────────────────────────────────────────────────────
[SERVICE]
    Flush           1          # seconds; lower=lower latency, higher CPU
    Daemon          Off        # Off in containers
    Log_Level       info       # info for prod, debug for troubleshooting
    Parsers_File    parsers.conf
    HTTP_Server     On         # enables /api/v1/health for K8s probes
    HTTP_Listen     0.0.0.0
    HTTP_Port       2020
    storage.metrics on

# ── INPUT ────────────────────────────────────────────────────────────────────
[INPUT]
    Name              tail
    Tag               kube.*
    Path              /var/log/containers/*.log
    Exclude_Path      /var/log/containers/*fluent-bit*.log
    Parser            docker
    DB                /var/log/flb_kube.db   # position tracking across restarts
    Mem_Buf_Limit     50MB                   # always set to prevent OOM
    Skip_Long_Lines   On
    Refresh_Interval  10

# ── FILTER ───────────────────────────────────────────────────────────────────
[FILTER]
    Name                kubernetes
    Match               kube.*
    Kube_URL            https://kubernetes.default.svc:443
    Kube_CA_File        /var/run/secrets/kubernetes.io/serviceaccount/ca.crt
    Kube_Token_File     /var/run/secrets/kubernetes.io/serviceaccount/token
    Kube_Tag_Prefix     kube.var.log.containers.
    Merge_Log           On
    Keep_Log            Off
    K8S-Logging.Parser  On
    K8S-Logging.Exclude On
    Labels              On
    Annotations         Off

[FILTER]
    Name    modify
    Match   *
    Add     cluster_name my-cluster
    Add     environment  production

# ── OUTPUT ───────────────────────────────────────────────────────────────────
[OUTPUT]
    Name              es
    Match             *
    Host              elasticsearch.logging.svc
    Port              9200
    Logstash_Format   On
    Logstash_Prefix   k8s
    Retry_Limit       3
    storage.total_limit_size 5M
    tls               On
    tls.verify        On
```
