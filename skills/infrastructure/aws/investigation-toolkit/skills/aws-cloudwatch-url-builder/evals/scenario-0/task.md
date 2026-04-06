# Build a CloudWatch Logs Insights Deep-Link URL

## Task

A developer needs to share a reproducible link to a CloudWatch Logs Insights query so a
colleague can open it directly in the browser without navigating manually.

**Requirements:**

- Region: `eu-west-1`
- Log group: `/aws/lambda/my-service-prod-Handler`
- Time range: 2026-04-05 14:00:00 UTC to 2026-04-05 15:00:00 UTC
- Query:
  ```
  fields @timestamp, @message
  | filter @message like /(?i)(error|exception)/
  | sort @timestamp asc
  | limit 50
  ```

Produce the complete CloudWatch Logs Insights URL for this query.
