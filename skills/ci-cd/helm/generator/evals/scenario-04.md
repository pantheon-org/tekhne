# Scenario 04: Config-Driven Web App with Hot Reload

## User Prompt

An e-commerce platform runs a `storefront` web application whose runtime configuration (feature flags, API endpoints) is stored in a Kubernetes ConfigMap and sensitive settings (payment gateway token) in a Kubernetes Secret. When either the ConfigMap or the Secret changes, the application pods need to restart automatically — but currently the team has to manually perform rolling restarts every time a config value is updated, which is error-prone and causes downtime windows.

The platform engineer wants the Helm chart wired so that config and secret changes automatically trigger pod restarts without any manual intervention. The chart should also conditionally enable or disable ConfigMap and Secret creation based on whether they are needed, since some installations skip the Secret entirely.

## Output Specification

Produce a Helm chart directory named `storefront/` containing:
- `Chart.yaml`
- `values.yaml` — with configMap and secret sections that can each be enabled/disabled
- `templates/deployment.yaml`
- `templates/configmap.yaml`
- `templates/secret.yaml`

The deployment must be set up to detect changes in the ConfigMap and Secret and trigger pod restarts. Both configmap.yaml and secret.yaml should only be rendered when their respective feature is enabled.

## Expected Behavior

1. Add a checksum annotation to the deployment pod template using `sha256sum` so pod restarts are triggered when the ConfigMap or Secret content changes (e.g., `checksum/config: {{ include (print $.Template.BasePath "/configmap.yaml") . | sha256sum }}`)
2. Wrap `configmap.yaml` in an `{{- if .Values.configMap.enabled }}` conditional so it is only rendered when enabled
3. Wrap `secret.yaml` in an `{{- if .Values.secret.enabled }}` conditional so it is only rendered when enabled
4. Define `configMap.enabled` and `secret.enabled` boolean fields in `values.yaml`
5. Use a named template helper for the resource name rather than a hardcoded string
6. Reference a labels helper in at least one resource template

## Success Criteria

- **Checksum annotation present**: deployment.yaml pod template annotations include a checksum annotation using sha256sum (e.g., checksum/config: {{ include ... | sha256sum }})
- **Checksum references configmap**: The checksum annotation uses print $.Template.BasePath "/configmap.yaml" or equivalent to hash the configmap template contents
- **configMap enabled guard**: configmap.yaml is wrapped in an {{- if .Values.configMap.enabled }} conditional so it is only rendered when enabled
- **secret enabled guard**: secret.yaml is wrapped in an {{- if .Values.secret.enabled }} conditional so it is only rendered when enabled
- **values.yaml enabled flags**: values.yaml contains configMap.enabled and secret.enabled boolean fields (or equivalent structure)
- **Helper used for name**: deployment.yaml uses a named template (include) for the resource name rather than a hardcoded string
- **Labels via helper**: At least one resource template references a labels helper (include "<chart>.labels") rather than inlining label key-value pairs
- **Resources block present**: deployment.yaml container spec includes a resources: block with at least requests or limits defined
- **No hardcoded image tag**: values.yaml image tag is empty string or references appVersion, NOT a hardcoded literal version string
- **values.yaml commented**: values.yaml contains at least 3 comment lines (starting with #) explaining individual settings

## Failure Conditions

- No checksum annotation is present in the deployment pod template
- The checksum annotation does not reference the ConfigMap template for hashing
- `configmap.yaml` renders unconditionally instead of being gated by `{{- if .Values.configMap.enabled }}`
- `secret.yaml` renders unconditionally instead of being gated by `{{- if .Values.secret.enabled }}`
- `values.yaml` lacks `configMap.enabled` or `secret.enabled` boolean fields
- Resource names in deployment.yaml are hardcoded strings instead of using a named template helper
- No labels helper is referenced in any resource template
- No `resources:` block is present in the deployment container spec
