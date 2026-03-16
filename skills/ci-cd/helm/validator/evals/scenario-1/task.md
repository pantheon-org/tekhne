# Task: Validate Helm Chart Structure

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
