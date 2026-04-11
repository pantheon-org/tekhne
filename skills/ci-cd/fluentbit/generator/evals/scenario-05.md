# Scenario 05: High-Reliability Log Forwarding to Kafka

## User Prompt

A data engineering team at a logistics company forwards container logs to Kafka for downstream stream processing. The team has experienced silent data loss during Kafka maintenance windows: when Kafka brokers are temporarily unavailable, Fluent Bit keeps buffering logs until it runs out of disk space, then drops all buffered logs without alerting anyone. Operations has now mandated that the configuration must be hardened for reliability: retries must be bounded, disk buffers must be capped, and the Fluent Bit pod must be health-checkable by the Kubernetes liveness probe.

The logs come from `/var/log/containers/*.log` on a Kubernetes cluster. The Kafka brokers are `kafka-1.data.svc:9092` and `kafka-2.data.svc:9092`. The target topic is `platform-logs`.

## Output Specification

Produce a `fluent-bit.conf` ready for Kubernetes DaemonSet deployment. Include a `reliability-notes.md` explaining how the configuration addresses the bounded retry and buffer exhaustion concerns, and how the health check endpoint can be used with a Kubernetes liveness probe.

## Expected Behavior

1. Set a numeric `Retry_Limit` on the Kafka OUTPUT (not `False`) to bound how long logs are retried before being dropped
2. Set `storage.total_limit_size` on the OUTPUT or SERVICE section to cap disk buffer growth and prevent disk exhaustion
3. Enable `HTTP_Server On` in the SERVICE section so the Kubernetes liveness probe can reach the health endpoint
4. Document in `reliability-notes.md` how to configure a Kubernetes liveness probe using the `/api/v1/health` endpoint or port 2020
5. Explain in `reliability-notes.md` how Retry_Limit and storage.total_limit_size address the data loss and disk exhaustion concerns
6. Add `Mem_Buf_Limit` and `Exclude_Path` on the tail INPUT, and a `DB` path for position tracking

## Success Criteria

- **Retry_Limit numeric**: The Kafka OUTPUT includes a Retry_Limit that is a number (e.g., 3, 5) — NOT Retry_Limit False
- **storage.total_limit_size set**: The OUTPUT or SERVICE section includes storage.total_limit_size to cap disk buffer size
- **HTTP_Server On**: The SERVICE section includes HTTP_Server On
- **HTTP health endpoint documented**: reliability-notes.md (or equivalent) references the /api/v1/health endpoint or port 2020 for Kubernetes liveness probe configuration
- **Reliability measures explained**: reliability-notes.md explains how Retry_Limit bounds retries and how storage.total_limit_size prevents disk exhaustion
- **Mem_Buf_Limit on input**: The tail INPUT plugin includes Mem_Buf_Limit
- **Exclude_Path on input**: The tail INPUT plugin excludes fluent-bit's own container logs via Exclude_Path
- **DB path on input**: The tail INPUT plugin includes a DB setting for position tracking
- **storage.metrics on**: The SERVICE section includes storage.metrics on to enable buffer monitoring

## Failure Conditions

- `Retry_Limit` is set to `False` or omitted entirely, allowing unbounded retries and silent data loss
- `storage.total_limit_size` is absent, leaving disk buffer growth uncapped
- `HTTP_Server` is not enabled, preventing Kubernetes liveness probe health checks
- `reliability-notes.md` does not reference the health endpoint or port 2020 for liveness probe configuration
- `reliability-notes.md` does not explain how Retry_Limit or storage.total_limit_size address the reliability concerns
- `Mem_Buf_Limit` is absent from the tail INPUT
- `Exclude_Path` is absent, leaving fluent-bit's own logs collected into the pipeline
