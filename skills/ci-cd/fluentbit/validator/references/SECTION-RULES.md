# Fluent Bit Configuration Section Validation Rules

This reference documents detailed validation rules and best practices for each Fluent Bit configuration section.

## SERVICE Section

### Required Fields

- `Flush` - Required, must be numeric (recommended: 1-5 seconds)

### Valid Fields

- `Log_Level` - Must be one of: `off`, `error`, `warn`, `info`, `debug`, `trace`
- `Parsers_File` - File path must exist if specified
- `HTTP_Server` - Boolean: `On` or `Off`
- `HTTP_Listen` - IP address (default: 0.0.0.0)
- `HTTP_Port` - Port number (default: 2020)
- `storage.metrics` - Boolean: `on` or `off`
- `storage.path` - Directory path for buffering
- `storage.sync` - Sync mode: `normal` or `full`
- `storage.checksum` - Boolean: `off` or `on`

### Best Practices

- **Flush interval:** 1-5 seconds for optimal performance
- **HTTP Server:** Enable with `HTTP_Server On` for health checks and metrics
- **Storage metrics:** Enable with `storage.metrics on` for monitoring buffer usage
- **Log level:** Use `info` for production, `debug` only for troubleshooting

## INPUT Section

### Required Fields

- `Name` - Must be a valid plugin name (see Valid Plugins below)
- `Tag` - Must follow tag format (alphanumeric, dots, hyphens, wildcards)

### Valid Plugins

- `tail` - Read from log files
- `forward` - Receive from Fluent Bit/Fluentd
- `http` - HTTP endpoint for log ingestion
- `tcp` - TCP listener
- `syslog` - Syslog protocol
- `systemd` - Systemd journal
- `kubernetes_events` - Kubernetes event stream

### Plugin-Specific Validation

#### tail Plugin

- `Path` - File path must exist (supports wildcards)
- `Path_Key` - Optional field name for file path
- `Read_from_Head` - Boolean: `On` or `Off`
- `Refresh_Interval` - Numeric (seconds)
- `Rotate_Wait` - Numeric (seconds)
- `Skip_Long_Lines` - Boolean: `On` or `Off`
- `Skip_Empty_Lines` - Boolean: `On` or `Off`
- `DB` - Database file for position tracking
- `DB.locking` - Boolean: `true` or `false`
- `Mem_Buf_Limit` - Memory limit (e.g., `5MB`, `100MB`)
- `Parser` - Must reference existing parser

#### forward Plugin

- `Listen` - IP address (default: 0.0.0.0)
- `Port` - Port number (default: 24224)
- `Buffer_Size` - Buffer size with unit (e.g., `32KB`)

#### http Plugin

- `Listen` - IP address
- `Port` - Port number
- `tag_key` - Optional tag field name
- `successful_response_code` - HTTP response code (default: 200)

### Best Practices

- **Memory limits:** Always set `Mem_Buf_Limit` (50-100MB recommended) to prevent OOM
- **Position tracking:** Use `DB` parameter for tail inputs to survive restarts
- **Long lines:** Set `Skip_Long_Lines On` to prevent memory issues
- **File rotation:** Set `Refresh_Interval 10` for rotated log detection
- **Empty lines:** Set `Skip_Empty_Lines On` to reduce noise

## FILTER Section

### Required Fields

- `Name` - Must be a valid filter plugin name
- `Match` or `Match_Regex` - At least one required

### Valid Plugins

- `parser` - Parse unstructured logs
- `kubernetes` - Enrich with Kubernetes metadata
- `modify` - Add/remove/rename fields
- `nest` - Nest/lift nested fields
- `grep` - Include/exclude records
- `record_modifier` - Modify record fields
- `rewrite_tag` - Re-emit with new tag
- `throttle` - Rate limiting
- `multiline` - Combine multiline logs

### Common Fields

- `Match` - Tag pattern (supports wildcards)
- `Match_Regex` - Regex pattern for tag matching

### Plugin-Specific Validation

#### kubernetes Filter

- `Kube_URL` - Kubernetes API URL
- `Kube_CA_File` - CA certificate path (must exist)
- `Kube_Token_File` - Token file path (must exist)
- `Kube_Tag_Prefix` - Tag prefix (default: `kube.`)
- `Merge_Log` - Boolean: `On` or `Off`
- `Keep_Log` - Boolean: `On` or `Off`
- `Buffer_Size` - Recommend `0` (unbuffered) for performance

#### parser Filter

- `Key_Name` - Field to parse (required)
- `Parser` - Parser name (must exist)
- `Reserve_Data` - Boolean: `On` or `Off`
- `Preserve_Key` - Boolean: `On` or `Off`

### Best Practices

- **Match patterns:** Avoid bare `*` unless intentional (matches everything)
- **Filter ordering:** Place parsers before modifiers
- **Kubernetes:** Use `Buffer_Size 0` for better performance
- **Field preservation:** Use `Reserve_Data On` to keep original fields

## OUTPUT Section

### Required Fields

- `Name` - Must be a valid output plugin name
- `Match` - Tag pattern to match

### Valid Plugins

- `elasticsearch` - Elasticsearch/OpenSearch
- `opensearch` - OpenSearch (dedicated plugin)
- `kafka` - Apache Kafka
- `loki` - Grafana Loki
- `s3` - Amazon S3
- `cloudwatch` - Amazon CloudWatch Logs
- `http` - Generic HTTP endpoint
- `forward` - Forward to Fluent Bit/Fluentd
- `file` - Write to file
- `stdout` - Write to stdout
- `opentelemetry` - OpenTelemetry protocol
- `splunk` - Splunk HEC
- `datadog` - Datadog

### Common Fields

- `Match` - Tag pattern (required)
- `Retry_Limit` - Number of retries (recommended: 3-5)
- `storage.total_limit_size` - Total buffer size (e.g., `5G`)
- `tls` - Boolean: `on` or `off`
- `tls.verify` - Boolean: `on` or `off`
- `tls.ca_file` - CA certificate path
- `tls.crt_file` - Client certificate path
- `tls.key_file` - Client key path
- `Compress` - Compression algorithm: `gzip`

### Plugin-Specific Validation

#### elasticsearch/opensearch Plugin

- `Host` - Hostname or IP (required)
- `Port` - Port number (default: 9200)
- `Index` - Index name
- `Type` - Document type (deprecated in ES 7+)
- `Logstash_Format` - Boolean: `On` or `Off`
- `Logstash_Prefix` - Index prefix (default: `logstash`)
- `HTTP_User` - Username (use env var)
- `HTTP_Passwd` - Password (use env var)
- `Suppress_Type_Name` - Boolean: `On` or `Off` (required for ES 7+)

#### opentelemetry Plugin

- `Host` - Hostname or IP (required)
- `Port` - Port number
- `metrics_uri` - Metrics endpoint (e.g., `/v1/metrics`)
- `logs_uri` - Logs endpoint (e.g., `/v1/logs`)
- `traces_uri` - Traces endpoint (e.g., `/v1/traces`)
- `header` - Custom headers (can specify multiple)

#### kafka Plugin

- `Brokers` - Comma-separated broker list (required)
- `Topics` - Topic name (required)
- `Format` - Message format: `json`, `msgpack`, `gelf`
- `Message_Key` - Optional partition key field
- `rdkafka.*` - librdkafka configuration options

### Best Practices

- **Retry configuration:** Set `Retry_Limit 3-5` to handle transient failures
- **Storage limits:** Configure `storage.total_limit_size` to prevent disk exhaustion
- **TLS:** Enable TLS in production with `tls on` and `tls.verify on`
- **Credentials:** Use environment variables instead of hardcoded credentials
- **Compression:** Enable `Compress gzip` for network outputs to reduce bandwidth
- **Elasticsearch:** Set `Suppress_Type_Name On` for Elasticsearch 7.x and newer

## PARSER Section

### Required Fields

- `Name` - Parser name (unique identifier)
- `Format` - Parser format type (see Valid Formats)

### Valid Formats

- `json` - JSON parser
- `regex` - Regular expression parser
- `logfmt` - Logfmt parser
- `ltsv` - LTSV (Labeled Tab-Separated Values) parser

### Format-Specific Fields

#### json Format

- `Time_Key` - Field containing timestamp
- `Time_Format` - strptime format string
- `Time_Keep` - Boolean: `On` or `Off`

#### regex Format

- `Regex` - Regular expression with named capture groups (required)
- `Time_Key` - Field containing timestamp
- `Time_Format` - strptime format string
- `Time_Keep` - Boolean: `On` or `Off`
- `Types` - Type conversion (e.g., `status:integer`)

#### logfmt Format

No additional required fields, but supports:
- `Time_Key` - Field containing timestamp
- `Time_Format` - strptime format string

### Validation Rules

- **Regex syntax:** Must be valid POSIX ERE (Extended Regular Expression)
- **Time format:** `Time_Format` requires `Time_Key` to be set
- **Named groups:** Regex must use named capture groups: `(?<field_name>pattern)`

### Best Practices

- **Built-in parsers:** Use built-in parsers when possible (docker, apache, nginx, syslog)
- **Multiline logs:** Use `MULTILINE_PARSER` section for stack traces and multiline formats
- **Type conversion:** Use `Types` parameter to convert strings to integers/floats
- **Time parsing:** Always set `Time_Key` and `Time_Format` for proper timestamp handling
- **Testing:** Test regex patterns with sample logs before deployment

## Common Validation Errors

### Structure Errors

- **Missing section header:** `[SECTION_NAME]` required before key-value pairs
- **Invalid key-value format:** Must be `Key Value` (space-separated)
- **Unclosed brackets:** Check for matching `[` and `]`
- **Mixed indentation:** Use consistent spaces (not tabs)

### Section Errors

- **Invalid plugin name:** Check spelling against valid plugin lists above
- **Missing required field:** Each section type has required fields
- **Invalid field value:** Check against allowed values for each field

### Tag Errors

- **Unmatched tags:** FILTER/OUTPUT Match patterns must correspond to INPUT tags
- **Orphaned filters:** FILTER without matching INPUT or OUTPUT
- **Wildcard issues:** Ensure wildcard patterns match intended tags

### Security Errors

- **Hardcoded credentials:** Never hardcode passwords, API keys, or tokens
- **TLS disabled:** Always enable TLS in production
- **Insecure verify:** Avoid `tls.verify Off` in production
- **World-readable secrets:** Check file permissions on DB files and certificates

### Performance Issues

- **Missing memory limits:** Always set `Mem_Buf_Limit` on tail inputs
- **No storage limits:** Set `storage.total_limit_size` on outputs
- **Long flush intervals:** Keep Flush at 1-5 seconds
- **No compression:** Enable compression on network outputs
