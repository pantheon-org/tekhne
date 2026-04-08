# CloudWatch URL Encoding Reference

## Encoding Rules

CloudWatch uses two different encoding layers depending on where a value appears.

### Path segment encoding (between `/` delimiters after `#`)

| Character | Encoded form |
|-----------|-------------|
| `?`       | `$3F`       |
| `=`       | `$3D`       |
| `&`       | `$26`       |

### Value encoding (inside parameter values after `~'`)

CloudWatch uses `*XX` (asterisk + uppercase hex), NOT standard `%XX`:

| Character | Encoded form |
|-----------|-------------|
| space     | `*20`       |
| `,`       | `*2c`       |
| `/`       | `*2f`       |
| `:`       | `*3a`       |
| `@`       | `*40`       |
| `[`       | `*5b`       |
| `]`       | `*5d`       |
| `{`       | `*7b`       |
| `\|`      | `*7c`       |
| `}`       | `*7d`       |
| `"`       | `*22`       |
| newline   | `*0a`       |

Structural characters `~`, `(`, `)`, `'` stay **literal** — do not encode them.

---

## URL Templates

### Logs Insights

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#logsV2:logs-insights$3FqueryDetail$3D~(end~'{end_iso}~start~'{start_iso}~timeType~'ABSOLUTE~tz~'UTC~editorString~'{encoded_query}~source~(~'{encoded_log_group})~lang~'CWLI~logClass~'STANDARD~queryBy~'logGroupName)
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `region` | AWS region code | `eu-west-1` |
| `start_iso` | ISO 8601 in **UTC** (`:` → `*3a`) | `2026-04-07T02*3a30*3a00.000Z` |
| `end_iso` | ISO 8601 in **UTC** (`:` → `*3a`) | `2026-04-07T02*3a50*3a00.000Z` |
| `encoded_query` | Query string with `*XX` encoding applied | see below |
| `encoded_log_group` | Log group name with `*XX` encoding | see below |

**Example encoded query** (newlines→`*0a`, spaces→`*20`, commas→`*2c`, `|`→`*7c`, `/`→`*2f`, `@`→`*40`):

Raw:
```
fields @timestamp, @message
| filter @message like /(?i)(error|exception)/
| sort @timestamp asc
| limit 50
```

Encoded:
```
fields*20*40timestamp*2c*20*40message*0a*7c*20filter*20*40message*20like*20*2f(?i)(error*7cexception)*2f*0a*7c*20sort*20*40timestamp*20asc*0a*7c*20limit*2050
```

**Example encoded log group** (`/` → `*2f`):

Raw: `/aws/lambda/my-service-prod-Handler`
Encoded: `*2faws*2flambda*2fmy-service-prod-Handler`

---

### Alarm Detail

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}?~(timeRange~(startDate~'{start_iso}~endDate~'{end_iso}))
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `alarm_name` | Full CloudWatch alarm name | `my-service-pr-ErrorAlarm52C86406-lobALzDtvOAb` |
| `start_iso` | ISO 8601 start in **UTC** (`:` → `*3a`) | `2026-04-07T02*3a40*3a00.000Z` |
| `end_iso` | ISO 8601 end in **UTC** (`:` → `*3a`) | `2026-04-07T02*3a56*3a00.000Z` |

The alarm detail page shows state history, metric graph with threshold, and alarm configuration.
Alarm names with hyphens or uppercase letters do not need additional encoding.

**Time range guidance:**
- Always use **UTC** (`Z` suffix). AWS CloudWatch times are always UTC — never substitute BST or other local times.
- Set `startDate` a few minutes before the first state transition (OK → ALARM).
- Set `endDate` a few minutes after the last transition back to OK, so the full incident window is visible.
- Without a time range the alarm page loads but defaults to a recent rolling window — the incident graph may not be visible.

---

### Metrics Graph

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#metricsV2:graph=~(metrics~(~(~'{namespace}~'{metric_name}~'{dimension_name}~'{dimension_value}))~start~'{start_iso}~end~'{end_iso}~stat~'{stat}~period~{period_seconds})
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `namespace` | CloudWatch namespace | `AWS*2fLambda` |
| `metric_name` | Metric name | `Errors` |
| `dimension_name` | Dimension key | `FunctionName` |
| `dimension_value` | Dimension value | `my-function` |
| `start_iso` | ISO 8601 start (`:` → `*3a`) | `2026-04-05T12*3a00*3a00Z` |
| `end_iso` | ISO 8601 end | `2026-04-05T16*3a00*3a00Z` |
| `stat` | Statistic | `Sum` |
| `period_seconds` | Period in seconds | `300` |

---

## Practical Notes

- Logs Insights uses **ISO 8601 UTC timestamps** (`2026-04-07T02*3a30*3a00.000Z`), not epoch seconds.
- Both Logs Insights and Metrics timestamps encode `:` as `*3a`.
- All times are **UTC** — never substitute BST or other local timezone offsets.
- Commas in query strings must be encoded as `*2c` (e.g. `fields @timestamp*2c @message`).
- `queryId` in Logs Insights URLs can be omitted or set to any UUID.
