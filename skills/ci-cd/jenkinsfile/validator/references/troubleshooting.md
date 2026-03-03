# Troubleshooting Guide

## Common Issues

### Issue: "Best practices check shows false negatives"

- **Cause**: Comment stripping may interfere with pattern detection
- **Solution**: Update to latest version

### Issue: "Syntax validation passes but pipeline fails on Jenkins"

- **Explanation**: Local validation catches structural issues but cannot verify:
  - Plugin availability
  - Agent/node availability
  - Credential existence
  - Network connectivity
- **Solution**: Validate on Jenkins using Replay feature or Pipeline Unit Testing Framework

### Issue: "Security scan shows passed but best practices finds credentials"

- **Solution**: Security scan now properly detects all credential patterns

### Issue: "Scripts not executable"

- **Solution**: Run `chmod +x scripts/*.sh`

## Debug Mode

Enable verbose output for troubleshooting:

```bash
# Run with bash debug mode
bash -x scripts/validate_jenkinsfile.sh Jenkinsfile

# Check individual validator output
bash scripts/validate_declarative.sh Jenkinsfile
bash scripts/best_practices.sh Jenkinsfile
bash scripts/common_validation.sh check_credentials Jenkinsfile
```

## Limitations

- **No Jenkins Server Required**: All validation is local (no live testing)
- **Plugin Steps**: Cannot fully validate custom plugin steps without documentation
- **Runtime Behavior**: Cannot detect runtime issues (permissions, network, etc.)
- **Complex Groovy**: Advanced Groovy constructs may not be fully validated
- **Shared Libraries**: Remote shared libraries are not fetched or validated

## Additional Capabilities

- **Dry-run Testing**: Validate without Jenkins server (all validation is local)
- **Plugin Version Checking**: Warn about deprecated plugin versions
- **Performance Analysis**: Identify potential performance bottlenecks
- **Compliance Checking**: Validate against organizational standards
- **Multi-file Support**: Validate multiple Jenkinsfiles in a directory
