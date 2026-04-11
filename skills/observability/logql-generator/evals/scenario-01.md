# Scenario 01: Nginx Access Log Analysis

## User Prompt

An e-commerce platform runs Nginx as its edge reverse proxy in Kubernetes. The platform is intermittently seeing slow response times, and the SRE team wants to investigate whether specific upstream services are causing 5xx errors. The Nginx instances emit access logs in a custom semi-structured format:

```
10.1.2.3 - alice [01/Mar/2026:10:15:00 +0000] "GET /api/products?page=2 HTTP/1.1" 502 1024 "https://shop.example.com" "Mozilla/5.0..."
```

The logs are collected into Grafana Loki with the following labels available on the streams: `job="nginx-access"`, `namespace="prod"`, `cluster="us-east"`.

The SRE team needs queries to help with two things:

1. **Filter to 5xx errors only** — return raw log lines so engineers can read them and spot patterns (endpoint, upstream, source IP).
2. **Count the rate of 5xx responses over time** — a metric query that can be graphed in a Grafana dashboard panel.

For the rate query, the team cares about separating counts by HTTP method (GET, POST, etc.) so they can tell whether writes or reads are failing more.

Produce a Markdown file named `queries.md` that contains:

- The two LogQL queries (clearly labelled as the log filter query and the metric query)
- A brief explanation of each query and how to interpret the results
- Notes on which labels or values to change to adapt the queries to different environments or time windows

Do not include setup or installation instructions.

## Expected Behavior

1. Place a line filter (e.g. `|= "50"` or `|~ "5[0-9]{2}"`) before the parser stage
2. Use the `pattern` parser to extract fields from the semi-structured Nginx log format
3. Include at least two label matchers in the stream selector (e.g. `job` and `namespace`)
4. After parsing, filter the status code using a label filter (e.g. `status >= 500`)
5. Use `rate()` for the metric query (not `count_over_time()`)
6. Aggregate the metric query using `sum by (method)` to separate HTTP method counts
7. Add explanations for each query and customization notes for adapting to different environments

## Success Criteria

- **Line filter before parser**: The 5xx log-filter query places a line filter (e.g. `|= "50"` or `|~ "5[0-9]{2}"`) BEFORE the parser stage — not after it
- **Pattern parser used**: Uses the pattern parser (`| pattern "..."`) to extract fields from the nginx access log format rather than regexp or json
- **Specific stream selector**: Stream selector includes at least two label matchers (e.g. job and namespace, or job and cluster) rather than a single broad label
- **Status code via label filter**: After parsing, the status code is filtered via a label filter (e.g. `status >= 500`) rather than only relying on the line filter
- **rate() for metric query**: The metric (rate) query uses `rate()` not `count_over_time()`
- **sum by method grouping**: The metric query aggregates using `sum by (method)` or equivalent grouping on the extracted HTTP method label
- **Customization notes present**: The output document includes notes identifying which labels or values to change (e.g. namespace, cluster, time window)
- **Query explanations present**: Each query is accompanied by a plain-English explanation of what it returns and how to interpret results

## Failure Conditions

- Places line filter after the parser stage instead of before it
- Uses regexp or json parser instead of the pattern parser for the semi-structured format
- Uses only a single label matcher in the stream selector
- Does not apply a label filter on the status code after parsing
- Uses `count_over_time()` instead of `rate()` for the metric query
- Omits HTTP method grouping from the metric aggregation
- Produces queries without customization notes
- Produces queries without explanations
