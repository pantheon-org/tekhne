# Scenario 3: Looks up the CRD before generating a cert-manager Certificate

## User Prompt

"Create a cert-manager Certificate for api.example.com."

## Repository State

The cluster runs cert-manager v1.14. A ClusterIssuer named `letsencrypt-prod` already exists. The resource belongs in the `shop` namespace and the TLS secret should be called `api-example-tls`.
