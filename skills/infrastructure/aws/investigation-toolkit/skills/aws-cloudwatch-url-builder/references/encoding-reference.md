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

Structural characters `~`, `(`, `)`, `'` stay **literal** — do not encode them.

---

## URL Templates

### Logs Insights

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#logsV2:logs-insights$3FqueryDetail$3D~(end~'{end_epoch}~start~'{start_epoch}~timeType~'ABSOLUTE~unit~'seconds~editorString~'{encoded_query}~source~(~'{encoded_log_group})~queryId~'{query_id})
```

| Parameter | Description | Example |
|-----------|-------------|---------|
| `region` | AWS region code | `eu-west-1` |
| `start_epoch` | Unix timestamp in **seconds** | `1775397600` |
| `end_epoch` | Unix timestamp in **seconds** | `1775404800` |
| `encoded_query` | Query string with `*XX` encoding applied | see below |
| `encoded_log_group` | Log group name with `*XX` encoding | see below |
| `query_id` | Optional UUID; may be omitted or randomised | `''` |

**Example encoded query** (newlines→`*0a`, spaces→`*20`, `|`→`*7c`, `/`→`*2f`, `@`→`*40`):

Raw:
```
fields @timestamp, @message
| filter @message like /(?i)(error|exception)/
| sort @timestamp asc
| limit 50
```

Encoded:
```
fields*20*40timestamp,*20*40message*0a*7c*20filter*20*40message*20like*20*2f(?i)(error*7cexception)*2f*0a*7c*20sort*20*40timestamp*20asc*0a*7c*20limit*2050
```

**Example encoded log group** (`/` → `*2f`):

Raw: `/aws/lambda/my-service-prod-Handler`
Encoded: `*2faws*2flambda*2fmy-service-prod-Handler`

---

### Alarm Detail

```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}
```

The alarm detail page shows state history, metric graph with threshold, and alarm configuration.
Alarm names with hyphens or uppercase letters do not need additional encoding.

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

- `queryId` in Logs Insights URLs can be omitted or set to any UUID.
- ISO timestamps for Metrics must encode `:` as `*3a`.
- Epoch timestamps for Logs Insights must be in **seconds** (not milliseconds).
