# Scenario 05: Security Scanning Suite for Java Microservice

## User Prompt

A fintech startup is preparing for a SOC 2 audit and needs to demonstrate automated security controls in their CI/CD pipeline. Their main Java 17 microservice currently has no automated security scanning. The security team has identified three specific gaps they need to close:

1. **Static analysis** — they want CodeQL scanning to run on every push to `main` and every pull request against `main`.
2. **Dependency vulnerability review** — every pull request should automatically flag newly introduced dependencies with known critical vulnerabilities before they can be merged.
3. **Supply chain attestation** — when a container image is built and pushed on merge to `main`, an SBOM attestation should be generated and pushed to the registry so they can prove the image contents to auditors.

The container registry is GitHub Container Registry (`ghcr.io`). The Java project is built with Maven.

## Output Specification

Produce one or more GitHub Actions workflow YAML files covering the three requirements above. Save files in the `.github/workflows/` directory with appropriate names.

## Expected Behavior

1. Grant `security-events: write` permission to the CodeQL workflow so SARIF results can be uploaded
2. Grant `id-token: write` and `attestations: write` permissions to the container build/attestation workflow
3. Include a step using `actions/dependency-review-action` triggered only on `pull_request` events
4. Pin all action references with full 40-character SHA hashes
5. Scope permissions specifically — do not use `permissions: write-all`
6. Enable Maven dependency caching via `setup-java` with `cache: 'maven'` or an explicit cache step
7. Include a `concurrency:` block in at least one workflow

## Success Criteria

- **security-events write permission**: The CodeQL workflow grants `security-events: write` permission (required for uploading SARIF results)
- **id-token write for attestation**: The container build/attestation workflow grants `id-token: write` permission
- **attestations write for attestation**: The container build/attestation workflow grants `attestations: write` permission
- **dependency-review-action used**: A workflow step references `actions/dependency-review-action` (any SHA or tag)
- **Dependency review on pull_request**: The dependency review step is triggered by (or only runs on) `pull_request` events
- **SHA-pinned actions**: All `uses:` steps reference actions with a full 40-character SHA (not tag or branch)
- **No write-all permissions**: No workflow uses `permissions: write-all` — permissions are scoped to specific needs
- **Maven caching enabled**: The Java/Maven build step uses a setup-java action with `cache: 'maven'` or an explicit `actions/cache` step for Maven dependencies
- **Concurrency block present**: At least one generated workflow contains a `concurrency:` block

## Failure Conditions

- The CodeQL workflow does not grant `security-events: write` permission
- The container build workflow does not grant `id-token: write` permission
- The container build workflow does not grant `attestations: write` permission
- No step references `actions/dependency-review-action`
- The dependency review step runs on events other than `pull_request` (e.g., on push to main)
- Any action uses a tag or branch reference instead of a full 40-character SHA
- Any workflow uses `permissions: write-all` instead of scoped permissions
- Maven caching is not configured in the Java build step
