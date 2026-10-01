# Common FILTER Patterns

Use as needed. Order matters: parsers before modifiers.

```ini
# Parse structured fields from a log key
[FILTER]
    Name          parser
    Match         *
    Key_Name      log
    Parser        json
    Reserve_Data  On

# Include/exclude by field value
[FILTER]
    Name          grep
    Match         *
    Regex         level (error|fatal|critical)
    Exclude       path /health

# Multi-line (stack traces)
[FILTER]
    Name                  multiline
    Match                 *
    multiline.key_content log
    multiline.parser      java, python, go

# Lua custom scripting
[FILTER]
    Name    lua
    Match   *
    script  /fluent-bit/scripts/filter.lua
    call    process_record

# Throttle (rate limiting)
[FILTER]
    Name      throttle
    Match     *
    Rate      1000
    Window    5
    Interval  1m
```
