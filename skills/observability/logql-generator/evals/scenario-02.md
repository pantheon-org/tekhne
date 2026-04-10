# Scenario 02: Payment Service Error Alerting

## User Prompt

A fintech startup processes card payments through a dedicated `payment-service` microservice. After several incidents where engineers only noticed elevated error rates after customers complained, the team has decided to proactively set up Grafana Loki alerting rules.

The payment-service logs are structured as JSON with a `level` field (`"error"`, `"warn"`, `"info"`) and a `transaction_type` field (`"charge"`, `"refund"`, `"void"`). Logs flow into Loki under labels `app="payment-service"` and `env="production"`.

The team wants two alerting expressions:

1. **High error rate alert** — fires when the absolute number of error-level log events per second exceeds a threshold of 2 errors/second, sustained over a 5-minute window.
2. **Error ratio alert** — fires when error-level events exceed 5% of all payment-service log events in the same window.

A previous intern wrote some alerts using raw event counts that kept misfiring after someone changed the dashboard time range. The team wants to make sure the new alerts won't have that problem. They also want the high error rate alert to never show "no data" in Grafana — it should evaluate to 0 when there are no errors.

Produce a file named `alert_expressions.md` containing:

- Both LogQL alert expressions (clearly labelled)
- A brief explanation of why each expression is structured the way it is
- One sentence identifying the failure mode of the previous intern's approach and how the new alerts avoid it

## Expected Behavior

1. Use `rate()` (not `count_over_time()`) in the high error rate alert expression
2. Apply the `or vector(0)` pattern to the high error rate expression to prevent "no data" gaps
3. Build the error ratio expression using `rate()` on both the numerator and denominator
4. Place line filters (e.g. `|= "error"`) before the json parser stage in both expressions
5. Set the high error rate threshold at `> 2` (per-second rate semantics, not a raw count)
6. Set the error ratio threshold at `0.05` (5% as a fraction, not 5)
7. Explain why `count_over_time()` breaks when the time range changes and how `rate()` avoids it
8. Clearly label which expression is the absolute rate alert and which is the ratio alert

## Success Criteria

- **rate() for absolute alert**: The high-error-rate alert expression uses `rate()` not `count_over_time()`
- **vector(0) fallback**: The high error rate expression uses `... or vector(0)` to ensure the metric evaluates to 0 when no errors are present
- **Error ratio uses rate()**: The error ratio expression divides `rate()` of errors by `rate()` of all events (not `count_over_time()` on either side)
- **Line filter before parser**: Both expressions place a line filter (e.g. `|= "error"`) before the json parser stage
- **Threshold set correctly**: The high error rate alert uses a threshold of `> 2` (not `> 100` or another count-based value) consistent with per-second rate semantics
- **Error ratio threshold 5%**: The error ratio expression compares against `0.05` (5%) not `5` or another non-fractional value
- **count_over_time failure explained**: The output document contains a sentence or note explaining that `count_over_time` breaks when the time range changes (raw count vs rate semantics)
- **Expressions labelled**: The output document clearly labels which expression is the absolute rate alert and which is the ratio alert

## Failure Conditions

- Uses `count_over_time()` instead of `rate()` in alert expressions
- Omits the `or vector(0)` fallback from the high error rate alert
- Builds the ratio expression using `count_over_time()` on either side of the division
- Places line filters after the json parser instead of before it
- Sets a count-based threshold (e.g. `> 100`) instead of a per-second rate threshold
- Compares the ratio against `5` instead of `0.05`
- Fails to explain the failure mode of raw `count_over_time()` alerting
- Omits labels distinguishing the two alert expressions
