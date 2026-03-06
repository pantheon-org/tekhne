# Validation Report Generator

## Problem Description

Your DevOps team needs a validation tool that produces detailed reports about Kubernetes manifest issues without automatically modifying files. The tool should suggest fixes in a clear format that developers can review and decide whether to apply.

The security team has concerns about automated file modification tools that might introduce unintended changes. They want validation reports that show what's wrong and how to fix it, but leave the actual fixes to human developers.

## Output Specification

Create these files:

1. `report-validator.sh` - A bash script that validates Kubernetes manifests and produces a report with suggested fixes, but NEVER modifies the input file. The script should:
   - Generate a validation report file
   - Show before/after code blocks for suggested fixes
   - Include fix complexity indicators ([Simple], [Medium], [Complex])
   - Exit without modifying the original manifest

2. `validation-philosophy.md` - Documentation explaining:
   - Why validation should be report-only
   - How to present suggested fixes effectively
   - The format for before/after code blocks
   - How developers should apply fixes after review

## Input Files

The following manifest has several fixable issues. Extract it before beginning.

=============== FILE: inputs/statefulset.yaml ===============
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: database
  namespace: default
spec:
  serviceName: "db"
  replicas: 3
  selector:
    matchLabels:
      app: db
  template:
    metadata:
      labels:
        app: db
    spec:
      containers:
      - name: postgres
        image: postgres:latest
        ports:
        - containerPort: "5432"
        env:
        - name: POSTGRES_PASSWORD
          value: "admin123"
=============== END FILE ===============
