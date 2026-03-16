# Internal Redis Cache Helm Chart

## Problem/Feature Description

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
