# Scenario 04: Full Validation Report for a Multi-Issue Bash Script

## User Prompt

Validate the following script using the bash-script-validator skill workflow. Produce a complete validation report following the exact response format from the skill: for each issue found, include (a) the problematic code, (b) the issue explanation with ShellCheck code, (c) the corrected code, and (d) why the fix improves the script. End with a summary section showing error/warning/info counts and a final PASSED or FAILED verdict.

```bash
#!/usr/bin/env bash

LOG_FILE=/var/log/app.log
CONFIG=$1

parse_config() {
    grep -r "setting=" $CONFIG | while read line; do
        key=$(echo $line | cut -d= -f1)
        value=$(echo $line | cut -d= -f2)
        export $key=$value
    done
}

write_log() {
    echo "[$(date)] $message" >> $LOG_FILE
}

main() {
    parse_config

    if [ $CONFIG == "" ]; then
        echo "No config provided"
        exit 1
    fi

    write_log
}

main
```

## Expected Behavior

1. Apply the four-part format to every issue: (a) problematic code, (b) issue explanation with ShellCheck code, (c) corrected code, (d) explanation of why the fix is better
2. Identify unquoted `$CONFIG` in `grep`, `cut` subshells, and the `[ ]` comparison; unquoted `$LOG_FILE` in the redirect; and unquoted `$key`/`$value` in `export`
3. Identify that `[ $CONFIG == "" ]` uses `==` which is not POSIX-compliant in `[ ]`; suggest `=` instead, and quote the variable
4. Identify that `grep ... | while read line` with unquoted `$line` is unsafe; suggest `IFS= read -r line` and quoting; or flag the grep pipe pattern
5. Identify that `export $key=$value` can overwrite critical environment variables if `CONFIG` is attacker-controlled
6. End with a summary section listing error count, warning count, info count, and a clear PASSED or FAILED verdict

## Success Criteria

- **Four-part format applied to every issue**: For each issue the response includes all four parts: (a) problematic code, (b) explanation with ShellCheck code, (c) corrected code, (d) explanation of improvement
- **Unquoted variables identified**: Agent identifies unquoted `$CONFIG` in grep, cut subshells, and `[ ]` comparison; unquoted `$LOG_FILE` in redirect; and unquoted `$key`/`$value` in export
- **== in [ ] identified**: Agent identifies that `[ $CONFIG == "" ]` uses `==` which is not POSIX-compliant in `[ ]`
- **Useless use of cat or piping anti-pattern flagged**: Agent identifies that `grep ... | while read line` with unquoted `$line` is unsafe; suggests `IFS= read -r line`
- **export key=value injection risk flagged**: Agent identifies that `export $key=$value` can overwrite critical environment variables if config is attacker-controlled
- **Summary section with counts and verdict**: Response ends with a summary section listing error count, warning count, info count, and a clear PASSED or FAILED verdict

## Failure Conditions

- Agent does not apply all four parts of the format to every issue
- Agent misses unquoted variable usage in any of the identified locations
- Agent does not identify the `==` vs `=` issue in `[ ]`
- Agent does not flag the `read line` without `-r` or the quoting issue in the `while` loop
- Agent does not identify the `export $key=$value` injection risk
- Agent omits the summary section with counts and PASSED/FAILED verdict
