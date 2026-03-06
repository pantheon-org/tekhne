# TLS Certificate Automation

## Problem Description

Your platform team uses cert-manager to automatically provision TLS certificates for applications. A new API service needs a TLS certificate for its public domain. The security team has already configured a ClusterIssuer named `letsencrypt-prod` that the certificate should use.

Create a cert-manager Certificate resource that will automatically provision and renew a TLS certificate for the API domain. The certificate should be stored in a Kubernetes Secret that can be referenced by Ingress resources.

## Output Specification

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
