# Scenario 03: Multi-Resource Validation Report

## User Prompt

A developer submitted a multi-resource YAML file containing several Kubernetes resources that need validation before deployment. The file has issues spread across multiple resources, and the validation tool must identify all of them without stopping at the first error.

Your task is to create a validation tool that handles multi-resource YAML files and produces a comprehensive report organized by resource, using file-absolute line numbers.

The following multi-resource file has issues in multiple resources:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  API_KEY: "secret-key-12345"
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-app
spec:
  replicas: 2
  selector:
    matchLabels:
      app: webapp
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: web
        image: nginx:latest
---
apiVersion: v1
kind: Service
metadata:
  name: web-service
spec:
  selector:
    app: webapp
  ports:
  - port: 80
    targetPort: "8080"
```

Create these files:

1. `multi-resource-validator.sh` — A bash script that validates a multi-resource YAML file and generates a comprehensive report. The script should:
   - Count the number of resources in the file
   - Validate all resources even if early ones have errors
   - Track which resource each issue belongs to
   - Use absolute line numbers for error locations

2. `validation-report-example.md` — A sample validation report demonstrating:
   - How to organize findings when multiple resources have issues
   - Proper line number reporting for multi-document files
   - Summary of issues per resource
   - Clear severity classification

## Expected Behavior

1. Script or report identifies 3 resources in the file
2. Report uses line numbers relative to file start (not per-resource), e.g., line 25 for Service issue
3. Report covers issues in all 3 resources (ConfigMap, Deployment, Service), not just the first one
4. Script validates all resources even after finding issues in early resources
5. Report identifies `API_KEY` as sensitive data in ConfigMap (should be Secret)
6. Report identifies selector mismatch between Deployment (`app: webapp`) and pod labels (`app: web`)
7. Report flags `nginx:latest` as problematic for production
8. Report identifies `targetPort "8080"` as string (should be integer)
9. Report groups or labels issues by which resource they belong to
10. Report assigns severity levels (Error/Warning/Info) to at least 2 issues

## Success Criteria

- **Resource count**: Script or report identifies 3 resources in the file
- **File-absolute line numbers**: Report uses line numbers relative to file start (not per-resource)
- **All resources validated**: Report covers issues in all 3 resources (ConfigMap, Deployment, Service)
- **Continue on error**: Script validates all resources even after finding issues in early resources
- **Secret in ConfigMap**: Report identifies `API_KEY` as sensitive data in ConfigMap (should be Secret)
- **Label mismatch**: Report identifies selector mismatch between Deployment (`app: webapp`) and pod labels (`app: web`)
- **Latest tag issue**: Report flags `nginx:latest` as problematic for production
- **String port issue**: Report identifies `targetPort "8080"` as string (should be integer)
- **Per-resource organization**: Report groups or labels issues by which resource they belong to
- **Severity assigned**: Report assigns severity levels (Error/Warning/Info) to at least 2 issues

## Failure Conditions

- Resource count is incorrect or not reported
- Report stops after finding issues in the first resource (does not continue to validate all resources)
- `API_KEY` in ConfigMap is not flagged as a security concern
- Selector mismatch between Deployment and pod template labels is not identified
- `nginx:latest` is not flagged as a production concern
- `targetPort "8080"` string type error is not identified
- Issues are not organized or labeled by resource
