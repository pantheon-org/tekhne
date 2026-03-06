# Multi-Resource Validation Report

## Problem Description

A developer has created a multi-resource YAML file containing a Deployment, Service, and ConfigMap for a new application. The file has several issues across different resources, and the team needs a comprehensive validation report that identifies problems in all resources, not just the first one encountered.

Create a validation reporting system that handles multi-resource files effectively, tracking issues per resource and providing clear location information for each problem.

## Output Specification

Create these files:

1. `multi-resource-validator.sh` - A bash script that validates a multi-resource YAML file and generates a comprehensive report. The script should:
   - Count the number of resources in the file
   - Validate all resources even if early ones have errors
   - Track which resource each issue belongs to
   - Use absolute line numbers for error locations

2. `validation-report-example.md` - A sample validation report demonstrating:
   - How to organize findings when multiple resources have issues
   - Proper line number reporting for multi-document files
   - Summary of issues per resource
   - Clear severity classification

## Input Files

The following multi-resource file has issues in multiple resources. Extract it before beginning.

## FILE: inputs/app-stack.yaml

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  API_KEY: "secret-key-12345"
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-app
spec:
  replicas: 2
  selector:
    matchLabels:
      app: webapp
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: web
        image: nginx:latest
---
apiVersion: v1
kind: Service
metadata:
  name: web-service
spec:
  selector:
    app: webapp
  ports:
- port: 80
    targetPort: "8080"
```

## END FILE
