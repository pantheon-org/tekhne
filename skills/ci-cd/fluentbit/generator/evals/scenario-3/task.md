# Application Log Parsing Pipeline with Field Extraction

## Problem/Feature Description

A backend platform team collects logs from three different internal services:

1. **Payment service** — emits JSON-structured logs (e.g., `{"level":"error","msg":"charge failed","amount":99.99}`)
2. **Auth service** — emits logfmt-style logs (e.g., `level=info msg="login ok" user_id=42`)
3. **Legacy billing service** — emits a proprietary format that requires a custom regex to extract fields

All three services write to files under `/var/services/logs/`. After parsing, logs should be filtered to retain only `error` and `warn` level events, then forwarded to an OpenTelemetry collector at `otel-collector.platform.svc:4318` over HTTPS.

The team has had performance complaints about the current config — a previous engineer used a single regex parser for all three services including the JSON and logfmt ones. The team suspects this is causing unnecessary CPU usage.

## Output Specification

Produce a `fluent-bit.conf` and a `parsers.conf` with the appropriate parser definitions. Include a brief `design-notes.md` explaining the parser choice for each service and the filter ordering decisions made.

## Input Files

The following files are provided as input. Extract them before beginning.

=============== FILE: inputs/service-log-samples.txt ===============
# Payment service (JSON)
{"level":"error","msg":"charge failed","amount":99.99,"trace_id":"abc123"}
{"level":"info","msg":"charge ok","amount":15.00,"trace_id":"def456"}

# Auth service (logfmt)
level=info msg="login ok" user_id=42 ip=10.0.0.1
level=warn msg="invalid token" user_id=0 ip=192.168.1.5

# Legacy billing service (custom format)
2024-01-15 14:23:01 ERROR BillingEngine: invoice_id=INV-9923 status=OVERDUE
2024-01-15 14:23:05 INFO  BillingEngine: invoice_id=INV-8801 status=PAID
