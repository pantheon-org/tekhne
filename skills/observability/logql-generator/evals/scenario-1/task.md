# Payment Service Error Alerting

## Problem Description

A fintech startup processes card payments through a dedicated `payment-service` microservice. After several incidents where engineers only noticed elevated error rates after customers complained, the team has decided to proactively set up Grafana Loki alerting rules.

The payment-service logs are structured as JSON with a `level` field (`"error"`, `"warn"`, `"info"`) and a `transaction_type` field (`"charge"`, `"refund"`, `"void"`). Logs flow into Loki under labels `app="payment-service"` and `env="production"`.

The team wants two alerting expressions:

1. **High error rate alert** — fires when the absolute number of error-level log events per second exceeds a threshold of 2 errors/second, sustained over a 5-minute window.
2. **Error ratio alert** — fires when error-level events exceed 5% of all payment-service log events in the same window.

A previous intern wrote some alerts using raw event counts that kept misfiring after someone changed the dashboard time range. The team wants to make sure the new alerts won't have that problem. They also want the high error rate alert to never show "no data" in Grafana — it should evaluate to 0 when there are no errors.

## Output Specification

Produce a file named `alert_expressions.md` containing:

- Both LogQL alert expressions (clearly labelled)
- A brief explanation of why each expression is structured the way it is
- One sentence identifying the failure mode of the previous intern's approach and how the new alerts avoid it
