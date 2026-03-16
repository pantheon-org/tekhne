# Task: Convert a sh Script with Bashisms to POSIX sh

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
