# Scenario 01: Validate Helm Chart Structure

## User Prompt

You are given the following Helm chart directory layout for a chart called `webapp`:

```
webapp/
├── Chart.yaml
├── values.yaml
└── templates/
    ├── deployment.yaml
    └── service.yaml
```

**Chart.yaml contents:**

```yaml
apiVersion: v1
name: webapp
description: A simple web application chart
version: 0.1.0
appVersion: "1.0"
```

**values.yaml contents:**

```yaml
replicaCount: 1
image:
  repository: nginx
  tag: latest
  pullPolicy: IfNotPresent
service:
  type: ClusterIP
  port: 80
```

**templates/deployment.yaml contents:**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "webapp.fullname" . }}
  labels:
    {{- include "webapp.labels" . | nindent 4 }}
spec:
  replicas: {{ .Values.replicaCount }}
  selector:
    matchLabels:
      {{- include "webapp.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      labels:
        {{- include "webapp.selectorLabels" . | nindent 8 }}
    spec:
      containers:
        - name: {{ .Chart.Name }}
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
          ports:
            - containerPort: 80
```

## Your Task

Perform Stage 2 (Chart Structure Validation) and Stage 3 (Helm Lint) of the helm-validator workflow on this chart.

1. Identify all required files that are present and all required/recommended files that are missing.
2. Identify any issues that `helm lint --strict` would flag, including the `apiVersion` value and any missing recommended fields.
3. Produce a findings report listing every issue categorised as Error, Warning, or Info, with the file and the reason.

Do NOT modify any files. List proposed fixes only.

## Expected Behavior

1. Confirm that Chart.yaml, values.yaml, and templates/ are present
2. Identify that `_helpers.tpl`, `NOTES.txt`, and `.helmignore` are missing and categorise them as Warnings or Info
3. Flag `apiVersion: v1` in Chart.yaml as an Error (Helm 3+ requires `apiVersion: v2`)
4. Flag `image.tag: latest` in values.yaml as a Warning with a recommendation to pin to a specific tag
5. List all proposed fixes without modifying any files
6. Categorise each finding as Error, Warning, or Info

## Success Criteria

- **Required file presence check**: Agent correctly identifies Chart.yaml, values.yaml, and templates/ as present
- **Missing recommended files identified**: Agent identifies that _helpers.tpl, NOTES.txt, and .helmignore are missing and categorises them as Warnings or Info
- **Deprecated apiVersion flagged**: Agent flags Chart.yaml using apiVersion: v1 instead of the Helm 3+ required apiVersion: v2 as an Error
- **Image :latest tag flagged**: Agent flags values.yaml image.tag: latest as a Warning with a recommendation to pin to a specific tag
- **Read-only posture maintained**: Agent proposes all fixes but does not modify any files. All proposed changes are listed in a summary section only
- **Issue categorisation**: Agent correctly categorises each finding as Error (must fix), Warning (should fix), or Info using the three-tier scheme

## Failure Conditions

- Required files (Chart.yaml, values.yaml, templates/) are not correctly identified as present
- Missing recommended files (_helpers.tpl, NOTES.txt, .helmignore) are not identified
- `apiVersion: v1` is not flagged as an Error (should be v2 for Helm 3+)
- `image.tag: latest` is not flagged as a Warning
- Agent modifies files instead of listing proposed fixes only
- Findings are not categorised as Error, Warning, or Info
