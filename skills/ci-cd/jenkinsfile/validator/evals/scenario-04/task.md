# Scenario 04: Pipeline Security Review

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
