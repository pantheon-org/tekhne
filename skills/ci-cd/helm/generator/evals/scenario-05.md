# Scenario 05: Migrate Raw Kubernetes Manifests to Helm

## User Prompt

A startup has been running their `notification-service` on Kubernetes using raw YAML manifests managed by kubectl. They have decided to adopt Helm so they can version the chart, parameterise environment differences, and share it with other teams. The raw manifests are provided below.

The existing manifests have several issues that the migration should fix: image tags are hardcoded, all resource names are hardcoded strings, there are no labels following Kubernetes recommended conventions, and no resource limits are set. The migration should produce a proper Helm chart that addresses these issues.

Additionally, the team wants a simple shell script `validate.sh` that documents their intended validation workflow for the chart — covering both static linting and rendered manifest checking.

## Output Specification

Produce:
1. A Helm chart directory named `notification-service/` with standard chart files including `_helpers.tpl`.
2. A `validate.sh` script at the repo root that shows the team's chart validation steps.

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/deployment.yaml ===============
apiVersion: apps/v1
kind: Deployment
metadata:
  name: notification-service
  labels:
    app: notification-service
spec:
  replicas: 2
  selector:
    matchLabels:
      app: notification-service
  template:
    metadata:
      labels:
        app: notification-service
    spec:
      containers:
      - name: notification-service
        image: company/notification-service:v3.0.1
        ports:
        - containerPort: 8080
        env:
        - name: LOG_LEVEL
          value: "info"
        - name: SMTP_PASSWORD
          value: "hardcoded-secret-do-not-use"

=============== FILE: inputs/service.yaml ===============
apiVersion: v1
kind: Service
metadata:
  name: notification-service
spec:
  selector:
    app: notification-service
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP

## Expected Behavior

1. Replace the hardcoded resource name `notification-service` with `{{ include "<chart>.fullname" . }}` or equivalent helper
2. Replace the minimal `app: notification-service` label with a full Kubernetes recommended label set via a named labels helper
3. Create `_helpers.tpl` defining at least a fullname and labels named template
4. Set the image tag in `values.yaml` to an empty string or `.Chart.AppVersion` fallback — not `v3.0.1` or `latest`
5. Move the SMTP_PASSWORD value out of the raw template — either reference it via a Kubernetes Secret or move the placeholder to `values.yaml`
6. Add a `resources:` block that was absent in the original manifest
7. Include a `helm template` command in `validate.sh` and pipe its output to `kubeval` or `kubeconform` for schema validation
8. Use `toYaml` or `nindent` when rendering complex values in templates

## Success Criteria

- **Helper used for name**: The Helm deployment template uses {{ include "<chart>.fullname" . }} or equivalent helper for the resource name — NOT the hardcoded string 'notification-service'
- **Standard labels helper**: At least one template uses {{ include "<chart>.labels" . }} or equivalent named template for labels, rather than manually listing app: label only
- **Helpers tpl exists**: _helpers.tpl exists and defines at least a fullname and labels named template
- **No hardcoded image tag**: values.yaml image tag is empty string or uses appVersion as fallback — NOT 'v3.0.1' or 'latest'
- **Secret extracted to values**: The SMTP_PASSWORD or equivalent secret value is NOT hardcoded in the template; it is either moved to a values.yaml key or referenced via a Kubernetes Secret
- **Resources block added**: The Helm deployment template includes a resources: block that was absent in the original raw manifest
- **validate.sh has helm template**: validate.sh contains a 'helm template' command (not just helm lint)
- **validate.sh has kubeval or kubeconform**: validate.sh pipes helm template output to kubeval or kubeconform for schema validation
- **toYaml or nindent used**: At least one template uses toYaml or nindent when rendering a complex value (labels, annotations, env, or similar)
- **Values extracted**: At least replica count and image repository are extracted into values.yaml as configurable keys rather than hardcoded in the template

## Failure Conditions

- Resource names in the deployment template are still hardcoded as `notification-service`
- Labels are still manually set as `app: notification-service` instead of using a named labels helper
- `_helpers.tpl` is absent or defines no named templates
- Image tag is `v3.0.1` or `latest` in `values.yaml`
- SMTP_PASSWORD is still a hardcoded literal string in the deployment template
- No `resources:` block is added to the deployment container spec
- `validate.sh` contains only `helm lint` and no `helm template` command
- `validate.sh` does not pipe `helm template` output to `kubeval` or `kubeconform`
