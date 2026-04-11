# Scenario 02: Complete Application Stack with Multi-Document YAML

## User Prompt

Your team needs to deploy a new web application to Kubernetes. The application requires a Deployment for the workload, a Service to expose it internally, and a ConfigMap to store non-sensitive configuration. The infrastructure team has requested manifests that follow a consistent structure for easier management.

Create a complete set of Kubernetes manifests for this application stack. The application is a Node.js web server running on port 3000 that needs configuration for database connection strings and feature flags.

Generate a single multi-document YAML file named `webapp-stack.yaml` containing three resources:

1. ConfigMap with sample configuration (database URL and feature flags)
2. Deployment for the web application (3 replicas)
3. Service to expose the application (ClusterIP type)

Application details:
- Name: webapp
- Namespace: apps
- Image: webapp:1.0.0
- Port: 3000
- Health endpoint: /healthz

Ensure all resources use consistent labeling that connects them as part of the same application stack.

## Expected Behavior

1. Order resources as: ConfigMap first, then Deployment, then Service
2. Include all three resource types: ConfigMap, Deployment, and Service
3. Use at least 2 common label keys with matching values across all three resources
4. Ensure the Service selector labels exactly match a subset of the Deployment pod template labels
5. Reference the ConfigMap from the Deployment via `envFrom`, `env`, or `volumeMounts`
6. Specify `namespace: apps` on all three resources
7. Set `type: ClusterIP` on the Service
8. Map the Service port/targetPort to container port 3000 correctly
9. Use `---` separator between YAML documents
10. Include at least one health probe (liveness or readiness) in the Deployment

## Success Criteria

- **Correct resource order**: Resources ordered: ConfigMap first, then Deployment, then Service
- **Three resources present**: File contains exactly ConfigMap, Deployment, and Service (3 resources)
- **Consistent labels**: All three resources share at least 2 common label keys with matching values
- **Service selector matches**: Service selector labels exactly match a subset of Deployment pod template labels
- **ConfigMap referenced**: Deployment references the ConfigMap via `envFrom` or `env` or `volumeMounts`
- **Namespace consistent**: All three resources specify the same namespace: apps
- **Service type ClusterIP**: Service has `type: ClusterIP`
- **Service port mapping**: Service port/targetPort correctly maps to container port 3000
- **Multi-document format**: Uses `---` separator between YAML documents
- **Probes included**: Deployment includes at least one health probe (liveness or readiness)

## Failure Conditions

- Resources are not in the correct order (ConfigMap → Deployment → Service)
- Fewer than three resource types are present (ConfigMap, Deployment, and Service all required)
- Labels are inconsistent across resources — Service selector does not match pod template labels
- ConfigMap is not referenced from the Deployment
- Namespace is not consistent across all three resources
- Service type is not ClusterIP, or port mapping does not target port 3000
- `---` document separators are absent in the multi-document YAML
