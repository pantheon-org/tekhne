# Task: Security Best Practices Audit (Stage 9)

You are given the following rendered Kubernetes Deployment manifest produced by `helm template`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-server
  namespace: production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-server
  template:
    metadata:
      labels:
        app: api-server
    spec:
      containers:
        - name: api-server
          image: myrepo/api-server:latest
          ports:
            - containerPort: 8080
          resources:
            requests:
              cpu: 100m
          env:
            - name: LOG_LEVEL
              value: info
```

## Your Task

Perform Stage 9 (Security Best Practices Check) on this rendered manifest.

For each of the following required checks, state whether the manifest passes or fails and explain why:

1. Pod securityContext: `runAsNonRoot`, `runAsUser`, `fsGroup`
2. Container securityContext: `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem`, `capabilities.drop: [ALL]`
3. Resource limits and requests for cpu and memory
4. No `:latest` image tags
5. Liveness and readiness probes present

Produce a final security summary table and list each proposed fix (before/after code block) without modifying any files.
