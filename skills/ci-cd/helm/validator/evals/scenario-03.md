# Scenario 03: CRD Detection and Validation (Stage 6)

## User Prompt

You are given the following rendered manifest file from a chart's `crds/` directory:

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: my-tls-cert
  namespace: default
spec:
  secretName: my-tls-secret
  issuerRef:
    name: letsencrypt-prod
  dnsNames:
    - example.com
```

And a second rendered manifest from `templates/`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
spec:
  replicas: 1
  template:
    spec:
      containers:
        - name: app
          image: myapp:v1.2.3
```

## Your Task

Perform Stage 6 (CRD Detection and Documentation Lookup) of the helm-validator workflow.

1. Identify which of the two manifests contains a CRD (Custom Resource Definition usage) and which is a native Kubernetes resource.
2. For the detected CRD, describe the documentation lookup process you would follow: which tool to use first (context7 MCP), what query parameters to use, and what fallback to use if it is unavailable.
3. Based on the `cert-manager.io/v1 Certificate` spec, list the required fields that are present and identify any commonly required fields that appear to be missing (e.g., `duration`, `renewBefore`, `privateKey`).
4. Explain why kubeconform would show "No schema found" for this resource and what the correct approach is to validate it.

## Expected Behavior

1. Correctly identify the `Certificate` resource (cert-manager.io/v1) as a CRD and the `Deployment` as a native Kubernetes resource
2. Describe using context7 MCP (mcp__context7__resolve-library-id then mcp__context7__get-library-docs) as the preferred documentation lookup approach, with WebSearch as the stated fallback
3. Mention resolving the library ID with a query like 'cert-manager' and fetching docs with a topic such as 'Certificate spec'
4. Identify at least one field commonly required or strongly recommended for cert-manager Certificates (e.g., `duration`, `renewBefore`, or `usages`) that is absent from the provided spec
5. Explain that kubeconform reports "No schema found" for CRDs because they are not in the default Kubernetes schema

## Success Criteria

- **CRD correctly identified**: Agent identifies the Certificate resource (cert-manager.io/v1) as a CRD and the Deployment as a native Kubernetes resource
- **Documentation lookup process described**: Agent describes using context7 MCP (mcp__context7__resolve-library-id then mcp__context7__get-library-docs) as the preferred approach, with WebSearch as the stated fallback
- **Correct context7 query parameters stated**: Agent mentions resolving the library ID with a query like 'cert-manager' and fetching docs with a topic such as 'Certificate spec'
- **Missing or optional fields identified**: Agent identifies at least one field that is commonly required or strongly recommended for cert-manager Certificates (e.g., duration, renewBefore, or usages) that is absent from the provided spec
- **kubeconform schema gap explained**: Agent explains that kubeconform will report 'No schema found' for CRDs because they are not in the default Kubernetes schema and that the CRDs-catalog or manual validation via Stage 6 docs is required

## Failure Conditions

- The Certificate resource is not identified as a CRD (or is confused with a native Kubernetes resource)
- The Deployment is not correctly identified as a native Kubernetes resource
- The documentation lookup process does not mention context7 MCP or an equivalent tool, and does not name WebSearch as a fallback
- No commonly recommended cert-manager Certificate fields (duration, renewBefore, usages) are identified as missing from the provided spec
- The kubeconform "No schema found" behavior for CRDs is not explained
