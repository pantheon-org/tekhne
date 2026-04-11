# Scenario 03: Application Configuration Management with ConfigMap and Secret

## User Prompt

A development team needs to configure their application with both sensitive credentials (database password, API keys) and non-sensitive settings (log level, feature flags, endpoints). They're new to Kubernetes and need guidance on proper configuration storage.

Create the appropriate Kubernetes configuration resources that demonstrate the correct way to separate sensitive from non-sensitive data. The application needs:

**Non-sensitive configuration:**
- LOG_LEVEL: info
- API_ENDPOINT: https://api.example.com
- FEATURE_X_ENABLED: true

**Sensitive configuration:**
- DB_PASSWORD: (example value)
- API_SECRET_KEY: (example value)

Generate two separate files:

1. `app-config.yaml` — For non-sensitive configuration
2. `app-secrets.yaml` — For sensitive data

Both files should be in the `production` namespace and labeled for the `backend-api` application. Include brief comments in each file explaining when to use that resource type.

## Expected Behavior

1. Use `kind: ConfigMap` in `app-config.yaml` for non-sensitive configuration
2. Use `kind: Secret` in `app-secrets.yaml` for sensitive data
3. Place `LOG_LEVEL`, `API_ENDPOINT`, and `FEATURE_X_ENABLED` in the ConfigMap (not the Secret)
4. Place `DB_PASSWORD` and `API_SECRET_KEY` in the Secret (not the ConfigMap)
5. Ensure the ConfigMap does NOT contain password, secret, key, or credential fields
6. Include `type: Opaque` or a valid equivalent in the Secret
7. Specify `namespace: production` in both resources
8. Include comments in at least one file explaining when to use that resource type
9. Use `data:` (base64) or `stringData:` (plaintext) in the Secret, not an invalid structure

## Success Criteria

- **ConfigMap for non-sensitive**: `app-config.yaml` uses `kind: ConfigMap`
- **Secret for sensitive**: `app-secrets.yaml` uses `kind: Secret`
- **Non-sensitive in ConfigMap**: `LOG_LEVEL`, `API_ENDPOINT`, and `FEATURE_X_ENABLED` are in ConfigMap (not Secret)
- **Sensitive in Secret**: `DB_PASSWORD` and `API_SECRET_KEY` are in Secret (not ConfigMap)
- **No secrets in ConfigMap**: ConfigMap does NOT contain password, secret, key, or credential fields
- **Secret type specified**: Secret includes `type: Opaque` or similar valid type
- **Namespace consistent**: Both resources specify `namespace: production`
- **Comments included**: At least one file includes comments explaining when to use that resource type
- **Proper data encoding**: Secret uses `data:` (base64) or `stringData:` (plaintext) field, not invalid structure

## Failure Conditions

- `app-config.yaml` does not use `kind: ConfigMap`
- `app-secrets.yaml` does not use `kind: Secret`
- Sensitive fields (`DB_PASSWORD`, `API_SECRET_KEY`) are placed in the ConfigMap instead of the Secret
- Non-sensitive fields are placed in the Secret
- ConfigMap contains credential or password fields
- Secret type is absent or invalid
- Either file does not specify `namespace: production`
- No comments are included to explain the distinction between the two resource types
