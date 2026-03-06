# Deployment Manifest Validation

## Problem Description

A junior developer created a Kubernetes Deployment manifest for a new service but is uncertain whether it follows best practices and will work correctly when applied to the cluster. Before submitting a pull request, they need a comprehensive validation report that identifies any issues.

Create a validation tool/script that demonstrates the proper workflow for validating Kubernetes YAML manifests. The tool should check multiple aspects of the manifest and produce a detailed report with all findings.

## Output Specification

Create these files:

1. `validate-manifest.sh` - A bash script that implements a complete validation workflow for Kubernetes YAML files. The script should check syntax, schema compliance, and any other relevant validation layers. Include comments explaining each validation stage.

2. `validation-report.md` - A sample validation report showing:
   - What each validation stage checks
   - The order in which validation should be performed
   - How to interpret results from each stage
   - Recommended severity categories for issues

The validation tool should demonstrate industry-standard practices for Kubernetes manifest validation.

## Input Files

The following deployment manifest needs validation. Extract it before beginning.

=============== FILE: inputs/web-deployment.yaml ===============
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: webapp
        image: nginx:latest
        ports:
        - containerPort: "80"
=============== END FILE ===============
