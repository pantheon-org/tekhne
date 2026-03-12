---
name: opencode-configure
description: Configure OpenCode via opencode.json and AGENTS.md with deterministic provider setup, model selection, permission policies, formatter behavior, and environment variable handling; use when editing opencode configuration, setting model/provider defaults, tightening agent permissions, or troubleshooting OpenCode config behavior.
---

# OpenCode Configuration

Navigation hub for OpenCode configuration tasks.

## Quick Start

**Add a provider** — create/edit `opencode.json` in your project root:

```json
{
  "providers": [
    {
      "id": "anthropic",
      "baseEnv": "ANTHROPIC_API_KEY",
      "models": [
        { "id": "claude-sonnet-4-5", "default": true }
      ]
    }
  ]
}
```

**Add project instructions** — create `AGENTS.md` in project root:

```markdown
## Workflow Rules
- Run `npm test` after every code change.
- Never modify `package.json` without user confirmation.
```

**Validate config**:
```bash
jq . opencode.json && opencode run "test"
```

---

## When to Use

- Adding or updating providers and model preferences.
- Editing permissions for tools, shell commands, or file access.
- Updating formatter or instruction configuration in `opencode.json`.
- Creating or refining `AGENTS.md` instructions.
- Diagnosing configuration precedence or behavior mismatches.

## When Not to Use

- General coding implementation tasks.
- Plugin/tool development outside standard config fields.
- Agent personality design unrelated to config structure.

## Mindset

- Prefer explicit, minimal configuration changes over broad rewrites.
- Treat permissions and secrets as security-critical defaults.
- Validate behavior immediately after each meaningful config edit.

## Scope

| File | Purpose |
| --- | --- |
| `opencode.json` | Main OpenCode runtime configuration |
| `AGENTS.md` | Project-level behavior and workflow rules |
| `.env`/shell env | Provider keys and environment-backed configuration |
| `references/*.md` | Deep configuration guidance |

Out of scope: plugin authoring and custom tool SDK development.

## Workflow

1. Identify target config file(s) from request intent.
2. Read current config state and precedence (project vs global).
3. Apply minimal configuration edits with explicit rationale.
4. Validate syntax: `jq . opencode.json >/dev/null && rg -n "API_KEY|baseEnv|permission" opencode.json .env*`
5. Run a smoke test: `opencode run "test"`
6. Confirm expected behavior and document key constraints. If errors surface, re-read config precedence before widening permissions.

## Quick Commands

```bash
rg -n "model|provider|permission|instructions" opencode.json AGENTS.md
```

```bash
opencode run "test"
```

```bash
jq . opencode.json >/dev/null
```

```bash
rg -n "API_KEY|baseEnv|permission" opencode.json .env*
```

## Configuration Examples

### Minimal `opencode.json` — provider with `baseEnv` pattern

```json
{
  "providers": [
    {
      "id": "openai",
      "baseEnv": "OPENAI_API_KEY",
      "models": [
        { "id": "gpt-4o-2024-08-06", "default": true }
      ]
    }
  ],
  "permissions": {
    "filesystem": { "read": ["./src", "./docs"], "write": ["./src"] },
    "shell": { "allow": ["npm test", "jq"] }
  },
  "formatter": { "enabled": true }
}
```

### Minimal `AGENTS.md` template

```markdown
# Project Agent Instructions

## Scope
- Work only within `src/` and `docs/` unless explicitly told otherwise.

## Workflow Rules
- Run `npm test` after every code change.
- Never modify `opencode.json` directly; propose changes for human review.

## Constraints
- Do not execute destructive shell commands (rm -rf, git push --force).
- Prefer read operations before any write or delete action.
```

## Gotchas

- Provider order matters when multiple providers can satisfy a model name.
- Agent-level permissions can be stricter than global defaults.
- Environment precedence is typically shell > dotenv loader > config defaults.
- Model identifiers must match provider format exactly.

## Production Caveats

- NEVER commit raw API keys to repository config files.
- Prefer least-privilege permissions and widen only when required.
- Re-test config after permission or provider edits.

## Anti-Patterns

### NEVER commit API keys directly in config

- **WHY**: secret leakage through source control history is irreversible.
- **BAD**: `"apiKey": "sk-..."`.
- **GOOD**: `"baseEnv": "OPENAI_API_KEY"`.

### NEVER use broad filesystem or shell permissions by default

- **WHY**: permissive defaults increase blast radius of mistakes.
- **BAD**: root-level read/write and unrestricted shell.
- **GOOD**: scoped paths and explicit command allowlists.

### NEVER use ambiguous model names

- **WHY**: providers may resolve generic model aliases differently.
- **BAD**: `"model": "gpt-4"`.
- **GOOD**: provider-qualified or exact model identifiers.

### NEVER skip verification after permission changes

- **WHY**: permission regressions are often silent until runtime.
- **BAD**: edit-and-commit without test.
- **GOOD**: run `opencode run "test"` and validate behavior.

### NEVER put AGENTS.md instructions in opencode.json and vice versa

- **WHY**: `opencode.json` is runtime config (providers, permissions, tools). `AGENTS.md` is behavioral guidance for the agent. Mixing them causes ignored instructions or broken config parsing.
- **BAD**: putting `instructions:` blocks inside `opencode.json`, or adding JSON config snippets inside `AGENTS.md`.
- **GOOD**: runtime settings → `opencode.json`; workflow rules, constraints, conventions → `AGENTS.md`.

### NEVER use global config for project-specific settings

- **WHY**: global config (`~/.config/opencode/opencode.json`) bleeds into unrelated projects, causing unexpected behavior across your entire environment.
- **BAD**: adding a project's `npm test` to the global shell allowlist, or setting a project-specific model globally.
- **GOOD**: put project-specific config in project-root `opencode.json`. Reserve global config for cross-project defaults (personal API keys via `baseEnv`, editor preferences).

## Quick Reference

| Topic | Reference |
| --- | --- |
| Provider setup and model mapping | [references/provider-configuration.md](references/provider-configuration.md) |
| Permission structure and patterns | [references/permission-schema.md](references/permission-schema.md) |
| Full config field reference | [references/config-schema.md](references/config-schema.md) |

## References

- [OpenCode Docs](https://opencode.ai/docs/)

## Eval Scenarios

- [Scenario 0: Configure Anthropic provider with env variable](evals/scenario-0/task.md)
- [Scenario 1: Place behavioral rules in AGENTS.md vs opencode.json](evals/scenario-1/task.md)
- [Scenario 2: Fix global vs project-level config scope issue](evals/scenario-2/task.md)
