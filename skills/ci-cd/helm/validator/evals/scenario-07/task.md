# Scenario 07: Diagnose Template Rendering Errors (Stage 4 & 5)

## User Prompt

You are given the following Helm template file `templates/configmap.yaml` and the error output from `helm template`:

**templates/configmap.yaml:**

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{ template "app.fullname" . }}
  labels:
{{ include "app.labels" . | indent 2 }}
data:
  config.json: |
    {
      "environment": {{ .Values.environment }},
      "replicas": {{ .Values.replicas | default 3 }}
    }
  database_url: {{ required "A database URL is required" .Values.databaseUrl }}
```

**Error output from `helm template myapp ./chart`:**

```
Error: template: app/templates/configmap.yaml:7:5: executing "app/templates/configmap.yaml"
at <include "app.labels" . | indent 2>: error calling indent: wrong type for value; expected string; got interface {}
```

**values.yaml excerpt:**

```yaml
environment: production
replicas: 2
databaseUrl: ""
```
