# Scenario 01: Security Audit: Hardcoded Secrets and Insecure Script Patterns

## User Prompt

A SaaS company's security team has flagged a GitLab CI/CD pipeline during a routine audit. The pipeline was written quickly for a hackathon and then promoted directly to production. The security team suspects hardcoded credentials and unsafe script patterns but has not provided specifics.

Audit the `.gitlab-ci.yml` below for security issues. Identify every security finding, assign a severity level, explain the risk each finding introduces, and produce a corrected version of the file with all critical and high-severity issues resolved.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` with all critical and high-severity security issues fixed
- A `security-report.md` listing each finding with: severity (CRITICAL/HIGH/MEDIUM/INFO), job name, description of the issue, and the fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - build
  - test
  - deploy

variables:
  DOCKER_REGISTRY: registry.example.com
  DEPLOY_TOKEN: glpat-xxxxxxxxxxxxxxxxxxxx
  DB_PASSWORD: SuperSecret123!

build:
  stage: build
  image: docker:latest
  script:
    - docker build -t $DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA .
    - docker push $DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA

test:
  stage: test
  image: node:18
  script:
    - npm ci
    - npm test
    - curl -sSL https://install.example.com/agent.sh | bash

deploy:
  stage: deploy
  image: alpine:3.18
  script:
    - apk add --no-cache curl
    - |
      curl -k https://deploy.internal.example.com/api/deploy \
        --header "Authorization: Bearer $DEPLOY_TOKEN" \
        --data '{"image": "$DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA"}'
  only:
    - main

## Expected Behavior

1. Detect and flag the hardcoded `DEPLOY_TOKEN` value (`glpat-xxxxxxxxxxxxxxxxxxxx`) as a credential exposure finding
2. Detect and flag the hardcoded `DB_PASSWORD` value as a credential exposure finding
3. Detect and flag the `curl -sSL ... | bash` pattern in the test job as an arbitrary code execution risk
4. Detect and flag the `curl -k` flag in the deploy job as a TLS certificate verification bypass
5. Remove the literal values for DEPLOY_TOKEN and DB_PASSWORD in the corrected YAML (use CI/CD variable references instead)
6. Replace or remove the `curl-pipe-to-bash` command in the corrected test job
7. Remove the `-k` flag from the curl command in the corrected deploy job
8. Assign severity levels and explain the risk for at least two findings

## Success Criteria

- **Hardcoded DEPLOY_TOKEN detected**: security-report.md (or equivalent) flags the DEPLOY_TOKEN variable value as a hardcoded credential
- **Hardcoded DB_PASSWORD detected**: security-report.md flags the DB_PASSWORD variable value as a hardcoded credential
- **curl-pipe-to-bash detected**: security-report.md flags `curl ... | bash` in the test job as an insecure script pattern
- **SSL bypass detected**: security-report.md flags `curl -k` in the deploy job as a TLS certificate verification bypass
- **Credentials replaced with CI/CD variable references**: In the corrected .gitlab-ci.yml, DEPLOY_TOKEN and DB_PASSWORD no longer contain literal values — they reference CI/CD masked/protected variables (e.g., the values are removed from the variables block, or a comment explains they must be set in GitLab CI/CD settings)
- **curl-pipe-to-bash remediated**: In the corrected .gitlab-ci.yml, the `curl ... | bash` command is replaced or removed
- **SSL bypass remediated**: In the corrected .gitlab-ci.yml, the -k flag is removed from the curl command in the deploy job
- **Severity levels assigned**: security-report.md assigns a severity level (CRITICAL, HIGH, MEDIUM, or INFO) to each finding
- **Risk explanation provided**: security-report.md explains the risk for at least two findings (e.g., credential exposure in logs, arbitrary code execution, MITM)

## Failure Conditions

- The hardcoded DEPLOY_TOKEN literal value is not flagged as a security finding
- The hardcoded DB_PASSWORD literal value is not flagged as a security finding
- The `curl | bash` pattern in the test job is not identified as an insecure script pattern
- The `curl -k` SSL bypass in the deploy job is not flagged
- Corrected pipeline still contains literal values for DEPLOY_TOKEN or DB_PASSWORD in the variables block
- Corrected pipeline retains the `curl | bash` pattern in the test job
- Corrected pipeline retains the `-k` flag in the curl command of the deploy job
- No severity levels are assigned to findings
