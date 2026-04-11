# Scenario 04: Report-Only Validation with Fix Proposals

## User Prompt

Your team needs a validation tool that identifies issues in Kubernetes manifests and proposes fixes — but never applies them automatically. Engineers have encountered problems in the past with tools that auto-applied incorrect fixes, so the workflow requires explicit human review before any changes are made.

Create a report-only validation tool that shows before/after code blocks for each suggested fix, labels the complexity of each fix, and explains the report-only philosophy.

The following manifest has several fixable issues:

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: database
  namespace: default
spec:
  serviceName: "db"
  replicas: 3
  selector:
    matchLabels:
      app: db
  template:
    metadata:
      labels:
        app: db
    spec:
      containers:
      - name: postgres
        image: postgres:latest
        ports:
        - containerPort: "5432"
        env:
        - name: POSTGRES_PASSWORD
          value: "admin123"
```

Create these files:

1. `report-validator.sh` — A bash script that validates Kubernetes manifests and produces a report with suggested fixes, but NEVER modifies the input file. The script should:
   - Generate a validation report file
   - Show before/after code blocks for suggested fixes
   - Include fix complexity indicators (`[Simple]`, `[Medium]`, `[Complex]`)
   - Exit without modifying the original manifest

2. `validation-philosophy.md` — Documentation explaining:
   - Why validation should be report-only
   - How to present suggested fixes effectively
   - The format for before/after code blocks
   - How developers should apply fixes after review

## Expected Behavior

1. Script does NOT modify the input YAML file (`statefulset.yaml` remains unchanged)
2. Script creates a separate report file (not stdout-only)
3. Report includes before and after code examples for at least 2 issues
4. Report labels fixes with complexity: `[Simple]`, `[Medium]`, or `[Complex]`
5. `validation-philosophy.md` explains why validation should be report-only
6. Report identifies at least 3 issues (latest tag, string port, plaintext password, missing resources)
7. Each suggested fix shows exact replacement code, not just a description
8. Report categorizes issues by severity (Error/Warning/Info)
9. Documentation explains how developers apply fixes after reviewing the report

## Success Criteria

- **No file modification**: Script does NOT modify the input YAML file (`statefulset.yaml` remains unchanged)
- **Report file generated**: Script creates a separate report file (not stdout-only)
- **Before/after blocks**: Report includes before and after code examples for at least 2 issues
- **Complexity indicators**: Report labels fixes with complexity: `[Simple]`, `[Medium]`, or `[Complex]`
- **Philosophy documented**: `validation-philosophy.md` explains why validation should be report-only
- **Issues identified**: Report identifies at least 3 issues (latest tag, string port, plaintext password, missing resources)
- **Suggested fixes clear**: Each suggested fix shows exact replacement code, not just a description
- **Severity classification**: Report categorizes issues by severity (Error/Warning/Info)
- **Developer workflow**: Documentation explains how developers apply fixes after reviewing the report

## Failure Conditions

- Script modifies the input manifest file directly
- Report is only printed to stdout without generating a report file
- Before/after code blocks are missing from the report
- Fix complexity indicators (`[Simple]`, `[Medium]`, `[Complex]`) are absent
- `validation-philosophy.md` does not explain the report-only approach
- Fewer than 3 issues are identified in the provided manifest
- Suggested fixes provide only descriptions rather than exact code replacements
