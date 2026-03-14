---
name: opencode-build-plugins
description: "Create OpenCode plugins using the opencode-ai/plugin SDK. Use when user wants to build a plugin, extend OpenCode, intercept tool execution, add custom tools, react to events, create a hook, block commands, add custom auth, or add automation to OpenCode. Also use for 'opencode plugin', 'custom tool', 'tool hook', 'plugin hook', 'logging for all tool calls', 'restrict what commands the AI can run', 'lifecycle hooks', 'middleware', 'tool.execute.before', 'tool.execute.after', 'auth hook', 'chat.params', 'intercept bash commands', 'block file writes', 'add audit logging', 'custom model provider auth', 'inject context before LLM call', 'fire-and-forget background task', 'watch session events'."
---

# Creating OpenCode Plugins

## Quick Start

Plugins live in `.opencode/plugins/<name>/index.ts` (project) or `~/.config/opencode/plugins/<name>/index.ts` (global).

```typescript
import type { Plugin } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async ({ project, client, $, directory, worktree }) => {
  return {
    // Add hooks here — see references/hooks.md for available hooks
  }
}
```

Register in `opencode.json`:
```jsonc
{ "plugin": ["file:///.opencode/plugins/my-plugin/index.ts"] }
```

Optional — document plugin purpose in a companion agent file:
```markdown
---
description: Plugin guard — intercepts tool calls, validates bash commands, blocks dangerous deletions, logs audit trail, hooks session events
---
```

Then test: `opencode run hi`

## Mindset

Plugins are **async factory functions** returning hook implementations. They run once on load. Hooks fire at lifecycle points (tool execution, events, config load). Think of them as middleware layers.

Use `tool.execute.before` to intercept/block, `tool.execute.after` to log, `event:` for file edits/session end, `auth:` for custom model auth, `chat.params:` to modify LLM params, `tool:` key to add callable tools.

**When to use**: Intercept/log tool calls, add custom auth, react to lifecycle events, or extend OpenCode with new callable tools.

**When NOT to use**: Built-in tool exists (check `client.tool.list()`). Only need a shortcut (use slash command). Need behavior guidance (use `AGENTS.md`). Need MCP (use Model Context Protocol).

Verify plugin is loaded: run `bun run opencode run "list your tools"` and confirm the plugin output appears.

## Anti-Patterns

NEVER call `client.registerTool()` — it does not exist. WHY: The SDK has no such method; calling it throws at runtime.
```typescript
// BAD - runtime error
client.registerTool("my-tool", { ... })

// GOOD - return the tool from the factory
export const Plugin: Plugin = async ({ client }) => ({
  tool: { "my-tool": myTool }
})
```

NEVER write sync hook handlers. WHY: The plugin loader validates async signatures; a sync handler causes the entire plugin to be rejected at load time with a type error.
```typescript
// BAD - rejected at load
"tool.execute.before": (input) => { log(input) }

// GOOD - always async
"tool.execute.before": async (input) => { log(input) }
```

NEVER mutate `input.args` in `tool.execute.before` to block a tool — the input is read-only and silently ignored. WHY: Mutations have no effect; to block execution you must throw.
```typescript
// BAD - silently ignored
"tool.execute.before": async (input) => { input.args.command = "echo safe" }

// GOOD - throw to block
"tool.execute.before": async (input) => {
  if (isDangerous(input.args)) throw new Error("Blocked")
}
```

See [`references/hook-patterns.md`](references/hook-patterns.md) for complete anti-pattern list + full hook/event/tool/UI/testing/publishing reference.

## Eval Scenarios

- [Scenario 0: Block dangerous bash commands with tool.execute.before](evals/scenario-0/task.md)
- [Scenario 1: Show toast notifications on file edit events](evals/scenario-1/task.md)
- [Scenario 2: Add custom tool to plugin using tool key](evals/scenario-2/task.md)
