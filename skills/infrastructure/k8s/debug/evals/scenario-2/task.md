# Performance Degradation Investigation

## Problem Description

A critical application pod with multiple containers (main app, sidecar proxy, and logging agent) is experiencing severe performance degradation. The pod is consuming excessive CPU and memory, causing other pods on the same node to be throttled. The platform team needs to identify which specific container is responsible for the resource spike.

Your task is to create a diagnostic script and analysis report that identifies the resource-hungry container and documents the investigation approach for future reference.

## Output Specification

Create these files:

1. `resource-analysis.sh` - A bash script that demonstrates how to identify resource consumption at the container level within a multi-container pod. Include commands that reveal per-container metrics.

2. `performance-report.md` - A report documenting:
   - The methodology for diagnosing resource issues in multi-container pods
   - Key commands for container-level resource visibility
   - The specific container causing the issue (based on the manifest provided)
   - Recommended next steps

## Input Files

The following manifest represents the problematic pod. Extract it before beginning.

=============== FILE: inputs/app-pod.yaml ===============
apiVersion: v1
kind: Pod
metadata:
  name: web-app
  namespace: production
  labels:
    app: web
spec:
  containers:
- name: web-server
    image: nginx:1.21
    resources:
      requests:
        memory: "64Mi"
        cpu: "100m"
      limits:
        memory: "128Mi"
        cpu: "200m"
- name: istio-proxy
    image: istio/proxyv2:1.16.0
    resources:
      requests:
        memory: "128Mi"
        cpu: "250m"
      limits:
        memory: "256Mi"
        cpu: "500m"
- name: log-collector
    image: fluentd:v1.15
    resources:
      requests:
        memory: "64Mi"
        cpu: "100m"
      limits:
        memory: "128Mi"
        cpu: "200m"
=============== END FILE ===============
