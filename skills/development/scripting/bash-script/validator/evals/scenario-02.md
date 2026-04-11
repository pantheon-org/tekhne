# Scenario 02: Convert a sh Script with Bashisms to POSIX sh

## User Prompt

The following script is intended to be a POSIX-compliant sh script but contains bash-specific syntax. The CI runner uses Alpine Linux where `/bin/sh` is dash (not bash).

```sh
#!/bin/sh
set -e

declare -a FAILED_TESTS=()
PASS=0
FAIL=0

run_tests() {
    local test_dir="$1"

    for test in $test_dir/*.test; do
        result=$(sh "$test" 2>&1)
        if [[ $? -eq 0 ]]; then
            PASS=$((PASS + 1))
        else
            FAILED_TESTS+=("$test")
            FAIL=$((FAIL + 1))
        fi
    done
}

run_tests ./tests

if [[ ${#FAILED_TESTS[@]} -gt 0 ]]; then
    echo "Failed tests:"
    for t in "${FAILED_TESTS[@]}"; do
        echo "  - $t"
    done
    exit 1
fi

echo "All $PASS tests passed"
```

Validate this script, identify every bashism, and produce a POSIX sh-compatible version. Explain each change made.

## Expected Behavior

1. Identify `declare -a` as bash-specific and not available in POSIX sh; explain that POSIX sh has no arrays
2. Identify `local` as a bash extension (not in POSIX sh spec) and suggest removing it or restructuring the function
3. Identify `[[ $? -eq 0 ]]` and `[[ ${#FAILED_TESTS[@]} -gt 0 ]]` as using bash double-bracket syntax; replace them with POSIX `[ ]` or equivalent constructs
4. Replace `FAILED_TESTS+=(...)` and `${FAILED_TESTS[@]}` array operations with a POSIX-compatible alternative (e.g., space-separated string, temp file, or counter-only approach)
5. Produce a complete corrected script using only POSIX sh features; shebang remains `#!/bin/sh`
6. Provide an explanation for each substitution, referencing why the original construct fails on dash/POSIX sh

## Success Criteria

- **declare -a identified as bashism**: Agent identifies that `declare -a` is bash-specific and explains that POSIX sh has no arrays
- **local identified as bashism**: Agent identifies that `local` is a bash extension and suggests removing it or restructuring
- **[[ ]] identified as bashism**: Agent identifies that `[[ ]]` double-bracket syntax is bash-specific and replaces with POSIX `[ ]` constructs
- **Array operations replaced with POSIX alternative**: Agent replaces `FAILED_TESTS+=()` and `${FAILED_TESTS[@]}` with a POSIX-compatible alternative
- **Corrected POSIX sh script produced**: Agent produces a complete corrected script using only POSIX sh features; shebang remains `#!/bin/sh`
- **Each change explained**: Agent provides an explanation for each substitution referencing why the original fails on dash/POSIX sh

## Failure Conditions

- Agent misses `declare -a` as a bashism
- Agent misses `local` as a bashism
- Agent misses `[[ ]]` double-bracket syntax as a bashism
- Agent does not replace array operations with a POSIX-compatible alternative
- Agent produces a corrected script that still contains bash-specific syntax
- Agent does not explain each change made
