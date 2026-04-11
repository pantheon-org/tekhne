# Scenario 05: Replace a Global ShellCheck Suppression with Targeted Suppressions

## User Prompt

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
2. Identify which uses of `$FILES_TO_PROCESS` are intentionally unsuppressed (word splitting desired) and which unquoted variables are genuine bugs
3. Produce a corrected version of the script that:
   - Removes the global suppression directive
   - Adds per-line suppression with a reason comment only where word splitting is genuinely intentional
   - Quotes all other variables that should be quoted

## Expected Behavior

1. Explain that a file-level `# shellcheck disable=` hides all future instances of that error and the original rationale is lost when new code is added
2. Correctly identify `for f in $FILES_TO_PROCESS` as the intentional case where word splitting is desired
3. Correctly identify `$f` in `cp`, `chmod`, `$WORKSPACE`, and `$(find $WORKSPACE ...)` as unintentional quoting gaps
4. Remove the global `# shellcheck disable=SC2086,SC2046` directive from the corrected script
5. Add `# shellcheck disable=SC2086 # word splitting intentional: FILES_TO_PROCESS is a space-separated list` (or equivalent) only on the `for f in $FILES_TO_PROCESS` line
6. Quote `$f`, `$WORKSPACE`, `$WORKSPACE/processed/$f`, and `$WORKSPACE/tmp` in the corrected script; use `"$WORKSPACE"` in the `find` argument

## Success Criteria

- **Global suppression anti-pattern explained**: Agent explains that a file-level `# shellcheck disable=` hides all future instances and the original rationale is lost
- **Intentional vs accidental word splitting distinguished**: Agent correctly identifies `for f in $FILES_TO_PROCESS` as intentional and all other unquoted variables as accidental
- **Global directive removed in corrected script**: The corrected script does not contain a file-level `# shellcheck disable=` directive
- **Per-line suppression with reason comment added**: The corrected script adds a per-line suppression with a reason comment only on the `for f in $FILES_TO_PROCESS` line
- **All other variables quoted in corrected script**: `$f`, `$WORKSPACE`, `$WORKSPACE/processed/$f`, and `$WORKSPACE/tmp` are all quoted; `find "$WORKSPACE"` uses a quoted argument

## Failure Conditions

- Agent does not explain why the global suppression is an anti-pattern
- Agent incorrectly identifies additional locations as intentionally needing word splitting
- Agent leaves the global `# shellcheck disable=` directive in the corrected script
- Agent adds per-line suppressions to more than just the `for f in $FILES_TO_PROCESS` line
- Agent does not quote `$f` in the `cp` and `chmod` commands
- Agent does not quote `$WORKSPACE` in `find`, `rm -rf`, or directory paths
