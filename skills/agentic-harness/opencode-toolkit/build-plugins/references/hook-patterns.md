# Hook Patterns Reference

> All hook implementation patterns with examples

## patterns

## 1. Event Hook (Reactive)

Listen to all events, discriminate by `type`:

```typescript
return {
  event: async ({ event }) => {
    switch (event.type) {
      case "session.idle":
        console.log("Session completed:", event.properties.sessionID)
        break
      case "file.edited":
        console.log("File changed:", event.properties.file)
        break
    }
  },
}
```

## 2. Custom Tools

Register tools the LLM can call:

```typescript
import { type Plugin, tool } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async (ctx) => {
  return {
    tool: {
      lint: tool({
        description: "Run ESLint on a file",
        args: {
          file: tool.schema.string().describe("File path to lint"),
          fix: tool.schema.boolean().optional().describe("Auto-fix issues"),
        },
        async execute(args, context) {
          const result = await ctx.$`eslint ${args.fix ? "--fix" : ""} ${args.file}`.quiet()
          return result.text()
        },
      }),
    },
  }
}
```

## 3. Tool Execution Hooks

Intercept before/after tool execution:

```typescript
return {
  // Modify args or throw to block
  "tool.execute.before": async (input, output) => {
    if (input.tool === "read" && output.args.filePath?.includes(".env")) {
      throw new Error("Reading .env files is blocked")
    }
  },

  // Modify output/title/metadata
  "tool.execute.after": async (input, output) => {
    console.log(`Tool ${input.tool} completed`)
    // Modify: output.title, output.output, output.metadata
  },
}
```

## 4. Permission Hook

Override permission decisions:

```typescript
return {
  "permission.ask": async (input, output) => {
    // input: { id, type, pattern, sessionID, messageID, title, metadata }
    // output.status: "ask" | "deny" | "allow"

    if (input.type === "bash" && input.metadata.command?.includes("rm -rf")) {
      output.status = "deny"
    }
  },
}
```

## 5. Chat Hooks

Modify messages or LLM parameters:

```typescript
return {
  // Intercept user messages
  "chat.message": async (input, output) => {
    // input: { sessionID, agent?, model?, messageID? }
    // output: { message: UserMessage, parts: Part[] }
    console.log("User message:", output.message)
  },

  // Modify LLM parameters per request
  "chat.params": async (input, output) => {
    // input: { sessionID, agent, model, provider, message }
    // output: { temperature, topP, topK, options }
    if (input.agent === "creative") {
      output.temperature = 0.9
    }
  },
}
```

## 6. Auth Hook

Add custom provider authentication:

```typescript
return {
  auth: {
    provider: "my-provider",
    methods: [
      {
        type: "api",
        label: "API Key",
        prompts: [
          {
            type: "text",
            key: "apiKey",
            message: "Enter your API key",
            validate: (v) => (v.length < 10 ? "Key too short" : undefined),
          },
        ],
        async authorize(inputs) {
          return { type: "success", key: inputs!.apiKey }
        },
      },
    ],
  },
}
```

## 7. Compaction Hook

Customize session compaction:

```typescript
return {
  "experimental.session.compacting": async (input, output) => {
    // Add context to default prompt
    output.context.push("Remember: user prefers TypeScript")

    // OR replace entire prompt
    output.prompt = "Summarize this session focusing on code changes..."
  },
}
```

## 8. Config Hook

Modify configuration on load:

```typescript
return {
  config: async (config) => {
    // Mutate config object
    config.theme = "dark"
  },
}
```

## quick_reference

## Hook Signature Quick Reference

| Hook                  | Signature                      | Mutate             |
| --------------------- | ------------------------------ | ------------------ |
| `event`               | `({ event }) => void`          | Read-only          |
| `config`              | `(config) => void`             | Mutate config      |
| `tool`                | Object of `tool()` definitions | N/A                |
| `auth`                | `AuthHook` object              | N/A                |
| `chat.message`        | `(input, output) => void`      | Mutate output      |
| `chat.params`         | `(input, output) => void`      | Mutate output      |
| `permission.ask`      | `(input, output) => void`      | Set output.status  |
| `tool.execute.before` | `(input, output) => void`      | Mutate output.args |
| `tool.execute.after`  | `(input, output) => void`      | Mutate output      |
| `experimental.*`      | `(input, output) => void`      | Mutate output      |

## Implementation Procedure

### Step 1: Verify SDK Reference (REQUIRED)

Before creating any plugin, regenerate the API reference to ensure accuracy:

```bash
bun run .opencode/skill/opencode-build-plugins/scripts/extract-plugin-api.ts
```

This generates `references/hooks.md`, `references/events.md`, `references/tool-helper.md`.

### Step 2: Validate Feasibility

**Feasible as plugins:**
- Intercepting/blocking tool calls
- Reacting to events (file edits, session completion, etc.)
- Adding custom tools for the LLM
- Modifying LLM parameters (temperature, etc.)
- Custom auth flows for providers
- Customizing session compaction
- Displaying status messages (toasts, inline)

**NOT feasible (inform user):**
- Modifying TUI rendering or layout
- Adding new built-in tools (requires OC source)
- Changing core agent behavior/prompts
- Intercepting assistant responses mid-stream
- Adding new keybinds or commands
- Modifying internal file read/write
- Adding new permission types

**If not feasible**, inform user clearly. Suggest:
- OC core changes: contribute to `packages/opencode`
- MCP tools: use MCP server configuration
- Simple automation: use shell scripts

### Step 3: Design Plugin

Follow modular design principles from [CODING-TS.MD](CODING-TS.MD):

- **Modular structure**: Split complex plugins into multiple focused files
- **Single purpose**: Each function does ONE thing well
- **DRY**: Extract common patterns into shared utilities
- **Small files**: Keep individual files under 150 lines
- **No monoliths**: MUST NOT put all plugin code in a single `index.ts`

### Step 4: Plugin Structure (Non-Monolithic)

For complex plugins, use a modular directory structure:

```
.opencode/plugins/my-plugin/
├── index.ts          # Entry point, exports Plugin
├── types.ts          # TypeScript types/interfaces
├── utils.ts          # Shared utilities
├── hooks/            # Hook implementations
│   ├── event.ts
│   └── tool-execute.ts
└── tools/            # Custom tool definitions
    └── my-tool.ts
```

**Example modular index.ts:**

```typescript
import type { Plugin } from "@opencode-ai/plugin"
import { eventHooks } from "./hooks/event"
import { toolHooks } from "./hooks/tool-execute"
import { customTools } from "./tools"

export const MyPlugin: Plugin = async ({ project, client }) => {
  return {
    ...eventHooks({ client }),
    ...toolHooks({ client }),
    tool: customTools,
  }
}
```

## Common Plugin Patterns

### Security Guard — Block Dangerous Commands

```typescript
import type { Plugin } from "@opencode-ai/plugin"

const BLOCKED = ["rm -rf", "git push --force", "DROP TABLE"]

export const SecurityPlugin: Plugin = async () => ({
  "tool.execute.before": async (input) => {
    if (input.tool === "bash") {
      const cmd = input.args?.command ?? ""
      if (BLOCKED.some((pattern) => cmd.includes(pattern))) {
        throw new Error(`Blocked dangerous command: ${cmd}`)
      }
    }
  }
})
```

### Audit Logger — Log All Tool Calls

```typescript
import type { Plugin } from "@opencode-ai/plugin"
import { appendFileSync } from "fs"

export const AuditPlugin: Plugin = async ({ project }) => ({
  "tool.execute.after": async (input, output) => {
    const entry = JSON.stringify({
      ts: new Date().toISOString(),
      tool: input.tool,
      args: input.args,
      success: !output.error
    })
    appendFileSync(".opencode/audit.log", entry + "\n")
  }
})
```

### Session Notifier — Toast on Session End

```typescript
import type { Plugin } from "@opencode-ai/plugin"

export const NotifyPlugin: Plugin = async ({ client }) => ({
  event: async ({ event }) => {
    if (event.type === "session.completed") {
      await client.tui.showToast({
        message: "Session complete",
        variant: "success"
      })
    }
  }
})
```

### Custom Tool — Expose a New Callable Function

```typescript
import type { Plugin } from "@opencode-ai/plugin"
import { tool } from "@opencode-ai/plugin"

const lookupUser = tool({
  description: "Look up a user by email in the internal directory",
  args: {
    email: tool.schema.string().describe("User email address")
  },
  async execute({ email }) {
    const result = await fetch(`https://api.example.com/users?email=${email}`)
    return JSON.stringify(await result.json())
  }
})

export const DirectoryPlugin: Plugin = async () => ({
  tool: { "lookup-user": lookupUser }
})
