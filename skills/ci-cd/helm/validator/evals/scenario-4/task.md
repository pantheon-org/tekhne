# Task: Diagnose Template Rendering Errors (Stage 4 & 5)

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

## Your Task

Perform Stage 4 (Template Rendering) diagnosis and Stage 5 (YAML Syntax Validation) analysis.

1. Identify all issues in the template, including:
   - The `template` vs `include` usage issue on line 3
   - The `indent` vs `nindent` usage issue on line 5
   - The JSON value quoting issue for `environment` on line 9
   - The `required` field evaluation with an empty string on line 12

2. For each issue, provide:
   - The file and approximate line number
   - What the problem is
   - A corrected code block

3. Explain why fixes must be made in the source template, not the rendered output.
