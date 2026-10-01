# Scenario 06: Unknown Plugin Step Investigation

## User Prompt

A team's pipeline uses a step called `grafanaAnnotate` to post deployment annotations to Grafana. The step is not a standard Jenkins built-in and the team is unsure whether it is properly configured. They want a validation report and a separate plugin research document before they commit to this approach.

Validate the provided Jenkinsfile, identify the unrecognized step, research the plugin, and produce:
- `validation-report.md` with issues found (including the unknown step with line number)
- `plugin-research.md` with the plugin parameters and security considerations

**inputs/Jenkinsfile:**

```groovy
pipeline {
    agent any

    stages {
        stage('Deploy') {
            steps {
                sh './scripts/deploy.sh'
            }
        }

        stage('Notify Grafana') {
            steps {
                grafanaAnnotate(
                    apiUrl: 'https://grafana.company.com',
                    tags: ['deployment', 'production'],
                    text: "Deployed ${env.BUILD_NUMBER}"
                )
            }
        }
    }
}
```
