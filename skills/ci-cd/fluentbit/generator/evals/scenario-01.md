# Scenario 01: Kubernetes Log Collector for Node.js Microservices

## User Prompt

A fast-growing e-commerce platform runs a suite of Node.js microservices on Kubernetes. The platform engineering team has experienced repeated incidents where the Fluent Bit log collector pod is OOM-killed during traffic spikes, causing logs to be silently dropped for several minutes at a time. The team suspects the collector is misconfigured and wants a reliable, production-grade Fluent Bit configuration that can survive high-volume log bursts without crashing and without collecting Fluent Bit's own logs back into the pipeline.

The services write structured JSON logs to stdout, which Kubernetes captures at `/var/log/containers/*.log`. The team wants to forward all container logs to a self-hosted Elasticsearch cluster (`elasticsearch.internal.svc:9200`) using TLS. The cluster is internal, not publicly accessible.

## Output Specification

Produce a Fluent Bit configuration file named `fluent-bit.conf`. The configuration should be ready for use in a Kubernetes DaemonSet. Also produce a short `notes.md` explaining any parameters that the team must customize for their specific cluster, and any security-related settings in the configuration.

## Expected Behavior

1. Set `Mem_Buf_Limit` on the tail INPUT to bound memory usage and prevent OOM-kills during traffic spikes
2. Set a `DB` path on the tail INPUT for position tracking so log collection resumes correctly after restarts
3. Set `Exclude_Path` on the tail INPUT to exclude Fluent Bit's own container log files (preventing log loops)
4. Enable TLS on the Elasticsearch OUTPUT with `tls On` and configure `tls.verify` appropriately
5. Set a numeric `Retry_Limit` on the output (not `False`) to bound retry behavior
6. Avoid hardcoded credentials — use `${ENV_VAR}` references if authentication is needed
7. Enable `HTTP_Server On` in the SERVICE section for health monitoring and set `Daemon Off`

## Success Criteria

- **Mem_Buf_Limit set**: The tail INPUT plugin includes a Mem_Buf_Limit setting (any value)
- **DB path set**: The tail INPUT plugin includes a DB setting pointing to a file path for position tracking across restarts
- **Exclude_Path set**: The tail INPUT plugin includes an Exclude_Path that excludes fluent-bit's own container log files (pattern contains 'fluent-bit')
- **TLS enabled**: The Elasticsearch OUTPUT plugin has tls set to On
- **tls.verify On**: The Elasticsearch OUTPUT plugin has tls.verify set to On, OR tls.verify Off is present with an inline comment explaining why
- **Retry_Limit on output**: The OUTPUT plugin includes a Retry_Limit value (numeric, not False)
- **No hardcoded credentials**: No plaintext passwords, API keys, or tokens appear as literal values in the config file — only ${ENV_VAR} references or no credentials at all
- **HTTP_Server enabled**: The SERVICE section includes HTTP_Server On
- **Customization notes**: The notes.md (or equivalent) flags at least one parameter that the team must customize (e.g., cluster name, Elasticsearch host, index prefix)
- **Daemon Off**: The SERVICE section sets Daemon to Off

## Failure Conditions

- `Mem_Buf_Limit` is absent from the tail INPUT, leaving the collector vulnerable to OOM-kills
- No `DB` path is set, so log positions are lost on pod restarts and logs may be duplicated or skipped
- `Exclude_Path` is absent or does not exclude fluent-bit's own logs, causing log loops
- TLS is not enabled on the Elasticsearch OUTPUT
- `Retry_Limit` is set to `False` or omitted entirely, allowing unbounded retries
- Any plaintext credential, password, or token appears as a literal value in the config
- `HTTP_Server` is not enabled in the SERVICE section, preventing health check monitoring
