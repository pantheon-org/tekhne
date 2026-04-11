# Scenario 03: Unknown Plugin Step Investigation

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

## Expected Behavior

The agent should invoke `bash scripts/validate_jenkinsfile.sh` as the first step. In `validation-report.md`, it must flag `grafanaAnnotate` as an unrecognized step with its line number. For plugin research, the agent should check `references/common_plugins.md` first before resorting to external lookup (Context7 MCP or WebSearch). The resulting `plugin-research.md` must document the plugin's parameters (at minimum `apiUrl`, `tags`, `text`) and include security considerations such as API token protection and network access implications.

Capability tested: Plugin documentation lookup workflow.

1. Invoke (or reference invoking) `bash scripts/validate_jenkinsfile.sh` as the primary validation step
2. Flag `grafanaAnnotate` as an unrecognized step in `validation-report.md`, including its line number
3. Check `references/common_plugins.md` first before using external tools for plugin research
4. Use Context7 MCP or WebSearch for external lookup after the local reference is checked
5. Document the plugin parameters (including `apiUrl`, `tags`, `text` or similar) in `plugin-research.md`
6. Include security considerations in `plugin-research.md` (API key/token protection, network access)

## Success Criteria

- **Main script invoked**: `validation-report.md` mentions invoking or attempting `bash scripts/validate_jenkinsfile.sh`
- **Unknown step flagged**: `validation-report.md` identifies `grafanaAnnotate` as an unrecognized step with its line number
- **common_plugins.md checked first**: `plugin-research.md` or the validation process mentions checking `references/common_plugins.md` as the first step before external search
- **External lookup attempted**: Agent uses Context7 MCP or WebSearch to research the plugin after it is not found locally
- **Plugin parameters documented**: `plugin-research.md` includes at least the required parameters for `grafanaAnnotate` (`apiUrl`, `tags`, `text` or similar)
- **Security considerations noted**: `plugin-research.md` includes security considerations (e.g., API key/token protection, network access implications)

## Failure Conditions

- `bash scripts/validate_jenkinsfile.sh` is not mentioned in `validation-report.md`
- `grafanaAnnotate` is not identified as an unrecognized step or its line number is omitted
- `references/common_plugins.md` is not consulted before external lookup
- No external lookup (Context7 MCP or WebSearch) is performed
- Plugin parameters are not documented in `plugin-research.md`
- No security considerations are included in `plugin-research.md`
