---
name: k8s-yaml-validator
description: Comprehensive toolkit for validating, linting, and testing Kubernetes YAML resources. Use this skill when validating Kubernetes manifests, debugging YAML syntax errors, performing dry-run tests on clusters, or working with Custom Resource Definitions (CRDs) that require documentation lookup.
---

# Kubernetes YAML Validator

## Validation Mindset

**Mental Model**: Validation is a layered funnel—syntax → schema → cluster constraints → runtime behavior. Each layer catches different classes of errors.

**Decision Framework**:
1. **Syntax first**: Fix YAML structure before schema issues (broken YAML can't be validated)
2. **Schema second**: Verify API compliance before cluster-specific checks
3. **Cluster third**: Test against real constraints (RBAC, quotas, admission webhooks)
4. **Runtime last**: Consider how the resource behaves in production

**When to use this skill**:
- Before applying any YAML to a cluster (development, staging, production)
- When debugging mysterious kubectl apply failures
- During CI/CD pipeline execution (catch errors early)
- When working with unfamiliar CRDs (validation guides learning)
- After generating YAML with k8s-yaml-generator

**Validation philosophy**: Report all issues, prioritize by severity, suggest fixes but never apply them automatically.

## Overview

This skill provides a comprehensive validation workflow for Kubernetes YAML resources, combining syntax linting, schema validation, cluster dry-run testing, and intelligent CRD documentation lookup. Validate any Kubernetes manifest with confidence before applying it to the cluster.

> **IMPORTANT — REPORT-ONLY:** Do NOT modify files, use the Edit tool, or offer to apply fixes. Generate a comprehensive validation report with suggested fixes shown as before/after code blocks, then let the user decide what to do next.

## Validation Workflow

Follow this sequential validation workflow. Each stage catches different types of issues:

### Stage 0: Pre-Validation Setup (Resource Count Check)

**Before running any validation tools, check the file complexity:**

1. Count the number of resources by counting `---` document separators or parsing the file
2. **If the file contains 3 or more resources**, immediately load `references/validation_workflow.md`:
   ```
   Read references/validation_workflow.md
   ```
3. Note the resource count for the validation report summary

### Stage 1: Tool Check

```bash
bash scripts/setup_tools.sh
```

Required tools: **yamllint**, **kubeconform**, **kubectl** (optional). If tools are missing, display installation instructions from script output and document missing tools in the validation report.

### Stage 2: YAML Syntax Validation

```bash
yamllint -c assets/.yamllint <file.yaml>
```

Report all syntax issues with `file:line` references. Show suggested before/after code blocks. Continue to the next stage to collect all issues before reporting.

### Stage 3: CRD Detection and Documentation Lookup

```bash
bash scripts/detect_crd_wrapper.sh <file.yaml>
```

The wrapper script automatically handles Python dependencies via a temporary virtual environment if PyYAML is not available.

**Resilient Parsing:** The script parses valid documents and reports errors for invalid ones — continuing processing rather than aborting. This matches kubeconform's behavior of validating 2/3 resources when 1/3 has syntax errors.

Script outputs JSON with resource information and parse status:
```json
{
  "resources": [
    {
      "kind": "Certificate",
      "apiVersion": "cert-manager.io/v1",
      "group": "cert-manager.io",
      "version": "v1",
      "isCRD": true,
      "name": "example-cert"
    }
  ],
  "parseErrors": [
    {
      "document": 1,
      "start_line": 2,
      "error_line": 6,
      "error": "mapping values are not allowed in this context"
    }
  ],
  "summary": {
    "totalDocuments": 3,
    "parsedSuccessfully": 2,
    "parseErrors": 1,
    "crdsDetected": 1
  }
}
```

**For each detected CRD:**

1. **Query library documentation:**
   ```
   tessl_query_library_docs: query: "<project> <kind> <version> spec fields"
   Example: "cert-manager Certificate v1 spec fields"
   Example: "istio VirtualService v1beta1 specification"
   ```

2. **Fallback to WebSearch if documentation is insufficient:**
   ```
   "<kind>" "<group>" kubernetes CRD "<version>" documentation spec
   ```

3. Extract required `spec` fields, field types, validation rules, and version-specific changes.

**Secondary CRD Detection:** If `detect_crd_wrapper.sh` fails (e.g., all documents have syntax errors) but kubeconform validates a CRD resource, parse kubeconform output to identify those CRDs and perform documentation lookups.

### Stage 4: Schema Validation

```bash
kubeconform \
  -schema-location default \
  -schema-location 'https://raw.githubusercontent.com/datreeio/CRDs-catalog/main/{{.Group}}/{{.ResourceKind}}_{{.ResourceAPIVersion}}.json' \
  -strict \
  -ignore-missing-schemas \
  -summary \
  -verbose \
  <file.yaml>
```

For CRDs where kubeconform reports "no schema found", use the documentation from Stage 3 to manually validate spec fields.

### Stage 5: Cluster Dry-Run (if available)

Try server-side dry-run first:

```bash
kubectl apply --dry-run=server -f <file.yaml>
```

**Fallback logic:**
- Connection error → try `--dry-run=client` and document "Limited validation (no cluster access)"
- Validation error → record error, continue to Stage 6
- Parse error → skip client dry-run, continue to Stage 6

For updates:
```bash
kubectl diff -f <file.yaml>
```

### Stage 6: Generate Detailed Validation Report (REPORT ONLY)

> **Constraint:** Present the report and let the user take action — do NOT use the Edit tool or prompt the user to apply fixes.

**Report structure:**

1. **Issue summary table** with severity, stage, location, issue, and suggested fix
2. **Severity categories:** Errors (must fix), Warnings (should fix), Info (optional)
3. **Before/after code blocks** for each issue with fix complexity: [Simple], [Medium], [Complex]
4. **Validation summary** with stage status table and next steps

Example issue format:
```
**Issue 1: deployment.yaml:21 - Wrong field type (Error) [Simple]**

Current:
```yaml
        - containerPort: "80"
```

Suggested Fix:
```yaml
        - containerPort: 80
```
```

## Best Practices Reference

Load when schema validation fails with type errors, reports missing required fields, or dry-run fails with validation errors:
```
Read references/k8s_best_practices.md
```

## Detailed Validation Workflow Reference

Load when the file contains 3+ resources, or validation produces unfamiliar errors spanning multiple stages:
```
Read references/validation_workflow.md
```

## Working with Multiple Resources

When a YAML file contains multiple resources (separated by `---`):

1. Validate the entire file first with yamllint and kubeconform
2. Identify which resource has issues by checking line numbers
3. For dry-run, the file is tested as a unit
4. Track issues per-resource in the report

**Partial parsing:** Tools parse valid documents and continue when some documents have syntax errors:

| Document | Resource | Parsing | Validation |
|----------|----------|---------|------------|
| 1 | Deployment | ❌ Syntax error (line 6) | Skipped |
| 2 | Service | ✅ Parsed | ✅ Valid |
| 3 | Certificate | ✅ Parsed | ✅ Valid |

**Always use file-absolute line numbers** (relative to the start of the entire file) — this matches what yamllint, kubeconform, and kubectl report, and lets users navigate directly to errors in their editor.

## Common Anti-Patterns

### NEVER: Skip Validation Because "It Looks Fine"

**BAD**:
```bash
# Directly applying without validation
kubectl apply -f deployment.yaml
```

**GOOD**:
```bash
# Validate first, apply only after clean report
yamllint deployment.yaml
kubeconform deployment.yaml
kubectl apply --dry-run=server -f deployment.yaml
# Then apply
kubectl apply -f deployment.yaml
```

### NEVER: Fix Only the First Error

**BAD**:
```bash
# Fix syntax error on line 10, re-run, fix next error, repeat...
# This takes 10x longer than collecting all issues upfront
```

**GOOD**:
```bash
# Run full validation workflow once
# Fix all errors in one pass using the comprehensive report
# Re-validate to confirm clean
```

### NEVER: Ignore Warnings

**BAD**:
```yaml
# "It's just a warning, ship it"
apiVersion: extensions/v1beta1  # Deprecated API version
kind: Deployment
```

**GOOD**:
```yaml
# Warnings indicate future breakage—fix them now
apiVersion: apps/v1
kind: Deployment
```

### NEVER: Validate Against Wrong Kubernetes Version

**BAD**:
```bash
# Using default kubeconform schemas (may not match cluster version)
kubeconform deployment.yaml
```

**GOOD**:
```bash
# Always validate against target cluster version
kubectl version  # Check cluster version
kubeconform -kubernetes-version 1.28.0 deployment.yaml
# Or use server-side dry-run (inherently correct)
kubectl apply --dry-run=server -f deployment.yaml
```

### NEVER: Trust Client-Side Validation Alone

**BAD**:
```bash
kubectl apply --dry-run=client -f deployment.yaml  # Misses cluster-specific issues
```

**GOOD**:
```bash
# Use server-side dry-run to catch admission webhooks, RBAC, quotas
kubectl apply --dry-run=server -f deployment.yaml
```

### ALWAYS: Validate Multi-Resource Files as a Unit

CRD order matters (e.g., namespace before resources in that namespace):

```bash
# Validate the full file, not individual resources
kubectl apply --dry-run=server -f all-resources.yaml
```

### ALWAYS: Check CRD Versions

```bash
# Verify CRD version matches what's installed in cluster
kubectl get crd <crd-name> -o jsonpath='{.spec.versions[*].name}'
```

## Error Handling Strategies

### Tool Not Available

- Run `scripts/setup_tools.sh` to check availability and get installation instructions
- Skip optional stages but document what was skipped

### Cluster Access Issues

- Fall back to client-side dry-run
- Document limitations in validation report

### CRD Documentation Not Found

- Document the failed lookup
- Attempt validation with kubeconform CRD schemas
- Suggest manual inspection:
  ```bash
  kubectl get crd <crd-name>.group -o yaml
  kubectl explain <kind>
  ```

### Validation Stage Failures

- Continue to next stage even if one fails
- Collect all errors before presenting to user
- Prioritize fixing earlier stage errors first

## Version Awareness

- Check for deprecated APIs (e.g., `extensions/v1beta1` → `apps/v1`)
- For CRDs, ensure the apiVersion matches what's in the cluster
- Use `kubectl api-versions` to list available API versions

## Verification After Fixes

After the user applies fixes:

1. **Re-run full validation workflow** to ensure clean report
2. **Verify zero errors** in all stages (syntax, schema, dry-run)
3. **Check for new warnings** introduced by fixes
4. **Test deployment** in non-production environment first:
   ```bash
   kubectl apply -f <file.yaml> --namespace=dev
   kubectl get <resource> -n dev -w  # Watch for successful creation
   ```

## References

### scripts/

**detect_crd_wrapper.sh** — Wrapper that manages Python dependencies (auto-creates venv if PyYAML unavailable) and calls `detect_crd.py`.
Usage: `bash scripts/detect_crd_wrapper.sh <file.yaml>`

**detect_crd.py** — Parses YAML files to identify CRDs; extracts kind, apiVersion, group, version; outputs JSON.
Usage: `python3 scripts/detect_crd.py <file.yaml>`

**setup_tools.sh** — Checks for required validation tools, reports versions, provides installation instructions.
Usage: `bash scripts/setup_tools.sh`

### references/

**k8s_best_practices.md** — Kubernetes YAML best practices: metadata, labels, resource limits, security context, common validation issues.

**validation_workflow.md** — Detailed workflow, command options, error handling strategies, complete workflow diagram.

### assets/

**.yamllint** — Pre-configured yamllint rules following Kubernetes conventions (2-space indentation, line length, etc.).
Usage: `yamllint -c assets/.yamllint <file.yaml>`
