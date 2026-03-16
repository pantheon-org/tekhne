---
name: promql-generator
description: Generate PromQL queries for calculating error rates, aggregating metrics across labels, creating histogram percentiles, writing recording rules, and building SLO burn-rate alerts following Prometheus best practices. Use when creating new PromQL queries, implementing monitoring and alerting rules, building observability dashboards, working with Prometheus metrics (counters, gauges, histograms, summaries), or applying RED (Rate, Errors, Duration) and USE (Utilization, Saturation, Errors) monitoring patterns.
---

# PromQL Query Generator

## Interactive Query Planning Workflow

**CRITICAL**: Always engage the user in collaborative planning **before** generating any query. Never skip the planning phase.

### Workflow (7 stages)

1. **Understand the goal** — Ask what the user wants to monitor (request rate, error rate, latency, resource usage, availability, SLO tracking) and the use case (dashboard, alert, recording rule, ad-hoc).
2. **Identify metrics** — Confirm metric names, types (counter/gauge/histogram/summary), and relevant labels. Suggest common naming patterns if uncertain.
3. **Determine parameters** — Confirm time range, label filters, aggregation, and thresholds. If the user already specified values (e.g., "5-minute window", "> 5% error rate"), acknowledge them as pre-filled defaults and allow quick confirmation rather than re-asking.
4. **Present the query plan** — Before writing any code, present a plain-English plan (goal, query structure, expected output, example interpretation) and ask for confirmation via **AskUserQuestion** with options: "Yes, generate this query" / "Modify [aspect]" / "Show alternatives".
5. **Generate the query** — Once confirmed, read the relevant reference file(s) before writing code, cite the applicable pattern, and apply the best practices below.
6. **Validate** — Automatically invoke `devops-skills:promql-validator`. Display structured results (syntax, best practices, explanation). Fix any issues and re-validate until all checks pass.
7. **Deliver** — Provide the final query, plain-English explanation, usage instructions (dashboard / alert / recording rule), customization notes, and related query suggestions.

> **Ask vs. Infer**: If the user's request already clearly specifies goal, use case, and context, acknowledge those details instead of re-asking. Only ask for missing or ambiguous information.

---

## Best Practices for Query Generation

**Always consult the relevant reference file before writing code.**

| Scenario | Reference File |
|---|---|
| Histogram queries | `references/metric_types.md` (Histogram section) |
| Error/latency patterns | `references/promql_patterns.md` (RED section) |
| Resource monitoring | `references/promql_patterns.md` (USE section) |
| Optimization / anti-patterns | `references/best_practices.md` |
| Specific functions | `references/promql_functions.md` |

### Key Rules

1. **Always add label filters** — reduces cardinality and improves performance.
2. **Match functions to metric types** — `rate()`/`increase()` on counters; `*_over_time()` or direct use for gauges; `histogram_quantile()` for histograms.
3. **Prefer `by()`/`without()`** on all aggregations.
4. **Prefer exact label matches** over regex when the value is known.
5. **Use recording rules** for queries that are expensive or reused frequently (naming: `level:metric:operations`).
6. **Format multi-line** for complex queries.

### Core Patterns

```promql
# Request rate (counter)
sum(rate(http_requests_total{job="api-server"}[5m])) by (endpoint)

# Error rate ratio
sum(rate(http_requests_total{job="api-server", status_code=~"5.."}[5m]))
/ sum(rate(http_requests_total{job="api-server"}[5m]))

# P95 latency (classic histogram)
histogram_quantile(0.95,
  sum by (le) (rate(http_request_duration_seconds_bucket{job="api-server"}[5m]))
)

# P95 latency (native histogram, Prometheus 3.x+)
histogram_quantile(0.95,
  sum by (job) (rate(http_request_duration_seconds[5m]))
)

# Availability
(count(up{job="api-server"} == 1) / count(up{job="api-server"})) * 100

# Burn rate (99.9% SLO, 1h window)
(
  sum(rate(http_requests_total{job="api", status_code=~"5.."}[1h]))
  / sum(rate(http_requests_total{job="api"}[1h]))
) / 0.001

# Multi-window burn-rate alert (page: 2% budget in 1h, burn rate 14.4)
(
  sum(rate(http_requests_total{job="api", status_code=~"5.."}[1h]))
  / sum(rate(http_requests_total{job="api"}[1h]))
) > 14.4 * 0.001
and
(
  sum(rate(http_requests_total{job="api", status_code=~"5.."}[5m]))
  / sum(rate(http_requests_total{job="api"}[5m]))
) > 14.4 * 0.001
```

For complete SLO patterns, Native Histogram functions (`histogram_count`, `histogram_sum`, `histogram_fraction`), subqueries, offset/@ modifiers, vector matching, and Kubernetes patterns — see the `assets/` files.

---

## Validation Checklist

After generating, invoke `devops-skills:promql-validator` and display results in this format:

```
## PromQL Validation Results

### Syntax Check
- Status: ✅ VALID / ⚠️ WARNING / ❌ ERROR
- Issues: [list any syntax errors]

### Best Practices Check
- Status: ✅ OPTIMIZED / ⚠️ CAN BE IMPROVED / ❌ HAS ISSUES
- Issues: [list problems found]
- Suggestions: [list optimizations]

### Query Explanation
- What it measures: [plain English]
- Output labels: [label list or "None (scalar)"]
- Expected result structure: [instant vector / scalar / etc.]
```

Fix all issues and re-validate until clean.

---

## Alerting and Recording Rule Snippets

```yaml
# Alerting rule with for clause
alert: HighErrorRate
expr: |
  (
    sum(rate(http_requests_total{status_code=~"5.."}[5m]))
    / sum(rate(http_requests_total[5m]))
  ) > 0.05
for: 10m

# Recording rule (naming: level:metric:operations)
- record: job:http_requests:rate5m
  expr: sum by (job) (rate(http_requests_total[5m]))
```

---

## Anti-Patterns

### NEVER use `rate()` on a gauge metric

- **WHY**: `rate()` computes per-second rate of increase and assumes monotonically increasing counters. Applied to a gauge, it produces nonsensical results because gauges can decrease.
- **BAD**: `rate(node_memory_MemFree_bytes[5m])` — memory is a gauge
- **GOOD**: `node_memory_MemFree_bytes` (direct use) or `delta(node_memory_MemFree_bytes[5m])` for change over time

### NEVER query a histogram with `avg()` across quantile labels

- **WHY**: Summary metrics expose pre-aggregated `{quantile="0.95"}` labels that cannot be re-aggregated across instances. Using `avg()` on them produces statistically meaningless results.
- **BAD**: `avg(http_request_duration_seconds{quantile="0.95"})`
- **GOOD**: `histogram_quantile(0.95, sum by (le) (rate(http_request_duration_seconds_bucket[5m])))` — use histogram type with `histogram_quantile()`

### NEVER use a high-cardinality label in `by()` without filtering first

- **WHY**: Aggregating by `user_id`, `request_id`, or `pod` without label filters produces thousands of series, overwhelming dashboards and recording rules.
- **BAD**: `sum by (user_id) (rate(http_requests_total[5m]))`
- **GOOD**: `sum by (job, status_code) (rate(http_requests_total{job="api"}[5m]))` — filter and aggregate on stable, low-cardinality labels

### NEVER use `increase()` for alerting thresholds

- **WHY**: `increase()` is extrapolated and can return non-integer values on sparse counters. For alerting, `rate()` produces stable per-second thresholds that are scrape-interval independent.
- **BAD**: `increase(http_requests_total{status=~"5.."}[5m]) > 10`
- **GOOD**: `rate(http_requests_total{status=~"5.."}[5m]) > 0.033` (2 errors/minute = 0.033/second)

### NEVER omit the `for` clause on alert rules

- **WHY**: Alerts without `for` fire immediately on a single evaluation, causing false positives from transient spikes. The `for` clause requires the condition to be true for a sustained period.
- **BAD**: `alert: HighErrorRate` with `expr: error_rate > 0.05` and no `for`
- **GOOD**: Add `for: 5m` to require the condition to hold for 5 minutes before firing

## Documentation Lookup

1. **context7 MCP (preferred)**: resolve `prometheus`, then fetch docs with the relevant topic.
2. **Fallback WebSearch**: `"Prometheus PromQL [function/operator] documentation [version] examples"`

---

## Error Handling Quick Reference

| Symptom | Likely Cause | Fix |
|---|---|---|
| Empty results | Wrong label filters or metric not scraped | Check `up{job="..."}`, verify label values |
| Too many series | High cardinality | Add label filters, aggregate, use recording rules |
| Wrong values | Wrong function for metric type | `rate()` on counters; direct or `*_over_time()` on gauges |
| Slow queries | Large range vectors or missing filters | Narrow time range, add filters, use recording rules |

---

## References

**Internal:**

- [PromQL Functions](references/promql_functions.md) — all PromQL functions with examples; read for specific function syntax questions
- [PromQL Patterns](references/promql_patterns.md) — RED/USE method patterns, alerting rules, recording rules; read for standard monitoring patterns
- [Best Practices](references/best_practices.md) — anti-patterns, performance optimization, cardinality management; read when optimizing queries
- [Metric Types](references/metric_types.md) — counter/gauge/histogram/summary guide; read to confirm correct function choice
- [Common Queries](assets/common_queries.promql) — reusable request rate, error rate, latency, and availability query templates
- [RED Method](assets/red_method.promql) — complete RED method (Rate, Errors, Duration) implementation
- [USE Method](assets/use_method.promql) — complete USE method (Utilization, Saturation, Errors) implementation
- [SLO Patterns](assets/slo_patterns.promql) — SLO, error budget, burn rate, and multi-window alerting patterns
- [Alerting Rules](assets/alerting_rules.yaml) — example alerting rules with thresholds and `for` clauses
- [Recording Rules](assets/recording_rules.yaml) — example recording rules with `level:metric:operations` naming
- [Kubernetes Patterns](assets/kubernetes_patterns.promql) — kube-state-metrics, cAdvisor, and vector matching examples
