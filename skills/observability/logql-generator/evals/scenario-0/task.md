# Nginx Access Log Analysis

## Problem Description

An e-commerce platform runs Nginx as its edge reverse proxy in Kubernetes. The platform is intermittently seeing slow response times, and the SRE team wants to investigate whether specific upstream services are causing 5xx errors. The Nginx instances emit access logs in a custom semi-structured format:

```
10.1.2.3 - alice [01/Mar/2026:10:15:00 +0000] "GET /api/products?page=2 HTTP/1.1" 502 1024 "https://shop.example.com" "Mozilla/5.0..."
```

The logs are collected into Grafana Loki with the following labels available on the streams: `job="nginx-access"`, `namespace="prod"`, `cluster="us-east"`.

The SRE team needs queries to help with two things:

1. **Filter to 5xx errors only** — return raw log lines so engineers can read them and spot patterns (endpoint, upstream, source IP).
2. **Count the rate of 5xx responses over time** — a metric query that can be graphed in a Grafana dashboard panel.

For the rate query, the team cares about separating counts by HTTP method (GET, POST, etc.) so they can tell whether writes or reads are failing more.

## Output Specification

Produce a Markdown file named `queries.md` that contains:

- The two LogQL queries (clearly labelled as the log filter query and the metric query)
- A brief explanation of each query and how to interpret the results
- Notes on which labels or values to change to adapt the queries to different environments or time windows

Do not include setup or installation instructions.
