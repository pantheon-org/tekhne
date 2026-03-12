# OpenCode Plugin API Reference

Complete API reference for `@opencode-ai/plugin`.

## Table of Contents

1. [Tool Definition](#tool-definition)
2. [Schema Types](#schema-types)
3. [Plugin System](#plugin-system)
4. [Plugin Hooks](#plugin-hooks)
5. [Auth Hooks](#auth-hooks)
6. [Type Definitions](#type-definitions)

---

## Tool Definition

### tool()

Create a custom tool with schema validation.

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: string,
  args: ZodRawShape,
  execute: (args: InferredArgs, context: ToolContext) => Promise<string>
})
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `description` | `string` | Human-readable description shown to AI |
| `args` | `ZodRawShape` | Zod schema defining tool arguments |
| `execute` | `Function` | Async function that returns result string |

#### ToolContext

```typescript
type ToolContext = {
  sessionID: string      // Current session identifier
  messageID: string      // Current message identifier
  agent: string          // Active agent name
  abort: AbortSignal     // Cancellation signal
}
```

#### Return Value

The `execute` function must return a `Promise<string>`. The string is passed back to the AI as the tool result.

---

## Schema Types

`tool.schema` is an alias for Zod. Use it to define argument types.

### String

```typescript
args: {
  // Required string
  name: tool.schema.string().describe("User name"),

  // Optional string
  title: tool.schema.string().optional().describe("Title"),

  // String with default
  format: tool.schema.string().default("json").describe("Format"),

  // String with constraints
  email: tool.schema.string().email().describe("Email address"),
  url: tool.schema.string().url().describe("URL"),
  uuid: tool.schema.string().uuid().describe("UUID"),

  // Min/max length
  code: tool.schema.string().min(1).max(100).describe("Code snippet")
}
```

### Number

```typescript
args: {
  // Required number
  count: tool.schema.number().describe("Item count"),

  // Integer
  index: tool.schema.number().int().describe("Index"),

  // With constraints
  age: tool.schema.number().min(0).max(120).describe("Age"),
  price: tool.schema.number().positive().describe("Price"),

  // Optional with default
  limit: tool.schema.number().default(10).describe("Limit")
}
```

### Boolean

```typescript
args: {
  // Required boolean
  enabled: tool.schema.boolean().describe("Enable feature"),

  // Optional with default
  verbose: tool.schema.boolean().default(false).describe("Verbose output")
}
```

### Enum

```typescript
args: {
  // String enum
  status: tool.schema.enum(["pending", "active", "done"]).describe("Status"),

  // Native enum
  level: tool.schema.nativeEnum(LogLevel).describe("Log level")
}
```

### Arrays

```typescript
args: {
  // String array
  tags: tool.schema.array(tool.schema.string()).describe("Tags"),

  // With constraints
  ids: tool.schema.array(tool.schema.string()).min(1).max(10).describe("IDs")
}
```

### Objects

```typescript
args: {
  config: tool.schema.object({
    host: tool.schema.string(),
    port: tool.schema.number()
  }).describe("Configuration object")
}
```

### Unions

```typescript
args: {
  // Literal union
  format: tool.schema.union([
    tool.schema.literal("json"),
    tool.schema.literal("xml"),
    tool.schema.literal("csv")
  ]).describe("Output format"),

  // Type union
  value: tool.schema.union([
    tool.schema.string(),
    tool.schema.number()
  ]).describe("String or number value")
}
```

### Nullable / Optional

```typescript
args: {
  // Optional (can be undefined)
  comment: tool.schema.string().optional(),

  // Nullable (can be null)
  parent: tool.schema.string().nullable(),

  // Both
  value: tool.schema.string().optional().nullable()
}
```

---

## Plugin System

### Plugin Function

Plugins are async functions that return hooks.

```typescript
import type { Plugin, Hooks } from "@opencode-ai/plugin"

const myPlugin: Plugin = async (input) => {
  // input contains client, project, directory, worktree, $

  const hooks: Hooks = {
    // Define hooks here
  }

  return hooks
}

export default myPlugin
```

### PluginInput

```typescript
type PluginInput = {
  client: OpencodeClient       // SDK client instance
  project: Project             // Current project info
  directory: string            // Plugin directory path
  worktree: string             // Git worktree path
  $: BunShell                  // Bun shell for commands
}
```

---

## Plugin Hooks

### event

Handle real-time server events.

```typescript
{
  event: async ({ event }) => {
    // event.type: string
    // event.data: any
    console.log("Event:", event.type)
  }
}
```

### config

Modify configuration on load.

```typescript
{
  config: async (config) => {
    // Modify config object
    config.theme = "dark"
  }
}
```

### tool

Register custom tools.

```typescript
{
  tool: {
    myTool: tool({
      description: "My tool",
      args: { input: tool.schema.string() },
      async execute({ input }) {
        return `Processed: ${input}`
      }
    }),

    anotherTool: tool({
      description: "Another tool",
      args: {},
      async execute() {
        return "Done"
      }
    })
  }
}
```

### chat.message

Modify messages before sending to LLM.

```typescript
{
  "chat.message": async (input, output) => {
    // input: { sessionID, agent?, model?, messageID? }
    // output: { message: UserMessage, parts: Part[] }

    // Modify parts
    output.parts.push({
      type: "text",
      text: "Additional context"
    })
  }
}
```

### chat.params

Modify LLM parameters.

```typescript
{
  "chat.params": async (input, output) => {
    // input: { sessionID, agent, model, provider, message }
    // output: { temperature, topP, options }

    output.temperature = 0.7
    output.topP = 0.9
    output.options.maxTokens = 4096
  }
}
```

### permission.ask

Handle permission requests.

```typescript
{
  "permission.ask": async (input, output) => {
    // input: Permission object
    // output: { status: "ask" | "deny" | "allow" }

    if (input.tool === "dangerous_tool") {
      output.status = "deny"
    } else {
      output.status = "allow"
    }
  }
}
```

### tool.execute.before

Pre-process tool arguments.

```typescript
{
  "tool.execute.before": async (input, output) => {
    // input: { tool, sessionID, callID }
    // output: { args: any }

    if (input.tool === "myTool") {
      output.args.timestamp = Date.now()
    }
  }
}
```

### tool.execute.after

Post-process tool output.

```typescript
{
  "tool.execute.after": async (input, output) => {
    // input: { tool, sessionID, callID }
    // output: { title, output, metadata }

    output.title = `[${input.tool}] ${output.title}`
    output.metadata.processedAt = Date.now()
  }
}
```

---

## Auth Hooks

Custom authentication providers.

```typescript
{
  auth: {
    provider: "my-provider",

    // Optional: Custom credential loader
    loader: async (auth, provider) => {
      const credentials = await auth()
      return { apiKey: credentials.key }
    },

    methods: [
      // API key method
      {
        type: "api",
        label: "API Key",
        prompts: [
          {
            type: "text",
            key: "apiKey",
            message: "Enter your API key",
            placeholder: "sk-...",
            validate: (value) => {
              if (!value.startsWith("sk-")) {
                return "Invalid key format"
              }
            }
          }
        ],
        authorize: async (inputs) => {
          return {
            type: "success",
            key: inputs!.apiKey
          }
        }
      },

      // OAuth method
      {
        type: "oauth",
        label: "Sign in with OAuth",
        prompts: [
          {
            type: "select",
            key: "region",
            message: "Select region",
            options: [
              { label: "US", value: "us", hint: "United States" },
              { label: "EU", value: "eu", hint: "Europe" }
            ]
          }
        ],
        authorize: async (inputs) => {
          const authUrl = `https://oauth.example.com/authorize?region=${inputs?.region}`

          return {
            url: authUrl,
            instructions: "Complete sign-in in browser",
            method: "auto",
            callback: async () => {
              // Poll for completion
              return {
                type: "success",
                key: "obtained-api-key"
              }
            }
          }
        }
      }
    ]
  }
}
```

### Auth Method Types

#### API Method

```typescript
{
  type: "api",
  label: string,
  prompts?: Array<TextPrompt | SelectPrompt>,
  authorize?: (inputs?: Record<string, string>) => Promise<
    | { type: "success", key: string, provider?: string }
    | { type: "failed" }
  >
}
```

#### OAuth Method

```typescript
{
  type: "oauth",
  label: string,
  prompts?: Array<TextPrompt | SelectPrompt>,
  authorize: (inputs?: Record<string, string>) => Promise<AuthOAuthResult>
}

type AuthOAuthResult = {
  url: string,
  instructions: string,
  method: "auto" | "code",
  callback: (code?: string) => Promise<
    | { type: "success", provider?: string } & (
        | { refresh: string, access: string, expires: number }
        | { key: string }
      )
    | { type: "failed" }
  >
}
```

### Prompt Types

```typescript
// Text input
{
  type: "text",
  key: string,
  message: string,
  placeholder?: string,
  validate?: (value: string) => string | undefined,
  condition?: (inputs: Record<string, string>) => boolean
}

// Select dropdown
{
  type: "select",
  key: string,
  message: string,
  options: Array<{
    label: string,
    value: string,
    hint?: string
  }>,
  condition?: (inputs: Record<string, string>) => boolean
}
```

---

## Type Definitions

### ToolDefinition

```typescript
type ToolDefinition = {
  description: string
  args: ZodRawShape
  execute: (args: any, context: ToolContext) => Promise<string>
}
```

### Hooks

```typescript
interface Hooks {
  event?: (input: { event: Event }) => Promise<void>
  config?: (input: Config) => Promise<void>
  tool?: { [key: string]: ToolDefinition }
  auth?: AuthHook
  "chat.message"?: (input: ChatMessageInput, output: ChatMessageOutput) => Promise<void>
  "chat.params"?: (input: ChatParamsInput, output: ChatParamsOutput) => Promise<void>
  "permission.ask"?: (input: Permission, output: PermissionOutput) => Promise<void>
  "tool.execute.before"?: (input: ToolExecuteInput, output: ToolBeforeOutput) => Promise<void>
  "tool.execute.after"?: (input: ToolExecuteInput, output: ToolAfterOutput) => Promise<void>
}
```

### ProviderContext

```typescript
type ProviderContext = {
  source: "env" | "config" | "custom" | "api"
  info: Provider
  options: Record<string, any>
}
```

---

## Multiple Tool Exports

Export multiple tools from one file:

```typescript
// file: mytools.ts

import { tool } from "@opencode-ai/plugin"

export const search = tool({
  description: "Search files",
  args: { query: tool.schema.string() },
  async execute({ query }) { /* ... */ }
})

export const replace = tool({
  description: "Replace text",
  args: {
    search: tool.schema.string(),
    replace: tool.schema.string()
  },
  async execute({ search, replace }) { /* ... */ }
})

// Creates tools: mytools_search, mytools_replace
```

---

## Best Practices

1. **Descriptive Names**: Tool names come from filenames - use clear, action-oriented names
2. **Detailed Descriptions**: Write descriptions that help AI understand when to use the tool
3. **Validate Everything**: Use Zod schemas to catch invalid inputs early
4. **Return Strings**: Execute must always return a string (use JSON.stringify for objects)
5. **Handle Abort**: Check context.abort for long-running operations
6. **Error Messages**: Return helpful error messages as strings, don't throw
