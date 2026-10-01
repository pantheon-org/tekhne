---
name: fluentbit-generator
description: Generates, validates, and optimizes Fluent Bit configurations for production use. Use when creating new Fluent Bit configs, implementing log collection pipelines (INPUT, FILTER, OUTPUT sections), configuring Kubernetes log collection with metadata enrichment, forwarding logs to destinations (Elasticsearch, Loki, S3, Kafka, CloudWatch, OpenTelemetry), building multi-line log parsing, or converting existing logging configurations to Fluent Bit.
---

# Fluent Bit Config Generator

## When to Use

Use this skill when you need to:

- Create a new Fluent Bit configuration with INPUT, FILTER, and OUTPUT sections.
- Build a Kubernetes log-collection pipeline with pod and namespace metadata enrichment.
- Forward logs to a destination (Elasticsearch, Loki, S3, Kafka, CloudWatch, OpenTelemetry).
- Add multi-line parsing or convert an existing logging config to Fluent Bit.

## When Not to Use

- Validating or debugging an existing config, which is the job of `fluentbit-validator`.
- Log processing that belongs in Fluentd or Logstash rather than Fluent Bit.
- Application-level structured logging, that is a code concern, not a Fluent Bit config concern.

## Philosophy

- **Bound every buffer.** Each INPUT sets `Mem_Buf_Limit` so a burst cannot exhaust memory.
- **Route explicitly.** Every OUTPUT matches a specific tag; `Match *` on all outputs duplicates and misroutes logs.
- **Secrets never live in the file.** Credentials and tokens come from environment variables or mounted secrets, not `fluent-bit.conf`.
- **Generate, then validate.** Hand the output to `fluentbit-validator` before calling it done.
- **Prefer the script, then the examples.** Reuse `scripts/generate_config.py` and the reference configs before writing a config by hand.

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
1. Read the closest example from `assets/`, production-ready reference configs are available for all 13 use cases (e.g. `kubernetes-elasticsearch.conf`, `kubernetes-loki.conf`, `application-multiline.conf`, `multi-destination.conf`, `full-production.conf`, and others).
2. Read `assets/parsers.conf`, reuse existing parsers (docker, cri, json, nginx, apache, syslog-rfc3164/5424, multiline-java/python/go/ruby) before creating custom ones.

**Manual configuration structure:** a `fluent-bit.conf` plus an optional `parsers.conf`, in the order SERVICE, INPUT, FILTER, OUTPUT. A minimal tail input looks like this; see `references/manual-config-structure.md` for the full skeleton.

```ini
[INPUT]
    Name              tail
    Tag               kube.*
    Path              /var/log/containers/*.log
    Parser            docker
    DB                /var/log/flb_kube.db
    Mem_Buf_Limit     50MB
    Skip_Long_Lines   On
```

**Filter example** (parse JSON from the `log` key):

```ini
[FILTER]
    Name          parser
    Match         *
    Key_Name      log
    Parser        json
    Reserve_Data  On
```

**Output example** (Elasticsearch, routed by tag, TLS verified):

```ini
[OUTPUT]
    Name              es
    Match             kube.*
    Host              elasticsearch.logging.svc
    Port              9200
    Retry_Limit       3
    tls               On
    tls.verify        On
```

**Filters and outputs:** order matters, parsers before modifiers. See `references/filter-patterns.md` for parser, grep, multiline, Lua and throttle filters, and `references/output-patterns.md` for Loki, S3, Kafka, CloudWatch, OpenTelemetry, HTTP and stdout outputs.

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

**Invoke the `fluentbit-validator` skill** on the generated config to run:
- Required field checks and plugin parameter validation
- Tag consistency and parser reference validation
- Security checks (plaintext credentials, TLS)
- Best practice recommendations
- Dry-run test if `fluent-bit` binary is available

Fix any reported issues and re-validate until all checks pass.

```bash
python3 <validator-skill-dir>/scripts/validate_config.py --file fluent-bit.conf --check all
fluent-bit -c fluent-bit.conf --dry-run
```

---

### Step 4: Communicate Results

When delivering a configuration:
1. **Explain section choices**, why each plugin/setting was selected
2. **Flag required customizations**, parameters the user must adjust (cluster names, hosts, bucket names)
3. **Credential reminders**, always use `${ENV_VAR}` syntax, never hardcode secrets
4. **TLS guidance**, use `tls.verify On` in production; if `Off` is needed add an inline comment explaining why (e.g., `# Internal cluster with self-signed certs`)
5. **Validation status**, summarise validator output and any fixes applied

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

**WHY:** Broadcasting all logs to every output creates duplicate records in each destination, generates unexpected ingestion costs, and leaks logs intended for one system (e.g., a debug sink) into another (e.g., a billed SaaS platform).

**BAD:** Three separate output plugins all configured with `Match *`.

**GOOD:** Use distinct tag namespaces (`kube.*`, `app.*`, `system.*`) and route each namespace to its intended destination with a specific `Match` pattern.

### NEVER omit `Mem_Buf_Limit` on INPUT plugins

**WHY:** Without a memory buffer limit, backpressure from a slow or unavailable output causes the input buffer to grow without bound, leading to OOM kills of the Fluent Bit process and log loss.

**BAD:** `[INPUT] Name tail Tag app.*` with no `Mem_Buf_Limit` setting.

**GOOD:** Add `Mem_Buf_Limit 50MB` to every tail input (adjust the value based on measured log volume).

### NEVER use `Retry_Limit False` in outputs without monitoring

**WHY:** Infinite retries mask persistent delivery failures. Retry buffers accumulate on disk, eventually exhausting storage and causing Fluent Bit to drop new logs to protect itself.

**BAD:** `Retry_Limit False` in an output plugin with no alerting on delivery failure metrics.

**GOOD:** Set `Retry_Limit 5` and monitor for delivery failures using Fluent Bit's built-in Prometheus metrics (`/api/v1/metrics`).

### NEVER parse structured logs with a regexp parser when `json` or `logfmt` parsers apply

**WHY:** Regexp parsers are fragile, they break when log format details change, and CPU-intensive compared to native parsers. Using them for standard formats sacrifices correctness and performance for no benefit.

**BAD:** `Parser regex_json` configured to extract fields from JSON-formatted log lines.

**GOOD:** `Parser json`, simpler, faster, and guaranteed to handle all valid JSON log output correctly.

### NEVER store sensitive values directly in `fluent-bit.conf`

**WHY:** Configuration files are routinely committed to source control, copied into container images, and displayed in support tickets. Inline credentials are then exposed to anyone who can read the file.

**BAD:** `HTTP_Passwd secretpassword` written directly in the config file.

**GOOD:** Reference environment variables, `HTTP_Passwd ${LOKI_PASSWORD}`, and inject the value at runtime via Kubernetes secrets or a secrets manager.

### NEVER skip `Exclude_Path` for Fluent Bit's own logs when tailing container logs

**WHY:** Fluent Bit reading its own container log creates a feedback loop in which each processed record produces more log lines to process.

**BAD:** `Path /var/log/containers/*.log` with no `Exclude_Path`.

**GOOD:** `Exclude_Path /var/log/containers/*fluent-bit*.log` alongside the tail `Path`.

### NEVER omit the `DB` position file on tail inputs

**WHY:** Without a `DB` file, Fluent Bit loses its read position on restart and re-reads or skips log lines.

**BAD:** A tail input with no `DB` setting.

**GOOD:** `DB /var/log/flb_kube.db` on tail inputs for crash recovery.

### NEVER disable TLS verification in production outputs

**WHY:** `tls.verify Off` exposes log traffic to interception and is easily carried over from testing.

**BAD:** `tls.verify Off` on an HTTP or OpenTelemetry output sending to an external host.

**GOOD:** `tls On` and `tls.verify On`; if `Off` is genuinely needed, add an inline comment explaining why.

### NEVER hand over a generated configuration without validating it

**WHY:** Generated or hand-written configs can contain tag mismatches, missing parsers or insecure settings that only a validation pass reveals.

**BAD:** Deliver the file straight after `scripts/generate_config.py` writes it.

**GOOD:** Run the `fluentbit-validator` skill on the output, fix reported issues and re-validate until all checks pass.

## References

| Topic | Reference | When to Use |
|---|---|---|
| Full config skeleton | `references/manual-config-structure.md` | Writing a config by hand because the script does not cover the use case |
| FILTER patterns | `references/filter-patterns.md` | Adding parser, grep, multiline, Lua or throttle filters |
| OUTPUT patterns | `references/output-patterns.md` | Configuring Loki, S3, Kafka, CloudWatch, OpenTelemetry, HTTP or stdout outputs |
| Generation script | `scripts/generate_config.py` | Template-based config generation (13 use cases) |
| Example configs | `assets/*.conf` | Production-ready reference configurations |
| Parser library | `assets/parsers.conf` | Reusing existing parsers before writing custom ones |
| Official docs | [docs.fluentbit.io](https://docs.fluentbit.io/manual) | Plugin reference |
| context7 docs | `/fluent/fluent-bit-docs` | MCP-accessible documentation |
