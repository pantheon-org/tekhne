# Migrate Raw Kubernetes Manifests to Helm

## Problem/Feature Description

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
