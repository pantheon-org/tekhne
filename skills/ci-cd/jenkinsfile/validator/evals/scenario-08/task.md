# Scenario 08: Shared Library Class Review

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
