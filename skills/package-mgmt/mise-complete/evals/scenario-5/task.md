# Scenario 5: Diagnose a Failing mise install

## User Prompt

A repository has a `mise.toml` pinning `node = "22.4.0"` and `python = "3.12.3"`. A teammate reports that `mise install` fails on their machine with errors about shims missing from PATH and a plugin that could not be found. Explain the exact commands you would run, in order, to diagnose and fix this and how you would prove the machine ends up in the state the file describes.
