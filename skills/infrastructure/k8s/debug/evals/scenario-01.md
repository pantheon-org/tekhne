# Scenario 01: Production Pod Crash Investigation

## User Prompt

Your company's e-commerce platform has been experiencing intermittent outages. The on-call engineer noticed that the `payment-processor` pod in the production namespace keeps restarting every few minutes. The logs from the current container instance are empty, and the service is unavailable to customers.

The operations team needs a detailed diagnostic report that identifies the root cause and documents the investigation process. They want to understand why the pod is failing and what steps were taken to diagnose the issue.

The following YAML represents the problematic pod manifest for reference:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: payment-processor
  namespace: production
  labels:
    app: payment
    tier: backend
spec:
  containers:
  - name: processor
    image: payment-app:v2.1.3
    ports:
    - containerPort: 8080
    env:
    - name: DB_CONNECTION
      value: "postgres://db.internal:5432/payments"
    resources:
      requests:
        memory: "128Mi"
        cpu: "250m"
      limits:
        memory: "256Mi"
        cpu: "500m"
```

Create a detailed diagnostic report saved as `diagnosis-report.md` that includes:
- The investigation methodology used
- Commands executed during the diagnosis
- Key findings from each diagnostic step
- The identified root cause of the crash
- Recommended remediation steps

Also create a bash script named `reproduce-diagnosis.sh` that demonstrates the systematic diagnostic workflow you followed. The script should use comments to explain each step and include the kubectl commands in the proper order for investigating this type of issue.

## Expected Behavior

1. Show a broad-to-narrow investigation approach: cluster/namespace level → pod level → container/logs
2. Use `kubectl get events` or `kubectl describe pod` to gather context before other diagnostics
3. Use `kubectl logs` with the `--previous` flag to check logs from the crashed container
4. Include `-n production` or `--namespace=production` on all kubectl commands
5. Include a `kubectl describe pod` command to view the Events section
6. Follow the order: events → describe → logs → resource status in the script or report
7. Document the specific root cause with evidence from diagnostic commands
8. Not suggest restarting the pod before gathering diagnostic context
9. Check for OOMKilled or resource limits in pod status or events
10. Use `--sort-by='.lastTimestamp'` when checking events or include timestamp flags

## Success Criteria

- **Layered approach**: Documentation or script shows broad-to-narrow investigation: cluster/namespace level → pod level → container/logs
- **Events checked first**: Uses `kubectl get events` or `kubectl describe pod` to gather context before other diagnostics
- **Previous logs flag**: Uses `kubectl logs` with `--previous` flag to check logs from crashed container
- **Namespace explicit**: All kubectl commands include `-n production` or `--namespace=production`
- **Describe pod used**: Includes `kubectl describe pod` command to view Events section
- **Systematic order**: Script or report follows order: events → describe → logs → resource status
- **Root cause documented**: Report documents specific root cause with evidence from diagnostic commands
- **No premature restart**: Does NOT suggest restarting pod before gathering diagnostic context
- **Resource constraints checked**: Checks for OOMKilled or resource limits in pod status or events
- **Timestamp sorting**: Uses `--sort-by='.lastTimestamp'` when checking events or includes timestamp flags

## Failure Conditions

- Investigation does not follow a broad-to-narrow layered approach
- Events or `kubectl describe pod` are not used before other diagnostics
- `kubectl logs --previous` is not used to check logs from the crashed container
- kubectl commands do not include the `-n production` namespace flag
- Root cause is not documented with evidence from diagnostic commands
- Pod restart is suggested before gathering diagnostic context
- OOMKilled or resource limit violations are not checked
