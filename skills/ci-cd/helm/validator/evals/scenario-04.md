# Scenario 04: Diagnose Template Rendering Errors (Stage 4 & 5)

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

## Expected Behavior

1. Explain that `template` cannot be used in pipelines (does not return a value) and recommend replacing it with `include`
2. Identify that `indent` requires a string input but `include` returns interface{}; recommend using `nindent` which adds a leading newline and handles the type correctly
3. Identify that `.Values.environment` in a JSON context needs to be quoted (using the `quote` function or wrapped in double quotes) to produce valid JSON
4. Identify that `required` passes when the value is set to an empty string `""` because the value is not nil; explain the user needs to provide a non-empty value or use a different validation approach
5. Explain that YAML/template errors must be fixed in the source template, not the rendered output, because helm template regenerates the output each time

## Success Criteria

- **template vs include issue identified**: Agent explains that 'template' cannot be used in pipelines (does not return a value) and recommends replacing it with 'include'
- **indent vs nindent issue identified**: Agent identifies that 'indent' requires a string input but 'include' returns interface{}; recommends using 'nindent' which adds a leading newline and handles the type correctly
- **JSON string quoting issue identified**: Agent identifies that .Values.environment in JSON context needs to be quoted (using the 'quote' function or wrapped in double quotes) to produce valid JSON
- **required with empty string behaviour explained**: Agent identifies that 'required' passes when the value is set to an empty string '' because the value is not nil; explains the user needs to provide a non-empty value or use a different validation approach
- **Source template fix rationale**: Agent explains that YAML/template errors must be fixed in the source template, not the rendered output, because helm template regenerates the output each time

## Failure Conditions

- The `template` vs `include` issue on line 3 is not identified
- The `indent` vs `nindent` type error on line 5 is not explained
- The unquoted `environment` value in the JSON block is not identified as a syntax problem
- The `required` function behavior with empty strings is not correctly explained
- The rationale for fixing source templates instead of rendered output is not provided
