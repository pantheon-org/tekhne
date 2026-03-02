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

## Resources

### references/
- **`promql_functions.md`** — All PromQL functions with examples. Read for specific function questions.
- **`promql_patterns.md`** — RED/USE method patterns, alerting, recording rules. Read for standard monitoring patterns.
- **`best_practices.md`** — Anti-patterns, performance, cardinality. Read when optimizing.
- **`metric_types.md`** — Counter/Gauge/Histogram/Summary guide. Read to confirm function choice.

### assets/
- **`common_queries.promql`** — Reusable request rate, error rate, latency, availability queries.
- **`red_method.promql`** — Complete RED method implementation.
- **`use_method.promql`** — Complete USE method implementation.
- **`slo_patterns.promql`** — SLO, error budget, burn rate, multi-window alerting.
- **`alerting_rules.yaml`** — Example alerting rules with thresholds.
- **`recording_rules.yaml`** — Example recording rules with naming conventions.
- **`kubernetes_patterns.promql`** — kube-state-metrics, cAdvisor, vector matching.

> Always read the relevant reference/asset file and cite the applicable pattern before generating a query.
