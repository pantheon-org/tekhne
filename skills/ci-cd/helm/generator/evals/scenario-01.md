# Scenario 01: New Microservice Helm Chart

## User Prompt

Your team is deploying a new Node.js API service called `user-api` to Kubernetes. The service runs on port 3000, exposes a `/healthz` liveness endpoint and a `/ready` readiness endpoint, and uses the `company/user-api` container image versioned at `v2.1.0`. The ops team wants a complete Helm chart that can be installed directly with `helm install`.

The platform team has complained that previous charts caused node pressure evictions because resource constraints were missing, and that image tags were baked into charts making rollbacks difficult. The new chart must address both of these operational concerns.

## Output Specification

Produce a complete Helm chart directory named `user-api/` containing at minimum:
- `Chart.yaml`
- `values.yaml`
- `templates/deployment.yaml`
- `templates/service.yaml`

The chart should be self-contained and ready to install. Document your intended CPU/memory defaults inside `values.yaml`.

## Expected Behavior

1. Set the image tag in `values.yaml` to an empty string or a variable reference (not a literal `v2.1.0`) so it can be overridden at install time and rollbacks are traceable
2. Use `.Chart.AppVersion` as the fallback when the tag value is empty
3. Define a `resources:` block with both `requests:` and `limits:` for CPU and memory
4. Add inline documentation comments to `values.yaml` for at least 3 distinct settings
5. Define `livenessProbe` and `readinessProbe` blocks in the deployment template
6. Include a `securityContext` section (pod-level or container-level)
7. Split the image into `repository:` and `tag:` sub-keys in `values.yaml` and group related settings under named keys

## Success Criteria

- **No hardcoded image tag**: The image tag in values.yaml is set to an empty string or a variable reference (e.g., tag: ""), NOT a literal version string like 'v2.1.0' or 'latest'
- **appVersion default**: Chart.yaml includes an appVersion field, and/or the deployment template falls back to .Chart.AppVersion when the tag value is empty
- **Resources block present**: The deployment template or values.yaml contains a resources: block with at least one CPU or memory entry under both requests and limits
- **Both requests and limits**: The resources block specifies values under BOTH requests: and limits: (not just one of them)
- **values.yaml comments**: values.yaml contains inline documentation comments (lines starting with #) describing the purpose of at least 3 distinct settings
- **Liveness probe defined**: The deployment template includes a livenessProbe block pointing to a path or exec command
- **Readiness probe defined**: The deployment template includes a readinessProbe block pointing to a path or exec command
- **Security context present**: Either values.yaml or the deployment template includes a securityContext section (pod-level or container-level)
- **Image repository separate**: In values.yaml the image is split into at least repository: and tag: sub-keys, rather than a single combined image: string
- **Related settings grouped**: values.yaml groups related settings under named keys (e.g., image:, resources:, probes: or similar) rather than a flat list of top-level keys

## Failure Conditions

- The image tag is hardcoded as `v2.1.0` or `latest` in values.yaml
- Chart.yaml lacks an appVersion field and the deployment template does not fall back to .Chart.AppVersion
- The resources block is absent or specifies only requests or only limits (not both)
- values.yaml has fewer than 3 inline documentation comment lines
- No livenessProbe block is defined in the deployment template
- No readinessProbe block is defined in the deployment template
- No securityContext section is present anywhere in the chart
- The image is stored as a single combined string in values.yaml instead of separate repository: and tag: sub-keys
