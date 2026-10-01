# Scenario 08: Generate Stage 10 Final Validation Report

## User Prompt

You have completed stages 1-9 of the helm-validator workflow on a chart called `monitoring-stack`. The results from each stage are summarised below:

- **Stage 1 (Tool Check):** helm v3.14.0, yamllint 1.35.1, kubeconform 0.6.4 all present. kubectl not available.
- **Stage 2 (Structure):** Chart.yaml present (apiVersion: v2). values.yaml present. templates/ present. Missing: .helmignore, NOTES.txt.
- **Stage 3 (Helm Lint):** 0 errors. 1 warning: `icon` field missing from Chart.yaml.
- **Stage 4 (Template Render):** 6 templates rendered successfully to ./rendered/.
- **Stage 5 (YAML Syntax):** 1 issue: trailing whitespace on line 14 of rendered/deployment.yaml.
- **Stage 6 (CRD Detection):** 1 CRD detected: `monitoring.coreos.com/v1 ServiceMonitor`. Documentation lookup performed — required fields: `selector`, `endpoints`.
- **Stage 7 (Schema Validation):** Deployment, Service, ConfigMap: valid. ServiceMonitor: "No schema found" (expected for CRD). Manual check using Stage 6 docs: `selector` present, `endpoints` present — passes.
- **Stage 8 (Dry-Run):** Skipped — no cluster access. Documented as limitation.
- **Stage 9 (Security):** Deployment has no pod securityContext. Container has no `allowPrivilegeEscalation: false`. Image tag is `v2.1.0` (pinned — good). No livenessProbe or readinessProbe. Resource limits missing for memory.
