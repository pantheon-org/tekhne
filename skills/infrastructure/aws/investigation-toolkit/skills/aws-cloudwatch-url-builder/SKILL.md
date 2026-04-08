---
name: aws-cloudwatch-url-builder
description: "Construct deep-link URLs for the AWS CloudWatch console — Logs Insights queries, Alarm detail pages, Metrics graphs. Use when encoding log groups, time ranges, query strings, metric dimensions, or opening CloudWatch views in a Playwright browser session. Keywords: CloudWatch deep-link Logs-Insights alarm metrics fragment-encoding *XX-encoding epoch timestamp hash-routing url-construction SPA alarm-detail metric-graph log-group query-string cloudwatch-console."
sources: []
---

# AWS CloudWatch URL Builder

## Mindset

CloudWatch URLs use the URL **fragment** (`#`) for all navigation state. The fragment is
never sent to the server — it is interpreted entirely by the browser-side CloudWatch SPA.
Standard `%XX` encoding does **not** apply inside fragment values; CloudWatch uses `*XX`
(asterisk + uppercase hex) instead.

**Build order:**

1. Identify the target page type: Logs Insights, Alarm detail, or Metrics graph.
2. Gather parameters: region, log group name, time range, query or metric name.
3. Apply `*XX` encoding to value portions only — structural characters (`~`, `(`, `)`, `'`) stay literal.
4. Compose the URL using the template in `references/encoding-reference.md`.
5. Verify: epoch timestamps are 10-digit seconds, region appears in both subdomain and query string.

**Rule of thumb for alarms:** Navigate to the **alarm detail page** first — it shows the exact
metric that triggered the alarm, which may be a Logs metric filter rather than a native
`AWS/Lambda` metric.

## When to Use This Skill

- Constructing a reproducible deep-link to a CloudWatch Logs Insights query
- Linking to an alarm detail page (state history + metric graph)
- Linking to a Metrics graph for a specific namespace/metric/dimension
- Generating URLs for Playwright screenshot capture of CloudWatch pages
- Documenting an incident with reproducible console evidence

## When NOT to Use This Skill

- The target page is outside CloudWatch (e.g. S3 console, EC2 dashboard) — encoding rules differ per service
- You only need the AWS Console root URL or a service listing — no fragment encoding is required

---

## Anti-Patterns

Each entry shows the **BAD** practice and the **GOOD** replacement.

---

❌ **NEVER use `%XX` percent-encoding inside CloudWatch fragment values**

WHY: CloudWatch fragment values use `*XX` (asterisk + uppercase hex). Using `%XX` produces a
broken URL the CloudWatch SPA silently ignores or misparsees, loading the wrong query or a
blank Logs Insights view.

✅ BAD: `%2faws%2flambda%2fmy-fn` → GOOD: `*2faws*2flambda*2fmy-fn`

---

❌ **NEVER use epoch integers for Logs Insights time range**

WHY: Logs Insights `start`/`end` require **ISO 8601 UTC strings**, not Unix epoch integers.
Passing epoch numbers (e.g. `start~'1775529000`) causes the SPA to fall back to epoch 0
(`1970-01-01`), producing a time window spanning decades and returning useless results.

✅ Use ISO 8601 with `*3a`-encoded colons: `start~'2026-04-07T02*3a30*3a00.000Z`
   Always include `tz~'UTC` and omit `unit~'seconds`.

---

❌ **NEVER navigate to `AWS/Lambda Errors` metric to verify an alarm**

WHY: Lambda alarms may be driven by a **CloudWatch Logs metric filter** rather than the native
`AWS/Lambda Errors` metric. The native metric counts failed invocations; the Logs filter counts
ERROR log lines. These diverge when an invocation exits cleanly but logs application-level errors.

✅ Always start at the alarm detail page — it shows exactly which metric triggered the alarm.

---

❌ **NEVER omit `region=` from the query string**

WHY: CloudWatch uses `region=` in the query string to determine which region's data to load.
Omitting it causes the console to default to the last-used region in that browser.

---

❌ **NEVER use local/BST time in alarm or metric URL time ranges**

WHY: All CloudWatch timestamp parameters (`startDate`, `endDate`, `start`, `end`) are **UTC**.
Writing BST (or any other local timezone) times with a `Z` suffix produces a URL that silently
opens the wrong time window — the graph will appear but show the wrong period.

✅ Always derive times from UTC event timestamps. If the alarm fired at `03:44 BST`, the UTC
time is `02:44 UTC` — use `2026-04-07T02*3a44*3a00.000Z`, not `2026-04-07T03*3a44*3a00.000Z`.

---

❌ **NEVER omit the time range from an alarm detail URL used as incident evidence**

WHY: Without a `?~(timeRange~(...))` suffix the alarm page loads but defaults to a rolling
recent window. If the screenshot is captured after the incident, the metric graph will no longer
show the alarm spike.

✅ Always append `?~(timeRange~(startDate~'{start_iso}~endDate~'{end_iso}))` with a UTC window
that brackets the full incident (a few minutes before OK→ALARM through to the final OK state).

## References

- [Encoding rules and URL templates](references/encoding-reference.md)
- **aws-console-navigator** — companion skill in this tile for SSO auth, region switching, and Playwright navigation
