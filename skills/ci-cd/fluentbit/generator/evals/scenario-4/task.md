# High-Reliability Log Forwarding to Kafka

## Problem/Feature Description

A data engineering team at a logistics company forwards container logs to Kafka for downstream stream processing. The team has experienced silent data loss during Kafka maintenance windows: when Kafka brokers are temporarily unavailable, Fluent Bit keeps buffering logs until it runs out of disk space, then drops all buffered logs without alerting anyone. Operations has now mandated that the configuration must be hardened for reliability: retries must be bounded, disk buffers must be capped, and the Fluent Bit pod must be health-checkable by the Kubernetes liveness probe.

The logs come from `/var/log/containers/*.log` on a Kubernetes cluster. The Kafka brokers are `kafka-1.data.svc:9092` and `kafka-2.data.svc:9092`. The target topic is `platform-logs`.

## Output Specification

Produce a `fluent-bit.conf` ready for Kubernetes DaemonSet deployment. Include a `reliability-notes.md` explaining how the configuration addresses the bounded retry and buffer exhaustion concerns, and how the health check endpoint can be used with a Kubernetes liveness probe.
