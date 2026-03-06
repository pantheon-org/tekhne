# CRD Validation Workflow

## Problem Description

Your platform team works with various Custom Resource Definitions (CRDs) from different operators like cert-manager, ArgoCD, and Istio. When validating manifests that include CRDs, standard validation tools may not have the schemas available, requiring additional steps to verify correctness.

Create a validation guide that demonstrates how to handle CRD validation, including detecting CRDs in manifests and finding their specifications when schemas aren't available locally.

## Output Specification

Create these files:

1. `crd-validation-guide.md` - Documentation that explains:
   - How to detect whether a manifest contains CRDs vs standard Kubernetes resources
   - What to do when CRD schemas aren't available to validation tools
   - Where to find CRD specifications and required fields
   - How to validate CRD fields manually when automated tools can't help

2. `detect-crds.sh` - A bash script that analyzes a YAML file and identifies any CRD resources present, extracting their apiVersion and kind. The script should output information about detected CRDs in a structured format.

## Input Files

The following manifest contains both standard Kubernetes resources and a CRD. Extract it before beginning.

=============== FILE: inputs/mixed-resources.yaml ===============
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
---
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
=============== END FILE ===============
