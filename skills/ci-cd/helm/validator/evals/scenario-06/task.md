# Scenario 06: CRD Detection and Validation (Stage 6)

## User Prompt

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
