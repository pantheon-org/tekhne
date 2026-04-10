# Scenario 03: Secure Deployment Pipeline with External API Calls

## User Prompt

You are building a deployment pipeline for a service called `billing-service`. The pipeline must perform an authenticated HTTP call to a service registry API after deploying to staging, and upload a deployment report to an artifact server.

Two credentials exist in the Jenkins Credentials Store:
- `service-registry-token` — a secret text bearer token for the registry API
- `artifact-server-creds` — a username/password pair for the artifact server

The pipeline must include stages for: build, deploy-staging, register-version (API call with bearer token), and upload-report. Secrets must never appear in shell command strings via Groovy interpolation.

## Expected Behavior

The agent should use `withCredentials([...])` to inject secrets and write shell commands that reference credential variables using single-quoted strings (e.g., `sh 'curl -H "Authorization: Bearer $TOKEN" ...'`). Double-quoted strings that interpolate Groovy variables containing secrets must be avoided. Both credential IDs must be referenced by their exact names. The pipeline must include `post { always { cleanWs() } }`, a `timeout()` in options, and `buildDiscarder` in options.

Capability tested: Single-quote sh & withCredentials secret handling.

1. Use single-quoted shell strings when referencing credential variables to prevent Groovy interpolation of secrets
2. Use `withCredentials([...])` to inject secrets — not pipeline `parameters` or hardcoded values
3. Reference both `service-registry-token` and `artifact-server-creds` credentials by their exact IDs
4. Include all four required stages: build, deploy-staging, register-version, upload-report
5. Include `post { always { cleanWs() } }` for workspace cleanup
6. Include `timeout()` in the `options` block
7. Include `buildDiscarder(logRotator(...))` in the `options` block

## Success Criteria

- **Single-quote sh with secrets**: Shell commands that reference credential variables use single-quoted strings (`sh 'curl ... $VAR'`) to prevent Groovy from interpolating them at pipeline parse time
- **withCredentials used**: Secrets are injected via `withCredentials([...])` blocks — not via hardcoded strings or pipeline parameters
- **Both credential IDs referenced**: The pipeline references both `service-registry-token` and `artifact-server-creds` by exact ID
- **All four stages present**: Stages for build, deploy-staging, register-version, and upload-report are all present
- **cleanWs in post**: Jenkinsfile `post { always { cleanWs() } }` or equivalent workspace cleanup is present
- **timeout in options**: Jenkinsfile `options` block includes `timeout()`
- **buildDiscarder in options**: Jenkinsfile `options` block includes `buildDiscarder(logRotator(...))`

## Failure Conditions

- Shell commands containing credential variables use double-quoted strings, allowing Groovy to expand secrets into the command string
- Secrets are hardcoded or passed through a `parameters` block instead of `withCredentials`
- One or both credential IDs (`service-registry-token`, `artifact-server-creds`) are absent or renamed
- Any of the four required stages (build, deploy-staging, register-version, upload-report) is missing
- No `post` block with `cleanWs()` is present
- `timeout()` is absent from `options`
- `buildDiscarder(logRotator(...))` is absent from `options`
