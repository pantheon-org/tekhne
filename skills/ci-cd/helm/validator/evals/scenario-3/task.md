# Task: CRD Detection and Validation (Stage 6)

You are given the following rendered manifest file from a chart's `crds/` directory:

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: my-tls-cert
  namespace: default
spec:
  secretName: my-tls-secret
  issuerRef:
    name: letsencrypt-prod
  dnsNames:
    - example.com
```

And a second rendered manifest from `templates/`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
spec:
  replicas: 1
  template:
    spec:
      containers:
        - name: app
          image: myapp:v1.2.3
```

## Your Task

Perform Stage 6 (CRD Detection and Documentation Lookup) of the helm-validator workflow.

1. Identify which of the two manifests contains a CRD (Custom Resource Definition usage) and which is a native Kubernetes resource.
2. For the detected CRD, describe the documentation lookup process you would follow: which tool to use first (context7 MCP), what query parameters to use, and what fallback to use if it is unavailable.
3. Based on the `cert-manager.io/v1 Certificate` spec, list the required fields that are present and identify any commonly required fields that appear to be missing (e.g., `duration`, `renewBefore`, `privateKey`).
4. Explain why kubeconform would show "No schema found" for this resource and what the correct approach is to validate it.
