# Scenario 07: Pipeline with Shared Library Dependency

## User Prompt

A team's Jenkinsfile loads a shared library called `platform-lib` using `@Library('platform-lib') _`. The pipeline uses two steps from that library: `buildAndPush` and `deployApp`. The team wants a validation report before deploying to production.

Validate the provided Jenkinsfile and produce a `validation-report.md` documenting all findings, including any limitations introduced by the unverified shared library dependency.

**inputs/Jenkinsfile:**

```groovy
@Library('platform-lib') _

pipeline {
    agent { label 'docker' }

    environment {
        IMAGE_NAME = "myapp"
        REGISTRY = "registry.company.com"
    }

    stages {
        stage('Build Image') {
            steps {
                buildAndPush(
                    imageName: "${REGISTRY}/${IMAGE_NAME}",
                    tag: env.BUILD_NUMBER
                )
            }
        }

        stage('Deploy') {
            steps {
                deployApp(
                    environment: 'staging',
                    imageTag: env.BUILD_NUMBER
                )
            }
        }
    }
}
```
