# Scenario 02: Investigate a Triggered CloudWatch Alarm

## User Prompt

An OpsGenie alert fired for alarm `bip-laas-api-facade-pr-PlayerLambdaFunctionErrorAlarmC578E080-LyOUE2A6wiS2` in region `eu-west-1`. You need to capture a screenshot of the metric graph that caused the alarm to fire.

Describe where to navigate and produce the direct URL to the alarm detail page.

Note: The Lambda function is named `bip-laas-api-facade-pr-PlayerLambdaFunction786A3E0-2j0cL2524eAR`.

## Expected Behavior

1. Navigate to the CloudWatch alarm detail page — not to Metrics or Logs directly
2. Construct a URL using the `#alarmsV2:alarm/{alarm_name}` fragment pattern
3. Include `eu-west-1` in both the URL subdomain and as a `region=` query parameter
4. Place the full alarm name in the URL without extra encoding of hyphens
5. Warn that the alarm may use a Logs metric filter rather than the native `AWS/Lambda` Errors metric
6. Not instruct navigating to Metrics > AWS/Lambda > Errors as the primary evidence source
7. State that the alarm detail page shows which metric triggered the alarm

## Success Criteria

- **Navigates to alarm detail page**: Response directs to the CloudWatch alarm detail page, not to Metrics or Logs directly
- **Correct alarm URL pattern**: URL uses the #alarmsV2:alarm/{alarm_name} fragment pattern
- **Region in URL**: URL includes eu-west-1 in both subdomain and region= query parameter
- **Alarm name correctly placed**: The full alarm name appears in the URL without extra encoding of hyphens
- **Warns about metric filter vs native metric**: Response notes that the alarm may use a Logs metric filter rather than AWS/Lambda Errors
- **Does NOT recommend navigating to AWS/Lambda Errors metric directly**: Response does not instruct navigating to Metrics > AWS/Lambda > Errors as primary evidence
- **Mentions alarm Metric section as source of truth**: Response states the alarm detail page shows which metric triggered the alarm

## Failure Conditions

- Navigates to CloudWatch Metrics or Logs instead of the alarm detail page
- URL does not use the `#alarmsV2:alarm/{alarm_name}` fragment pattern
- Region appears in only one location in the URL
- Alarm name is incorrectly encoded or modified
- Does not warn about the metric filter vs native metric distinction
- Recommends navigating to `Metrics > AWS/Lambda > Errors` as the primary investigation step
- Does not mention that the alarm detail page is the authoritative source for which metric triggered the alarm
