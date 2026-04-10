# Scenario 02: Security Best Practices Audit (Stage 9)

## User Prompt

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

## Expected Behavior

1. Identify that `runAsNonRoot`, `runAsUser`, and `fsGroup` are all absent from the pod spec and mark each as a failure
2. Identify that `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem`, and `capabilities.drop: [ALL]` are all absent from the container spec and mark each as a failure
3. Identify that memory requests and all resource limits (cpu and memory) are absent — only cpu request is present
4. Flag the `image: myrepo/api-server:latest` tag as a Warning and propose pinning to a specific version
5. Identify that neither `livenessProbe` nor `readinessProbe` is defined
6. Provide at least one before/after YAML code block for a proposed fix without modifying any files

## Success Criteria

- **Pod securityContext checks**: Agent identifies that runAsNonRoot, runAsUser, and fsGroup are all absent from the pod spec and marks each as a failure
- **Container securityContext checks**: Agent identifies that allowPrivilegeEscalation:false, readOnlyRootFilesystem, and capabilities.drop:[ALL] are all absent and marks each as a failure
- **Resource limits missing**: Agent identifies that memory requests and all resource limits (cpu and memory) are absent. Only cpu request is present
- **:latest tag flagged**: Agent flags the image tag :latest as a Warning and proposes pinning to a specific digest or version tag
- **Missing probes identified**: Agent identifies that neither livenessProbe nor readinessProbe is defined and classifies this as a Warning
- **Proposed fixes with before/after blocks**: Agent provides at least one before/after YAML code block for a proposed fix and maintains read-only posture (no files modified)

## Failure Conditions

- Missing pod securityContext fields (runAsNonRoot, runAsUser, fsGroup) are not identified
- Missing container securityContext fields (allowPrivilegeEscalation, readOnlyRootFilesystem, capabilities.drop) are not identified
- Missing memory requests and resource limits are not flagged as failures
- The `:latest` image tag is not flagged
- Missing liveness and readiness probes are not identified
- No before/after code block is provided for any proposed fix
