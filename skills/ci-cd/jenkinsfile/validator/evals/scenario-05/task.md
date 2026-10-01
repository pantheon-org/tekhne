# Scenario 05: Shared Library Step Review

## User Prompt

A platform team maintains a Jenkins Shared Library. A new contributor added a custom step file but the step isn't loading correctly in pipelines and the team suspects the file has structural issues. They want a thorough review before pushing to the production shared library.

Validate the provided shared library vars file and produce a `library-report.md` listing all issues found with explanations and corrected code where applicable.

**inputs/vars/deploy_step.groovy:**

```groovy
#!/usr/bin/env groovy

@NonCPS
def execute(String environment, String version) {
    sh "kubectl set image deployment/app app=${version} -n ${environment}"
    sh "kubectl rollout status deployment/app -n ${environment}"
    return "deployed to ${environment}"
}

def validateInput(String env) {
    if (!env) {
        error "Environment must be specified"
    }
}
```
