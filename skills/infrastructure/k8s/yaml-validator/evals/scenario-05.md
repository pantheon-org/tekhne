# Scenario 05: Schema Validation with kubeconform

## User Prompt

Your CI/CD pipeline needs automated schema validation for Kubernetes manifests to catch API compliance issues before deployment. The validation should verify that manifests conform to Kubernetes API schemas and detect deprecated API versions.

Standard tools can validate both built-in Kubernetes resources and common CRDs, but the validation must be configured correctly to be effective. The platform team needs a reference implementation showing proper schema validation configuration.

The following manifest has schema-related issues:

```yaml
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
```

Create these files:

1. `schema-validator.sh` — A bash script that performs schema validation using industry-standard tools. The script should:
   - Use `kubeconform` or similar schema validation tool
   - Configure it to check both standard resources and CRDs
   - Enable strict mode to catch additional issues
   - Include the datree CRDs catalog for common operator CRDs
   - Provide verbose output with summary

2. `schema-validation-guide.md` — Documentation explaining:
   - What schema validation checks that syntax validation doesn't
   - How to configure schema sources (default + CRD catalogs)
   - The importance of strict mode
   - How to interpret schema validation output

## Expected Behavior

1. Script uses `kubeconform` command for schema validation
2. Includes `-strict` flag to catch additional issues
3. Configures `-schema-location` for CRDs catalog (e.g., `datreeio/CRDs-catalog`)
4. Includes `-schema-location default` for standard Kubernetes resources
5. Uses `-summary` flag to show validation summary
6. Uses `-verbose` flag for detailed error information
7. Report identifies `memory: 512` as a type error (should be string with unit)
8. Report identifies `storage` field as invalid in limits (not a valid resource type)
9. Guide explains that schema validation catches type/field errors that syntax validation misses
10. Includes `-ignore-missing-schemas` flag to handle CRDs without schemas gracefully

## Success Criteria

- **Kubeconform used**: Script uses `kubeconform` command for schema validation
- **Strict mode enabled**: Includes `-strict` flag to catch additional issues
- **CRD schema location**: Configures `-schema-location` for CRDs catalog (e.g., `datreeio/CRDs-catalog`)
- **Default schema location**: Includes `-schema-location default` for standard Kubernetes resources
- **Summary output**: Uses `-summary` flag to show validation summary
- **Verbose output**: Uses `-verbose` flag for detailed error information
- **Type error detected**: Report identifies `memory: 512` as type error (should be string with unit)
- **Invalid field detected**: Report identifies `storage` field as invalid in limits (not a valid resource type)
- **Schema vs syntax**: Guide explains that schema validation catches type/field errors that syntax validation misses
- **Ignore missing schemas**: Includes `-ignore-missing-schemas` flag to handle CRDs without schemas gracefully

## Failure Conditions

- `kubeconform` is not used for schema validation
- `-strict` flag is absent
- `-schema-location` for CRD catalog is not configured
- `-summary` or `-verbose` flags are not included
- `memory: 512` type error is not identified
- `storage` as an invalid limits field is not flagged
- Guide does not explain the difference between schema and syntax validation
