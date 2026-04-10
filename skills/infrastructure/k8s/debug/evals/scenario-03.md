# Scenario 03: Multi-Container Resource Analysis

## User Prompt

A critical application pod with multiple containers (main app, sidecar proxy, and logging agent) is experiencing severe performance degradation. The pod is consuming excessive CPU and memory, causing other pods on the same node to be throttled. The platform team needs to identify which specific container is responsible for the resource spike.

Your task is to create a diagnostic script and analysis report that identifies the resource-hungry container and documents the investigation approach for future reference.

The following manifest represents the problematic pod:

```yaml
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
```

Create these files:

1. `resource-analysis.sh` — A bash script that demonstrates how to identify resource consumption at the container level within a multi-container pod. Include commands that reveal per-container metrics.

2. `performance-report.md` — A report documenting:
   - The methodology for diagnosing resource issues in multi-container pods
   - Key commands for container-level resource visibility
   - The specific container causing the issue (based on the manifest provided)
   - Recommended next steps

## Expected Behavior

1. Use `kubectl top pod` with the `--containers` flag to show per-container resource usage
2. Acknowledge in the documentation that the pod has multiple containers and each needs separate analysis
3. Check resource requests and limits for each container in the manifest
4. Check for OOMKilled status or memory-related termination in pod status/events
5. Use `kubectl logs -c` flag to target specific containers by name
6. Include `-n production` namespace flag on all kubectl commands
7. Document checking: overall pod resources → per-container breakdown → identify culprit
8. Mention or check `kubectl describe resourcequota` to understand namespace-level constraints
9. Identify which specific container (by name) is causing the resource spike
10. Use `--timestamps` or similar flags to show when resource issues began

## Success Criteria

- **Container-level metrics**: Uses `kubectl top pod` with `--containers` flag to show per-container resource usage
- **Multi-container context**: Documentation acknowledges pod has multiple containers and each needs separate analysis
- **Resource limits checked**: Script or report checks resource requests and limits for each container in the manifest
- **OOM investigation**: Checks for OOMKilled status or memory-related termination in pod status/events
- **Container-specific logs**: Uses `kubectl logs -c` flag to target specific containers by name
- **Namespace specified**: All kubectl commands include `-n production` namespace flag
- **Systematic approach**: Report documents checking: overall pod resources → per-container breakdown → identify culprit
- **Resource quota context**: Mentions or checks `kubectl describe resourcequota` to understand namespace-level constraints
- **Specific container ID**: Report identifies which specific container (by name) is causing the resource spike
- **Timestamps included**: Uses `--timestamps` or similar flags to show when resource issues began

## Failure Conditions

- `kubectl top pod --containers` is not used (per-container metrics not obtained)
- Documentation does not acknowledge the multi-container nature of the pod
- Resource requests and limits for individual containers are not checked
- OOMKilled or memory-related termination is not investigated
- `kubectl logs -c <container-name>` is not used to get container-specific logs
- No specific container is identified by name as the resource culprit
