# Scenario 01: Pipeline Security Review

## User Prompt

A development team is preparing to merge a new CI pipeline to their main branch. Before merge, the team lead wants a thorough security and best-practices review of the Jenkinsfile. They need a report that identifies any issues and includes corrected code for each problem found.

Run a validation on the provided Jenkinsfile and produce a `validation-report.md` with: all issues found (with line numbers), severity for each issue, and corrected code snippets for any errors or warnings.

**inputs/Jenkinsfile:**

```groovy
pipeline {
    agent any

    environment {
        APP_NAME = 'payments-service'
        DB_PASSWORD = 'hunter2'
        DEPLOY_ENV = 'production'
    }

    stages {
        stage('Test') {
            steps {
                retry(3) {
                    sh 'npm test'
                }
            }
        }

        stage('Deploy') {
            steps {
                sh """
                    ./scripts/deploy.sh ${DEPLOY_ENV}
                """
            }
        }
    }

    post {
        failure {
            mail to: 'team@company.com',
                 subject: "Build Failed: ${env.JOB_NAME}",
                 body: "See ${env.BUILD_URL}"
        }
    }
}
```

## Expected Behavior

The agent should invoke `bash scripts/validate_jenkinsfile.sh` as the primary validation step. It should identify the hardcoded credential (`DB_PASSWORD = 'hunter2'`) on the correct line and provide a corrected version using `withCredentials` binding. The `retry(3)` without a justification comment should be flagged. Each issue in `validation-report.md` must include a severity label. The report must not declare the pipeline deployment-ready without environment-specific verification.

Capability tested: Security issue detection and fix suggestions.

1. Invoke (or reference invoking) `bash scripts/validate_jenkinsfile.sh` as the primary validation entry point
2. Identify `DB_PASSWORD = 'hunter2'` as a hardcoded credential and provide a corrected version using credentials binding
3. Flag the `retry(3)` usage and note that no justification is present
4. Include severity labels (Error/Warning/Info) for each issue in the report
5. Avoid declaring the pipeline deployment-ready without confirming environment-specific settings

## Success Criteria

- **Main script invoked**: Report mentions invoking or attempting `bash scripts/validate_jenkinsfile.sh` — not a sub-script or ad-hoc manual check only
- **Hardcoded credential flagged**: Report identifies `DB_PASSWORD = 'hunter2'` as a hardcoded credential and provides a corrected version using `withCredentials` or `credentials()` binding
- **Retry flagged**: Report flags `retry(3)` on the test stage and notes that no justification or context is provided for why three retries are warranted
- **Severity labels present**: Each issue in `validation-report.md` is labelled with a severity (e.g., Error, Warning, Info)
- **No unconditional deployment-ready declaration**: Report does not declare the pipeline fully deployment-ready without noting that environment-specific configuration must be verified

## Failure Conditions

- `bash scripts/validate_jenkinsfile.sh` is not mentioned in the report
- `DB_PASSWORD = 'hunter2'` is not identified as a hardcoded credential
- No corrected version of the credential handling is provided
- `retry(3)` is not flagged as noteworthy
- Issues lack severity labels
- The report declares the pipeline deployment-ready without environment-specific caveats
