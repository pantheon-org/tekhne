# Create Recording Rules to Speed Up Slow Dashboard Queries

## Problem/Feature Description

A DevOps team supports a multi-tenant SaaS platform. Their Grafana dashboards are timing out because several expensive PromQL queries run on every panel refresh. The head of infrastructure has determined that three queries are the main culprits — they are all computed in multiple dashboard panels and run over large time ranges. The team wants to pre-compute these queries as Prometheus recording rules so they can be reused across dashboards without re-evaluating the full expression each time.

Additionally, a teammate proposed the following query for a "per-user request breakdown" panel:

```promql
sum by (user_id) (rate(http_requests_total[5m]))
```

The senior engineer flagged this as potentially dangerous to the Prometheus instance but hasn't explained why. The team wants the recording rules written up, and also wants an explanation of the issue with the proposed query and what a safer alternative looks like.

The three queries to pre-compute:
1. The per-job request rate over 5 minutes: `sum by (job) (rate(http_requests_total[5m]))`
2. The per-job 5xx error rate over 5 minutes: `sum by (job) (rate(http_requests_total{status_code=~"5.."}[5m]))`
3. The per-job P95 latency: `histogram_quantile(0.95, sum by (job, le) (rate(http_request_duration_seconds_bucket[5m])))`

## Output Specification

Produce two files:

1. `recording_rules.yaml` — Prometheus recording rule YAML for the three queries above, using correct recording rule naming conventions.
2. `cardinality_analysis.md` — A short Markdown document (1-2 paragraphs) explaining the problem with the `sum by (user_id)` query and providing a safer alternative query.
