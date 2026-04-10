# Scenario 01: Build a CloudWatch Logs Insights Deep-Link URL

## User Prompt

A developer needs to share a reproducible link to a CloudWatch Logs Insights query so a colleague can open it directly in the browser without navigating manually.

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

## Expected Behavior

1. Use `*XX` encoding (asterisk prefix) for all special characters in fragment values — not `%XX` percent encoding
2. Use Unix epoch seconds (10-digit numbers) for `start` and `end` time values
3. Include `eu-west-1` in both the URL subdomain and as a `region=` query parameter
4. Encode `?` as `$3F` and `=` as `$3D` between path segments after the `#` fragment
5. Encode pipe characters (`|`) in the query as `*7c`
6. Encode forward slashes in query text and log group name as `*2f`
7. Encode newlines between query clauses as `*0a`
8. Encode `@` symbols (in `@timestamp`, `@message`) as `*40`
9. Leave structural CloudWatch URL characters (`~`, `(`, `)`, `'`) unencoded
10. Start the URL fragment with `#logsV2:logs-insights`
11. Set `timeType~'ABSOLUTE` to use explicit start/end times
12. Include `unit~'seconds`

## Success Criteria

- **Uses *XX not %XX for value encoding**: All encoded characters in fragment values use asterisk prefix (e.g. *2f, *7c) not percent prefix (%2f, %7c)
- **Epoch timestamps in seconds**: start and end values are Unix epoch seconds (10-digit numbers), not milliseconds (13-digit)
- **Region in both URL subdomain and query string**: URL contains eu-west-1 in the subdomain AND as region= query parameter
- **Path segment separators encoded correctly**: ? encoded as $3F and = encoded as $3D between path segments after the # fragment
- **Pipe character encoded as *7c**: The | characters in the query are encoded as *7c
- **Slash encoded as *2f in query and log group**: Forward slashes in query text and log group name are encoded as *2f
- **Newlines encoded as *0a**: Line breaks between query clauses are encoded as *0a
- **@ symbol encoded as *40**: The @ in @timestamp and @message is encoded as *40
- **Structural characters not encoded**: Characters ~ ( ) ' used as CloudWatch URL structural delimiters are left literal, not encoded
- **Correct base path logsV2:logs-insights**: URL fragment starts with #logsV2:logs-insights
- **timeType set to ABSOLUTE**: URL includes timeType~'ABSOLUTE to use explicit start/end times
- **unit set to seconds**: URL includes unit~'seconds

## Failure Conditions

- Uses `%XX` percent encoding instead of `*XX` asterisk encoding for fragment values
- Uses millisecond (13-digit) timestamps instead of second (10-digit) epoch values
- Region appears in only one location (subdomain or query string, not both)
- `?` and `=` path segment separators are not encoded as `$3F` and `$3D`
- Pipe characters are not encoded as `*7c`
- Slashes are not encoded as `*2f`
- Newlines between query clauses are not encoded as `*0a`
- `@` symbols are not encoded as `*40`
- Structural characters (`~`, `(`, `)`, `'`) are encoded when they should be literal
- URL fragment does not start with `#logsV2:logs-insights`
- `timeType` is not set to `ABSOLUTE`
- `unit~'seconds` is missing from the URL
