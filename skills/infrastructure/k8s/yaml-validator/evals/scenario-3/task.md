# Scenario 3: Keeps going when one document in a multi-resource file does not parse

## User Prompt

"Validate all-resources.yaml please."

## Repository State

`all-resources.yaml` contains this content:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: api
  namespace: shop
spec:
  selector:
    app: api
  ports:
    - port: 80
---
apiVersion: apps/v1
kind Deployment
metadata:
  name: api
  namespace: shop
spec:
  replicas: 1
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      containers:
        - name: api
          image: api:1.0.0
---
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: api-tls
  namespace: shop
spec:
  secretName: api-tls
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsNames:
    - api.example.com
```

yamllint, kubeconform and kubectl are installed and the cluster is reachable.
