# Security Scanning Suite for Java Microservice

## Problem/Feature Description

A fintech startup is preparing for a SOC 2 audit and needs to demonstrate automated security controls in their CI/CD pipeline. Their main Java 17 microservice currently has no automated security scanning. The security team has identified three specific gaps they need to close:

1. **Static analysis** — they want CodeQL scanning to run on every push to `main` and every pull request against `main`.
2. **Dependency vulnerability review** — every pull request should automatically flag newly introduced dependencies with known critical vulnerabilities before they can be merged.
3. **Supply chain attestation** — when a container image is built and pushed on merge to `main`, an SBOM attestation should be generated and pushed to the registry so they can prove the image contents to auditors.

The container registry is GitHub Container Registry (`ghcr.io`). The Java project is built with Maven.

## Output Specification

Produce one or more GitHub Actions workflow YAML files covering the three requirements above. Save files in the `.github/workflows/` directory with appropriate names.
