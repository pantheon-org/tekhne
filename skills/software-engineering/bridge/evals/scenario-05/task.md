# Task: Refuse Capture When Workspace Root Is Not Configured

The user issues:

```
/bridge PROJ-A → PROJ-B: this pattern transfers
```

But the workspace root environment variable (e.g. `$PRAXIS_DIR`) is not set. The skill must detect this, refuse to write any file, and display the warning message directing the user to configure the workspace root. It must not proceed with file creation.
