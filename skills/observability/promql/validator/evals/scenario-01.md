# Scenario 01: Detect `rate()` Misuse on a Gauge Metric

## User Prompt

A developer submitted the following PromQL query for a Grafana dashboard panel intended to show the current memory usage of a service in megabytes. The metric `process_resident_memory_bytes` is a **gauge** — it tracks the current value and can go up or down.

**Query Under Review:**

```promql
rate(process_resident_memory_bytes{job="api-service"}[5m])
```

> "This query is for a dashboard panel showing memory usage. I want to see how much memory the API service is using right now."

Produce a validation report `validation-report.md` that:

1. States whether the query is syntactically valid.
2. Identifies the metric type misuse: `rate()` is being applied to a gauge, which is semantically incorrect (rate is only meaningful for monotonically increasing counters).
3. Explains in plain English what the current query actually calculates versus what the user intends.
4. Provides the correct query to show current memory usage of the service.
5. Notes any high-cardinality concerns (or confirms none for this query).

Produce a single file `validation-report.md` structured with sections: Syntax Check, Metric Type Analysis, Intent vs Implementation, Recommended Query, Cardinality Notes.

## Expected Behavior

1. Explicitly state that `rate()` is only valid for counters (monotonically increasing metrics) and that `process_resident_memory_bytes` is a gauge
2. Explain what `rate()` on a gauge actually computes (per-second rate of change) versus what the user wants (current absolute value)
3. Provide a recommended query that removes `rate()` and uses a direct gauge query or `avg_over_time()` to show current memory usage
4. Include a clear statement that the query is syntactically valid before explaining the semantic issue
5. Structure the report with all five required sections: Syntax Check, Metric Type Analysis, Intent vs Implementation, Recommended Query, and Cardinality Notes

## Success Criteria

- **Metric type misuse is explicitly identified**: The report explicitly states that `rate()` is only valid for counters (monotonically increasing metrics) and that `process_resident_memory_bytes` is a gauge
- **Intent vs implementation mismatch is explained**: The report explains what `rate()` on a gauge actually computes (per-second rate of change) versus what the user wants (current absolute value)
- **Correct query is provided**: The recommended query removes `rate()` and uses a direct gauge query or `avg_over_time()` as appropriate for showing current memory usage
- **Syntax validity is stated**: The report includes a clear statement that the query is syntactically valid (it parses correctly) before explaining the semantic issue
- **Report uses the required section structure**: The output file contains all five required sections: Syntax Check, Metric Type Analysis, Intent vs Implementation, Recommended Query, and Cardinality Notes

## Failure Conditions

- Fails to identify that `rate()` is being applied to a gauge metric
- Does not explain the difference between what `rate()` on a gauge computes versus what the user intends
- Provides no corrected query, or provides a corrected query that still applies `rate()` to the gauge
- Omits the syntax validity statement (or claims the query is syntactically invalid when it is not)
- Missing one or more of the five required sections in the report
