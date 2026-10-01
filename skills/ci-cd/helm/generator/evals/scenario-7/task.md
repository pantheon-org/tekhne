# Scenario 7: Chart with a ServiceMonitor and a CRD

## User Prompt

"Our chart should ship the Prometheus ServiceMonitor for the app, and we also bundle our own CRD called `Widget`. Add both to the chart and make sure users can validate their values."

## Repo State

An existing chart `widget-operator/` with a Deployment, a Service and a `values.yaml`. Prometheus operator documentation is not stored locally beyond the skill's CRD patterns reference.
