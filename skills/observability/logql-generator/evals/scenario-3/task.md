# API Gateway Latency and Top Endpoints Dashboard

## Problem Description

A cloud API gateway team wants to populate a Grafana dashboard with four panels to monitor their `gateway-service`. The service runs in a Kubernetes cluster and emits logfmt-formatted logs. Each log line includes fields such as `level`, `method`, `path`, `status`, `duration_ms` (integer, milliseconds), and `upstream`. The service logs flow into Loki under labels `app="gateway-service"`, `namespace="platform"`, and `env="prod"`.

The four panels they need:

1. **p95 request latency over time** — the 95th-percentile request duration in milliseconds, across all requests, sampled over 5-minute windows.

2. **Top 10 slowest endpoints** — a snapshot of which `path` values have the highest average response time right now (over the last 15 minutes), showing the top 10.

3. **Error rate by upstream** — the per-second rate of requests where `status >= 500`, broken down by the `upstream` label, over 5-minute windows.

4. **Anomaly detection: today vs yesterday** — the difference between the current error rate and the error rate exactly 24 hours ago, so on-call engineers can quickly see whether error volumes are trending up.

For all panels, the team cares about correctness and query performance given the high volume of traffic on this service.

## Output Specification

Produce a file named `dashboard_queries.md` with all four LogQL queries, each clearly labelled (Panel 1 through Panel 4). Include a short explanation beneath each query describing how the result should be interpreted in a Grafana panel.
