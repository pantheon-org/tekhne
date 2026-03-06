# Production Pod Crash Investigation

## Problem Description

Your company's e-commerce platform has been experiencing intermittent outages. The on-call engineer noticed that the `payment-processor` pod in the production namespace keeps restarting every few minutes. The logs from the current container instance are empty, and the service is unavailable to customers.

The operations team needs a detailed diagnostic report that identifies the root cause and documents the investigation process. They want to understand why the pod is failing and what steps were taken to diagnose the issue.

## Output Specification

Create a detailed diagnostic report saved as `diagnosis-report.md` that includes:
- The investigation methodology used
- Commands executed during the diagnosis
- Key findings from each diagnostic step
- The identified root cause of the crash
- Recommended remediation steps

Also create a bash script named `reproduce-diagnosis.sh` that demonstrates the systematic diagnostic workflow you followed. The script should use comments to explain each step and include the kubectl commands in the proper order for investigating this type of issue.

## Input Files

The following YAML represents the problematic pod manifest for reference. Extract it before beginning.

=============== FILE: inputs/payment-processor-pod.yaml ===============
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
=============== END FILE ===============
