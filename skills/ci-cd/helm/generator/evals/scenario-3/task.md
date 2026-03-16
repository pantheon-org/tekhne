# Config-Driven Web App with Hot Reload

## Problem/Feature Description

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
