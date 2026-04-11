# Scenario 01: Production Microservice Deployment Manifest

## User Prompt

Your team is deploying a new payment processing microservice to production. The application runs on port 8080, requires specific resource allocation due to processing load, and needs health monitoring to ensure high availability. The security team requires all production workloads to follow least-privilege principles.

Create a production-ready Kubernetes Deployment manifest that the platform team can use for the initial rollout. The manifest must follow industry best practices for resource management, health monitoring, and security.

Generate a file named `payment-service-deployment.yaml` containing a complete Deployment manifest with the following requirements:

- Application: payment-processor
- Image: payment-app:2.1.0
- Port: 8080
- Production namespace: payments
- 3 replicas
- Health endpoint: /health (for both liveness and readiness)
- Include appropriate metadata labels

The manifest should be ready for production deployment without requiring modifications for basic security or reliability concerns.

## Expected Behavior

1. Include `resources.requests` with memory and cpu values in the container spec
2. Include `resources.limits` with memory and cpu values in the container spec
3. Include a `livenessProbe` with httpGet to the `/health` endpoint on port 8080
4. Include a `readinessProbe` with httpGet to the `/health` endpoint on port 8080
5. Include a `securityContext` with `runAsNonRoot: true` or `runAsUser` set to a non-zero value
6. Use the image `payment-app:2.1.0` (not `:latest`)
7. Specify `namespace: payments` in the metadata
8. Use at least 3 `app.kubernetes.io/*` format labels (name, instance, version, component, part-of, or managed-by)
9. Use `apps/v1` for the Deployment API version (not deprecated `extensions/v1beta1`)
10. Include `initialDelaySeconds` or `periodSeconds` on at least one probe

## Success Criteria

- **Resource requests defined**: Includes `resources.requests` with memory and cpu values
- **Resource limits defined**: Includes `resources.limits` with memory and cpu values
- **Liveness probe configured**: Includes `livenessProbe` with `httpGet` to `/health` endpoint on port 8080
- **Readiness probe configured**: Includes `readinessProbe` with `httpGet` to `/health` endpoint on port 8080
- **Non-root security**: Includes `securityContext` with `runAsNonRoot` or `runAsUser` set to non-zero
- **Specific version tag**: Uses image `payment-app:2.1.0` (NOT `:latest`)
- **Namespace specified**: Includes `namespace: payments` in metadata
- **Recommended labels**: Uses at least 3 of the `app.kubernetes.io/*` label format (name, instance, version, component, part-of, managed-by)
- **Current API version**: Uses `apps/v1` for Deployment (not deprecated `extensions/v1beta1`)
- **Probe timing configured**: Includes `initialDelaySeconds` or `periodSeconds` on at least one probe

## Failure Conditions

- Resource requests or limits are absent from the container spec
- Liveness or readiness probe is missing or does not point to the `/health` endpoint on port 8080
- No `securityContext` is included, or the container runs as root
- Image uses `:latest` instead of a specific version tag
- Namespace is not specified or is incorrect
- Fewer than 3 `app.kubernetes.io/*` labels are used
- Deprecated `extensions/v1beta1` API version is used instead of `apps/v1`
