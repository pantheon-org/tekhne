# Safe Deployment Restart Procedure

## Problem Description

A legacy application deployment is exhibiting stale cache issues that require a restart to resolve. The application doesn't support graceful cache invalidation, so a rolling restart is the only solution. However, previous restarts have caused brief service outages because they weren't performed correctly.

The SRE team needs a standardized restart procedure that ensures zero downtime and proper validation at each step. Create a safe restart runbook that can be used by on-call engineers.

## Output Specification

Create these files:

1. `safe-restart.sh` - A bash script that performs a deployment restart with all necessary verification steps. Include error handling and checkpoints.

2. `restart-procedure.md` - Documentation that explains:
   - Pre-restart verification steps
   - The proper restart command sequence
   - How to monitor the restart progress
   - Post-restart validation to confirm success
   - Rollback procedure if restart fails
   - How to document the restart in the deployment metadata

## Input Files

The following represents the deployment requiring restart. Extract it before beginning.

=============== FILE: inputs/cache-app-deployment.yaml ===============
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
      - name: cache
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
=============== END FILE ===============
