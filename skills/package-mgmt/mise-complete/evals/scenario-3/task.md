# Migrate from asdf .tool-versions to mise.toml

## Problem Description

A project currently uses asdf for tool version management. The `.tool-versions` file is:

```
nodejs 20.11.0
python 3.11.7
ruby 3.3.0
terraform 1.7.4
```

The team also has these npm scripts in `package.json`:

```json
{
  "scripts": {
    "dev": "node server.js",
    "test": "jest --coverage",
    "lint": "eslint src/"
  }
}
```

The project has no existing `mise.toml`. Migrate the tooling to Mise.

Produce:
1. `mise.toml` — with all four tools pinned at the same versions, the three tasks defined as Mise tasks, and no secrets
2. `MIGRATION.md` — listing the steps taken and any instructions for team members (including what to do about the old `.tool-versions` file and any overlap with asdf)
