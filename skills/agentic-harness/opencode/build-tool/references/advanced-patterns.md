# Advanced SDK Patterns

Advanced patterns for building robust OpenCode tools and plugins.

## Pattern: Tool with External HTTP API

```typescript
import { tool } from "@opencode-ai/plugin"

export default tool({
  description: "Query a REST API and return results",
  args: {
    endpoint: tool.schema.string().describe("API endpoint path (e.g. /users/123)"),
    method: tool.schema.enum(["GET", "POST", "PUT", "DELETE"]).default("GET").describe("HTTP method"),
    body: tool.schema.string().optional().describe("Request body as JSON string")
  },
  async execute({ endpoint, method, body }, { abort }) {
    const BASE_URL = process.env.API_BASE_URL ?? "https://api.example.com"
    
    const response = await fetch(`${BASE_URL}${endpoint}`, {
      method,
      headers: { 
        "Content-Type": "application/json",
        "Authorization": `Bearer ${process.env.API_TOKEN}`
      },
      body: body ?? undefined,
      signal: abort  // Respect cancellation
    })

    if (!response.ok) {
      return `Error: ${response.status} ${response.statusText}`
    }

    const data = await response.json()
    return JSON.stringify(data, null, 2)
  }
})
```

## Pattern: Plugin with Audit Logging

```typescript
import type { Plugin } from "@opencode-ai/plugin"
import { appendFileSync } from "fs"

const LOG_FILE = ".opencode/audit.log"

function log(entry: string) {
  const timestamp = new Date().toISOString()
  appendFileSync(LOG_FILE, `[${timestamp}] ${entry}\n`)
}

export const AuditPlugin: Plugin = async () => ({
  "tool.execute.before": async (input) => {
    log(`TOOL_START tool=${input.tool} sessionID=${input.sessionID}`)
  },
  "tool.execute.after": async (input, output) => {
    const status = output.error ? "ERROR" : "SUCCESS"
    log(`TOOL_END tool=${input.tool} status=${status}`)
  }
})
```

## Pattern: Plugin with Allowlist Enforcement

```typescript
import type { Plugin } from "@opencode-ai/plugin"

const BLOCKED_TOOLS = ["bash", "file_write", "run_command"]
const ALLOWED_BASH_PATTERNS = [/^npm test/, /^npm run lint/, /^git status/]

export const SafetyPlugin: Plugin = async () => ({
  "tool.execute.before": async (input) => {
    if (BLOCKED_TOOLS.includes(input.tool)) {
      throw new Error(`Tool '${input.tool}' is blocked by safety policy`)
    }
    
    // For bash specifically, check command allowlist
    if (input.tool === "bash" && input.args?.command) {
      const allowed = ALLOWED_BASH_PATTERNS.some(p => p.test(input.args.command))
      if (!allowed) {
        throw new Error(`Bash command not in allowlist: ${input.args.command}`)
      }
    }
  }
})
```

## Pattern: SDK Client for CI Automation

```typescript
import { createOpencode } from "@opencode-ai/sdk"

async function runCITask(prompt: string) {
  const { client } = await createOpencode({
    hostname: "127.0.0.1",
    port: 4096,
    timeout: 30000
  })

  // Create a fresh session
  const { data: session } = await client.session.create()
  
  // Send prompt
  await client.session.prompt({
    path: { id: session.id },
    body: { parts: [{ type: "text", text: prompt }] }
  })

  // Stream events until session completes
  const events = await client.event.subscribe()
  for await (const event of events.events) {
    if (event.type === "session.completed" && event.data.id === session.id) {
      break
    }
    if (event.type === "message.text") {
      process.stdout.write(event.data.content ?? "")
    }
  }

  // Retrieve final messages
  const { data: messages } = await client.session.messages({
    path: { id: session.id }
  })
  return messages
}
```

## Pattern: Tool with File Persistence

```typescript
import { tool } from "@opencode-ai/plugin"
import { readFileSync, writeFileSync, existsSync } from "fs"

const STATE_FILE = ".opencode/tool-state.json"

function loadState(): Record<string, unknown> {
  if (!existsSync(STATE_FILE)) return {}
  return JSON.parse(readFileSync(STATE_FILE, "utf8"))
}

function saveState(state: Record<string, unknown>) {
  writeFileSync(STATE_FILE, JSON.stringify(state, null, 2))
}

export default tool({
  description: "Store and retrieve key-value pairs across sessions",
  args: {
    action: tool.schema.enum(["get", "set", "list"]).describe("Action to perform"),
    key: tool.schema.string().optional().describe("Key name"),
    value: tool.schema.string().optional().describe("Value to store (for set)")
  },
  async execute({ action, key, value }) {
    const state = loadState()
    
    switch (action) {
      case "get":
        return key ? String(state[key] ?? "not found") : "key required"
      case "set":
        if (!key || value === undefined) return "key and value required"
        state[key] = value
        saveState(state)
        return `Stored ${key} = ${value}`
      case "list":
        return Object.keys(state).join(", ") || "no keys stored"
    }
  }
})
```

## Pattern: Multi-Tool Plugin

When you need multiple tools in one plugin, use the `tool` key in the plugin return object:

```typescript
import type { Plugin } from "@opencode-ai/plugin"
import { tool } from "@opencode-ai/plugin"

const searchTool = tool({
  description: "Search project files",
  args: { pattern: tool.schema.string().describe("Search pattern") },
  async execute({ pattern }) { return `Searching for ${pattern}` }
})

const statusTool = tool({
  description: "Get project status",
  args: {},
  async execute() { return "Project status: OK" }
})

export const ProjectPlugin: Plugin = async () => ({
  tool: {
    "project-search": searchTool,
    "project-status": statusTool
  }
})
```

## Error Handling Best Practices

```typescript
export default tool({
  description: "Robust tool with error handling",
  args: { input: tool.schema.string().describe("Input to process") },
  async execute({ input }, { abort }) {
    try {
      if (abort.aborted) return "Operation cancelled"
      
      const result = await dangerousOperation(input)
      return result
    } catch (error) {
      // ALWAYS return a string, even on error
      if (error instanceof Error) {
        return `Error: ${error.message}`
      }
      return `Unexpected error: ${String(error)}`
    }
  }
})
```

**Rules:**
1. Always return a string — never throw from `execute`
2. Check `abort.aborted` before starting expensive work
3. Wrap in try-catch and return error as string
4. Use specific error messages the AI can act on
