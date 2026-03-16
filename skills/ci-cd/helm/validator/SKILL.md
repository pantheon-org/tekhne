---
name: helm-validator
description: Comprehensive toolkit for validating, linting, testing, and analyzing Helm charts and their rendered Kubernetes resources. Use this skill when working with Helm charts, validating templates, debugging chart issues, working with Custom Resource Definitions (CRDs) that require documentation lookup, or checking Helm best practices.
---

# Helm Chart Validator & Analysis Toolkit

## Overview

This skill provides a comprehensive validation and analysis workflow for Helm charts, combining Helm-native linting, template rendering, YAML validation, schema validation, CRD documentation lookup, and security best practices checking.

**IMPORTANT: This is a READ-ONLY validator.** It analyzes charts and proposes improvements but does NOT modify any files. All proposed changes are listed in the final summary for the user to review and apply manually or via the helm-generator skill.

## Validation & Testing Workflow

Follow this sequential workflow. Each stage catches different types of issues.

### Stage 1: Tool Check

```bash
bash scripts/setup_tools.sh
```

Required: `helm` (v3+), `yamllint`, `kubeconform`, `kubectl` (optional). If tools are missing, provide installation instructions and ask the user before proceeding.

### Stage 2: Chart Structure Validation

```bash
bash scripts/validate_chart_structure.sh <chart-directory>
```

Validates required files (Chart.yaml, values.yaml, templates/) and recommended files (_helpers.tpl, NOTES.txt, .helmignore).

### Stage 3: Helm Lint

```bash
helm lint <chart-directory> --strict
# Optional: --values <file>, --set key=value, --debug
```

### Stage 4: Template Rendering

```bash
helm template <release-name> <chart-directory> \
  --values <values-file> \
  --debug \
  --output-dir ./rendered
```

Useful flags: `--validate`, `--include-crds`, `--is-upgrade`, `--kube-version 1.28.0`, `--show-only templates/<file>.yaml`.

### Stage 5: YAML Syntax Validation

```bash
yamllint -c assets/.yamllint ./rendered/*.yaml
```

Fix template-generated YAML issues in the source template — not the rendered output.

### Stage 6: CRD Detection and Documentation Lookup

```bash
bash scripts/detect_crd_wrapper.sh <chart-directory>/crds/*.yaml
bash scripts/detect_crd_wrapper.sh ./rendered/*.yaml
```

Output:
```json
[{"kind": "Certificate", "apiVersion": "cert-manager.io/v1", "group": "cert-manager.io", "version": "v1", "isCRD": true}]
```

**For each detected CRD:**

1. **Try context7 MCP first (preferred):**
   - `mcp__context7__resolve-library-id` with the CRD project name (e.g. `"cert-manager"`)
   - `mcp__context7__get-library-docs` with the resolved ID, topic (e.g. `"Certificate spec"`), tokens: 5000

2. **Fallback to WebSearch:**
   ```
   "<Kind>" "<group>" kubernetes CRD "<version>" documentation spec
   ```

Extract required fields, types, validation rules, and version-specific deprecations.

### Stage 7: Schema Validation

```bash
kubeconform \
  -schema-location default \
  -schema-location 'https://raw.githubusercontent.com/datreeio/CRDs-catalog/main/{{.Group}}/{{.ResourceKind}}_{{.ResourceAPIVersion}}.json' \
  -summary -verbose \
  ./rendered/*.yaml
```

Add `-strict` for production, `-ignore-missing-schemas` for internal CRDs, `-kubernetes-version 1.28.0` for version pinning. "No schema found" for CRDs is expected — validate those manually using Stage 6 docs.

### Stage 8: Cluster Dry-Run (if available)

```bash
helm install <release-name> <chart-directory> --dry-run --debug --values <values-file>
helm upgrade <release-name> <chart-directory> --dry-run --debug --values <values-file>
# With helm-diff plugin:
helm diff upgrade <release-name> <chart-directory>
```

Catches admission controller rejections, policy violations, quota issues, and webhook errors. Skip and document if no cluster access.

### Stage 9: Security Best Practices Check (MANDATORY)

Analyze rendered Deployment/Pod templates:

```bash
grep -l "securityContext" ./rendered/*.yaml
grep -l "resources:" ./rendered/*.yaml
grep "image:.*:latest" ./rendered/*.yaml
```

**Required checks:**
- Pod securityContext: `runAsNonRoot`, `runAsUser`, `fsGroup`
- Container securityContext: `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem`, `capabilities.drop: [ALL]`
- Resource limits/requests for cpu and memory
- No `:latest` image tags
- Liveness and readiness probes present

### Stage 10: Final Report (MANDATORY)

**This stage is MANDATORY even if all validations pass.**

#### Step 1: Load References (when issues found)

```
Read references/helm_best_practices.md
Read references/k8s_best_practices.md
```

#### Step 2: Validation Summary Table

```
| Stage | Status | Issues |
|-------|--------|--------|
| 1. Tool Check         | ✅ Passed  | All tools available       |
| 2. Structure          | ⚠️ Warning | Missing: .helmignore      |
| 3. Helm Lint          | ✅ Passed  | 0 errors                  |
| 4. Template Render    | ✅ Passed  | 5 templates rendered      |
| 5. YAML Syntax        | ✅ Passed  | No errors                 |
| 6. CRD Detection      | ✅ Passed  | 1 CRD documented          |
| 7. Schema Validation  | ✅ Passed  | All resources valid       |
| 8. Dry-Run            | ✅ Passed  | No cluster errors         |
| 9. Security Check     | ⚠️ Warning | Missing securityContext   |
```

#### Step 3: Categorize Issues

- **❌ Errors (must fix):** Template syntax errors, missing required fields, schema failures, dry-run failures
- **⚠️ Warnings (should fix):** Deprecated APIs, missing securityContext, missing resource limits, `:latest` tag, missing recommended files
- **ℹ️ Info:** Missing values.schema.json, missing README.md, optimization opportunities

#### Step 4: Proposed Changes (DO NOT APPLY)

For each issue:
- File path and line number
- Before/after code blocks
- Explanation of why the change is recommended

#### Step 5: Automation Opportunities

**Common fixes:**
- Missing `_helpers.tpl`: `bash scripts/generate_helpers.sh <chart>`
- Missing `.helmignore`: Copy from `assets/.helmignore`
- Missing `values.schema.json`: Copy/customize from `assets/values.schema.json`
- Use `include` instead of `template` for pipeline support
- Add `nindent` for proper YAML indentation
- Add `default` function for optional values
- Add `required` for critical values

#### Step 6: Final Summary

```
## Validation Summary
**Chart:** <chart-name>
**Status:** ⚠️ Warnings Found  (or ✅ Ready for Deployment)
**Issues:** Errors: X  Warnings: Y  Info: Z
**Proposed Changes:** N changes recommended
**Next Steps:**
1. Review proposed changes above
2. Apply manually or use helm-generator skill
3. Re-run validation to confirm fixes
```

## Helm Templating Reference

For complex templating tasks, load the dedicated reference:

```
Read references/template_functions.md
```

Standard helper patterns (`templates/_helpers.tpl`) — including `fullname`, `labels`, and `selectorLabels` definitions — are documented in `references/template_functions.md`.

**Key template functions:** `required`, `default`, `quote`, `include`, `tpl`, `toYaml`, `merge`, `lookup` — see `references/template_functions.md` for full reference with examples.

## macOS Extended Attributes Issue

**Symptom:** Helm reports "Chart.yaml file is missing" even though the file exists.

**Diagnosis & Fix:**
```bash
xattr /path/to/chart/Chart.yaml      # check for attributes
xattr -cr /path/to/chart/            # remove all recursively
```

**Prevention:** Use `helm create` as a base, or create files with shell heredocs (`cat > file << 'EOF'`).

## Error Handling

- **Tool not available:** Run `scripts/setup_tools.sh`, skip optional stages, document what was skipped.
- **Template errors:** Show file and line number, check values.yaml definitions, use `--debug`.
- **Cluster access issues:** Fall back to `kubectl apply --dry-run=server -f ./rendered/`, document limitations.
- **CRD schema not found:** Use kubeconform CRD catalog; fall back to `kubectl explain <kind>`.
- **Stage failures:** Continue to next stage, collect all errors, present together in Stage 10.

## Version Awareness

- Ensure `apiVersion: v2` in Chart.yaml (Helm 3+)
- Check for deprecated Kubernetes APIs
- Use `kubectl api-versions` to list available API versions
- Set `kubeVersion` constraint in Chart.yaml when needed
- Target specific K8s version with `--kube-version` in helm template / kubeconform

## References

### scripts/
- `setup_tools.sh`: Check/install required tools
- `validate_chart_structure.sh`: Validate chart directory structure
- `detect_crd_wrapper.sh`: Detect CRDs in YAML files (manages Python venv)
- `detect_crd.py`: Parse YAML to identify CRDs, output JSON
- `generate_helpers.sh`: Generate standard `_helpers.tpl`

### references/
- `helm_best_practices.md`: Chart structure, template conventions, values organization
- `k8s_best_practices.md`: Metadata, labels, resource limits, security context
- `template_functions.md`: All built-in Helm/Sprig functions with examples, standard helper patterns

### assets/
- `.helmignore`: Standard ignore patterns for chart packaging
- `.yamllint`: Pre-configured yamllint rules for Kubernetes YAML
- `values.schema.json`: Example JSON Schema template for values validation
