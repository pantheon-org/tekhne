# Scenario 2: Fix Anti-Patterns in a Broken Command Template

## User Prompt

A user has a `/review` command that currently looks like this:

```markdown
---
description: Review code
agent: plan
---

Review $ARGUMENTS for bugs. After fixing $ARGUMENTS, run /test to verify.
```

Identify the problems with this command and produce a corrected version.

## Output Specification

Produce the corrected command markdown file.
