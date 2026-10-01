# Scenario 05: Security Best Practices Audit (Stage 9)

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
