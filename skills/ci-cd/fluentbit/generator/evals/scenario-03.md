# Scenario 03: Dual-Destination Log Pipeline: Loki + S3 Archival

## User Prompt

A DevOps team at a media streaming startup needs to route logs from two sources to two different destinations. Application logs from their API servers (written to `/var/app/logs/*.log`) should be sent to Grafana Loki (`loki.internal.svc:3100`) for real-time querying by developers. All container logs in Kubernetes (`/var/log/containers/*.log`) should additionally be archived to an S3 bucket (`media-logs-archive`) in `eu-west-1` for compliance purposes.

A previous engineer set up a prototype that sent everything to both destinations, causing doubled ingestion costs on Loki and making log queries confusing because every record appeared twice. The team now wants each log source routed only to its intended destination.

## Output Specification

Produce a `fluent-bit.conf` that correctly routes logs from each source to only its intended destination. Include a `routing.md` that explains, for each INPUT defined, which OUTPUT(s) it routes to and why.

## Expected Behavior

1. Assign distinct tag namespaces to the two INPUT plugins (e.g., `app.*` for the API logs and `kube.*` for container logs) so routing can be scoped
2. Ensure Loki OUTPUT and S3 OUTPUT do NOT both use `Match *` — at least one must have a specific match pattern
3. Configure each OUTPUT's `Match` to correspond only to its intended INPUT tag namespace
4. Document the tag-to-output routing in `routing.md`
5. Add `Retry_Limit` to both outputs and `Mem_Buf_Limit` to both inputs

## Success Criteria

- **Distinct tag namespaces**: The two INPUT plugins use different Tag values (e.g., app.* and kube.*) rather than the same tag on both
- **No shared Match ***: The Loki OUTPUT and S3 OUTPUT do NOT both use Match * — at least one has a specific match pattern
- **Scoped Match patterns**: Each OUTPUT's Match value corresponds to only the tag namespace of its intended INPUT source
- **Routing documented**: routing.md (or equivalent) maps each INPUT to its corresponding OUTPUT(s) by tag
- **Retry_Limit on both outputs**: Both the Loki OUTPUT and S3 OUTPUT include a numeric Retry_Limit setting
- **Mem_Buf_Limit on inputs**: Both tail INPUT plugins include Mem_Buf_Limit
- **S3 compression**: The S3 OUTPUT includes compression gzip
- **storage.total_limit_size**: At least one OUTPUT includes storage.total_limit_size
- **Exclude_Path on container input**: The tail INPUT watching /var/log/containers/ includes an Exclude_Path that filters out fluent-bit's own logs

## Failure Conditions

- Both INPUT plugins use the same Tag value, making per-source routing impossible
- Both Loki OUTPUT and S3 OUTPUT use `Match *`, causing all logs to go to both destinations
- Any OUTPUT's Match pattern does not correspond to its intended INPUT tag namespace
- `routing.md` does not explain which INPUT routes to which OUTPUT
- `Retry_Limit` is absent or set to `False` on either output
- `Mem_Buf_Limit` is absent from either input plugin
