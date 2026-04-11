# Scenario 05: Shared Library Class Review

## User Prompt

A team is contributing a utility class to their Jenkins Shared Library. The class will be used by multiple Jenkinsfiles to handle Docker build operations. During testing, pipelines using this class fail with serialization errors and the class can't be found when imported. They need a code review to fix structural issues before merging.

Validate the provided class file and produce a `class-review.md` listing all structural issues with explanations and corrected code snippets.

**inputs/src/com/company/BuildUtils.groovy:**

```groovy
import com.company.*
import jenkins.model.*

class BuildHelper {

    def steps

    BuildHelper(steps) {
        this.steps = steps
    }

    def buildImage(String imageName, String tag) {
        steps.sh "docker build -t ${imageName}:${tag} ."
        steps.sh "docker push ${imageName}:${tag}"
    }

    static def getVersion(String branch) {
        return branch.replaceAll('/', '-')
    }
}
```

## Expected Behavior

The agent should invoke `bash scripts/validate_shared_library.sh` for the `src` class file. It must flag: the missing `package com.company` declaration, the class name mismatch (`BuildHelper` in a file named `BuildUtils.groovy`), the missing `implements Serializable` (required for Jenkins CPS serialization), and the wildcard imports. Corrected code must be provided for at least one issue.

Capability tested: Shared library src class validation.

1. Invoke (or reference invoking) `bash scripts/validate_shared_library.sh` for the src class file
2. Identify the missing `package com.company` declaration
3. Flag the class name `BuildHelper` as a mismatch with the filename `BuildUtils.groovy`
4. Identify the missing `implements Serializable` required for Jenkins CPS serialization
5. Flag the wildcard imports (`import com.company.*`, `import jenkins.model.*`)
6. Provide corrected code for at least one of the identified issues

## Success Criteria

- **Shared library script used**: `class-review.md` mentions invoking or attempting `bash scripts/validate_shared_library.sh` for the src class file
- **Missing package declaration flagged**: Report identifies that the file lacks a `package` declaration (should be `package com.company`)
- **Class-filename mismatch flagged**: Report identifies that the class is named `BuildHelper` but the file is named `BuildUtils.groovy` — these must match in Groovy
- **Missing Serializable flagged**: Report identifies that the class does not implement `Serializable`, which is required for Jenkins to serialize pipeline state during CPS execution
- **Wildcard imports flagged**: Report identifies the wildcard import statements (`import com.company.*`) as a violation of shared library best practices
- **Corrected code provided**: Report includes corrected code for at least one of the identified issues

## Failure Conditions

- `bash scripts/validate_shared_library.sh` is not mentioned in the report
- The missing `package com.company` declaration is not identified
- The class name / filename mismatch (`BuildHelper` vs `BuildUtils.groovy`) is not flagged
- The missing `implements Serializable` is not identified as a cause of serialization errors
- Wildcard imports are not flagged
- No corrected code is provided for any of the identified issues
