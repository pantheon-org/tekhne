---
name: opencode-build-tool
description: |-
  Build custom tools and plugins using the OpenCode SDK packages. Use when creating custom tools, implementing tool schemas, building plugins, managing OpenCode sessions programmatically, using opencode-ai/sdk, using opencode-ai/plugin, creating a tool.ts file, writing an execute handler, tool.schema, ToolContext, createOpencode, plugin hooks, custom auth, integrating with OpenCode programmatically, automating OpenCode from a script, building a CI tool that uses OpenCode, or creating a tool that calls an external API.
---

# OpenCode SDK Development

Guide for creating custom tools and plugins using the OpenCode SDK.

## Quick Start: Custom Tool in 3 Steps

**1. Create the file** in `.opencode/tool/my-tool.ts` (or `~/.config/opencode/tool/`):

```yaml
---
name: my-tool
description: Search for files matching a glob pattern
input:
  pattern:
    description: Glob pattern to match files
    type: string
  directory:
    description: Root directory to search from
    type: string
---
```

In TypeScript:

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: "Brief description of what the tool does",
  args: {
    query: tool.schema.string().describe("Search query")
  },
  async execute({ query }, context) {
    return `Result for: ${query}`
  }
})
```

**2. Restart OpenCode** — tools are auto-discovered on startup.

**3. The filename becomes the tool name** — `my-tool.ts` → tool named `my-tool`.

---

## Mindset

OpenCode SDK has two distinct roles:

| Package | Role | When to use |
|---------|------|-------------|
| `@opencode-ai/plugin` | **Tools + Plugin hooks** | Adding capabilities the LLM can invoke; intercepting tool execution; reacting to events |
| `@opencode-ai/sdk` | **Client API** | Controlling OpenCode programmatically from external scripts; session management; event streaming |

**Tools** are single-purpose callable functions the AI invokes. **Plugins** are full lifecycle integrations with hooks for events, auth, config, and tool interception. **SDK client** is for external programs that talk to a running OpenCode instance.

### Decision Guide: Tool vs Plugin vs SDK Client

| Goal | Use |
|------|-----|
| Give the AI a new callable function | Custom Tool (`@opencode-ai/plugin` `tool()`) |
| Intercept or log all tool calls | Plugin hook (`tool.execute.before/after`) |
| React to events (file edits, session end) | Plugin hook (`event`) |
| Modify LLM temperature globally | Plugin hook (`chat.params`) |
| Custom auth for a model provider | Plugin hook (`auth`) |
| Automate OpenCode from a CI script | SDK Client (`createOpencode()`) |
| List sessions / send prompts externally | SDK Client (`client.session.*`) |
| Stream real-time events from outside | SDK Client (`client.event.subscribe()`) |

### When Not to Use the SDK

- **When a built-in tool already exists** — check `client.tool.list()` before building a custom tool. Duplicating built-in tools is a common pitfall.
- **When you only need shell commands** — use `$` from Bun directly; wrapping simple shell invocations in a tool adds overhead without benefit.
- **When your logic belongs in the system prompt** — behavioral guidance (personas, constraints, formatting rules) lives in `opencode.json` or AGENTS.md, not in SDK code. Production plugin code should handle integration concerns, not LLM instruction.
- **When you need MCP tools** — the Model Context Protocol handles third-party tool registries; SDK tools are for custom first-party integrations only.

---

## Overview

OpenCode provides two main packages for SDK development:

| Package | Purpose |
|---------|---------|
| `@opencode-ai/sdk` | Client SDK for interacting with OpenCode server (sessions, messages, files) |
| `@opencode-ai/plugin` | Plugin system for creating custom tools with schema validation |

---

## Custom Tools

Custom tools extend OpenCode's capabilities. Tools are TypeScript/JavaScript files auto-discovered from:
- **Local**: `.opencode/tool/` in project directory
- **Global**: `~/.config/opencode/tool/`

The **filename becomes the tool name**.

### Basic Tool Structure

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: "Brief description of what the tool does",
  args: {
    paramName: tool.schema.string().describe("Parameter description")
  },
  async execute(args, context) {
    // context provides: sessionID, messageID, agent, abort
    return "Result string returned to the AI"
  }
})
```

### Schema Definition

Use `tool.schema` (which is Zod) for argument validation:

```typescript
args: {
  // String with description
  query: tool.schema.string().describe("Search query"),

  // Optional string
  path: tool.schema.string().optional().describe("File path"),

  // Number with constraints
  limit: tool.schema.number().min(1).max(100).default(10).describe("Max results"),

  // Enum/literal union
  format: tool.schema.enum(["json", "text"]).describe("Output format"),

  // Boolean
  recursive: tool.schema.boolean().default(false).describe("Search recursively")
}
```

### Tool Context

The execute function receives a context object:

```typescript
type ToolContext = {
  sessionID: string      // Current session ID
  messageID: string      // Current message ID
  agent: string          // Current agent identifier
  abort: AbortSignal     // Signal for cancellation
}
```

### Example: File Search Tool

```typescript
import { tool } from "@opencode-ai/plugin"
import { $ } from "bun"

export default tool({
  description: "Search for files matching a pattern",
  args: {
    pattern: tool.schema.string().describe("Glob pattern to match"),
    directory: tool.schema.string().default(".").describe("Directory to search")
  },
  async execute({ pattern, directory }) {
    const result = await $`find ${directory} -name "${pattern}"`.text()
    return result || "No files found"
  }
})
```

---

## Plugin Development

Plugins provide more comprehensive integrations with hooks for events, authentication, and tool modification.

### Plugin Structure

```typescript
import type { Plugin } from "@opencode-ai/plugin"

const plugin: Plugin = async (input) => {
  const { client, project, directory, worktree, $ } = input

  return {
    // Custom tools
    tool: {
      myTool: tool({ /* definition */ })
    },

    // Event hooks
    event: async ({ event }) => { /* handle events */ },

    // Configuration hooks
    config: async (config) => { /* modify config */ },

    // Message hooks
    "chat.message": async (input, output) => { /* modify messages */ },

    // Tool execution hooks
    "tool.execute.before": async (input, output) => { /* pre-processing */ },
    "tool.execute.after": async (input, output) => { /* post-processing */ }
  }
}

export default plugin
```

### Available Hooks

| Hook | Purpose |
|------|---------|
| `event` | Handle real-time events from server |
| `config` | Modify configuration on load |
| `tool` | Register custom tools |
| `auth` | Custom authentication providers |
| `chat.message` | Modify messages before sending |
| `chat.params` | Modify LLM parameters (temperature, topP) |
| `permission.ask` | Handle permission requests |
| `tool.execute.before` | Pre-process tool arguments |
| `tool.execute.after` | Post-process tool output |

---

## SDK Client Usage

The SDK client provides programmatic access to OpenCode functionality.

### Initialization

```typescript
import { createOpencode, createOpencodeClient } from "@opencode-ai/sdk"

// Create both client and server
const { client, server } = await createOpencode({
  hostname: "127.0.0.1",
  port: 4096,
  timeout: 5000
})

// Or just the client
const client = createOpencodeClient({
  baseUrl: "http://127.0.0.1:4096"
})
```

### Client API Categories

| Category | Methods |
|----------|---------|
| `client.session` | list, create, get, delete, prompt, messages, fork, share |
| `client.project` | list, current |
| `client.file` | list, read, status |
| `client.find` | text, files, symbols |
| `client.tool` | ids, list |
| `client.event` | subscribe (SSE streaming) |
| `client.mcp` | status, add |
| `client.tui` | appendPrompt, submitPrompt, showToast |

### Session Management

```typescript
// List sessions
const { data: sessions } = await client.session.list()

// Create session
const { data: session } = await client.session.create()

// Send prompt
const { data: response } = await client.session.prompt({
  path: { id: sessionId },
  body: {
    parts: [{ type: "text", text: "Your message here" }]
  }
})

// Get messages
const { data: messages } = await client.session.messages({
  path: { id: sessionId }
})
```

### Event Streaming

```typescript
const result = await client.event.subscribe()

for await (const event of result.events) {
  console.log("Event:", event.type, event.data)
}
```

---

## Installation

```bash
# Install SDK
npm install @opencode-ai/sdk

# Install plugin package (for tools)
npm install @opencode-ai/plugin
```

Requires TypeScript >= 4.9.

## Tool File Location

| Location | Scope |
|----------|-------|
| `.opencode/tool/*.ts` | Project-specific tools |
| `~/.config/opencode/tool/*.ts` | Global tools for all projects |

Multiple exports create multiple tools: `filename_exportname`.

---

## Anti-Patterns

### 1. Returning non-string values from `execute`

**WHY:** The `execute` function MUST return a `string`. NEVER return objects, numbers, or `undefined` — these cause runtime errors or garbled AI output. ALWAYS serialize complex data before returning.

**Bad**:
```typescript
async execute({ query }) {
  return { results: [], count: 0 } // NEVER return an object
}
```

**Good**:
```typescript
async execute({ query }) {
  const results = await search(query)
  return JSON.stringify(results) // ALWAYS serialize to string
}
```

---

### 2. Using `@opencode-ai/sdk` for tool definitions

**WHY:** Tools MUST be defined with `@opencode-ai/plugin`, NEVER with `@opencode-ai/sdk`. The SDK package is for external client access to a running OpenCode server — it has no `tool()` function.

**Bad**:
```typescript
import { tool } from "@opencode-ai/sdk" // NEVER — no such export
```

**Good**:
```typescript
import { tool } from "@opencode-ai/plugin" // ALWAYS use plugin package
```

---

### 3. Ignoring the `abort` signal in long-running tools

**WHY:** When the user cancels an operation, the `abort` signal fires. NEVER ignore it — doing so leaves dangling processes and locked resources. ALWAYS pass `signal: abort` to async operations.

**Bad**:
```typescript
async execute({ query }) {
  const result = await expensiveOperation(query) // NEVER ignore abort
  return result
}
```

**Good**:
```typescript
async execute({ query }, { abort }) {
  if (abort.aborted) return "Cancelled"
  const result = await expensiveOperation(query, { signal: abort })
  return result
}
```

---

### 4. Missing `.describe()` on schema fields

**WHY:** Without descriptions, the LLM has no context for what each argument means and will guess incorrectly. ALWAYS call `.describe()` on every schema field — NEVER leave arg schemas without descriptions.

**Bad**:
```typescript
args: {
  q: tool.schema.string(), // NEVER omit .describe()
  n: tool.schema.number()
}
```

**Good**:
```typescript
args: {
  query: tool.schema.string().describe("Full-text search query"),
  limit: tool.schema.number().default(10).describe("Max results to return (1-100)")
}
```

---

### 5. Using `createOpencode` inside plugin hooks instead of the provided `client`

**WHY:** Plugins receive a `client` parameter in their factory function. NEVER call `createOpencode()` inside hooks — it adds unnecessary overhead and can cause connection conflicts. ALWAYS use the factory's `client` parameter.

**Bad**:
```typescript
const plugin: Plugin = async () => ({
  event: async ({ event }) => {
    const { client } = await createOpencode({ port: 4096 }) // NEVER — redundant
  }
})
```

**Good**:
```typescript
const plugin: Plugin = async ({ client }) => ({
  event: async ({ event }) => {
    await client.session.list() // ALWAYS use provided client
  }
})
```

---

### 6. Placing tool files in the wrong directory

**WHY:** Tool files MUST be in `.opencode/tool/` (project) or `~/.config/opencode/tool/` (global). NEVER place tools in `tools/` (plural) or any other path — they are silently ignored.

**Bad**:
```
.opencode/tools/my-tool.ts    ← NEVER use "tools" (plural)
src/tools/my-tool.ts          ← NEVER place outside .opencode/
opencode-tools/my-tool.ts     ← NEVER use wrong directory name
```

**Good**:
```
.opencode/tool/my-tool.ts     ← ALWAYS use singular "tool" directory
~/.config/opencode/tool/my-tool.ts  ← ALWAYS use global path for user tools
```

---

### 7. Using synchronous hook handlers

**WHY:** All plugin hook handlers MUST be `async`. NEVER define synchronous handlers — TypeScript will report an error and the plugin may fail to load silently in some environments.

**Bad**:
```typescript
event: ({ event }) => {         // NEVER — missing async
  console.log(event.type)
}
```

**Good**:
```typescript
event: async ({ event }) => {   // ALWAYS async
  console.log(event.type)
}
```

---

### 8. Creating a tool in a `.js` file without TypeScript

**WHY:** `.js` tool files lose schema inference, type safety, and IDE support. NEVER use `.js` for tool files — the `tool.schema` Zod integration relies on TypeScript inference to generate correct JSON schemas for the AI. ALWAYS use `.ts`.

**Bad**:
```javascript
// my-tool.js — NEVER use .js for tools
export default {
  description: "Search tool",
  execute: async ({ q }) => { return `result for ${q}` }
}
```

**Good**: ALWAYS use `.ts` with proper imports:
```typescript
import { tool } from "@opencode-ai/plugin"
export default tool({
  description: "Search tool",
  args: { q: tool.schema.string().describe("Search query") },
  async execute({ q }) { return `result for ${q}` }
})
```

---

### 9. Expecting tools to persist state between calls

**WHY:** Each tool `execute()` call is stateless. Module-level variables reset when the process restarts. NEVER rely on module-level state for persistence — ALWAYS use external storage.

**Bad**:
```typescript
let callCount = 0 // NEVER use module-level state for persistence
export default tool({
  description: "Count API calls",
  args: {},
  async execute() {
    callCount++ // Unreliable across sessions
    return `Called ${callCount} times`
  }
})
```

**Good**: ALWAYS use external storage (file system, database, environment) for persistence:
```typescript
import { readFileSync, writeFileSync } from "fs"
export default tool({
  description: "Count API calls",
  args: {},
  async execute() {
    const file = ".opencode/call-count.json"
    const count = JSON.parse(readFileSync(file, "utf8").toString() || "0") + 1
    writeFileSync(file, JSON.stringify(count))
    return `Called ${count} times`
  }
})
```

---

### Cross-Language Tool

```typescript
import { tool } from "@opencode-ai/plugin"
import { $ } from "bun"

export default tool({
  description: "Run Python analysis script",
  args: {
    file: tool.schema.string().describe("File to analyze")
  },
  async execute({ file }) {
    return await $`python3 analyze.py ${file}`.text()
  }
})
```

### Tool with Context

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: "Get current session info",
  args: {},
  async execute(args, context) {
    return JSON.stringify({
      session: context.sessionID,
      message: context.messageID,
      agent: context.agent
    }, null, 2)
  }
})
```

---

## Troubleshooting

**Tool not appearing:**
- Verify file is in `.opencode/tool/` or `~/.config/opencode/tool/`
- Check file exports a valid tool definition
- Restart OpenCode to reload tools

**Schema errors:**
- Ensure all required args are provided
- Check type constraints (string vs number)
- Verify optional fields use `.optional()`

**Execution errors:**
- Check `execute` returns a string
- Verify async operations complete
- Handle errors and return error messages as strings

---

## Additional Resources

### Reference Files

For detailed API documentation:
- [sdk-api.md](references/sdk-api.md) - Complete SDK client API reference
- [plugin-api.md](references/plugin-api.md) - Full plugin hooks and types
- [advanced-patterns.md](references/advanced-patterns.md) - Advanced patterns: HTTP API tools, audit logging, allowlist enforcement, CI automation, multi-tool plugins

### Example Files

Working examples in `references/`:
- [basic-tool.ts](references/basic-tool.ts) - Simple tool implementation
- [full-plugin.ts](references/full-plugin.ts) - Complete plugin with hooks

### External Documentation

- [OpenCode SDK Docs](https://opencode.ai/docs/sdk/)
- [Custom Tools Guide](https://opencode.ai/docs/custom-tools/)
- [OpenCode GitHub](https://github.com/sst/opencode)

## Eval Scenarios

- [Scenario 0: Create a custom todo-search tool with schema validation](evals/scenario-0/task.md)
- [Scenario 1: Use SDK client to create session and send prompt](evals/scenario-1/task.md)
- [Scenario 2: Implement abort signal handling in long-running tools](evals/scenario-2/task.md)
