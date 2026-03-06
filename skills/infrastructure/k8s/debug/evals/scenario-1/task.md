# Service Not Routing Traffic

## Problem Description

The development team deployed a new version of the API gateway service, but requests are timing out. The pods are running and healthy according to their readiness probes, but the service endpoint returns connection refused errors. The team suspects a configuration issue but isn't sure where to look.

You need to create a troubleshooting guide that helps junior engineers diagnose similar service connectivity issues in the future. The guide should demonstrate the proper investigation workflow and explain how to verify that all components are configured correctly.

## Output Specification

Create two files:

1. `service-debug-workflow.sh` - A bash script that demonstrates the systematic process for diagnosing service connectivity issues. Use comments to explain what each command checks and why it matters.

2. `connectivity-checklist.md` - A troubleshooting checklist that explains:
   - The most common causes of service connectivity failures
   - Specific verification steps to confirm proper configuration
   - The key relationships between services, selectors, endpoints, and pod labels

## Input Files

The following manifests represent the deployed resources. Extract them before beginning.

=============== FILE: inputs/api-gateway-deployment.yaml ===============
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
  namespace: default
spec:
  replicas: 3
  selector:
    matchLabels:
      app: gateway
      version: v2
  template:
    metadata:
      labels:
        app: gateway
        version: v2
    spec:
      containers:
      - name: gateway
        image: api-gateway:2.0.0
        ports:
        - containerPort: 8080
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
=============== END FILE ===============

=============== FILE: inputs/api-gateway-service.yaml ===============
apiVersion: v1
kind: Service
metadata:
  name: api-gateway-svc
  namespace: default
spec:
  type: ClusterIP
  selector:
    app: api-gateway
    tier: frontend
  ports:
- protocol: TCP
    port: 80
    targetPort: 8080
=============== END FILE ===============
