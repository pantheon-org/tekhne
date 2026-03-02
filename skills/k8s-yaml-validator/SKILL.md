---
name: k8s-yaml-validator
description: Comprehensive toolkit for validating, linting, and testing Kubernetes YAML resources. Use this skill when validating Kubernetes manifests, debugging YAML syntax errors, performing dry-run tests on clusters, or working with Custom Resource Definitions (CRDs) that require documentation lookup.
---

# Kubernetes YAML Validator

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

1. **Try context7 MCP first (preferred):**
   ```
   Use mcp__context7__resolve-library-id with the CRD project name
   Example: "cert-manager" for cert-manager.io CRDs

   Then use mcp__context7__get-library-docs with:
   - context7CompatibleLibraryID from resolve step
   - topic: The CRD kind (e.g., "Certificate")
   - tokens: 5000 (adjust based on need)
   ```

2. **Fallback to WebSearch if context7 fails:**
   ```
   "<kind>" "<group>" kubernetes CRD "<version>" documentation spec
   ```

3. Extract required `spec` fields, field types, validation rules, and version-specific changes.

**Secondary CRD Detection:** If `detect_crd_wrapper.sh` fails (e.g., all documents have syntax errors) but kubeconform validates a CRD resource, parse kubeconform output to identify those CRDs and perform context7/WebSearch lookups.

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

Try server-side dry-run first (runs admission controllers and webhooks):

```bash
kubectl apply --dry-run=server -f <file.yaml>
```

**Fallback logic:**
- Connection error → try `--dry-run=client` and document "Limited validation (no cluster access)"
- Validation error → record error, continue to Stage 6
- Parse error → skip client dry-run (same error), continue to Stage 6

For updates to existing resources:
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

**Why:** containerPort must be an integer.
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

### Partial Parsing Behavior

When a multi-document YAML file has some valid and some invalid documents:
- `detect_crd.py` parses valid documents and skips invalid ones
- kubeconform validates resources it can parse and reports errors for unparseable ones

**Example — 3 documents, document 1 has a syntax error:**

| Document | Resource | Parsing | Validation |
|----------|----------|---------|------------|
| 1 | Deployment | ❌ Syntax error (line 6) | Skipped |
| 2 | Service | ✅ Parsed | ✅ Valid |
| 3 | Certificate | ✅ Parsed | ✅ Valid |

**Always use file-absolute line numbers** (relative to the start of the entire file) — this matches what yamllint, kubeconform, and kubectl report, and lets users navigate directly to errors in their editor.

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

## Communication Guidelines

When presenting validation results:
- Use `file:line` references for all issues
- Explain why issues matter (e.g., "This will cause pod creation to fail")
- Group related issues (e.g., all missing label issues together)
- Provide fix complexity indicators ([Simple], [Medium], [Complex])

## Performance Optimization

### Parallel Tool Execution

**Can run in parallel:**
- `yamllint` (Stage 2) and `detect_crd_wrapper.sh` (Stage 3) operate independently on the input file

**Must run sequentially:**
- Stage 0 → Stage 1 → Stages 2+3 (parallel) → Stage 4 → Stage 5 → Stage 6

Parallelising is most beneficial for files with more than 5 resources.

## Version Awareness

- Check for deprecated APIs (e.g., `extensions/v1beta1` → `apps/v1`)
- For CRDs, ensure the apiVersion matches what's in the cluster
- Use `kubectl api-versions` to list available API versions

## Test Coverage Guidance

For detailed test scenarios and expected outputs, load:
```
Read references/validation_workflow.md
```

### Test Files

| Test File | Purpose | Expected Behavior |
|-----------|---------|-------------------|
| `deployment-test.yaml` | Valid standard K8s resource | All stages pass, no errors |
| `certificate-crd-test.yaml` | Valid CRD resource | CRD detected, context7 lookup performed, no errors |
| `comprehensive-test.yaml` | Multi-resource with intentional errors | Syntax error detected, partial parsing works, CRD found |

### Validation Paths to Test

1. **Happy Path** — `deployment-test.yaml`: All stages pass, 0 errors, 0 warnings
2. **CRD Detection** — `certificate-crd-test.yaml`: CRD detected, context7 MCP called, documentation retrieved
3. **Syntax Error Path** — `comprehensive-test.yaml`: yamllint catches error, kubeconform partial validation, dry-run blocked
4. **Multi-Resource Partial Parsing** — `comprehensive-test.yaml`: 2/3 resources validated, parse error for document 1
5. **No Cluster Access** — Any valid file without kubectl cluster configured: Server-side fails, falls back to client-side
6. **Missing Tools** — Remove a tool from PATH: `setup_tools.sh` reports missing, validation continues with available tools

### Expected Report Checklist

- [ ] Summary table with issue counts by severity
- [ ] Stage-by-stage status table (passed/failed/skipped)
- [ ] Document parsing table (for multi-resource files)
- [ ] Before/after code blocks for each issue
- [ ] Fix complexity indicators ([Simple], [Medium], [Complex])
- [ ] File-absolute line numbers
- [ ] "Next Steps" section

## Resources

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
