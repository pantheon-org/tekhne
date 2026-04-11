# Scenario 01: Validate a Bash Script with Quoting Issues

## User Prompt

A developer has submitted the following bash script for review. Validate it using the bash-script-validator skill's workflow and produce a corrected version.

```bash
#!/usr/bin/env bash
set -e

BACKUP_DIR=/tmp/backups
SOURCE=$1
DEST=$2

backup() {
    if [ ! -f $SOURCE ]; then
        echo "Source file $SOURCE not found"
        exit 1
    fi

    mkdir -p $BACKUP_DIR
    cp $SOURCE $DEST/$SOURCE

    for file in $(ls $BACKUP_DIR); do
        echo Processing $file
    done
}

backup
```

Your response must:
1. List all issues found with their line numbers and ShellCheck codes where applicable
2. For each issue: show the problematic code, explain the risk, show the fix, explain why the fix is better
3. Produce a corrected version of the entire script
4. State the final validation result (PASSED/FAILED) with counts

## Expected Behavior

1. Identify that `$SOURCE` and `$DEST` are used unquoted in conditions, `cp`, and `mkdir` commands; reference SC2086 or equivalent; explain word-splitting risk for filenames with spaces
2. Identify that `for file in $(ls ...)` is unreliable (SC2045 or equivalent) due to word-splitting and glob expansion; suggest glob expansion instead (`for file in "$BACKUP_DIR"/*`)
3. Apply the four-part issue format for each issue: (a) problematic code, (b) issue explanation with ShellCheck code, (c) corrected code, (d) why the fix is better
4. Produce a complete corrected script with all identified issues fixed
5. State a clear PASSED or FAILED result with error and warning counts matching the issues found

## Success Criteria

- **Unquoted $SOURCE and $DEST identified**: Agent identifies that `$SOURCE` and `$DEST` are used unquoted; references SC2086 or equivalent; explains word-splitting risk
- **$(ls $BACKUP_DIR) anti-pattern identified**: Agent identifies that `for file in $(ls ...)` is unreliable and suggests glob expansion instead
- **Four-part issue format used**: For each issue the agent shows: (a) the problematic code, (b) the issue explanation with ShellCheck code, (c) the corrected code, (d) why the fix is better
- **Corrected script produced**: Agent produces a complete corrected script where all identified issues are fixed
- **Validation result stated**: Agent states a clear PASSED or FAILED result with error and warning counts

## Failure Conditions

- Agent misses the unquoted `$SOURCE` and `$DEST` variables
- Agent misses the `$(ls ...)` anti-pattern
- Agent does not apply the four-part issue format (problematic code, explanation, fix, improvement rationale)
- Agent produces a corrected script that still contains unquoted variables or the `$(ls ...)` pattern
- Agent does not state a PASSED or FAILED verdict with counts
