# Common OUTPUT Patterns

Loki, S3, Kafka, CloudWatch Logs, OpenTelemetry (OTLP/HTTP), HTTP and stdout. Credentials use `${ENV_VAR}` syntax; set `tls.verify On` in production.

```ini
# Grafana Loki
[OUTPUT]
    Name              loki
    Match             *
    Host              loki.default.svc
    Port              3100
    labels            job=fluent-bit, namespace=$kubernetes['namespace_name'], pod=$kubernetes['pod_name']
    label_keys        $stream
    remove_keys       kubernetes,stream
    auto_kubernetes_labels on
    line_format       json
    Retry_Limit       3

# AWS S3
[OUTPUT]
    Name              s3
    Match             *
    bucket            my-logs-bucket
    region            us-east-1
    total_file_size   100M
    upload_timeout    10m
    compression       gzip
    s3_key_format     /fluent-bit-logs/%Y/%m/%d/$TAG[0]/%H-%M-%S-$UUID.gz
    Retry_Limit       3

# Kafka
[OUTPUT]
    Name              kafka
    Match             *
    Brokers           kafka-broker-1:9092,kafka-broker-2:9092
    Topics            logs
    Format            json
    Timestamp_Key     @timestamp
    Retry_Limit       3

# AWS CloudWatch Logs
[OUTPUT]
    Name              cloudwatch_logs
    Match             *
    region            us-east-1
    log_group_name    /aws/fluent-bit/logs
    log_stream_prefix from-fluent-bit-
    auto_create_group On
    Retry_Limit       3

# OpenTelemetry (OTLP/HTTP)
[OUTPUT]
    Name              opentelemetry
    Match             *
    Host              opentelemetry-collector.observability.svc
    Port              4318
    logs_uri          /v1/logs
    add_label         cluster my-cluster
    add_label         environment production
    tls               On
    tls.verify        On
    Retry_Limit       3

# HTTP endpoint
[OUTPUT]
    Name              http
    Match             *
    Host              logs.example.com
    Port              443
    URI               /api/logs
    Format            json
    tls               On
    tls.verify        On
    Header            Authorization Bearer ${API_TOKEN}
    Compress          gzip
    Retry_Limit       3

# stdout (debug only)
[OUTPUT]
    Name    stdout
    Match   *
    Format  json_lines
```
