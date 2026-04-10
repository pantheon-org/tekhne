# Scenario 05: Safe Deployment Restart Procedure

## User Prompt

A legacy application deployment is exhibiting stale cache issues that require a restart to resolve. The application doesn't support graceful cache invalidation, so a rolling restart is the only solution. However, previous restarts have caused brief service outages because they weren't performed correctly.

The SRE team needs a standardized restart procedure that ensures zero downtime and proper validation at each step. Create a safe restart runbook that can be used by on-call engineers.

The following represents the deployment requiring restart:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cache-service
  namespace: production
  labels:
    app: cache
    version: v1.2.0
spec:
  replicas: 5
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: cache
  template:
    metadata:
      labels:
        app: cache
    spec:
      containers:
      - name: cache-app
        image: cache-app:1.2.0
        ports:
        - containerPort: 6379
        livenessProbe:
          tcpSocket:
            port: 6379
          initialDelaySeconds: 10
        readinessProbe:
          tcpSocket:
            port: 6379
          initialDelaySeconds: 5
```

Create these files:

1. `safe-restart.sh` — A bash script that performs a deployment restart with all necessary verification steps. Include error handling and checkpoints.

2. `restart-procedure.md` — Documentation that explains:
   - Pre-restart verification steps
   - The proper restart command sequence
   - How to monitor the restart progress
   - Post-restart validation to confirm success
   - Rollback procedure if restart fails
   - How to document the restart in the deployment metadata

## Expected Behavior

1. Check `kubectl rollout status` before performing the restart
2. Include checking events, logs, or describe before deciding the restart is needed
3. Use `kubectl rollout restart deployment/<name>` as the restart command
4. Use `kubectl rollout status` with a `--timeout` flag to verify the restart completes
5. Include `-n production` namespace on all kubectl commands
6. Verify pods are Running and Ready after restart with `kubectl get pods`
7. Use `kubectl annotate` to document the restart reason in deployment metadata
8. Not restart as first action without checking current state and gathering context
9. Include `--timeout` flag on the rollout status command (e.g., `--timeout=120s`)

## Success Criteria

- **Pre-restart status**: Script checks `kubectl rollout status` before performing restart
- **Diagnostic context**: Documentation includes checking events, logs, or describe before deciding restart is needed
- **Rollout restart command**: Uses `kubectl rollout restart deployment/<name>` command
- **Post-restart verification**: Uses `kubectl rollout status` with `--timeout` flag to verify restart completes
- **Namespace specified**: All kubectl commands include `-n production` namespace
- **Pod status checked**: Verifies pods are Running and Ready after restart with `kubectl get pods`
- **Restart documented**: Uses `kubectl annotate` to document the restart reason in deployment metadata
- **No immediate restart**: Does NOT restart as first action without checking current state and gathering context
- **Timeout specified**: Includes `--timeout` flag on rollout status command (e.g., `--timeout=120s`)

## Failure Conditions

- `kubectl rollout status` is not checked before performing the restart
- Restart is performed as the first action without checking current state or gathering diagnostic context
- `kubectl rollout restart deployment/<name>` is not used (e.g., using `kubectl delete pods` instead)
- No `--timeout` flag is included on the rollout status verification command
- Pods are not verified as Running and Ready after restart
- `kubectl annotate` is not used to document the restart in deployment metadata
