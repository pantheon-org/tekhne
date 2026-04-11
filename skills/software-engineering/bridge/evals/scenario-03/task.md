# Task: List All Bridges

The user issues the command:

```
/bridge list
```

Glob all YAML files in the configured bridges directory, parse each file, sort by date descending, cap output at 10 items, and display using the canonical `🔗 Bridges (last 10)` format. If the directory is empty, show the empty-state message with a usage hint. Do not write or modify any files.
