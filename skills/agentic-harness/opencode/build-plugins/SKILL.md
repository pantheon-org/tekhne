---
name: opencode-build-plugins
description: |-
  Create OpenCode plugins using the opencode-ai/plugin SDK. Use when user wants to build a plugin, extend OpenCode, intercept tool execution, add custom tools, react to events, create a hook, block commands, add custom auth, or add automation to OpenCode. Also use for ".opencode/plugin", "write a plugin", "opencode plugin", "custom tool", "tool hook", "plugin hook", "logging for all tool calls", "restrict what commands the AI can run", "run code when a file is saved", "notify me when a session ends", "validate tool input before execution".
---

# Creating OpenCode Plugins

Re-read this file periodically during plugin development to refresh context and ensure you're following the correct procedure.

## Quick Start

Plugins live in `.opencode/plugins/<name>/index.ts` (project-scoped) or `~/.config/opencode/plugins/<name>/index.ts` (global).

```typescript
import type { Plugin } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async ({ project, client, $, directory, worktree }) => {
  return {
    // Add hooks here — see Step 3 for available hooks
  }
}
```

Register in `opencode.json`:
```jsonc
{ "plugin": ["file:///path/to/plugin/index.ts"] }
```

Then test:
```bash
opencode run hi
```

---

## Mental Model

Plugins are **async factory functions** that return hook implementations. They run once on load. Hooks fire at specific lifecycle points (tool execution, events, config load). Think of them as middleware layers — each hook can read, transform, or block the data flowing through it.

**Plugin = factory + hooks**. The factory sets up shared state; hooks react to runtime events.

### When to Use a Plugin vs Other Approaches

| Need | Best approach |
|------|---------------|
| Intercept / block tool calls | Plugin: `tool.execute.before` |
| Log all tool calls | Plugin: `tool.execute.after` |
| Add a custom tool the LLM can call | Plugin: `tool:` key OR standalone `.opencode/tool/` file |
| React to file edits, session end | Plugin: `event:` hook |
| Custom auth for a model provider | Plugin: `auth:` hook |
| Modify LLM temperature/params | Plugin: `chat.params:` hook |
| Automate OpenCode from a script | SDK Client: `createOpencode()` |
| Simple command shortcut for users | Slash Command: `.opencode/command/` |
| Reusable agent instructions | Skill: `.opencode/skill/` |

---

## Procedure Overview

| Step | Action               | Read                                                                            |
| ---- | -------------------- | ------------------------------------------------------------------------------- |
| 1    | Verify SDK reference | Run extract script                                                              |
| 2    | Validate feasibility | This file                                                                       |
| 3    | Design plugin        | [hooks.md](references/hooks.md), [hook-patterns.md](references/hook-patterns.md), [CODING-TS.MD](references/CODING-TS.MD) |
| 4    | Implement            | [tool-helper.md](references/tool-helper.md) (if custom tools)                   |
| 5    | Add UI feedback      | [toast-notifications.md](references/toast-notifications.md), [ui-feedback.md](references/ui-feedback.md) (if needed) |
| 6    | Test                 | [testing.md](references/testing.md)                                             |
| 7    | Publish              | [publishing.md](references/publishing.md), [update-notifications.md](references/update-notifications.md) (if npm) |

---

## Step 1: Verify SDK Reference (REQUIRED)

Before creating any plugin, MUST regenerate the API reference to ensure accuracy:

```bash
bun run .opencode/skill/opencode-build-plugins/scripts/extract-plugin-api.ts
```

This generates:

- `references/hooks.md` - All available hooks and signatures
- `references/events.md` - All event types and properties
- `references/tool-helper.md` - Tool creation patterns

---

## Step 2: Validate Feasibility (REQUIRED)

MUST determine if the user's concept is achievable with available hooks.

### Feasible as plugins

- Intercepting/blocking tool calls
- Reacting to events (file edits, session completion, etc.)
- Adding custom tools for the LLM
- Modifying LLM parameters (temperature, etc.)
- Custom auth flows for providers
- Customizing session compaction
- Displaying status messages (toasts, inline)

### NOT feasible (inform user)

- Modifying TUI rendering or layout
- Adding new built-in tools (requires OC source)
- Changing core agent behavior/prompts
- Intercepting assistant responses mid-stream
- Adding new keybinds or commands
- Modifying internal file read/write
- Adding new permission types

**If not feasible**, MUST inform user clearly. Suggest:

- OC core changes: contribute to `packages/opencode`
- MCP tools: use MCP server configuration
- Simple automation: use shell scripts

---

## Step 3: Design Plugin

**READ**: [hooks.md](references/hooks.md) for available hooks, [hook-patterns.md](references/hook-patterns.md) for implementation patterns.

**READ**: [CODING-TS.MD](references/CODING-TS.MD) for code architecture principles. MUST follow these design guidelines:

- **Modular structure**: Split complex plugins into multiple focused files (types, utilities, hooks, tools)
- **Single purpose**: Each function does ONE thing well
- **DRY**: Extract common patterns into shared utilities immediately
- **Small files**: Keep individual files under 150 lines - split into smaller modules as needed
- **No monoliths**: MUST NOT put all plugin code in a single `index.ts` file

### Plugin Locations

| Scope   | Path                                        | Use Case                   |
| ------- | ------------------------------------------- | -------------------------- |
| Project | `.opencode/plugins/<name>/index.ts`          | Team-shared, repo-specific |
| Global  | `~/.config/opencode/plugins/<name>/index.ts` | Personal, all projects     |

### Basic Structure

```typescript
import type { Plugin } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async ({ project, client, $, directory, worktree }) => {
  // Setup code runs once on load

  return {
    // Hook implementations - see references/hook-patterns.md
  }
}
```

### Context Parameters

| Parameter   | Type       | Description                               |
| ----------- | ---------- | ----------------------------------------- |
| `project`   | `Project`  | Current project info (id, worktree, name) |
| `client`    | SDK Client | OpenCode API client                       |
| `$`         | `BunShell` | Bun shell for commands                    |
| `directory` | `string`   | Current working directory                 |
| `worktree`  | `string`   | Git worktree path                         |

---

## Step 4: Implement

**READ**: [hook-patterns.md](references/hook-patterns.md) for hook implementation examples.

**READ**: [tool-helper.md](references/tool-helper.md) if adding custom tools (Zod schemas).

**READ**: [events.md](references/events.md) if using event hook (event types/properties).

**READ**: [examples.md](references/examples.md) for complete plugin examples.

**ALWAYS READ**: [CODING-TS.MD](references/CODING-TS.MD) and follow modular design principles.

### Plugin Structure (Non-Monolithic)

For complex plugins, MUST use a modular directory structure:

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

**Example modular index.ts**:

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

Keep each file under 150 lines. Split as complexity grows.

---

## Step 5: Add UI Feedback (Optional)

Only if plugin needs user-visible notifications:

**READ**: [toast-notifications.md](references/toast-notifications.md) for transient alerts (brief popups)

**READ**: [ui-feedback.md](references/ui-feedback.md) for persistent inline status messages

Choose based on:

| Need                         | Use            |
| ---------------------------- | -------------- |
| Brief alerts, warnings       | Toast          |
| Detailed stats, multi-line   | Inline message |
| Config validation errors     | Toast          |
| Session completion notice    | Toast or inline|

---

## Step 6: Test

**READ**: [testing.md](references/testing.md) for full testing procedure.

### Quick Test Steps

1. Create test folder with `opencode.json`:

   ```jsonc
   {
     "plugin": ["file:///path/to/your/plugin/index.ts"],
   }
   ```

2. Verify plugin loads:

   ```bash
   cd /path/to/test-folder
   opencode run hi
   ```

3. Test interactively:

   ```bash
   opencode
   ```

4. SHOULD recommend specific tests based on hook type used.

---

## Step 7: Publish (Optional)

**READ**: [publishing.md](references/publishing.md) for npm publishing.

**READ**: [update-notifications.md](references/update-notifications.md) for version update toasts (for users with pinned versions).

---

## Anti-Patterns

### 1. Using `client.registerTool()` instead of `tool:` hook

**WHY:** `client.registerTool()` does not exist in the plugin API. Tools MUST be registered via the `tool:` key in the object returned by the plugin factory. NEVER call `client.registerTool()` — it will throw a runtime error on load.

**Bad**:
```typescript
export const MyPlugin: Plugin = async (client) => {
  client.registerTool("my-tool", { ... }) // NEVER use this — does not exist
  return {}
}
```

**Good**:
```typescript
export const MyPlugin: Plugin = async () => ({
  tool: {
    "my-tool": myTool  // ALWAYS register tools via the tool: hook
  }
})
```

---

### 2. Synchronous event handlers

**WHY:** All plugin hook handlers MUST be declared `async`. A synchronous function will cause a type error because the Plugin interface requires `Promise<void>` returns. NEVER omit `async` from hook handlers — OpenCode will reject the plugin at load time.

**Bad**:
```typescript
export const MyPlugin: Plugin = async () => ({
  event: (event) => { console.log(event) } // NEVER omit async
})
```

**Good**: Use a modular directory structure — separate files for `hooks/`, `tools/`, `types.ts`, `utils.ts`.

---

### 5. Attempting to intercept or modify TUI rendering

**WHY:** TUI layout and rendering are not exposed through the plugin API. NEVER attempt to use a `tui.render` or `layout` hook — these do not exist and will error silently.

**Bad**: Searching for a `tui.render` or `layout` hook — these do not exist.

**Good**: ALWAYS use `client.tui.showToast()` or `client.tui.appendPrompt()` for the supported TUI interactions.

---

### 6. Forgetting to export the plugin as a named or default export

**WHY:** OpenCode loads plugin files and looks for a default or named export. A plugin that is defined but NEVER exported is silently ignored — no error, no hooks registered. ALWAYS export the plugin.

**Bad**:
```typescript
import type { Plugin } from "@opencode-ai/plugin"

const MyPlugin: Plugin = async () => ({
  event: async ({ event }) => { console.log(event) }
})
// NEVER leave the plugin unexported — it will never load
```

**Good**:
```typescript
import type { Plugin } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async () => ({
  event: async ({ event }) => { console.log(event) }
})
// OR
export default MyPlugin  // ALWAYS export the plugin
```

---

### 7. Using absolute file paths in `plugin` config instead of relative paths

**WHY:** Absolute paths are machine-specific and will break for teammates. NEVER use absolute paths in `plugin` config. ALWAYS use `file://` relative paths or project-relative conventions.

**Bad**:
```jsonc
{ "plugin": ["/Users/alice/my-project/.opencode/plugin/my-plugin/index.ts"] }
```

**Good**:
```jsonc
{ "plugin": ["file:///.opencode/plugins/my-plugin/index.ts"] }
```

---

### 8. Trying to modify `input.args` in `tool.execute.before` to change tool behavior

**WHY:** The `input` parameter in `tool.execute.before` is read-only. NEVER mutate `input.args` — it has no effect and the original arguments are passed to the tool unmodified.

**Bad**:
```typescript
"tool.execute.before": async (input) => {
  input.args.path = "/safe/path" // NEVER mutate input.args — mutation ignored
}
```

**Good**: To block dangerous calls, `throw`. To transform input, ALWAYS use a custom tool wrapper instead of `tool.execute.before`.

---

## Reference Files Summary

| File                      | Purpose                           | When to Read              |
| ------------------------- | --------------------------------- | ------------------------- |
| [hooks.md](references/hooks.md)                | Hook signatures (auto-generated)  | Step 3-4                  |
| [events.md](references/events.md)               | Event types (auto-generated)      | Step 4 (if using events)  |
| [tool-helper.md](references/tool-helper.md)          | Zod tool schemas (auto-generated) | Step 4 (if custom tools)  |
| [hook-patterns.md](references/hook-patterns.md)        | Hook implementation examples      | Step 3-4                  |
| [CODING-TS.MD](references/CODING-TS.MD)            | Code architecture principles      | Step 3 (Design)           |
| [examples.md](references/examples.md)             | Complete plugin examples          | Step 4                    |
| [toast-notifications.md](references/toast-notifications.md)  | Toast popup API                   | Step 5 (if toasts needed) |
| [ui-feedback.md](references/ui-feedback.md)          | Inline message API                | Step 5 (if inline needed) |
| [testing.md](references/testing.md)              | Testing procedure                 | Step 6                    |
| [publishing.md](references/publishing.md)           | npm publishing                    | Step 7                    |
| [update-notifications.md](references/update-notifications.md) | Version toast pattern             | Step 7 (for npm plugins)  |

## Eval Scenarios

- [Scenario 0: Block dangerous bash commands with tool.execute.before](evals/scenario-0/task.md)
- [Scenario 1: Show toast notifications on file edit events](evals/scenario-1/task.md)
- [Scenario 2: Add custom tool to plugin using tool key](evals/scenario-2/task.md)
