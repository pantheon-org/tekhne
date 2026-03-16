---
name: k8s-debug
description: Inspect pod logs, analyze resource quotas, trace network policies, check deployment rollout status, and run cluster health checks for Kubernetes. Use this skill when diagnosing Kubernetes cluster issues, debugging failing pods, investigating network connectivity problems, analyzing resource usage, troubleshooting deployments, or performing cluster health checks.
---

# Kubernetes Debugging Skill

## Debugging Mindset

**Mental Model**: Kubernetes debugging follows a layered approach—start broad (cluster health), narrow to affected components (pods, services), then drill into specific failures (logs, events, resource constraints).

**Decision Framework**:
1. **Gather context** before jumping to solutions. Check `kubectl get events` and `describe` resources first.
2. **Verify assumptions** about selectors, labels, and namespaces—label mismatches are the most common root cause.
3. **Test hypotheses** systematically: network → resource → configuration → application.
4. **Document findings** as you go—Kubernetes issues often involve multiple interacting failures.

**When to use this skill**:
- Pods stuck in Pending, CrashLoopBackOff, ImagePullBackOff, or Error states
- Services not routing traffic despite healthy pods
- Resource exhaustion (OOMKilled, CPU throttling)
- Deployments failing to roll out or stuck in progress
- Network policies blocking expected traffic

## Quick Diagnostic Patterns

### Pod Not Starting

```bash
# Quick assessment
kubectl get pod <pod-name> -n <namespace>
kubectl describe pod <pod-name> -n <namespace>  # Events section is key

# Detailed diagnostics
python3 scripts/pod_diagnostics.py <pod-name> -n <namespace>

# Check previous logs if CrashLoopBackOff
kubectl logs <pod-name> -n <namespace> --previous
```

### Service Connectivity Issues

```bash
# Verify service endpoints match pod IPs
kubectl get svc <service-name> -n <namespace>
kubectl get endpoints <service-name> -n <namespace>

# Network diagnostics
./scripts/network_debug.sh <namespace> <pod-name>

# Test from debug pod
kubectl run tmp-shell --rm -i --tty --image nicolaka/netshoot -- /bin/bash
```

### Performance Degradation

```bash
# Resource usage by container
kubectl top pods -n <namespace> --containers

# Check for OOMKilled
kubectl get pod <pod-name> -n <namespace> -o yaml | grep -A 10 lastState

# Review recent logs
kubectl logs <pod-name> -n <namespace> --tail=100 --timestamps
```

### Cluster Health Check

```bash
# Comprehensive health report
./scripts/cluster_health.sh > cluster-health-$(date +%Y%m%d-%H%M%S).txt
```

## Key Debugging Commands

Focus on non-obvious flags and patterns most useful during debugging:

### Pod Debugging
```bash
# Cross-namespace pod overview
kubectl get pods -A -o wide --field-selector=status.phase!=Running

# Previous container logs (post-crash)
kubectl logs <pod-name> -n <namespace> --previous

# Multi-container pod: target specific container
kubectl logs <pod-name> -n <namespace> -c <container>

# Stream logs with timestamps
kubectl logs <pod-name> -n <namespace> -f --timestamps

# Describe for Events section — most useful first stop
kubectl describe pod <pod-name> -n <namespace>

# Full pod YAML including status conditions
kubectl get pod <pod-name> -n <namespace> -o yaml
```

### Service and Network Debugging
```bash
# Confirm endpoint IPs match running pod IPs (label selector mismatch shows empty)
kubectl get endpoints <service-name> -n <namespace>

# Test DNS from within the cluster
kubectl exec <pod-name> -n <namespace> -- nslookup kubernetes.default

# Sort events by time to find recent failures
kubectl get events -n <namespace> --sort-by='.lastTimestamp'
```

### Resource Monitoring
```bash
# Per-container resource usage (reveals which container is the culprit)
kubectl top pod <pod-name> -n <namespace> --containers

# Resource quota consumption vs. limits
kubectl describe resourcequota -n <namespace>
```

## Emergency Operations

> ⚠️ These commands are destructive or disruptive. Follow the verification steps before and after each operation.

### Restart Deployment
```bash
# Verify current rollout state first
kubectl rollout status deployment/<name> -n <namespace>

# Restart
kubectl rollout restart deployment/<name> -n <namespace>

# Verify rollout completes successfully
kubectl rollout status deployment/<name> -n <namespace> --timeout=120s
```

### Rollback Deployment
```bash
# Check rollout history to pick the correct revision
kubectl rollout history deployment/<name> -n <namespace>

# Rollback to previous revision
kubectl rollout undo deployment/<name> -n <namespace>

# Confirm rollback success and pods are running
kubectl rollout status deployment/<name> -n <namespace>
kubectl get pods -n <namespace> -l app=<name>
```

### Force Delete Stuck Pod
```bash
# Confirm the pod is genuinely stuck (not just slow to terminate)
kubectl get pod <pod-name> -n <namespace> -w   # Watch for 60s before proceeding

# Force delete only if pod remains Terminating with no progress
kubectl delete pod <pod-name> -n <namespace> --force --grace-period=0

# Verify the pod is gone and not rescheduled with an error state
kubectl get pod <pod-name> -n <namespace>
```

### Drain Node (Maintenance)
```bash
# Review what will be evicted before draining
kubectl get pods --all-namespaces --field-selector spec.nodeName=<node-name>

# Cordon first to prevent new scheduling
kubectl cordon <node-name>

# Drain (evicts pods gracefully)
kubectl drain <node-name> --ignore-daemonsets --delete-emptydir-data

# Verify node is drained and no workloads remain
kubectl get pods --all-namespaces --field-selector spec.nodeName=<node-name>

# After maintenance, uncordon to restore scheduling
kubectl uncordon <node-name>
kubectl get node <node-name>   # Confirm status returns to Ready
```

## Common Anti-Patterns

### NEVER: Jump to Solutions Without Context

**BAD**:
```bash
# Immediately restarting without investigation
kubectl rollout restart deployment/app
```

**GOOD**:
```bash
# Gather diagnostic context first
kubectl describe deployment/app
kubectl get events --sort-by='.lastTimestamp' | tail -20
kubectl logs -l app=app --tail=50
# THEN decide if restart is appropriate
```

### NEVER: Ignore Namespace Context

**BAD**:
```bash
# Assuming default namespace
kubectl get pods
kubectl logs my-pod
```

**GOOD**:
```bash
# Always specify namespace explicitly
kubectl get pods -n production
kubectl logs my-pod -n production
# Or use -A to search all namespaces
kubectl get pods -A | grep my-pod
```

### NEVER: Force Delete as First Resort

**BAD**:
```bash
# Immediate force delete
kubectl delete pod stuck-pod --force --grace-period=0
```

**GOOD**:
```bash
# Investigate why pod is stuck first
kubectl describe pod stuck-pod
kubectl get pod stuck-pod -o yaml | grep -A 10 finalizers
# Try normal delete first
kubectl delete pod stuck-pod
# Wait 60s, watch for termination
# Force delete only if confirmed stuck
```

### NEVER: Debug in Production Directly

**BAD**:
```bash
# Exec into production pod and make changes
kubectl exec -it prod-pod -- /bin/bash
```

**GOOD**:
```bash
# Create debug copy for investigation
kubectl debug prod-pod --copy-to=debug-pod --share-processes
# Or use ephemeral debug container (K8s 1.23+)
kubectl debug prod-pod -it --image=nicolaka/netshoot
```

### ALWAYS: Check Labels and Selectors First

Service connectivity issues are almost always label mismatches:

```bash
# Verify service selector matches pod labels
kubectl get svc my-service -o jsonpath='{.spec.selector}'
kubectl get pods -l app=my-app --show-labels
kubectl get endpoints my-service  # Should show pod IPs
```

### ALWAYS: Use --previous Flag for CrashLoopBackOff

```bash
# Current logs may be empty if container crashed immediately
kubectl logs failing-pod --previous
```

## Advanced Debugging Techniques

### Debug Containers (Kubernetes 1.23+)
```bash
# Attach ephemeral debug container
kubectl debug <pod-name> -n <namespace> -it --image=nicolaka/netshoot

# Create debug copy of pod
kubectl debug <pod-name> -n <namespace> -it --copy-to=<debug-pod-name> --container=<container>
```

### Port Forwarding for Testing
```bash
# Forward pod port to local machine
kubectl port-forward pod/<pod-name> -n <namespace> <local-port>:<pod-port>

# Forward service port
kubectl port-forward svc/<service-name> -n <namespace> <local-port>:<service-port>
```

### Proxy for API Access
```bash
# Start kubectl proxy
kubectl proxy --port=8080

# Access API
curl http://localhost:8080/api/v1/namespaces/<namespace>/pods/<pod-name>
```

### Custom Column Output
```bash
# Custom pod info
kubectl get pods -o custom-columns=NAME:.metadata.name,STATUS:.status.phase,IP:.status.podIP

# Node taints
kubectl get nodes -o custom-columns=NAME:.metadata.name,TAINTS:.spec.taints
```

## Verification and Validation

After any debugging intervention:

1. **Confirm the fix**: Don't assume—verify pods are Running and Ready
   ```bash
   kubectl get pods -n <namespace> -w
   kubectl rollout status deployment/<name>
   ```

2. **Check for side effects**: Ensure fix didn't break other components
   ```bash
   kubectl get events --sort-by='.lastTimestamp' | tail -20
   ```

3. **Test functionality**: Validate the application works end-to-end
   ```bash
   kubectl port-forward svc/<service> 8080:80
   curl http://localhost:8080/healthz
   ```

4. **Document the root cause**: Add annotations to resources for future reference
   ```bash
   kubectl annotate deployment/<name> debug.issue="ImagePullBackOff due to missing secret"
   ```

## Anti-Patterns

### NEVER exec into a running production pod before checking logs

- **WHY**: Interactive exec changes the live state and may affect running workloads; logs and describe output are safe to read without impact and usually contain the root cause.
- **BAD**: Immediately running `kubectl exec -it pod-name -- bash` on a production pod for any issue.
- **GOOD**: Start with `kubectl logs pod-name --previous` and `kubectl describe pod pod-name` to gather context before considering exec.

### NEVER delete and recreate pods to fix issues without understanding the cause

- **WHY**: Blindly recycling pods hides the root cause and may leave a resource in a broken state for the next incident.
- **BAD**: `kubectl delete pod pod-name` as a first troubleshooting step.
- **GOOD**: Diagnose with logs and events first; identify whether the issue is a crash loop, OOM kill, config error, or node failure before taking corrective action.

### NEVER use `kubectl get pods` without `-n` or `--all-namespaces` in multi-tenant clusters

- **WHY**: Missing pods often means you are looking in the wrong namespace; always specify scope explicitly to avoid false negatives.
- **BAD**: `kubectl get pods` returning "No resources found" and assuming the workload does not exist.
- **GOOD**: `kubectl get pods -n <namespace>` or `kubectl get pods -A | grep <app>`

### NEVER port-forward to a single pod as a proxy for full service testing

- **WHY**: Port-forwarding targets one pod; issues may be pod-specific or service-routing-specific and require testing both paths independently.
- **BAD**: Testing all application behavior via `kubectl port-forward pod/name 8080:80` and assuming the result represents the service.
- **GOOD**: Use pod port-forward to isolate pod-specific behavior; use `kubectl port-forward svc/<name>` to test service routing separately.

## References

### Detailed Troubleshooting Guides

See [references/troubleshooting_workflow.md](references/troubleshooting_workflow.md) for:
- Step-by-step workflows for each issue type
- Decision trees for diagnosis
- Command sequences for systematic debugging
- Quick reference command cheat sheet

### Common Issues Database

See [references/common_issues.md](references/common_issues.md) for:
- Detailed explanations of each common issue (ImagePullBackOff, CrashLoopBackOff, OOMKilled, etc.)
- Symptoms and root causes
- Specific debugging steps
- Solutions and fixes
- Prevention strategies
