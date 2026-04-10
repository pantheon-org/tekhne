# Scenario 01: Rewrite an Incident Summary for Executives

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

## Expected Behavior

1. Identify or acknowledge the audience as executive (CEO/CFO) before or at the start of the rewrite
2. Open with what happened, the business impact, and what decision or action is needed — not background context
3. Replace all technical terms (Nginx, Kubernetes, ConfigMap, OPA Gatekeeper, IaC) with plain-language equivalents or remove them entirely
4. Define any acronyms retained in the rewrite (e.g., API, UTC, MTTR) inline on their first occurrence
5. Surface the budget request or recommendation near the beginning, not buried after multiple paragraphs of technical background

## Success Criteria

- **Audience explicitly identified**: Response identifies or acknowledges the audience as executive (CEO/CFO) before or at the start of the rewrite.
- **Key message leads the opening paragraph**: Opening paragraph states what happened, the business impact, and what decision or action is needed — not background context.
- **Technical jargon translated or removed**: All technical terms (Nginx, Kubernetes, ConfigMap, OPA Gatekeeper, IaC) are replaced with plain-language equivalents or removed entirely.
- **Acronyms defined on first use**: Any acronyms retained in the rewrite (e.g., API, UTC, MTTR) are defined inline on their first occurrence.
- **Decision not buried**: The budget request or recommendation is not buried after multiple paragraphs of technical background.

## Failure Conditions

- Audience is not identified or acknowledged before the rewrite
- Opening paragraph starts with technical background rather than business impact and the decision needed
- Technical terms like Nginx, Kubernetes, ConfigMap, or OPA Gatekeeper appear without translation
- Acronyms (API, UTC, MTTR) are used without inline definitions on first use
- Budget request or recommendation appears only at the end after extensive technical explanation
