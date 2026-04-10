# Scenario 01: CI Pipeline for Python REST API

## User Prompt

You are building a CI pipeline for a Python REST API service called `data-gateway`. The team uses Jenkins with a Docker-based registry for image storage. Credentials for the Docker registry are already stored in the Jenkins Credentials Store under the ID `registry-creds`.

The pipeline should cover: checkout, lint, test, Docker build, and Docker push stages. Do not expose credentials through pipeline parameters.

## Expected Behavior

The agent should use Declarative pipeline syntax (`pipeline { ... }`) rather than Scripted syntax. It should retrieve Docker registry credentials from the Jenkins Credentials Store via `credentials()` binding or `withCredentials([...])`, never hardcoding them. The pipeline must include a `post` block with `always { cleanWs() }` for workspace cleanup and standard `options` such as `buildDiscarder`, `disableConcurrentBuilds`, and `timeout`.

Capability tested: Declarative syntax & credentials store.

1. Use `pipeline { }` Declarative syntax (not `node { }`)
2. Access registry credentials via `credentials()` in `environment` or `withCredentials([...])` in steps — never via `parameters` block or hardcoded strings
3. Include `post { always { cleanWs() } }` for workspace cleanup
4. Include a `post` block at the pipeline level
5. Include `buildDiscarder(logRotator(...))` in `options`
6. Include `disableConcurrentBuilds()` in `options`
7. Include `timeout()` in `options`

## Success Criteria

- **Declarative syntax**: Jenkinsfile starts with `pipeline {` (Declarative syntax) — NOT `node {` (Scripted syntax)
- **No credential parameters**: Jenkinsfile does NOT contain a `parameters` block with string or password entries for credentials (API keys, passwords, tokens)
- **Credentials Store used**: Registry credentials are accessed via `credentials()` binding in an environment block OR via `withCredentials([...])` — NOT hardcoded strings
- **cleanWs in post**: Jenkinsfile includes a `post` block with an `always { cleanWs() }` or `always { deleteDir() }` call
- **post block exists**: A `post` block is present at the pipeline level
- **buildDiscarder in options**: Jenkinsfile `options` block includes `buildDiscarder(logRotator(...))`
- **disableConcurrentBuilds in options**: Jenkinsfile `options` block includes `disableConcurrentBuilds()`
- **timeout in options**: Jenkinsfile `options` block includes `timeout()`

## Failure Conditions

- Jenkinsfile uses Scripted (`node { }`) instead of Declarative (`pipeline { }`) syntax
- A `parameters` block contains credential values (passwords, tokens, API keys)
- Registry credentials appear as hardcoded strings rather than credentials bindings
- No `post` block with `cleanWs()` or `deleteDir()` is present
- No `post` block exists at the pipeline level
- `buildDiscarder(logRotator(...))` is absent from `options`
- `disableConcurrentBuilds()` is absent from `options`
- `timeout()` is absent from `options`
