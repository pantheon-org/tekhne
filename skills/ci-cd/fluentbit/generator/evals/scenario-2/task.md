# Dual-Destination Log Pipeline: Loki + S3 Archival

## Problem/Feature Description

A DevOps team at a media streaming startup needs to route logs from two sources to two different destinations. Application logs from their API servers (written to `/var/app/logs/*.log`) should be sent to Grafana Loki (`loki.internal.svc:3100`) for real-time querying by developers. All container logs in Kubernetes (`/var/log/containers/*.log`) should additionally be archived to an S3 bucket (`media-logs-archive`) in `eu-west-1` for compliance purposes.

A previous engineer set up a prototype that sent everything to both destinations, causing doubled ingestion costs on Loki and making log queries confusing because every record appeared twice. The team now wants each log source routed only to its intended destination.

## Output Specification

Produce a `fluent-bit.conf` that correctly routes logs from each source to only its intended destination. Include a `routing.md` that explains, for each INPUT defined, which OUTPUT(s) it routes to and why.
