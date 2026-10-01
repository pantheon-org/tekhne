# Scenario 07: Application Log Parsing Pipeline with Field Extraction

## User Prompt

A backend platform team collects logs from three different internal services:

1. **Payment service** — emits JSON-structured logs (e.g., `{"level":"error","msg":"charge failed","amount":99.99}`)
2. **Auth service** — emits logfmt-style logs (e.g., `level=info msg="login ok" user_id=42`)
3. **Legacy billing service** — emits a proprietary format that requires a custom regex to extract fields

All three services write to files under `/var/services/logs/`. After parsing, logs should be filtered to retain only `error` and `warn` level events, then forwarded to an OpenTelemetry collector at `otel-collector.platform.svc:4318` over HTTPS.

The team has had performance complaints about the current config — a previous engineer used a single regex parser for all three services including the JSON and logfmt ones. The team suspects this is causing unnecessary CPU usage.
