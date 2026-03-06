# Terminating Pod Recovery

## Problem Description

After a failed deployment rollback, one of the pods is stuck in the "Terminating" state for over 10 minutes. The deployment controller is unable to bring up new pods because the old pod won't release its resources. This is blocking a critical security patch from being deployed.

The operations team needs a documented procedure for handling stuck pods that can be followed safely without causing data loss or unnecessary cluster disruption. Create a runbook that demonstrates the proper escalation workflow for recovering from this situation.

## Output Specification

Create these files:

1. `stuck-pod-recovery.sh` - A bash script demonstrating the safe escalation process for removing stuck pods. The script should include comments explaining the verification steps at each stage.

2. `recovery-runbook.md` - Documentation that includes:
   - The diagnostic steps to confirm a pod is actually stuck (not just slow to terminate)
   - The escalation workflow from graceful to forceful deletion
   - Verification checkpoints before each escalation step
   - Post-recovery validation steps

## Input Files

The following represents the deployment that triggered the issue. Extract it before beginning.

=============== FILE: inputs/backend-deployment.yaml ===============
apiVersion: apps/v1
kind: Deployment
metadata:
  name: backend-api
  namespace: production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: backend
  template:
    metadata:
      labels:
        app: backend
      finalizers:
        - kubernetes.io/pvc-protection
    spec:
      containers:
      - name: api
        image: backend-api:v1.5.2
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: data
          mountPath: /data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: backend-data
=============== END FILE ===============
