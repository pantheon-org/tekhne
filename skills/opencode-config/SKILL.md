---
name: opencode-config
description: Configure OpenCode via opencode.json and AGENTS.md with deterministic provider setup, model selection, permission policies, formatter behavior, and environment variable handling; use when editing opencode configuration, setting model/provider defaults, tightening agent permissions, or troubleshooting OpenCode config behavior.
---

# OpenCode Configuration

Navigation hub for OpenCode configuration tasks.

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
4. Validate syntax and run OpenCode config test.
5. Confirm expected behavior and document key constraints.

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

WHY: secret leakage through source control history is irreversible.
BAD: `"apiKey": "sk-..."`. GOOD: `"baseEnv": "OPENAI_API_KEY"`.

### NEVER use broad filesystem or shell permissions by default

WHY: permissive defaults increase blast radius of mistakes.
BAD: root-level read/write and unrestricted shell. GOOD: scoped paths and explicit command allowlists.

### NEVER use ambiguous model names

WHY: providers may resolve generic model aliases differently.
BAD: `"model": "gpt-4"`. GOOD: provider-qualified or exact model identifiers.

### NEVER skip verification after permission changes

WHY: permission regressions are often silent until runtime.
BAD: edit-and-commit without test. GOOD: run `opencode run "test"` and validate behavior.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Provider setup and model mapping | [references/provider-configuration.md](references/provider-configuration.md) |
| Permission structure and patterns | [references/permission-schema.md](references/permission-schema.md) |
| Full config field reference | [references/config-schema.md](references/config-schema.md) |

## References

- [OpenCode Docs](https://opencode.ai/docs/)
