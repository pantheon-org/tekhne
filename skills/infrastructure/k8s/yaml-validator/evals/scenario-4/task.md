# Scenario 4: Does not accept a missing CRD schema as a pass

## User Prompt

"Validate cert.yaml. It's a cert-manager Certificate and our cluster has cert-manager v1.14."

## Repository State

`cert.yaml` contains this Certificate:

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: api-tls
  namespace: shop
spec:
  secretName: api-tls
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsName: api.example.com
```

The default kubeconform schema location has no schema for Certificate. yamllint, kubeconform and kubectl are installed and the cluster is reachable.
