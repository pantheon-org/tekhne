# Unknown Plugin Step Investigation

## Problem Description

A team inherited a Jenkins pipeline from another department and discovered it uses a custom step they've never seen before. When they tried to run the pipeline on a new Jenkins instance, it failed at the `grafanaAnnotate` step with "No such DSL method found". They need to understand what this plugin does, how to configure it correctly, and what parameters are required.

Run validation on the pipeline and research the unknown step. Produce two files:
- `validation-report.md` — the validation findings with line numbers and severities
- `plugin-research.md` — documentation on the unknown step including required parameters and any security considerations

## Input Files

The following file is provided. Extract it before beginning.

=============== FILE: inputs/Jenkinsfile ===============
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
