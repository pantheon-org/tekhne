# Schema Validation Implementation

## Problem Description

Your CI/CD pipeline needs automated schema validation for Kubernetes manifests to catch API compliance issues before deployment. The validation should verify that manifests conform to Kubernetes API schemas and detect deprecated API versions.

Standard tools can validate both built-in Kubernetes resources and common CRDs, but the validation must be configured correctly to be effective. The platform team needs a reference implementation showing proper schema validation configuration.

## Output Specification

Create these files:

1. `schema-validator.sh` - A bash script that performs schema validation using industry-standard tools. The script should:
   - Use kubeconform or similar schema validation tool
   - Configure it to check both standard resources and CRDs
   - Enable strict mode to catch additional issues
   - Include the datree CRDs catalog for common operator CRDs
   - Provide verbose output with summary

2. `schema-validation-guide.md` - Documentation explaining:
   - What schema validation checks that syntax validation doesn't
   - How to configure schema sources (default + CRD catalogs)
   - The importance of strict mode
   - How to interpret schema validation output

## Input Files

The following manifest has schema-related issues. Extract it before beginning.

=============== FILE: inputs/job-manifest.yaml ===============
apiVersion: batch/v1
kind: Job
metadata:
  name: data-migration
spec:
  template:
    spec:
      containers:
      - name: migrator
        image: migrator:1.0.0
        resources:
          requests:
            memory: "256Mi"
            cpu: "500m"
          limits:
            memory: 512
            storage: "10Gi"
      restartPolicy: OnFailure
  backoffLimit: 4
  activeDeadlineSeconds: "300"
=============== END FILE ===============
