# Eval Scenario: Generate CRD Instance with Documentation Lookup

## Objective
Generate a cert-manager Certificate CRD instance with proper field validation.

## Task
Create a Certificate resource for:
- Domain: `myapp.example.com`
- Issuer: `letsencrypt-prod` (ClusterIssuer)
- Secret name: `myapp-tls-secret`
- Namespace: `production`

## Expected Actions
1. Query CRD documentation: `tessl_query_library_docs: query: "cert-manager Certificate v1 spec fields"`
2. Generate YAML with all required fields
3. Validate against CRD schema

## Expected Output
```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: myapp-tls
  namespace: production
spec:
  secretName: myapp-tls-secret
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsNames:
  - myapp.example.com
```

## Success Criteria
- [ ] Queries library documentation for CRD spec
- [ ] Includes all required fields (secretName, issuerRef, dnsNames)
- [ ] Uses correct apiVersion (cert-manager.io/v1)
- [ ] Specifies issuerRef.kind correctly
- [ ] Validates with k8s-yaml-validator
- [ ] Handles fallback to WebSearch if docs insufficient

## Validation
Agent must demonstrate CRD documentation lookup workflow from Step 2 of the skill.
