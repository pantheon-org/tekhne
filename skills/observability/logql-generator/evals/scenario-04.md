# Scenario 04: API Gateway Latency and Top Endpoints Dashboard

## User Prompt

A cloud API gateway team wants to populate a Grafana dashboard with four panels to monitor their `gateway-service`. The service runs in a Kubernetes cluster and emits logfmt-formatted logs. Each log line includes fields such as `level`, `method`, `path`, `status`, `duration_ms` (integer, milliseconds), and `upstream`. The service logs flow into Loki under labels `app="gateway-service"`, `namespace="platform"`, and `env="prod"`.

The four panels they need:

1. **p95 request latency over time** — the 95th-percentile request duration in milliseconds, across all requests, sampled over 5-minute windows.

2. **Top 10 slowest endpoints** — a snapshot of which `path` values have the highest average response time right now (over the last 15 minutes), showing the top 10.

3. **Error rate by upstream** — the per-second rate of requests where `status >= 500`, broken down by the `upstream` label, over 5-minute windows.

4. **Anomaly detection: today vs yesterday** — the difference between the current error rate and the error rate exactly 24 hours ago, so on-call engineers can quickly see whether error volumes are trending up.

For all panels, the team cares about correctness and query performance given the high volume of traffic on this service.

Produce a file named `dashboard_queries.md` with all four LogQL queries, each clearly labelled (Panel 1 through Panel 4). Include a short explanation beneath each query describing how the result should be interpreted in a Grafana panel.

## Expected Behavior

1. Use the `logfmt` parser (`| logfmt`) for all queries that extract fields — not regexp or json
2. Use `quantile_over_time(0.95, ... | unwrap duration_ms ...)` for the p95 latency panel
3. Use `| unwrap duration_ms` to extract the numeric field before `quantile_over_time`
4. Use `topk(10, ...)` to select the top 10 slowest endpoints for Panel 2
5. Group Panel 3's error rate using `sum by (upstream)` or equivalent
6. Use `rate()` (not `count_over_time()`) for Panels 3 and 4
7. Apply the `offset` modifier (e.g. `[5m] offset 1d`) to reference the same metric 24 hours ago in Panel 4
8. Subtract the offset expression from the current expression using the minus operator for Panel 4

## Success Criteria

- **logfmt parser used**: All queries that extract fields use the logfmt parser (`| logfmt`), not regexp or json
- **quantile_over_time for p95**: Panel 1 uses `quantile_over_time(0.95, ... | unwrap duration_ms ...)` to compute the 95th percentile latency
- **unwrap used for numeric field**: The latency query uses `| unwrap duration_ms` to extract the numeric field before `quantile_over_time`
- **topk for top endpoints**: Panel 2 uses `topk(10, ...)` to select the top 10 slowest endpoints
- **sum by upstream for error rate**: Panel 3 groups the error rate using `sum by (upstream)` or equivalent grouping on upstream
- **rate() for error rate panels**: Panels 3 and 4 use `rate()` not `count_over_time()` for computing error rates
- **offset modifier for comparison**: Panel 4 uses the offset modifier (e.g. `[5m] offset 1d`) to reference the same metric 24 hours ago
- **Subtraction for delta**: Panel 4 subtracts the offset expression from the current expression using the minus operator

## Failure Conditions

- Uses regexp or json parser instead of logfmt for logfmt-formatted logs
- Uses `avg_over_time` or a non-quantile function instead of `quantile_over_time` for p95
- Omits `| unwrap duration_ms` before the quantile calculation
- Uses `sort_desc` or manual filtering instead of `topk(10, ...)` for Panel 2
- Omits `upstream` grouping from the error rate panel
- Uses `count_over_time()` instead of `rate()` for Panels 3 or 4
- Omits the `offset` modifier from Panel 4's historical comparison
- Fails to subtract the offset expression in Panel 4
