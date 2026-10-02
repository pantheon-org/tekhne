# Scenario 1: Rewrite an Incident Summary for Executives

## User Prompt

You are asked to rewrite the following incident post-mortem excerpt for the CEO and CFO, who have no technical background. They need to decide whether to approve an emergency budget for infrastructure improvements.

Here is the input text:

```
At 14:32 UTC, a misconfigured Nginx reverse proxy caused a cascading failure
in the Kubernetes pod autoscaler, resulting in a 503 surge across all API
endpoints. The root cause was traced to an incorrect upstream timeout value
(60s vs 5s expected) introduced in the 2.4.1 rollout. MTTR was 47 minutes.
Remediation required patching the ConfigMap and rolling restart of the
ingress controller. To prevent recurrence, we recommend implementing
admission webhooks and enforcing IaC policy gates via OPA Gatekeeper.
```

Rewrite this incident summary for the CEO and CFO so they can decide on the emergency budget request. Follow the plain-english workflow.
