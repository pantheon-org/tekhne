# Product Analytics Warehouse (Databricks example — no equivalent MCP connected in this session by default; adapt for Snowflake, BigQuery, ClickHouse, dbt)

## What this source contains

The product-analytics, data-pipeline, and warehouse-telemetry layer. It complements infrastructure observability: that's the *infra/runtime* view, this is the *product/data* view (what users did, which experiments ran, how feature usage evolved, where a threshold constant came from).

- **Product analytics events.** User behavior: feature invocations, clicks, accepts/rejects, submissions, client-reported errors.
- **Usage & billing events.** For cost- or volume-driven decisions.
- **Experiment / feature-flag data.** Exposure and outcome tables. **Schema is company-specific.** Probe with `SHOW TABLES` before assuming names.
- **System tables.** Query history, compute/warehouse metadata, billing, audit logs. Answer "was this query expensive?", "how often did anyone run this?", "when did warehouse load spike?"
- **Pipeline lineage (e.g. dbt).** Models reveal what pipelines depend on a table/field; upstream changes frequently motivate consumer-code changes.
- **Notebooks.** Exploratory analyses engineers wrote before code changes. **Usually not queryable via a SQL-only MCP.** If you suspect the rationale lives in a notebook, name it as a gap.

## How to search it

Confirm a warehouse MCP is actually connected this session before assuming — none is connected by default here. If one exists, use its read-only SQL execution tool.

**Orient before querying.** Schemas are company-specific; probe before trusting a table name (`SHOW TABLES`, `DESCRIBE TABLE`).

**Time-bound every query.** These tables are huge and unconstrained scans time out. Filter on the event timestamp column with a window bracketing the ship date, typically ~30 days before and after, wider only for strong reason.

**Prefer typed/deduplicated models over raw event tables** when both exist.

### Investigation patterns that tend to pay off

1. **Event usage trajectory.** Daily counts on the relevant table across a ±30d window around the PR/MR merge. A step function from zero to steady volume within a day or two of the merge is strong circumstantial evidence the PR launched the feature. A decay to zero suggests a deprecation or deletion.
2. **Guard-rail / defensive-check origin.** Distribution (median / p99 / max) of the relevant column in the 14 days *before* the PR/MR. A p99 that matches the target's threshold constant suggests the number was chosen from data.
3. **Experiment / feature-flag lookup.** Find the exposure table, then pull exposure counts by variant for the relevant flag key near the PR/MR date.
4. **Query-history evidence for migrations, backfills, or perf rewrites.** Filter query-history system tables by statement text matching the target table/symbol with a tight time window; sort by duration or bytes read to surface the expensive queries that likely motivated the change.
5. **Pipeline lineage.** If the target reads from or writes into a warehouse model, the model's own git history (in whichever repo defines it) often carries the rationale. Hand that lead back to the source-control investigator rather than chasing it yourself.

## What good evidence looks like here

- An error-classifying event's count drops to near zero in the days after a defensive-code PR/MR. Suggests the PR/MR resolved that error class
- An exposure table row names the target's feature-flag key with a "shipped" / "concluded" decision around the PR/MR ship date

## Common pitfalls

- **Instrumented ≠ caused.** An event's existence means someone cared enough to log it, not that the target code exists *because* of it. Pair with a commit/PR/MR citation before claiming causation.
- **Silent instrumentation changes.** A step function in event volume may mean a new event started being logged, not that user behavior changed. Check for instrumentation PRs/MRs in the same window before reading the ramp as a feature-launch signal.
- **Schema drift.** Event properties evolve; a column on today's typed model may not have existed when the target was written.
- **Pipeline refresh lag.** Aggregated/typed tables are rebuilt on a schedule. For events from the last few hours, fall back to the raw table and deduplicate.
- **Company-specific tables.** Experiment, feature-flag, billing, and usage tables vary. Reporting a result from a table whose existence you never confirmed is a classic failure mode. Probe first.
- **Retention cliff.** If the relevant window predates the table's retention or the model's creation date, that's a *gap*, not a null result. Name it explicitly so the synthesizer doesn't read "no results" as "no activity."
- **Notebooks aren't queryable.** If you suspect the rationale lives in one, return a gap.

## What to return

For each relevant finding:
- Type (product event / experiment exposure / usage or billing event / system-table row / pipeline model)
- Fully-qualified table name and the exact query you ran
- Time window queried
- Compact numeric summary (counts, percentiles, first/last-seen timestamps). **Don't dump raw rows.**
- Temporal correlation with the target's ship date (e.g., "first row 2024-08-15; PR/MR #49074 merged 2024-08-14")
- Relevance + strength: direct / circumstantial / weak
