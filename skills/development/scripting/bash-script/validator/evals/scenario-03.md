# Scenario 03: Security Review of a Bash Script

## User Prompt

A deployment script has been flagged for security review. Identify all security vulnerabilities, explain the attack vector for each, and produce a hardened version.

```bash
#!/usr/bin/env bash

DEPLOY_ENV=$1
PLUGIN_CMD=$2
WORK_DIR=/tmp/deploy_$DEPLOY_ENV

setup_workspace() {
    mkdir -p $WORK_DIR
    cd $WORK_DIR
}

load_plugin() {
    # Allow operators to provide custom processing commands
    eval $PLUGIN_CMD
}

cleanup() {
    rm -rf /tmp/deploy_$DEPLOY_ENV
    return 0
}

validate() {
    if check_prereqs || true; then
        echo "Validation passed"
        return 0
    fi
}

setup_workspace
load_plugin
cleanup
```

Your response must:
1. Identify every security vulnerability with the specific risk (injection type, data loss potential, etc.)
2. Identify any error-handling anti-patterns
3. Produce a hardened version of the script
4. Explain what attack would be possible with the original script for each issue

## Expected Behavior

1. Identify that `eval $PLUGIN_CMD` allows arbitrary command execution; provide a concrete example attack (e.g., `PLUGIN_CMD='rm -rf /'`)
2. Identify that `rm -rf /tmp/deploy_$DEPLOY_ENV` is dangerous when `DEPLOY_ENV` is empty or contains spaces, potentially deleting `/tmp/deploy_` or unintended paths
3. Identify that `check_prereqs || true` in `validate()` always returns success, hiding failures, and that `return 0` compounds this
4. Identify that `cd $WORK_DIR` without `|| exit` (SC2164) means subsequent commands run in the wrong directory if `cd` fails
5. Produce a corrected script that removes `eval`, quotes the `rm -rf` variable, removes `|| true` suppression, and adds `cd || exit` or equivalent
6. Describe a concrete attack scenario for each security issue

## Success Criteria

- **eval $PLUGIN_CMD identified as command injection**: Agent identifies that `eval` with an unvalidated variable allows arbitrary command execution; describes a concrete example attack
- **rm -rf with unquoted variable identified**: Agent identifies that `rm -rf /tmp/deploy_$DEPLOY_ENV` is dangerous when `DEPLOY_ENV` is empty or contains spaces
- **|| true error suppression anti-pattern identified**: Agent identifies that `check_prereqs || true` always returns success, hiding failures
- **cd without error handling identified**: Agent identifies that `cd $WORK_DIR` without `|| exit` means subsequent commands run in the wrong directory
- **Hardened script produced**: Agent produces a corrected script that removes `eval`, quotes the `rm -rf` variable, removes `|| true` suppression, and adds `cd || exit`
- **Attack vectors explained**: For each security issue, agent describes a concrete attack scenario

## Failure Conditions

- Agent misses the `eval $PLUGIN_CMD` command injection vulnerability
- Agent misses the unquoted `rm -rf` variable risk
- Agent does not explain the `|| true` error suppression anti-pattern
- Agent does not identify the `cd` without error handling issue
- Agent's hardened script still uses `eval` or leaves `|| true` in place
- Agent does not provide concrete attack scenarios for the vulnerabilities
