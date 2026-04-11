# Scenario 01: Deployment Manifest Validation Workflow

## User Prompt

A junior developer created a Kubernetes Deployment manifest for a new service but is uncertain whether it follows best practices and will work correctly when applied to the cluster. Before submitting a pull request, they need a comprehensive validation report that identifies any issues.

Create a validation tool/script that demonstrates the proper workflow for validating Kubernetes YAML manifests. The tool should check multiple aspects of the manifest and produce a detailed report with all findings.

The following deployment manifest needs validation:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: webapp
        image: nginx:latest
        ports:
        - containerPort: "80"
```

Create these files:

1. `validate-manifest.sh` — A bash script that implements a complete validation workflow for Kubernetes YAML files. The script should check syntax, schema compliance, and any other relevant validation layers. Include comments explaining each validation stage.

2. `validation-report.md` — A sample validation report showing:
   - What each validation stage checks
   - The order in which validation should be performed
   - How to interpret results from each stage
   - Recommended severity categories for issues

## Expected Behavior

1. Document or script explains validation stages: syntax → schema → cluster/other
2. Script uses `yamllint` or mentions YAML syntax checking as first stage
3. Script uses `kubeconform` or mentions schema validation as second stage
4. Script checks for required tools or mentions tool availability verification
5. Report includes Error/Warning/Info severity levels or similar categorization
6. Script generates a report but does NOT automatically modify the input file
7. Script runs stages in order: syntax check before schema check
8. Script or documentation indicates continuing to next stage even if earlier stage fails
9. Report identifies at least 2 actual issues in the provided manifest (e.g., `:latest` tag, missing resources, string port)
10. Script includes comments explaining what each validation stage checks

## Success Criteria

- **Layered approach documented**: Report or script explains validation stages: syntax → schema → cluster/other
- **Syntax validation**: Script uses `yamllint` or mentions YAML syntax checking as first stage
- **Schema validation**: Script uses `kubeconform` or mentions schema validation as second stage
- **Tool check stage**: Script checks for required tools or mentions tool availability verification
- **Severity categories**: Report includes Error/Warning/Info severity levels or similar categorization
- **Report-only approach**: Script generates report but does NOT automatically modify the input file
- **Sequential execution**: Script runs stages in order: syntax check before schema check
- **Continue on failure**: Script or documentation indicates continuing to next stage even if earlier stage fails
- **Issue identification**: Report identifies at least 2 actual issues in the provided manifest (e.g., latest tag, missing resources, string port)
- **Stage comments**: Script includes comments explaining what each validation stage checks

## Failure Conditions

- Validation stages are not documented or explained
- `yamllint` or syntax checking is not part of the workflow
- `kubeconform` or schema validation is not included
- Script auto-modifies the input manifest instead of generating a report only
- Severity categories are absent from the report
- Validation stages are not run in order (syntax before schema)
- Fewer than 2 issues are identified in the provided manifest
