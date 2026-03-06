# Application Configuration Management

## Problem Description

A development team needs to configure their application with both sensitive credentials (database password, API keys) and non-sensitive settings (log level, feature flags, endpoints). They're new to Kubernetes and need guidance on proper configuration storage.

Create the appropriate Kubernetes configuration resources that demonstrate the correct way to separate sensitive from non-sensitive data. The application needs:

**Non-sensitive configuration:**
- LOG_LEVEL: info
- API_ENDPOINT: <https://api.example.com>
- FEATURE_X_ENABLED: true

**Sensitive configuration:**
- DB_PASSWORD: (example value)
- API_SECRET_KEY: (example value)

## Output Specification

Generate two separate files:

1. `app-config.yaml` - For non-sensitive configuration
2. `app-secrets.yaml` - For sensitive data

Both files should be in the `production` namespace and labeled for the `backend-api` application. Include brief comments in each file explaining when to use that resource type.
