# OpenCode SDK API Reference

Complete API reference for `@opencode-ai/sdk`.

## Table of Contents

1. [Initialization](#initialization)
2. [Session API](#session-api)
3. [Project API](#project-api)
4. [File API](#file-api)
5. [Find API](#find-api)
6. [Tool API](#tool-api)
7. [Event API](#event-api)
8. [Config API](#config-api)
9. [Provider API](#provider-api)
10. [MCP API](#mcp-api)
11. [TUI API](#tui-api)
12. [PTY API](#pty-api)
13. [Auth API](#auth-api)

---

## Initialization

### createOpencode

Creates both an OpenCode client and starts a local server.

```typescript
import { createOpencode } from "@opencode-ai/sdk"

const { client, server } = await createOpencode({
  hostname?: string,    // Default: "127.0.0.1"
  port?: number,        // Default: 4096
  timeout?: number,     // Default: 5000ms
  config?: Config       // Custom config overrides
})

// Clean up
server.close()
```

### createOpencodeClient

Creates only the client (connects to existing server).

```typescript
import { createOpencodeClient } from "@opencode-ai/sdk"

const client = createOpencodeClient({
  baseUrl?: string,         // Default: "http://127.0.0.1:4096"
  directory?: string,       // Working directory
  throwOnError?: boolean,   // Throw on API errors
  responseStyle?: "data" | "fields"  // Response format
})
```

---

## Session API

`client.session.*`

### list

List all sessions.

```typescript
const { data, error } = await client.session.list()
// data: Session[]
```

### create

Create a new session.

```typescript
const { data, error } = await client.session.create({
  body?: {
    title?: string,
    parentID?: string
  }
})
// data: Session
```

### get

Get a specific session.

```typescript
const { data, error } = await client.session.get({
  path: { id: string }
})
// data: Session
```

### delete

Delete a session.

```typescript
const { data, error } = await client.session.delete({
  path: { id: string }
})
```

### update

Update session properties.

```typescript
const { data, error } = await client.session.update({
  path: { id: string },
  body: {
    title?: string,
    pinned?: boolean
  }
})
```

### prompt

Send a message to a session.

```typescript
const { data, error } = await client.session.prompt({
  path: { id: string },
  body: {
    parts: Part[],           // Message content
    model?: {                // Optional model override
      providerID: string,
      modelID: string
    },
    noReply?: boolean        // Don't trigger AI response
  }
})
```

#### Part Types

```typescript
type Part =
  | { type: "text", text: string }
  | { type: "file", path: string, content?: string }
  | { type: "image", url: string }
```

### promptAsync

Send message and return immediately (async processing).

```typescript
const { data, error } = await client.session.promptAsync({
  path: { id: string },
  body: {
    parts: Part[],
    model?: { providerID: string, modelID: string }
  }
})
```

### messages

Get all messages for a session.

```typescript
const { data, error } = await client.session.messages({
  path: { id: string }
})
// data: Message[]
```

### message

Get a specific message.

```typescript
const { data, error } = await client.session.message({
  path: { id: string, messageID: string }
})
// data: Message
```

### command

Send a slash command.

```typescript
const { data, error } = await client.session.command({
  path: { id: string },
  body: {
    command: string  // e.g., "/help", "/init"
  }
})
```

### shell

Execute a shell command.

```typescript
const { data, error } = await client.session.shell({
  path: { id: string },
  body: {
    command: string
  }
})
```

### status

Get session status (idle, busy, etc.).

```typescript
const { data, error } = await client.session.status({
  path: { id: string }
})
```

### abort

Abort current session activity.

```typescript
const { data, error } = await client.session.abort({
  path: { id: string }
})
```

### fork

Fork a session at a specific message.

```typescript
const { data, error } = await client.session.fork({
  path: { id: string },
  body: {
    messageID: string
  }
})
```

### children

Get child sessions (forks).

```typescript
const { data, error } = await client.session.children({
  path: { id: string }
})
```

### share / unshare

Share or unshare a session.

```typescript
const { data, error } = await client.session.share({
  path: { id: string }
})
// Returns share URL

const { data, error } = await client.session.unshare({
  path: { id: string }
})
```

### diff

Get the git diff for session changes.

```typescript
const { data, error } = await client.session.diff({
  path: { id: string }
})
```

### revert / unrevert

Revert or restore messages.

```typescript
const { data, error } = await client.session.revert({
  path: { id: string },
  body: { messageID: string }
})

const { data, error } = await client.session.unrevert({
  path: { id: string }
})
```

### summarize

Generate session summary.

```typescript
const { data, error } = await client.session.summarize({
  path: { id: string }
})
```

### todo

Get todo list for session.

```typescript
const { data, error } = await client.session.todo({
  path: { id: string }
})
```

### init

Initialize AGENTS.md for project.

```typescript
const { data, error } = await client.session.init({
  path: { id: string }
})
```

---

## Project API

`client.project.*`

### list

List all projects.

```typescript
const { data, error } = await client.project.list()
// data: Project[]
```

### current

Get current project.

```typescript
const { data, error } = await client.project.current()
// data: Project
```

---

## File API

`client.file.*`

### list

List files in a directory.

```typescript
const { data, error } = await client.file.list({
  query: {
    path: string,
    recursive?: boolean
  }
})
```

### read

Read file content.

```typescript
const { data, error } = await client.file.read({
  query: {
    path: string
  }
})
```

### status

Get file status (git status).

```typescript
const { data, error } = await client.file.status()
```

---

## Find API

`client.find.*`

### text

Search for text in files.

```typescript
const { data, error } = await client.find.text({
  query: {
    query: string,
    path?: string,
    caseSensitive?: boolean,
    regex?: boolean
  }
})
```

### files

Find files by name.

```typescript
const { data, error } = await client.find.files({
  query: {
    query: string,
    path?: string
  }
})
```

### symbols

Find code symbols.

```typescript
const { data, error } = await client.find.symbols({
  query: {
    query: string
  }
})
```

---

## Tool API

`client.tool.*`

### ids

Get list of all tool IDs.

```typescript
const { data, error } = await client.tool.ids()
// data: string[]
```

### list

Get tools with JSON schemas for a provider/model.

```typescript
const { data, error } = await client.tool.list({
  query: {
    providerID: string,
    modelID: string
  }
})
```

---

## Event API

`client.event.*`

### subscribe

Subscribe to server-sent events.

```typescript
const result = await client.event.subscribe()

for await (const event of result.events) {
  // event.type: string
  // event.data: any
  console.log(event)
}

// Or with options
const result = await client.event.subscribe({
  query: { sessionID?: string }
})
```

#### Event Types

- `session.created`
- `session.deleted`
- `session.updated`
- `message.created`
- `message.updated`
- `tool.started`
- `tool.completed`
- `permission.requested`

---

## Config API

`client.config.*`

### get

Get current configuration.

```typescript
const { data, error } = await client.config.get()
```

### update

Update configuration.

```typescript
const { data, error } = await client.config.update({
  body: {
    theme?: string,
    model?: { providerID: string, modelID: string }
  }
})
```

### providers

List configured providers.

```typescript
const { data, error } = await client.config.providers()
```

---

## Provider API

`client.provider.*`

### list

List available providers.

```typescript
const { data, error } = await client.provider.list()
```

### auth

Get authentication methods for providers.

```typescript
const { data, error } = await client.provider.auth()
```

### oauth.authorize

Start OAuth flow.

```typescript
const { data, error } = await client.provider.oauth.authorize({
  path: { provider: string }
})
```

### oauth.callback

Handle OAuth callback.

```typescript
const { data, error } = await client.provider.oauth.callback({
  path: { provider: string },
  query: { code: string, state?: string }
})
```

---

## MCP API

`client.mcp.*`

### status

Get MCP server status.

```typescript
const { data, error } = await client.mcp.status()
```

### add

Dynamically add MCP server.

```typescript
const { data, error } = await client.mcp.add({
  body: {
    name: string,
    command: string,
    args?: string[],
    env?: Record<string, string>
  }
})
```

---

## TUI API

`client.tui.*`

Control the terminal UI programmatically.

### appendPrompt

Append text to the prompt.

```typescript
const { data, error } = await client.tui.appendPrompt({
  body: { text: string }
})
```

### submitPrompt

Submit the current prompt.

```typescript
const { data, error } = await client.tui.submitPrompt()
```

### clearPrompt

Clear the prompt.

```typescript
const { data, error } = await client.tui.clearPrompt()
```

### showToast

Show a toast notification.

```typescript
const { data, error } = await client.tui.showToast({
  body: {
    message: string,
    type?: "info" | "success" | "warning" | "error"
  }
})
```

### openHelp / openSessions / openThemes / openModels

Open various dialogs.

```typescript
await client.tui.openHelp()
await client.tui.openSessions()
await client.tui.openThemes()
await client.tui.openModels()
```

### executeCommand

Execute a TUI command.

```typescript
const { data, error } = await client.tui.executeCommand({
  body: { command: string }
})
```

---

## PTY API

`client.pty.*`

Manage pseudo-terminal sessions.

### list

```typescript
const { data, error } = await client.pty.list()
```

### create

```typescript
const { data, error } = await client.pty.create({
  body: {
    cols?: number,
    rows?: number,
    cwd?: string
  }
})
```

### get / update / remove / connect

```typescript
await client.pty.get({ path: { id: string } })
await client.pty.update({ path: { id: string }, body: { cols, rows } })
await client.pty.remove({ path: { id: string } })
await client.pty.connect({ path: { id: string } })
```

---

## Auth API

`client.auth.*`

### set

Set authentication credentials.

```typescript
const { data, error } = await client.auth.set({
  path: { provider: string },
  body: {
    type: "key" | "oauth",
    key?: string,
    access?: string,
    refresh?: string,
    expires?: number
  }
})
```

---

## Type Imports

Import types directly from the SDK:

```typescript
import type {
  Session,
  Message,
  Part,
  Project,
  Model,
  Provider,
  Permission,
  UserMessage,
  Auth,
  Config,
  Event
} from "@opencode-ai/sdk"
```

---

## Error Handling

```typescript
const { data, error, response } = await client.session.get({
  path: { id: "invalid" }
})

if (error) {
  console.error("Error:", error)
  console.error("Status:", response.status)
} else {
  console.log("Session:", data)
}

// Or with throwOnError
try {
  const client = createOpencodeClient({ throwOnError: true })
  const { data } = await client.session.get({ path: { id: "test" } })
} catch (e) {
  console.error("API Error:", e)
}
```
