# Scenario 1: Detect `rate()` Misuse on a Gauge Metric

## Context

A developer submitted the following PromQL query for a Grafana dashboard panel intended
to show the current memory usage of a service in megabytes. The metric
`process_resident_memory_bytes` is a **gauge** — it tracks the current value and can
go up or down.

## Query Under Review

```promql
rate(process_resident_memory_bytes{job="api-service"}[5m])
```

## User Context

> "This query is for a dashboard panel showing memory usage. I want to see how much
> memory the API service is using right now."

## Task

Produce a validation report `validation-report.md` that:

1. States whether the query is syntactically valid.
2. Identifies the metric type misuse: `rate()` is being applied to a gauge, which is
   semantically incorrect (rate is only meaningful for monotonically increasing counters).
3. Explains in plain English what the current query actually calculates versus what the
   user intends.
4. Provides the correct query to show current memory usage of the service.
5. Notes any high-cardinality concerns (or confirms none for this query).

## Output Specification

Produce a single file `validation-report.md` structured with sections: Syntax Check,
Metric Type Analysis, Intent vs Implementation, Recommended Query, Cardinality Notes.
