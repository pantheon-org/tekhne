# Scenario 03: Internal Redis Cache Helm Chart

## User Prompt

The infrastructure team needs a Helm chart for an internal Redis cache deployment. Redis listens on port 6379 (not HTTP) and runs as a StatefulSet because it needs stable network identity. The chart will be used by multiple teams, so it needs to be flexible: node affinity, tolerations, and pod topology spread should all be configurable but optional — teams that don't need them shouldn't have to supply them.

A senior engineer reviewed a previous chart draft and flagged that all the Kubernetes resource names were hardcoded strings instead of using the chart's naming conventions, and that labels were duplicated manually across every resource instead of being defined once and referenced.

Produce the chart such that naming and labelling are handled consistently across all templates, and any optional scheduling configuration (nodeSelector, tolerations, affinity) is only rendered when actually provided.

## Output Specification

Produce a Helm chart directory named `redis-cache/` containing:
- `Chart.yaml`
- `values.yaml`
- `templates/_helpers.tpl`
- `templates/statefulset.yaml`
- `templates/service.yaml`

The `_helpers.tpl` must define named templates for at least the chart fullname and standard labels.

## Expected Behavior

1. Use `{{ include "<chart>.fullname" . }}` in statefulset.yaml and service.yaml for resource names — not hardcoded strings
2. Use `{{- include "<chart>.labels" . | nindent N }}` in at least one template to avoid manually duplicating label key-value pairs
3. Define `_helpers.tpl` with at least one named template using `{{- define "..." -}}` syntax
4. Use a non-HTTP port name for Redis (e.g., `redis`, `tcp-redis`, or `client`) — not `http`
5. Wrap `nodeSelector`, `tolerations`, and `affinity` in `{{- with .Values.nodeSelector }}` conditionals so they are only rendered when provided
6. Use `toYaml` to render complex optional objects and `nindent` (not `indent`) when including helpers

## Success Criteria

- **Helper for name**: statefulset.yaml and service.yaml use {{ include "<chart>.fullname" . }} (or equivalent named template) for the resource name, not hardcoded strings
- **Helper for labels**: At least one template file uses {{- include "<chart>.labels" . | nindent N }} (or equivalent) rather than manually duplicated label key-value pairs
- **Helpers tpl defined**: _helpers.tpl exists and defines at least one named template using {{- define "..." -}} syntax
- **Non-HTTP port name**: The service or statefulset port definition uses a name other than 'http' (e.g., 'redis', 'tcp-redis', or 'client') — not 'http'
- **Conditional nodeSelector**: statefulset.yaml wraps the nodeSelector block in a {{- with .Values.nodeSelector }} ... {{- end }} conditional (only rendered when non-empty)
- **Conditional tolerations**: statefulset.yaml wraps the tolerations block in a {{- with .Values.tolerations }} or equivalent conditional
- **toYaml for complex objects**: At least one optional block (nodeSelector, tolerations, or affinity) uses toYaml to render the value rather than manually templating each field
- **Resources block present**: The statefulset template includes a resources: block (or values.yaml defines resources with at least cpu/memory entries)
- **No hardcoded image tag**: values.yaml image tag is empty string or uses .Chart.AppVersion as fallback, NOT a literal pinned version or 'latest'
- **nindent used**: At least one template uses the nindent function (rather than indent) to handle newline + indentation when including helpers

## Failure Conditions

- Resource names in statefulset.yaml or service.yaml are hardcoded strings instead of using a named template helper
- Labels are manually duplicated across every resource instead of referencing a shared helper
- `_helpers.tpl` is absent or contains no named template definitions
- Redis port is named `http` instead of a semantically correct name like `redis`
- `nodeSelector`, `tolerations`, or `affinity` are always rendered regardless of whether they are provided in values.yaml
- Complex optional objects use manual field-by-field templating instead of `toYaml`
- No `resources:` block is present in the statefulset template
- `indent` is used instead of `nindent` when including helpers
