# Scenario 02: Shared Library Step Review

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

## Expected Behavior

The agent should invoke `bash scripts/validate_shared_library.sh` (not the main Jenkinsfile validator). It should flag the filename `deploy_step.groovy` as a camelCase naming convention violation (should be `deployStep.groovy`). The missing `call()` method must be identified as a required entry point for `vars/*.groovy` files. The `@NonCPS` annotation on `execute()` must be flagged as a misuse: `execute()` calls `sh` steps which are CPS pipeline steps, and `@NonCPS` methods cannot call CPS functions. Corrected code must be provided for at least one issue.

Capability tested: Shared library vars file validation.

1. Invoke (or reference invoking) `bash scripts/validate_shared_library.sh` — not the main Jenkinsfile validator
2. Identify the `deploy_step.groovy` filename as a camelCase naming convention violation
3. Identify the missing `call()` method as a required entry point for `vars/*.groovy` step files
4. Flag `@NonCPS` on `execute()` as a misuse because the method calls CPS pipeline steps (`sh`)
5. Provide corrected code for at least one of the identified issues

## Success Criteria

- **Uses shared library script**: Report mentions invoking or attempting `bash scripts/validate_shared_library.sh` — not the main `validate_jenkinsfile.sh`
- **Naming convention violation flagged**: Report identifies that `deploy_step.groovy` violates camelCase naming conventions for `vars/` files (should be `deployStep.groovy`)
- **Missing call() method flagged**: Report identifies that the file lacks a `call()` method, which is the required entry point for `vars/*.groovy` step files
- **@NonCPS misuse flagged**: Report identifies that `@NonCPS` is incorrectly applied to `execute()` because the method calls `sh` steps, which are CPS pipeline steps — `@NonCPS` methods cannot call CPS functions
- **Corrected code provided**: Report includes corrected code for at least one of the identified issues (e.g., renamed file, added `call()` method, or removed `@NonCPS`)

## Failure Conditions

- `bash scripts/validate_shared_library.sh` is not mentioned in the report
- The `deploy_step.groovy` naming convention violation is not identified
- The missing `call()` method is not flagged
- The `@NonCPS` misuse on a method that calls CPS steps is not explained
- No corrected code is provided for any of the identified issues
