---
name: obsidian-cli
description: Interact with Obsidian vaults using the Obsidian CLI to read, create, search, and manage notes, tasks, properties, and more. Also supports plugin and theme development with commands to reload plugins, run JavaScript, capture errors, take screenshots, and inspect the DOM. Use when the user asks to interact with their Obsidian vault, manage notes, search vault content, perform vault operations from the command line, or develop and debug Obsidian plugins and themes.
---

# Obsidian CLI

Use the `obsidian` CLI to interact with a running Obsidian instance. Requires Obsidian to be open.

## Command reference

Run `obsidian help` to see all available commands. This is always up to date. Full docs: [help.obsidian.md/cli](https://help.obsidian.md/cli)

## Syntax

**Parameters** take a value with `=`. Quote values with spaces:

```bash
obsidian create name="My Note" content="Hello world"
```

**Flags** are boolean switches with no value:

```bash
obsidian create name="My Note" silent overwrite
```

For multiline content use `\n` for newline and `\t` for tab.

## File targeting

Many commands accept `file` or `path` to target a file. Without either, the active file is used.

- `file=<name>` — resolves like a wikilink (name only, no path or extension needed)
- `path=<path>` — exact path from vault root, e.g. `folder/note.md`

## Vault targeting

Commands target the most recently focused vault by default. Use `vault=<name>` as the first parameter to target a specific vault:

```bash
obsidian vault="My Vault" search query="test"
```

## Common patterns

```bash
obsidian read file="My Note"
obsidian create name="New Note" content="# Hello" template="Template" silent
obsidian append file="My Note" content="New line"
obsidian search query="search term" limit=10
obsidian daily:read
obsidian daily:append content="- [ ] New task"
obsidian property:set name="status" value="done" file="My Note"
obsidian tasks daily todo
obsidian tags sort=count counts
obsidian backlinks file="My Note"
```

Use `--copy` on any command to copy output to clipboard. Use `silent` to prevent files from opening. Use `total` on list commands to get a count.

## Plugin development

### Develop/test cycle

After making code changes to a plugin or theme, follow this workflow:

1. **Reload** the plugin to pick up changes:
   ```bash
   obsidian plugin:reload id=my-plugin
   ```
2. **Check for errors** — if errors appear, fix and repeat from step 1:
   ```bash
   obsidian dev:errors
   ```
3. **Verify visually** with a screenshot or DOM inspection:
   ```bash
   obsidian dev:screenshot path=screenshot.png
   obsidian dev:dom selector=".workspace-leaf" text
   ```
4. **Check console output** for warnings or unexpected logs:
   ```bash
   obsidian dev:console level=error
   ```

### Additional developer commands

Run JavaScript in the app context:

```bash
obsidian eval code="app.vault.getFiles().length"
```

Inspect CSS values:

```bash
obsidian dev:css selector=".workspace-leaf" prop=background-color
```

Toggle mobile emulation:

```bash
obsidian dev:mobile on
```

Run `obsidian help` to see additional developer commands including CDP and debugger controls.

## Common Mistakes

### 1. Using `path=` when `file=` is sufficient

**NEVER** use `path=` when the note name is unique across the vault. `path=` requires the exact path from vault root including extension. `file=` resolves like a wikilink — name only, no path, no extension needed. Prefer `file=` unless you need to disambiguate files with the same name in different folders.

**WHY:** Using `path=` tightly couples commands to the vault's folder structure. If the file is ever moved, every command referencing it breaks silently or errors out.

```bash
# BAD — unnecessarily verbose, breaks if file moves
obsidian read path="folder/subfolder/My Note.md"

# GOOD — resolves by name, works regardless of location
obsidian read file="My Note"
```

### 2. Forgetting to quote parameter values that contain spaces

**NEVER** pass parameter values that contain spaces without surrounding quotes. Unquoted values are parsed as separate tokens and the command fails or targets the wrong file.

**WHY:** The CLI tokenises on whitespace, so `file=Meeting Notes` is read as `file=Meeting` with `Notes` treated as an unknown flag, causing silent misbehaviour or hard-to-diagnose errors.

```bash
# BAD — "Meeting" is parsed as the file name, "Notes" as a flag
obsidian read file=Meeting Notes

# GOOD — full name passed as a single value
obsidian read file="Meeting Notes"
```

### 3. Not using `silent` when creating or updating files

**NEVER** omit the `silent` flag when creating or updating files from an agent workflow. Without `silent`, Obsidian opens the created or updated file in the editor. When running CLI commands from an agent workflow, this disrupts the user's current view and focus.

**WHY:** Every file-write command without `silent` hijacks the user's editor pane, yanking them away from whatever they were doing and making automated multi-step workflows feel broken and intrusive.

```bash
# BAD — Obsidian opens the new file, pulling the user away from their work
obsidian create name="Draft" content="# Draft"

# GOOD — file is created without changing the editor state
obsidian create name="Draft" content="# Draft" silent
```

### 4. Using `obsidian eval` in production plugin code

**NEVER** embed `obsidian eval` calls in production plugin code. `obsidian eval` runs arbitrary JavaScript in the app context. It is a development and debugging tool only. Embedding eval calls in plugin code creates security risks and unpredictable behaviour.

**WHY:** `obsidian eval` bypasses normal plugin sandboxing. In production it exposes the full app context to arbitrary string execution, making the plugin a vector for code injection and violating Obsidian's plugin review guidelines.

```bash
# BAD — production plugin calling eval to inspect state
obsidian eval code="app.vault.getFiles().map(f => f.path)"

# GOOD — use eval only during development to inspect or prototype, then replace with proper plugin API calls
obsidian eval code="app.vault.getFiles().length"   # dev/debug only
```

### 5. Assuming the CLI targets the right vault when multiple vaults are open

**NEVER** omit `vault=<name>` when the user may have more than one vault open and vault identity matters. By default, commands target the most recently focused vault. When the user has two or more vaults open, this is unpredictable. Always specify `vault=<name>` as the first parameter when vault identity matters.

**WHY:** Focus can shift between vaults at any moment. Omitting `vault=` means writes or reads silently land in the wrong vault with no error, causing data to appear in unexpected places or tasks to be missed entirely.

```bash
# BAD — may silently write to the wrong vault if focus has shifted
obsidian daily:append content="- [ ] Review PR"

# GOOD — explicitly targets the intended vault
obsidian vault="Work" daily:append content="- [ ] Review PR"
```

## References

- [Obsidian CLI docs](https://help.obsidian.md/cli)
