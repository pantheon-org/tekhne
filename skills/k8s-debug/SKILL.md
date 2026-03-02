---
name: k8s-debug
description: Inspect pod logs, analyze resource quotas, trace network policies, check deployment rollout status, and run cluster health checks for Kubernetes. Use this skill when diagnosing Kubernetes cluster issues, debugging failing pods, investigating network connectivity problems, analyzing resource usage, troubleshooting deployments, or performing cluster health checks.
---

# Kubernetes Debugging Skill

## Overview

Systematic toolkit for debugging and troubleshooting Kubernetes clusters, pods, services, and deployments. Provides scripts, workflows, and reference guides for identifying and resolving common Kubernetes issues efficiently.

## Debugging Workflow

Follow this systematic approach for any Kubernetes issue:

### 1. Identify the Problem Layer

Categorize the issue at the appropriate layer: Application, Pod, Service, Node, Cluster, Storage, or Configuration.

### 2. Gather Diagnostic Information

Use the appropriate diagnostic script based on scope:

#### Pod-Level Diagnostics
```bash
python3 scripts/pod_diagnostics.py <pod-name> -n <namespace>
# Save output: python3 scripts/pod_diagnostics.py <pod-name> -n <namespace> -o diagnostics.txt
```

#### Cluster-Level Health Check
```bash
./scripts/cluster_health.sh
```

#### Network Diagnostics
```bash
./scripts/network_debug.sh <namespace> <pod-name>
```

### 3. Follow Issue-Specific Workflow

Based on the identified issue, consult `references/troubleshooting_workflow.md` for detailed workflows:

- **Pod Pending**: Resource/scheduling workflow
- **CrashLoopBackOff**: Application crash workflow
- **ImagePullBackOff**: Image pull workflow
- **Service issues**: Network connectivity workflow
- **DNS failures**: DNS troubleshooting workflow
- **Resource exhaustion**: Performance investigation workflow
- **Storage issues**: PVC binding workflow
- **Deployment stuck**: Rollout workflow

### 4. Apply Targeted Fixes

Refer to `references/common_issues.md` for specific solutions to common problems.

## Common Debugging Patterns

### Pattern 1: Pod Not Starting

```bash
# Quick assessment
kubectl get pod <pod-name> -n <namespace>
kubectl describe pod <pod-name> -n <namespace>

# Detailed diagnostics
python3 scripts/pod_diagnostics.py <pod-name> -n <namespace>

# Check common causes:
# - ImagePullBackOff: Verify image exists and credentials
# - CrashLoopBackOff: Check logs with --previous flag
# - Pending: Check node resources and scheduling
```

### Pattern 2: Service Connectivity Issues

```bash
# Verify service and endpoints
kubectl get svc <service-name> -n <namespace>
kubectl get endpoints <service-name> -n <namespace>

# Network diagnostics
./scripts/network_debug.sh <namespace> <pod-name>

# Test connectivity from debug pod
kubectl run tmp-shell --rm -i --tty --image nicolaka/netshoot -- /bin/bash
# Inside: curl <service-name>.<namespace>.svc.cluster.local:<port>

# Check network policies
kubectl get networkpolicies -n <namespace>
```

### Pattern 3: Application Performance Issues

```bash
# Check resource usage
kubectl top nodes
kubectl top pods -n <namespace> --containers

# Get pod resources
kubectl get pod <pod-name> -n <namespace> -o yaml | grep -A 10 resources

# Check for OOMKilled
kubectl get pod <pod-name> -n <namespace> -o yaml | grep -A 10 lastState

# Review application logs
kubectl logs <pod-name> -n <namespace> --tail=100
```

### Pattern 4: Cluster Health Assessment

```bash
# Run comprehensive health check
./scripts/cluster_health.sh > cluster-health-$(date +%Y%m%d-%H%M%S).txt

# Review output for:
# - Node conditions and resource pressure
# - Failed or pending pods
# - Recent error events
# - Component health status
# - Resource quota usage
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

## Troubleshooting Checklist

Before escalating issues, verify:

- [ ] Reviewed pod events: `kubectl describe pod`
- [ ] Checked pod logs (current and previous)
- [ ] Verified resource availability on nodes
- [ ] Confirmed image exists and is accessible
- [ ] Validated service selectors match pod labels
- [ ] Tested DNS resolution from pods
- [ ] Checked network policies
- [ ] Reviewed recent cluster events
- [ ] Confirmed ConfigMaps/Secrets exist
- [ ] Validated RBAC permissions
- [ ] Checked for resource quotas/limits
- [ ] Reviewed cluster component health

## Reference Documentation

### Detailed Troubleshooting Guides

Consult `references/troubleshooting_workflow.md` for:
- Step-by-step workflows for each issue type
- Decision trees for diagnosis
- Command sequences for systematic debugging
- Quick reference command cheat sheet

### Common Issues Database

Consult `references/common_issues.md` for:
- Detailed explanations of each common issue
- Symptoms and causes
- Specific debugging steps
- Solutions and fixes
- Prevention strategies
