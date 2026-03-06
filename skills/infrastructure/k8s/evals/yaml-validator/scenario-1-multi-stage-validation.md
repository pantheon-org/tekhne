# Eval Scenario: Validate Multi-Resource YAML with Errors

## Objective
Run comprehensive validation on a YAML file with syntax and schema errors.

## Setup
```yaml
# test-resources.yaml (3 resources with errors)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
spec:
  replicas: "3"  # ERROR: should be integer
  selector:
    matchLabels: { app: myapp }
  template:
    metadata:
      labels: { app: myapp }
    spec:
      containers:
      - name: myapp
        image: myapp:latest  # WARNING: use specific version
---
apiVersion: v1
kind: Service
metadata
  name: myapp-service  # ERROR: missing colon after metadata
spec:
  selector: { app: myapp }
  ports:
  - port: 80
---
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: myapp-cert
spec:
  secretName: myapp-tls
  # ERROR: missing required field issuerRef
  dnsNames:
  - myapp.example.com
```

## Task
Validate the file using the complete 6-stage validation workflow.

## Expected Actions
1. Count resources (3 total)
2. Run yamllint for syntax errors
3. Detect CRDs and lookup documentation
4. Run kubeconform for schema validation
5. Attempt cluster dry-run (optional)
6. Generate comprehensive validation report

## Success Criteria
- [ ] Identifies syntax error in Service (missing colon)
- [ ] Identifies type error in Deployment (replicas as string)
- [ ] Detects missing issuerRef in Certificate
- [ ] Warns about `latest` image tag
- [ ] Reports all issues with line numbers
- [ ] Generates before/after fix suggestions
- [ ] Does NOT use Edit tool to apply fixes
- [ ] Produces report-only output

## Validation
Agent must follow report-only constraint and complete all 6 stages sequentially.
