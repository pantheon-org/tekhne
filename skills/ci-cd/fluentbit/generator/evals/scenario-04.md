# Scenario 04: Application Log Parsing Pipeline with Field Extraction

## User Prompt

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

## Expected Behavior

1. Use the `json` parser for the payment service — not a regex parser — since the logs are already JSON-structured
2. Use the `logfmt` parser for the auth service — not a regex parser — since logfmt is a native Fluent Bit parser type
3. Use a `regexp` parser only for the legacy billing service that genuinely requires pattern extraction
4. Place FILTER sections for parsing before the FILTER section for level-based grep filtering
5. Retain only `error` and `warn` level events via a grep FILTER
6. Enable TLS on the OpenTelemetry OUTPUT and set a numeric Retry_Limit

## Success Criteria

- **JSON service uses json parser**: The payment service input or filter uses Parser json (not a regexp parser variant)
- **Logfmt service uses logfmt parser**: The auth service input or filter uses Parser logfmt (not a regexp parser variant)
- **Regexp only for legacy**: A regexp/regex parser is used only for the legacy billing service, not for the payment or auth services
- **Parser choice explained**: design-notes.md (or equivalent) explicitly states why json/logfmt parsers were chosen for the structured services
- **Parse filters before grep**: In fluent-bit.conf, FILTER sections for parsing (parser filter) appear before the FILTER section for level filtering (grep filter)
- **Level filtering present**: A grep or equivalent FILTER retains only error/warn/warning level events
- **TLS on OTEL output**: The opentelemetry OUTPUT includes tls On
- **Retry_Limit on output**: The opentelemetry OUTPUT includes a numeric Retry_Limit
- **Mem_Buf_Limit on inputs**: Tail INPUT plugins include Mem_Buf_Limit

## Failure Conditions

- The payment service uses a regexp parser instead of the native `json` parser
- The auth service uses a regexp parser instead of the native `logfmt` parser
- Regexp is used for the payment or auth service (not restricted to the legacy billing service)
- `design-notes.md` does not explain the parser choice rationale
- Grep filter appears before parsing filters, causing level filtering on unparsed data
- No grep or level FILTER is present to retain only error/warn events
- TLS is not enabled on the OpenTelemetry OUTPUT
- `Retry_Limit` is absent or set to `False` on the OUTPUT
