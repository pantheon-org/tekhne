# Task: Validate a Bash Script with Quoting Issues

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
