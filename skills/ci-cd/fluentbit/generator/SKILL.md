---
name: fluentbit-generator
description: Generates, validates, and optimizes Fluent Bit configurations for production use. Use when creating new Fluent Bit configs, implementing log collection pipelines (INPUT, FILTER, OUTPUT sections), configuring Kubernetes log collection with metadata enrichment, forwarding logs to destinations (Elasticsearch, Loki, S3, Kafka, CloudWatch, OpenTelemetry), building multi-line log parsing, or converting existing logging configurations to Fluent Bit.
---

# Fluent Bit Config Generator

## Workflow: 4 Essential Steps

### Step 1: Gather Requirements

Identify the following before generating:
- **Input sources:** tail, systemd, tcp/udp, forward, http, syslog, exec
- **Processing:** parsing (JSON/regex/logfmt), multi-line, filtering, K8s enrichment, transformation
- **Output destinations:** Elasticsearch, Loki, S3, Kafka, CloudWatch, OpenTelemetry, HTTP, stdout
- **Constraints:** buffer limits, flush intervals, retry logic, TLS, worker threads

Use AskUserQuestion if key information is missing.

---

### Step 2: Generate Configuration

#### 2a. Always try the script first

```bash
python3 scripts/generate_config.py --help
```

**Supported use cases:** `kubernetes-elasticsearch`, `kubernetes-loki`, `kubernetes-cloudwatch`, `kubernetes-opentelemetry`, `application-multiline`, `syslog-forward`, `file-tail-s3`, `http-kafka`, `multi-destination`, `prometheus-metrics`, `lua-filtering`, `stream-processor`, `custom`

```bash
python3 scripts/generate_config.py --use-case kubernetes-elasticsearch --output fluent-bit.conf
python3 scripts/generate_config.py --use-case kubernetes-opentelemetry --cluster-name my-cluster --output fluent-bit.conf
```

#### 2b. Manual generation (when script doesn't cover the use case)

State explicitly why the script was not used (e.g., "Manual generation chosen because grep filter for log levels is not supported by the script").

**Before writing any manual config:**
1. Read the closest example from `examples/` — production-ready reference configs are available for all 13 use cases (e.g. `kubernetes-elasticsearch.conf`, `kubernetes-loki.conf`, `application-multiline.conf`, `multi-destination.conf`, `full-production.conf`, and others).
2. Read `examples/parsers.conf` — reuse existing parsers (docker, cri, json, nginx, apache, syslog-rfc3164/5424, multiline-java/python/go/ruby) before creating custom ones.

**Manual configuration structure** (`fluent-bit.conf` + optional `parsers.conf`):

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

**Common FILTER patterns** (use as needed, order matters — parsers before modifiers):

```ini
# Parse structured fields from a log key
[FILTER]
    Name          parser
    Match         *
    Key_Name      log
    Parser        json
    Reserve_Data  On

# Include/exclude by field value
[FILTER]
    Name          grep
    Match         *
    Regex         level (error|fatal|critical)
    Exclude       path /health

# Multi-line (stack traces)
[FILTER]
    Name                  multiline
    Match                 *
    multiline.key_content log
    multiline.parser      java, python, go

# Lua custom scripting
[FILTER]
    Name    lua
    Match   *
    script  /fluent-bit/scripts/filter.lua
    call    process_record

# Throttle (rate limiting)
[FILTER]
    Name      throttle
    Match     *
    Rate      1000
    Window    5
    Interval  1m
```

**Common OUTPUT patterns:**

```ini
# Grafana Loki
[OUTPUT]
    Name              loki
    Match             *
    Host              loki.default.svc
    Port              3100
    labels            job=fluent-bit, namespace=$kubernetes['namespace_name'], pod=$kubernetes['pod_name']
    label_keys        $stream
    remove_keys       kubernetes,stream
    auto_kubernetes_labels on
    line_format       json
    Retry_Limit       3

# AWS S3
[OUTPUT]
    Name              s3
    Match             *
    bucket            my-logs-bucket
    region            us-east-1
    total_file_size   100M
    upload_timeout    10m
    compression       gzip
    s3_key_format     /fluent-bit-logs/%Y/%m/%d/$TAG[0]/%H-%M-%S-$UUID.gz
    Retry_Limit       3

# Kafka
[OUTPUT]
    Name              kafka
    Match             *
    Brokers           kafka-broker-1:9092,kafka-broker-2:9092
    Topics            logs
    Format            json
    Timestamp_Key     @timestamp
    Retry_Limit       3

# AWS CloudWatch Logs
[OUTPUT]
    Name              cloudwatch_logs
    Match             *
    region            us-east-1
    log_group_name    /aws/fluent-bit/logs
    log_stream_prefix from-fluent-bit-
    auto_create_group On
    Retry_Limit       3

# OpenTelemetry (OTLP/HTTP)
[OUTPUT]
    Name              opentelemetry
    Match             *
    Host              opentelemetry-collector.observability.svc
    Port              4318
    logs_uri          /v1/logs
    add_label         cluster my-cluster
    add_label         environment production
    tls               On
    tls.verify        On
    Retry_Limit       3

# HTTP endpoint
[OUTPUT]
    Name              http
    Match             *
    Host              logs.example.com
    Port              443
    URI               /api/logs
    Format            json
    tls               On
    tls.verify        On
    Header            Authorization Bearer ${API_TOKEN}
    Compress          gzip
    Retry_Limit       3

# stdout (debug only)
[OUTPUT]
    Name    stdout
    Match   *
    Format  json_lines
```

**Plugin documentation lookup** (when needed for unfamiliar plugins):
1. Try context7 MCP: `mcp__context7__resolve-library-id` with `"fluent-bit"`, then `mcp__context7__get-library-docs` with the plugin topic.
2. Fallback: WebSearch `"fluent-bit" "<plugin-type>" "<plugin-name>" "configuration" site:docs.fluentbit.io`

---

### Step 3: Validate

**Syntax check** before finalizing:
- Section headers use `[SECTION]` format
- Key-value pairs are space-indented (not tabs)
- All `Match` tags are consistent with `Tag` values on inputs
- Parser references in filters exist in `parsers.conf` or `Parsers_File`

**Invoke devops-skills:fluentbit-validator** on the generated config to run:
- Required field checks and plugin parameter validation
- Tag consistency and parser reference validation
- Security checks (plaintext credentials, TLS)
- Best practice recommendations
- Dry-run test if `fluent-bit` binary is available

Fix any reported issues and re-validate until all checks pass.

---

### Step 4: Communicate Results

When delivering a configuration:
1. **Explain section choices** — why each plugin/setting was selected
2. **Flag required customizations** — parameters the user must adjust (cluster names, hosts, bucket names)
3. **Credential reminders** — always use `${ENV_VAR}` syntax, never hardcode secrets
4. **TLS guidance** — use `tls.verify On` in production; if `Off` is needed add an inline comment explaining why (e.g., `# Internal cluster with self-signed certs`)
5. **Validation status** — summarise validator output and any fixes applied

---

## Key Best Practices (Quick Reference)

| Concern | Recommendation |
|---|---|
| OOM prevention | `Mem_Buf_Limit 50MB` on every tail input |
| Crash recovery | `DB /var/log/flb_kube.db` on tail inputs |
| Log loops | `Exclude_Path *fluent-bit*.log` |
| Credentials | `${ENV_VAR}` only, never hardcode |
| TLS | `tls On` + `tls.verify On` in production |
| Retries | `Retry_Limit 3-5` on all outputs |
| Disk buffer | `storage.total_limit_size` to prevent exhaustion |
| Health checks | `HTTP_Server On`, probe `GET :2020/api/v1/health` |
| Bandwidth | Enable `compression gzip` on network outputs |
| Structured logs | Prefer JSON app logs; use `Merge_Log On` in K8s filter |

## Anti-Patterns

### NEVER use `Match *` on all output plugins simultaneously

- **WHY**: Broadcasting all logs to every output creates duplicate records in each destination, generates unexpected ingestion costs, and leaks logs intended for one system (e.g., a debug sink) into another (e.g., a billed SaaS platform).
- **BAD**: Three separate output plugins all configured with `Match *`.
- **GOOD**: Use distinct tag namespaces (`kube.*`, `app.*`, `system.*`) and route each namespace to its intended destination with a specific `Match` pattern.

### NEVER omit `Mem_Buf_Limit` on INPUT plugins

- **WHY**: Without a memory buffer limit, backpressure from a slow or unavailable output causes the input buffer to grow without bound, leading to OOM kills of the Fluent Bit process and log loss.
- **BAD**: `[INPUT] Name tail Tag app.*` with no `Mem_Buf_Limit` setting.
- **GOOD**: Add `Mem_Buf_Limit 50MB` to every tail input (adjust the value based on measured log volume).

### NEVER use `Retry_Limit False` in outputs without monitoring

- **WHY**: Infinite retries mask persistent delivery failures. Retry buffers accumulate on disk, eventually exhausting storage and causing Fluent Bit to drop new logs to protect itself.
- **BAD**: `Retry_Limit False` in an output plugin with no alerting on delivery failure metrics.
- **GOOD**: Set `Retry_Limit 5` and monitor for delivery failures using Fluent Bit's built-in Prometheus metrics (`/api/v1/metrics`).

### NEVER parse structured logs with a regexp parser when `json` or `logfmt` parsers apply

- **WHY**: Regexp parsers are fragile — they break when log format details change — and CPU-intensive compared to native parsers. Using them for standard formats sacrifices correctness and performance for no benefit.
- **BAD**: `Parser regex_json` configured to extract fields from JSON-formatted log lines.
- **GOOD**: `Parser json` — simpler, faster, and guaranteed to handle all valid JSON log output correctly.

### NEVER store sensitive values directly in `fluent-bit.conf`

- **WHY**: Configuration files are routinely committed to source control, copied into container images, and displayed in support tickets. Inline credentials are then exposed to anyone who can read the file.
- **BAD**: `HTTP_Passwd secretpassword` written directly in the config file.
- **GOOD**: Reference environment variables — `HTTP_Passwd ${LOKI_PASSWORD}` — and inject the value at runtime via Kubernetes secrets or a secrets manager.

## References

| Resource | Purpose |
|---|---|
| `scripts/generate_config.py` | Template-based config generation (13 use cases) |
| `examples/*.conf` | Production-ready reference configurations |
| `examples/parsers.conf` | Reusable parser library |
| [docs.fluentbit.io](https://docs.fluentbit.io/manual) | Official plugin reference |
| context7 `/fluent/fluent-bit-docs` | MCP-accessible documentation |
