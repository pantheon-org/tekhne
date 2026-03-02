# Validation Rules Reference

## Syntax Issues

- Missing required sections (agent, stages, steps)
- Invalid section names or misplaced directives
- Groovy syntax errors
- Missing braces, quotes, or brackets
- Semicolons at end of lines (unnecessary in Jenkins pipelines)

## Best Practices

- **Combine Shell Commands**: Use single `sh` step with multiple commands instead of multiple `sh` steps
- **Credential Management**: Use `credentials()` or `withCredentials`, never hardcode secrets
- **Agent Operations**: Perform heavy operations on agents, not controller
- **Parallel Execution**: Use `parallel` for independent stages
- **Error Handling**: Wrap critical sections in try-catch blocks
- **Timeouts**: Define timeouts in options to prevent hung builds
- **Clean Workspace**: Clean workspace before/after builds

## Variable Usage

- Proper variable declaration and scoping
- Correct interpolation syntax (`${VAR}` vs `$VAR`)
- Undefined variable detection
- Environment variable usage

## Security

- No hardcoded passwords, API keys, or tokens
- Proper use of Jenkins Credentials Manager
- Secrets management best practices
- Role-based access control recommendations

## Error Reporting Format

Validation results include:
- **Line numbers** for each issue
- **Severity levels**: Error, Warning, Info
- **Descriptions**: Clear explanation of the issue
- **Suggestions**: How to fix the problem
- **References**: Links to documentation

### Example Output

```
ERROR [Line 5]: Missing required section 'agent'
  → Add 'agent any' or specific agent configuration at top level

WARNING [Line 12]: Multiple consecutive 'sh' steps detected
  → Combine into single sh step with triple-quoted string
  → See: best_practices.md#combine-shell-commands

INFO [Line 23]: Consider using parallel execution for independent stages
  → See: references/declarative_syntax.md#parallel-stages
```

## Best Practice Examples

### Good: Combined Shell Commands

```groovy
sh '''
  echo "Building..."
  mkdir build
  ./gradlew build
  echo "Build complete"
'''
```

### Bad: Multiple Shell Steps

```groovy
sh 'echo "Building..."'
sh 'mkdir build'
sh './gradlew build'
sh 'echo "Build complete"'
```

### Good: Credential Management

```groovy
withCredentials([string(credentialsId: 'api-key', variable: 'API_KEY')]) {
  sh 'curl -H "Authorization: Bearer $API_KEY" ...'
}
```

### Bad: Hardcoded Credentials

```groovy
sh 'curl -H "Authorization: Bearer abc123xyz" ...'
```
