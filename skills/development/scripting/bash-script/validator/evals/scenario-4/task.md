# Task: Full Validation Report for a Multi-Issue Bash Script

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
