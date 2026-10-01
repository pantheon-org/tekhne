---
name: aws-cloudwatch-url-builder
description: "Construct deep-link URLs for the AWS CloudWatch console — Logs Insights queries, Alarm detail pages, Metrics graphs. Use when encoding log groups, time ranges, query strings, metric dimensions, or opening CloudWatch views in a Playwright browser session. Keywords: CloudWatch deep-link Logs-Insights alarm metrics fragment-encoding *XX-encoding epoch timestamp hash-routing url-construction SPA alarm-detail metric-graph log-group query-string cloudwatch-console."
sources: []
---

# AWS CloudWatch URL Builder

## Mindset

CloudWatch URLs use the URL **fragment** (`#`) for all navigation state. The fragment is
never sent to the server: it is interpreted entirely by the browser-side CloudWatch SPA.
Standard `%XX` encoding does **not** apply inside fragment values; CloudWatch uses `*XX`
(asterisk + uppercase hex) instead.

## Philosophy

- Treat the fragment as a small language of its own, not as an ordinary URL.
- Encode values only; leave structural characters (`~`, `(`, `)`, `'`) literal.
- Express every time in UTC, derived from the event timestamps rather than local time.
- Start alarm investigations at the alarm detail page, because it shows the metric that really fired.

## When to Use

- Constructing a reproducible deep-link to a CloudWatch Logs Insights query
- Linking to an alarm detail page (state history + metric graph)
- Linking to a Metrics graph for a specific namespace/metric/dimension
- Generating URLs for Playwright screenshot capture of CloudWatch pages
- Documenting an incident with reproducible console evidence

## When Not to Use

- The target page is outside CloudWatch (for example the S3 console or an EC2 dashboard): encoding rules differ per service
- You only need the AWS Console root URL or a service listing: no fragment encoding is required
- You need to query logs or metrics programmatically: this skill only builds links to console views

## Build Order

1. Identify the target page type: Logs Insights, Alarm detail, or Metrics graph.
2. Gather parameters: region, log group name, time range, query or metric name.
3. Apply `*XX` encoding to value portions only. Structural characters (`~`, `(`, `)`, `'`) stay literal.
4. Compose the URL using the template in `references/encoding-reference.md`.
5. Verify: timestamps are ISO 8601 UTC strings with `*3a`-encoded colons, and the region appears in both the subdomain and the query string.

**Rule of thumb for alarms:** navigate to the **alarm detail page** first. It shows the exact
metric that triggered the alarm, which may be a Logs metric filter rather than a native
`AWS/Lambda` metric.

## Quick Reference

Common `*XX` encodings for value portions:

| Character | Encoded |
|-----------|---------|
| space | `*20` |
| `,` | `*2c` |
| `/` | `*2f` |
| `:` | `*3a` |
| `@` | `*40` |
| `\|` | `*7c` |
| newline | `*0a` |

Between path segments after the `#`, `?` becomes `$3F`, `=` becomes `$3D` and `&` becomes `$26`.

### Encode a value from the shell

```bash
python3 -c "import sys; print(''.join(c if c.isalnum() or c in \"-_.~()'?\" else '*%02x' % ord(c) for c in sys.argv[1]))" '/aws/lambda/my-service-prod-Handler'
```

Expected result: `*2faws*2flambda*2fmy-service-prod-Handler`.

### Logs Insights template

```text
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#logsV2:logs-insights$3FqueryDetail$3D~(end~'{end_iso}~start~'{start_iso}~timeType~'ABSOLUTE~tz~'UTC~editorString~'{encoded_query}~source~(~'{encoded_log_group})~lang~'CWLI~logClass~'STANDARD~queryBy~'logGroupName)
```

Expected result: a link that opens the query pre-filled for the given UTC window.

### Alarm detail template

```text
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}?~(timeRange~(startDate~'{start_iso}~endDate~'{end_iso}))
```

Expected result: the alarm page showing the metric graph for the whole incident window.

### Metrics graph template

```text
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#metricsV2:graph=~(metrics~(~(~'{namespace}~'{metric_name}~'{dimension_name}~'{dimension_value}))~start~'{start_iso}~end~'{end_iso}~stat~'{stat}~period~{period_seconds})
```

Expected result: a graph for one metric and dimension with the chosen statistic and period.

### ISO timestamp format

```text
2026-04-07T02*3a30*3a00.000Z
```

Expected result: a UTC timestamp whose colons are written as `*3a`.

## Anti-Patterns

### NEVER use `%XX` percent-encoding inside CloudWatch fragment values

**WHY:** CloudWatch fragment values use `*XX` (asterisk + uppercase hex). Using `%XX` produces a
broken URL the SPA silently ignores or misparses, loading the wrong query or a blank Logs Insights view.

**BAD:** `%2faws%2flambda%2fmy-fn`
**GOOD:** `*2faws*2flambda*2fmy-fn`

### NEVER use epoch integers for the Logs Insights time range

**WHY:** Logs Insights `start`/`end` require **ISO 8601 UTC strings**. Epoch numbers (for example
`start~'1775529000`) make the SPA fall back to epoch 0 (`1970-01-01`), giving a window spanning
decades and useless results.

**BAD:** `start~'1775529000`
**GOOD:** `start~'2026-04-07T02*3a30*3a00.000Z` with `tz~'UTC` and no `unit~'seconds`.

### NEVER navigate to the `AWS/Lambda Errors` metric to verify an alarm

**WHY:** Lambda alarms may be driven by a CloudWatch Logs metric filter rather than the native
metric. The native metric counts failed invocations; the filter counts ERROR log lines, and the two
diverge when an invocation exits cleanly but logs application-level errors.

**BAD:** Open Metrics, then `AWS/Lambda`, then `Errors`, and treat the graph as the alarm's evidence.
**GOOD:** Open the alarm detail page, which shows exactly which metric triggered the alarm.

### NEVER omit `region=` from the query string

**WHY:** CloudWatch uses `region=` to decide which region's data to load. Without it the console
defaults to the last-used region in that browser.

**BAD:** `https://eu-west-1.console.aws.amazon.com/cloudwatch/home#alarmsV2:alarm/{name}`
**GOOD:** `https://eu-west-1.console.aws.amazon.com/cloudwatch/home?region=eu-west-1#alarmsV2:alarm/{name}`

### NEVER use local/BST time in alarm or metric URL time ranges

**WHY:** All CloudWatch timestamp parameters (`startDate`, `endDate`, `start`, `end`) are UTC.
Writing BST times with a `Z` suffix silently opens the wrong window.

**BAD:** An alarm fired at `03:44 BST` written as `2026-04-07T03*3a44*3a00.000Z`.
**GOOD:** `2026-04-07T02*3a44*3a00.000Z`, derived from the UTC event timestamp.

### NEVER omit the time range from an alarm detail URL used as incident evidence

**WHY:** Without `?~(timeRange~(...))` the page defaults to a rolling recent window. A screenshot
taken after the incident no longer shows the alarm spike.

**BAD:** `...#alarmsV2:alarm/{alarm_name}`
**GOOD:** Append `?~(timeRange~(startDate~'{start_iso}~endDate~'{end_iso}))` with a UTC window
bracketing the incident from a few minutes before OK to ALARM until the final OK state.

### NEVER encode the structural characters `~`, `(`, `)` or `'`

**WHY:** They are the fragment's own syntax. Encoding them changes the structure the SPA parses.

**BAD:** `*7etimeRange*7e*28startDate`
**GOOD:** `~(timeRange~(startDate~'2026-04-07T02*3a44*3a00.000Z))`

### NEVER leave `?`, `=` or `&` unencoded between path segments after the `#`

**WHY:** Between path segments they must be written `$3F`, `$3D` and `$26`, otherwise the router
splits the fragment in the wrong place.

**BAD:** `#logsV2:logs-insights?queryDetail=~(...)`
**GOOD:** `#logsV2:logs-insights$3FqueryDetail$3D~(...)`

### NEVER reuse these encoding rules for other AWS consoles

**WHY:** Encoding rules differ per service, so a link built with CloudWatch rules may not open
the S3 or EC2 console correctly.

**BAD:** Applying `*XX` encoding to an S3 console URL.
**GOOD:** Look up the encoding rules for that service before building its link.

## References

| Topic | Reference | When to Use |
|-------|-----------|-------------|
| Encoding rules and URL templates | [encoding-reference.md](references/encoding-reference.md) | Composing any Logs Insights, alarm or metrics URL |

Companion skill **aws-console-navigator**, in the same toolkit, covers SSO auth, region switching
and Playwright navigation.
