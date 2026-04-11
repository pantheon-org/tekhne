# Scenario 02: CRD Detection and Validation Workflow

## User Prompt

Your platform team works with various Custom Resource Definitions (CRDs) from different operators like cert-manager, ArgoCD, and Istio. When validating manifests that include CRDs, standard validation tools may not have the schemas available, requiring additional steps to verify correctness.

Create a validation guide that demonstrates how to handle CRD validation, including detecting CRDs in manifests and finding their specifications when schemas aren't available locally.

The following manifest contains both a standard Kubernetes resource and a CRD:

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: api-cert
spec:
  secretName: api-tls
  issuerRef:
    name: letsencrypt
    kind: ClusterIssuer
  dnsNames:
  - api.example.com
---
apiVersion: v1
kind: Service
metadata:
  name: api-service
spec:
  selector:
    app: api
  ports:
  - port: 443
    targetPort: 8443
```

Create these files:

1. `crd-validation-guide.md` — Documentation that explains:
   - How to detect whether a manifest contains CRDs vs standard Kubernetes resources
   - What to do when CRD schemas aren't available to validation tools
   - Where to find CRD specifications and required fields
   - How to validate CRD fields manually when automated tools can't help

2. `detect-crds.sh` — A bash script that analyzes a YAML file and identifies any CRD resources present, extracting their `apiVersion` and `kind`. The script should output information about detected CRDs in a structured format.

## Expected Behavior

1. Guide explains that CRDs are identified by non-standard `apiVersion` groups (e.g., `cert-manager.io`)
2. Script parses YAML to extract `apiVersion` and `kind` from each resource
3. Script handles multiple YAML documents separated by `---`
4. Script output identifies `Certificate` as a CRD (`cert-manager.io/v1`)
5. Script or guide distinguishes `Service` (standard) from `Certificate` (CRD)
6. Guide mentions querying documentation or specs when CRD schemas are unavailable
7. Guide explains checking CRD required fields (e.g., `secretName`, `issuerRef` for Certificate)
8. Detection output includes full `apiVersion` (not just group) for accurate schema lookup
9. Script outputs results in structured format (JSON, table, or clear key-value pairs)
10. Guide mentions fallback strategies when CRD schema is not found by `kubeconform`

## Success Criteria

- **CRD identification**: Guide explains that CRDs are identified by non-standard `apiVersion` groups (e.g., `cert-manager.io`)
- **Detection script**: Script parses YAML to extract `apiVersion` and `kind` from each resource
- **Multi-document parsing**: Script handles multiple YAML documents separated by `---`
- **Correct CRD identified**: Script output identifies `Certificate` as a CRD (`cert-manager.io/v1`)
- **Standard resource distinguished**: Script or guide distinguishes `Service` (standard) from `Certificate` (CRD)
- **Documentation lookup**: Guide mentions querying documentation or specs when CRD schemas unavailable
- **Required fields check**: Guide explains checking CRD required fields (e.g., `secretName`, `issuerRef` for Certificate)
- **Version included**: Detection output includes full `apiVersion` (not just group) for accurate schema lookup
- **Structured output**: Script outputs results in structured format (JSON, table, or clear key-value pairs)
- **Schema fallback**: Guide mentions fallback strategies when CRD schema not found by `kubeconform`

## Failure Conditions

- Guide does not explain how to distinguish CRDs from standard Kubernetes resources by their `apiVersion`
- Script does not parse `apiVersion` and `kind` from each resource
- Script cannot handle multi-document YAML files separated by `---`
- `Certificate` from `cert-manager.io/v1` is not identified as a CRD
- Standard `Service` is not distinguished from `Certificate` (CRD)
- No guidance is provided on how to validate CRD fields manually or find CRD specifications
