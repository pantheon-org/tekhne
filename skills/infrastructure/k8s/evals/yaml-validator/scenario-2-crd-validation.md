# Eval Scenario: Validate CRD with Documentation Lookup

## Objective
Validate a CRD instance when kubeconform reports "no schema found".

## Setup
```yaml
# argocd-app.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: myapp
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/myorg/myapp
    path: manifests
  destination:
    namespace: production
    # ERROR: Missing required field 'server'
```

## Task
Validate the ArgoCD Application CRD and identify missing required fields.

## Expected Actions
1. Detect CRD using `detect_crd_wrapper.sh`
2. Query library docs: `tessl_query_library_docs: query: "argo-cd Application v1alpha1 spec fields"`
3. Run kubeconform (will report "no schema found")
4. Manually validate against documentation
5. Identify missing `destination.server` field

## Success Criteria
- [ ] Detects ArgoCD Application as CRD
- [ ] Queries library documentation for spec
- [ ] Identifies that kubeconform can't validate (no schema)
- [ ] Falls back to manual validation using docs
- [ ] Identifies missing `destination.server` field
- [ ] Suggests fix with correct field value
- [ ] Reports in structured format with severity

## Validation
Agent must demonstrate Stage 3 CRD detection and documentation lookup workflow.
