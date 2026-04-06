---
name: aws-cloudwatch-url-builder
description: "Construct deep-link URLs for the AWS CloudWatch console — Logs Insights queries, Alarm detail pages, Metrics graphs. Use when encoding log groups, time ranges, query strings, metric dimensions, or opening CloudWatch views in a Playwright browser session. Keywords: CloudWatch deep-link Logs-Insights alarm metrics fragment-encoding *XX-encoding epoch timestamp hash-routing url-construction SPA alarm-detail metric-graph log-group query-string cloudwatch-console."
sources: []
---

# AWS CloudWatch URL Builder

## Mindset

CloudWatch URLs use the URL **fragment** (`#`) for all navigation state. The fragment is
never sent to the server — it is interpreted entirely by the browser-side CloudWatch SPA.
This means:

- Standard `%XX` percent-encoding does **not** apply inside CloudWatch fragment values.
  CloudWatch uses `*XX` (asterisk + uppercase hex) for characters inside parameter values.
- Structural characters (`~`, `(`, `)`, `'`) must remain literal — do **not** encode them.
- Build the decoded form first, then apply `*XX` encoding only to the value portions.

**Rule of thumb for alarms:** When investigating a triggered alarm, always navigate to the
**alarm detail page** (not the Metrics graph) first. The alarm detail shows the *exact*
metric that triggered it — which may be a CloudWatch Logs metric filter, not a native
`AWS/Lambda` metric. The two diverge when an invocation exits cleanly but logs application
errors.

## When to Use This Skill

- You need to open a specific CloudWatch Logs Insights query directly in a browser
- You need to link to an alarm detail page (state history + metric graph)
- You need to link to a Metrics graph for a specific namespace/metric/dimension
- You are capturing AWS Console screenshots via Playwright and need the exact URL
- You are documenting an incident and want reproducible deep-link evidence

---

## URL Encoding Rules

CloudWatch uses two different encoding layers depending on where a value appears in the URL.

### Path segment encoding (between `/` delimiters, after the `#`)

| Character | Encoded form |
|-----------|-------------|
| `?`       | `$3F`       |
| `=`       | `$3D`       |
| `&`       | `$26`       |

### Query string encoding (inside parameter values, after `~'`)

CloudWatch uses `*XX` (asterisk + uppercase hex) rather than `%XX`:

| Character | Encoded form |
|-----------|-------------|
| space     | `*20`       |
| `@`       | `*40`       |
| newline   | `*0a`       |
| `\|`      | `*7c`       |
| `/`       | `*2f`       |
| `:`       | `*3a`       |
| `[`       | `*5b`       |
| `]`       | `*5d`       |
| `{`       | `*7b`       |
| `}`       | `*7d`       |
| `"`       | `*22`       |

---

## URL Templates

### 1. Logs Insights

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#logsV2:logs-insights$3FqueryDetail$3D~(end~'{end_epoch}~start~'{start_epoch}~timeType~'ABSOLUTE~unit~'seconds~editorString~'{encoded_query}~source~(~'{encoded_log_group})~queryId~'{query_id})
```

**Parameters:**

| Parameter | Description | Example |
|-----------|-------------|---------|
| `region` | AWS region code | `eu-west-1` |
| `start_epoch` | Unix timestamp (seconds) — query window start | `1775397600` |
| `end_epoch` | Unix timestamp (seconds) — query window end | `1775404800` |
| `encoded_query` | Query string with `*XX` encoding | see below |
| `encoded_log_group` | Log group name with `*XX` encoding | see below |

**Encoding a query:**

Raw:
```
fields @timestamp, @message
| filter @message like /(?i)(error|exception)/
| sort @timestamp asc
| limit 50
```

Encoded (newlines → `*0a`, spaces → `*20`, `|` → `*7c`, `/` → `*2f`, `@` → `*40`):
```
fields*20*40timestamp,*20*40message*0a*7c*20filter*20*40message*20like*20*2f(?i)(error*7cexception)*2f*0a*7c*20sort*20*40timestamp*20asc*0a*7c*20limit*2050
```

**Encoding a log group:**

Raw: `/aws/lambda/bip-laas-api-facade-pr-PlayerLambdaFunction786A3E0-2j0cL2524eAR`

Encoded (`/` → `*2f`):
```
*2faws*2flambda*2fbip-laas-api-facade-pr-PlayerLambdaFunction786A3E0-2j0cL2524eAR
```

---

### 2. CloudWatch Alarm Detail

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `region` | AWS region code | `eu-west-1` |
| `alarm_name` | Full CloudWatch alarm name | `bip-laas-api-facade-pr-PlayerLambdaFunctionErrorAlarmC578E080-LyOUE2A6wiS2` |

The alarm detail page shows:
- Alarm state history (OK → ALARM transitions with timestamps)
- The alarm's metric graph with the threshold line
- Alarm configuration (metric, threshold, evaluation periods, SNS actions)

---

### 3. Metrics Graph

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#metricsV2:graph=~(metrics~(~(~'{namespace}~'{metric_name}~'{dimension_name}~'{dimension_value}))~start~'{start_iso}~end~'{end_iso}~stat~'{stat}~period~{period_seconds})
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `namespace` | CloudWatch namespace | `AWS*2fLambda` |
| `metric_name` | Metric name | `Errors` |
| `dimension_name` | Dimension key | `FunctionName` |
| `dimension_value` | Dimension value | `my-function-name` |
| `start_iso` | ISO 8601 start time | `2026-04-05T12*3a00*3a00Z` |
| `end_iso` | ISO 8601 end time | `2026-04-05T16*3a00*3a00Z` |
| `stat` | Statistic | `Sum` |
| `period_seconds` | Period in seconds | `300` |

---

## Anti-Patterns

❌ **NEVER use `%XX` percent-encoding inside CloudWatch fragment values**

CloudWatch fragment values use `*XX` (asterisk + uppercase hex), not the standard `%XX`.
Using `%XX` produces a broken URL that the CloudWatch SPA either ignores or misparses,
silently loading the wrong query or defaulting to a blank Logs Insights view.

---

❌ **NEVER use millisecond timestamps for Logs Insights time range**

Logs Insights `start`/`end` parameters require **seconds** (Unix epoch).
Passing milliseconds results in queries with a time range in the year 57,000+ — returning
zero results with no error, causing misleading "no data" investigations.

✅ Convert: `Date.now() / 1000 | 0` or `new Date('2026-04-05T12:00:00Z').getTime() / 1000`

---

❌ **NEVER navigate to `AWS/Lambda Errors` metric to verify an alarm**

Lambda alarms may be driven by a **CloudWatch Logs metric filter** (e.g.
`ErrorCount-PlayerLambdaFunction`) rather than the native `AWS/Lambda Errors` metric.
The native metric counts failed invocations; the Logs filter counts ERROR log lines. These
diverge when the invocation exits cleanly (exit 0) but logs application-level errors.

✅ Always start at the alarm detail page — it shows exactly which metric triggered the alarm.

---

❌ **NEVER hardcode the region in the URL path without also setting `region=` in the query string**

CloudWatch uses `region=` in the query string to determine which region's data to load.
The subdomain (`{region}.console.aws.amazon.com`) handles routing but the query param
drives data. Omitting it causes the console to default to the last-used region for that browser.

---

## Practical Notes

- Epoch timestamps for Logs Insights must be in **seconds** (not milliseconds).
- ISO timestamps for Metrics must encode `:` as `*3a`.
- The `queryId` parameter in Logs Insights URLs can be omitted or set to a random UUID.
- Alarm names containing special characters (e.g. hyphens, uppercase) do not need additional
  encoding beyond the standard URL path encoding — hyphens are safe.

## References

- **aws-console-navigator** (companion skill in this tile) — step-by-step SSO auth, region switching, and CloudWatch page navigation in Playwright or manual browser sessions
