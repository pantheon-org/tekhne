# Scenario 05: TLS Certificate Automation with cert-manager CRD

## User Prompt

Your platform team uses cert-manager to automatically provision TLS certificates for applications. A new API service needs a TLS certificate for its public domain. The security team has already configured a ClusterIssuer named `letsencrypt-prod` that the certificate should use.

Create a cert-manager Certificate resource that will automatically provision and renew a TLS certificate for the API domain. The certificate should be stored in a Kubernetes Secret that can be referenced by Ingress resources.

Generate a file named `api-certificate.yaml` containing a cert-manager Certificate CRD with the following specifications:

- Certificate name: api-tls
- Domain: api.company.com
- Namespace: production
- Secret name: api-tls-secret (where the certificate will be stored)
- Issuer: ClusterIssuer named letsencrypt-prod

Also create a file named `certificate-usage.md` that explains:
- What this Certificate resource does
- How applications will consume the generated secret
- The correct API version for cert-manager Certificate resources

## Expected Behavior

1. Use `apiVersion: cert-manager.io/v1`
2. Use `kind: Certificate`
3. Include `spec.secretName: api-tls-secret`
4. Include a `spec.issuerRef` section
5. Set `issuerRef.kind` to `ClusterIssuer` (not `Issuer`)
6. Set `issuerRef.name` to `letsencrypt-prod`
7. Include `spec.dnsNames` with `api.company.com`
8. Set `metadata.namespace: production`
9. Set `metadata.name: api-tls`
10. Include `certificate-usage.md` explaining the Certificate purpose or how to consume the secret

## Success Criteria

- **Correct API group**: Uses `apiVersion: cert-manager.io/v1`
- **Correct kind**: Uses `kind: Certificate`
- **SecretName field**: Includes `spec.secretName: api-tls-secret`
- **IssuerRef present**: Includes `spec.issuerRef` section
- **ClusterIssuer kind**: `issuerRef.kind` is `ClusterIssuer` (not `Issuer`)
- **Issuer name correct**: `issuerRef.name` is `letsencrypt-prod`
- **DNS names specified**: Includes `spec.dnsNames` with `api.company.com`
- **Production namespace**: `metadata.namespace: production`
- **Certificate name**: `metadata.name: api-tls`
- **Usage documented**: `certificate-usage.md` explains Certificate purpose or how to consume the secret

## Failure Conditions

- Wrong `apiVersion` is used (not `cert-manager.io/v1`)
- `kind: Certificate` is not used
- `spec.secretName` is missing or set to the wrong value
- `spec.issuerRef` is absent
- `issuerRef.kind` is `Issuer` instead of `ClusterIssuer`
- `issuerRef.name` is not `letsencrypt-prod`
- `spec.dnsNames` does not include `api.company.com`
- Namespace is not `production` or certificate name is not `api-tls`
