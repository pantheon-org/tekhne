# Scenario 03: Build a CloudWatch Metrics Graph URL

## User Prompt

Build a CloudWatch Metrics graph URL for the following requirements:

- Region: `us-east-1`
- Namespace: `AWS/Lambda`
- Metric: `Duration`
- Dimension: `FunctionName` = `checkout-service-prod`
- Time range: 2026-04-05 09:00:00 UTC to 2026-04-05 17:00:00 UTC
- Statistic: `Maximum`
- Period: 5 minutes

## Expected Behavior

1. Encode the `AWS/Lambda` namespace slash as `*2f` (producing `AWS*2fLambda`)
2. Encode colons in ISO start/end timestamps as `*3a`
3. Use `#metricsV2:graph=` as the URL fragment base
4. Set the period value to `300` (seconds) — not `5` (minutes)
5. Set `stat` to `Maximum`
6. Include `AWS*2fLambda`, `Duration`, `FunctionName`, and `checkout-service-prod` in the metrics tuple
7. Use `us-east-1` in both the URL subdomain and as a `region=` query parameter
8. Leave structural CloudWatch URL characters (`~`, `(`, `)`, `'`) unencoded

## Success Criteria

- **Namespace slash encoded as *2f**: AWS/Lambda namespace has the slash encoded as *2f (producing AWS*2fLambda)
- **ISO timestamp colons encoded as *3a**: Colons in start/end ISO timestamps are encoded as *3a
- **Correct metricsV2 fragment base**: URL fragment starts with #metricsV2:graph=
- **Period in seconds**: Period value is 300 (seconds), not 5 (minutes)
- **Stat set to Maximum**: stat parameter is set to Maximum
- **Namespace metric dimension value all present**: URL contains AWS*2fLambda, Duration, FunctionName, and checkout-service-prod in the metrics tuple
- **Region correct in both subdomain and query param**: us-east-1 appears in both the URL subdomain and region= query parameter
- **Structural characters unencoded**: Tilde ~ parentheses ( ) and single quote ' are left literal as structural CloudWatch URL characters

## Failure Conditions

- Namespace slash is not encoded as `*2f` (e.g., uses `%2F` or leaves `/` literal)
- ISO timestamp colons are not encoded as `*3a`
- URL fragment does not start with `#metricsV2:graph=`
- Period value is `5` (minutes) instead of `300` (seconds)
- `stat` is not set to `Maximum`
- Any of the four metric parameters (namespace, metric, dimension key, dimension value) is missing
- Region appears in only one location (subdomain or query string)
- Structural characters (`~`, `(`, `)`, `'`) are percent-encoded or asterisk-encoded
