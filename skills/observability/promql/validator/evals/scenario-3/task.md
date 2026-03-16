# Scenario 3: Identify High-Cardinality Risk

## Context

A developer is building a Grafana dashboard for an e-commerce platform. The platform
runs 40 microservices, each exposing HTTP metrics. The developer submitted the following
query to show total request rates.

## Query Under Review

```promql
sum(rate(http_requests_total[5m]))
```

## User Context

> "I want a single panel showing the overall request rate across all our services.
> The platform has about 40 microservices and the metric has labels: job, instance,
> method, path, status_code."

## Task

Produce a validation report `cardinality-report.md` that:

1. Identifies that the query has no label filters and explains the high-cardinality risk
   this creates (fan-out to potentially thousands of series given the `path` and
   `status_code` labels across 40 services).
2. Explains when this pattern IS acceptable (e.g., the `sum()` aggregation means the
   result is a single scalar, so query load is high but dashboard performance is
   acceptable IF the cluster can handle the fan-out).
3. Provides two alternative queries:
   a. A filtered version that restricts to a specific service for per-service panels.
   b. A recording rule definition that pre-aggregates the total rate to reduce query cost.
4. Notes whether the query itself is syntactically and semantically correct for the
   user's stated goal.

## Output Specification

Produce a single file `cardinality-report.md` with sections: Cardinality Analysis,
When This Pattern Is Acceptable, Alternative Queries, Recording Rule Recommendation.
