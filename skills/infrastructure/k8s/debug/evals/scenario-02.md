# Scenario 02: Service Not Routing Traffic

## User Prompt

The development team deployed a new version of the API gateway service, but requests are timing out. The pods are running and healthy according to their readiness probes, but the service endpoint returns connection refused errors. The team suspects a configuration issue but isn't sure where to look.

You need to create a troubleshooting guide that helps junior engineers diagnose similar service connectivity issues in the future. The guide should demonstrate the proper investigation workflow and explain how to verify that all components are configured correctly.

The following manifests represent the deployed resources:

```yaml
# api-gateway-deployment.yaml
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
```

```yaml
# api-gateway-service.yaml
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
```

Create two files:

1. `service-debug-workflow.sh` — A bash script that demonstrates the systematic process for diagnosing service connectivity issues. Use comments to explain what each command checks and why it matters.

2. `connectivity-checklist.md` — A troubleshooting checklist that explains:
   - The most common causes of service connectivity failures
   - Specific verification steps to confirm proper configuration
   - The key relationships between services, selectors, endpoints, and pod labels

## Expected Behavior

1. Identify that the service selector labels (`app: api-gateway`, `tier: frontend`) do not match the pod labels (`app: gateway`, `version: v2`) as the root cause
2. Include `kubectl get svc` with `-o jsonpath` or `yaml` to extract selector labels
3. Include `kubectl get pods --show-labels` or similar to display pod labels
4. Use `kubectl get endpoints` to verify whether the service has associated pod IPs
5. State in the documentation that label mismatches are the most common root cause of service connectivity issues
6. Check in systematic order: service definition → pod labels → endpoints → port matching
7. Explain that empty endpoints indicate a selector mismatch
8. Compare service selector against actual pod labels to find the mismatch
9. Not jump to network debugging tools before verifying label configuration

## Success Criteria

- **Label mismatch identified**: Documentation identifies that service selector labels don't match pod labels as the root cause
- **Selector verification**: Script includes `kubectl get svc` with `-o jsonpath` or `yaml` to extract selector labels
- **Pod labels checked**: Script includes `kubectl get pods --show-labels` or similar to display pod labels
- **Endpoints verification**: Uses `kubectl get endpoints` to verify whether service has associated pod IPs
- **Common cause highlighted**: Documentation states that label mismatches are the most common root cause of service connectivity issues
- **Systematic workflow**: Script checks in order: service definition → pod labels → endpoints → port matching
- **Empty endpoints noted**: Documentation explains that empty endpoints indicate selector mismatch
- **Label comparison**: Script or checklist compares service selector against actual pod labels to find mismatch
- **No network tools first**: Does NOT jump to network debugging tools before verifying label configuration

## Failure Conditions

- Label mismatch between service selector and pod labels is not identified as the root cause
- `kubectl get svc` with selector extraction is not included
- `kubectl get pods --show-labels` or equivalent is not used
- `kubectl get endpoints` is not used to check endpoint population
- Documentation does not explain that label mismatches are the most common connectivity cause
- Network debugging tools are used before verifying label and selector configuration
