# Scenario 03: Identify High-Cardinality Risk

## User Prompt

A developer is building a Grafana dashboard for an e-commerce platform. The platform runs 40 microservices, each exposing HTTP metrics. The developer submitted the following query to show total request rates.

**Query Under Review:**

```promql
sum(rate(http_requests_total[5m]))
```

> "I want a single panel showing the overall request rate across all our services. The platform has about 40 microservices and the metric has labels: job, instance, method, path, status_code."

Produce a validation report `cardinality-report.md` that:

1. Identifies that the query has no label filters and explains the high-cardinality risk this creates (fan-out to potentially thousands of series given the `path` and `status_code` labels across 40 services).
2. Explains when this pattern IS acceptable (e.g., the `sum()` aggregation means the result is a single scalar, so query load is high but dashboard performance is acceptable IF the cluster can handle the fan-out).
3. Provides two alternative queries:
   a. A filtered version that restricts to a specific service for per-service panels.
   b. A recording rule definition that pre-aggregates the total rate to reduce query cost.
4. Notes whether the query itself is syntactically and semantically correct for the user's stated goal.

Produce a single file `cardinality-report.md` with sections: Cardinality Analysis, When This Pattern Is Acceptable, Alternative Queries, Recording Rule Recommendation.

## Expected Behavior

1. State that the query has no label filters and explain that this causes fan-out across all series for the metric (referencing `path`, `status_code`, and the multi-service context)
2. Explain that `sum()` aggregation produces a single scalar result, so the query is acceptable IF the cluster can handle the fan-out — do not simply label the query as "bad"
3. Include a query that adds a `job=` or `service=` label filter to restrict the query to a single service
4. Include a YAML recording rule definition that pre-aggregates the total request rate
5. Confirm that the query is syntactically valid and semantically correct for computing total request rate (counter metric with `rate()` and `sum()`)

## Success Criteria

- **High-cardinality risk is identified and explained**: The report states that the query has no label filters and explains that this causes fan-out across all series for the metric (referencing the `path`, `status_code`, and multi-service context)
- **Nuanced acceptability analysis provided**: The report explains that `sum()` aggregation produces a single scalar result, so the query is acceptable IF the cluster can handle the fan-out — it does not simply label the query as "bad"
- **Filtered per-service alternative is provided**: The report includes a query that adds a `job=` or `service=` label filter to restrict the query to a single service
- **Recording rule definition is provided**: The report includes a YAML recording rule definition that pre-aggregates the total request rate, reducing per-query scan cost
- **Semantic and syntactic correctness confirmed**: The report confirms that the query is syntactically valid and semantically correct for computing total request rate (counter metric with `rate()` and `sum()`)

## Failure Conditions

- Fails to identify the high-cardinality risk or the lack of label filters
- Only labels the query as "bad" without acknowledging when the pattern is acceptable
- Does not provide a per-service filtered alternative
- Does not provide a recording rule definition
- Claims the query is semantically incorrect for computing total request rate
