# Scenario 04: Kubernetes Log Collector for Node.js Microservices

## User Prompt

A fast-growing e-commerce platform runs a suite of Node.js microservices on Kubernetes. The platform engineering team has experienced repeated incidents where the Fluent Bit log collector pod is OOM-killed during traffic spikes, causing logs to be silently dropped for several minutes at a time. The team suspects the collector is misconfigured and wants a reliable, production-grade Fluent Bit configuration that can survive high-volume log bursts without crashing and without collecting Fluent Bit's own logs back into the pipeline.

The services write structured JSON logs to stdout, which Kubernetes captures at `/var/log/containers/*.log`. The team wants to forward all container logs to a self-hosted Elasticsearch cluster (`elasticsearch.internal.svc:9200`) using TLS. The cluster is internal, not publicly accessible.
