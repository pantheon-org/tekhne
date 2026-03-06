# Eval Scenario: Generate Production-Ready Deployment

## Objective
Generate a complete Deployment manifest with best practices for a web application.

## Task
Create a Kubernetes Deployment YAML for:
- Application: `myapp`
- Image: `myorg/myapp:1.2.3`
- Namespace: `production`
- Replicas: 3
- Container port: 8080
- Health check endpoint: `/health`

## Expected Output
Generated YAML must include:
- Resource requests and limits
- Liveness and readiness probes
- Recommended labels (app.kubernetes.io/*)
- SecurityContext (non-root user)
- Explicit namespace
- Specific image tag (not `latest`)

## Success Criteria
- [ ] Uses apps/v1 apiVersion (not deprecated)
- [ ] Includes resource requests/limits on all containers
- [ ] Configures both liveness and readiness probes
- [ ] Uses recommended label structure
- [ ] Specifies namespace explicitly
- [ ] Uses specific image tag, not `latest`
- [ ] Validates YAML using k8s-yaml-validator before delivery
- [ ] Does NOT hardcode secrets in the manifest

## Validation
Generated YAML must pass kubeconform validation and follow all anti-patterns from the skill.
