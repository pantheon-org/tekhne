---
name: opencode-build-tool
description: |-
  Build custom tools and plugins using the OpenCode SDK packages. Use when creating custom tools, implementing tool schemas, building plugins, managing OpenCode sessions programmatically, using opencode-ai/sdk, using opencode-ai/plugin, creating a tool.ts file, writing an execute handler, tool.schema, ToolContext, createOpencode, plugin hooks, custom auth, integrating with OpenCode programmatically, automating OpenCode from a script, building a CI tool that uses OpenCode, or creating a tool that calls an external API. Also use for "Zod schema validation", "execute handler", "@opencode-ai/plugin", "@opencode-ai/sdk", "event streaming", "session management", "abort signal", "tool discovery", "auto-discover tool file", "multiple tools per file", "tool factory", "string return type", "tool description LLM sees".

  Examples:
  - user: "Create a tool that searches our internal API" → tool() with execute handler + abort signal
  - user: "Build a CI script that runs OpenCode on a prompt" → createOpencode() SDK client
  - user: "Add a tool with required and optional params" → tool.schema.string() + .optional()
  - user: "Stream OpenCode events from a script" → SDK client with event streaming
  - user: "Make my tool cancellable" → propagate abort signal to fetch/child process
---

# OpenCode SDK Development

## Quick Start: Custom Tool in 3 Steps

1. Create `.opencode/tool/my-tool.ts`:

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: "Brief description of what the tool does",
  args: {
    query: tool.schema.string().describe("Search query")
  },
  async execute({ query }, { abort }) {
    return `Result for: ${query}`
  }
})
```

1. Restart OpenCode — tools are auto-discovered.
1. Filename = tool name: `my-tool.ts` → `my-tool`.

## Mindset

- `@opencode-ai/plugin` — **Tools + Hooks**: Add LLM-callable tools, intercept execution, react to events
- `@opencode-ai/sdk` — **Client API**: Control OpenCode from external scripts, session management

**When to use**: Adding new capabilities the agent doesn't have, automating OpenCode from CI/scripts, or integrating external APIs as first-class tools.
**When NOT to use**: Built-in tool exists (`client.tool.list()`). Need shell (use `$` from Bun). Logic belongs in system prompt (`AGENTS.md`). Need MCP.

> Full decision guide in [advanced-patterns.md](references/advanced-patterns.md).

## Tool File Locations

```bash
.opencode/tool/*.ts          # project-specific
~/.config/opencode/tool/*.ts  # global
```

Multiple named exports per file → multiple tools: `filename_exportname`

## Anti-Patterns

NEVER return non-string from `execute`. WHY: The tool runtime expects a string response; returning an object throws at runtime.
```typescript
// BAD - throws at runtime
async execute({ query }) { return { results: [] } }
// GOOD - serialize first
async execute({ query }) { return JSON.stringify({ results: [] }) }
```

NEVER `import { tool } from "@opencode-ai/sdk"`. WHY: The SDK package has no `tool()` export. The tool factory is only in `@opencode-ai/plugin`.
```typescript
// BAD - import error at runtime
import { tool } from "@opencode-ai/sdk"
// GOOD
import { tool } from "@opencode-ai/plugin"
```

NEVER ignore the `abort` signal in long-running operations. WHY: Ignoring abort leaves processes dangling after the user cancels, consuming resources.
```typescript
// BAD - dangling fetch after cancel
async execute({ url }) { return await fetch(url).then(r => r.text()) }
// GOOD - propagate abort
async execute({ url }, { abort }) { return await fetch(url, { signal: abort }).then(r => r.text()) }
```

## Reference Files

- [advanced-patterns.md](references/advanced-patterns.md) — patterns, anti-patterns, SDK examples; [sdk-api.md](references/sdk-api.md) — SDK client API; [plugin-api.md](references/plugin-api.md) — plugin hooks
- [basic-tool.ts](references/basic-tool.ts) — simple tool; [full-plugin.ts](references/full-plugin.ts) — complete plugin

Run `bun run opencode run "list tools"` to verify your tool is discovered and named correctly.

```yaml
---
description: custom tools opencode SDK plugin execute handler abort signal Zod schema session management event streaming
---
```

## Eval Scenarios

- [Scenario 0: Create a custom todo-search tool with schema validation](evals/scenario-0/task.md)
- [Scenario 1: Use SDK client to create session and send prompt](evals/scenario-1/task.md)
- [Scenario 2: Implement abort signal handling in long-running tools](evals/scenario-2/task.md)
