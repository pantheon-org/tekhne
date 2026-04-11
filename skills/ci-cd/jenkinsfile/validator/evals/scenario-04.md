# Scenario 04: Pipeline with Shared Library Dependency

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

## Expected Behavior

The agent should invoke `bash scripts/validate_jenkinsfile.sh` and identify the `@Library('platform-lib')` declaration. The unpinned library version (no tag or commit specified) must be flagged as a risk. The agent must name `buildAndPush` and `deployApp` as shared library steps that require cross-reference validation. The report must warn that the Jenkinsfile cannot be fully validated in isolation and that a clean result does not confirm deployment readiness.

Capability tested: Library reference cross-validation.

1. Invoke (or reference invoking) `bash scripts/validate_jenkinsfile.sh`
2. Identify the `@Library('platform-lib')` declaration and name the library being loaded
3. Flag that the `@Library` declaration specifies no version/tag/commit, relying on the default branch
4. Identify `buildAndPush` and `deployApp` as shared library steps requiring library cross-reference
5. Warn that the Jenkinsfile cannot be validated in isolation without the shared library
6. Note that a clean validation result does not confirm deployment readiness given the unverified library dependency

## Success Criteria

- **Main script invoked**: Report mentions invoking or attempting `bash scripts/validate_jenkinsfile.sh`
- **@Library call noted**: Report identifies the `@Library('platform-lib')` declaration and names the library being loaded
- **Unpinned version flagged**: Report flags that the `@Library` declaration does not specify a version/tag/commit and uses the default branch — which can cause unexpected breakage when the library updates
- **Library steps identified**: Report identifies `buildAndPush` and `deployApp` as shared library steps (not standard Jenkins steps) that require library cross-reference
- **Isolation warning issued**: Report warns that the Jenkinsfile cannot be validated in isolation — the shared library step signatures and current library state must be verified
- **Deployment caution**: Report notes that a clean validation result does not confirm deployment readiness given the unverified library dependency

## Failure Conditions

- `bash scripts/validate_jenkinsfile.sh` is not mentioned in the report
- The `@Library('platform-lib')` declaration is not identified or the library name is not stated
- The unpinned library version is not flagged as a risk
- `buildAndPush` and `deployApp` are not identified as shared library steps requiring cross-reference
- No warning is issued about the limitations of isolated validation
- The report implies full deployment readiness without caveats about the unverified library
