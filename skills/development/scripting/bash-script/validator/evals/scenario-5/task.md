# Task: Replace a Global ShellCheck Suppression with Targeted Suppressions

A developer has silenced all ShellCheck SC2086 (unquoted variables) and SC2046 (word-splitting in command substitution) warnings globally. The script processes filenames intentionally with word splitting in one specific place but should be quoted everywhere else.

```bash
#!/usr/bin/env bash
# shellcheck disable=SC2086,SC2046

WORKSPACE=$1
FILES_TO_PROCESS=$2  # intentionally space-separated list

process_files() {
    for f in $FILES_TO_PROCESS; do  # intentional: FILES_TO_PROCESS is a space-separated list
        cp $f $WORKSPACE/processed/
        chmod 644 $WORKSPACE/processed/$f
    done
}

build_index() {
    wc -l $(find $WORKSPACE -name "*.txt")
}

cleanup() {
    rm -rf $WORKSPACE/tmp
}

process_files
build_index
cleanup
```

Your tasks:
1. Explain why the global suppression violates the bash-script-validator anti-patterns
2. Identify which uses of $FILES_TO_PROCESS are intentionally unsuppressed (word splitting desired) and which unquoted variables are genuine bugs
3. Produce a corrected version of the script that:
   - Removes the global suppression directive
   - Adds per-line suppression with a reason comment only where word splitting is genuinely intentional
   - Quotes all other variables that should be quoted
