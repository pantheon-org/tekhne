# Scenario 1: Runs the full funnel on a manifest with several kinds of problem

## User Prompt

"Please validate deployment.yaml before I apply it. The cluster is reachable and runs Kubernetes 1.28."

## Repository State

`deployment.yaml` contains this Deployment:

```yaml
apiVersion: extensions/v1beta1
kind: Deployment
metadata:
  name: web
  namespace: shop
spec:
  replicas: 2
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
        - name: web
          image: web:latest
          ports:
            - containerPort: "80"
```

yamllint and kubeconform are installed. No policy checker is installed.
