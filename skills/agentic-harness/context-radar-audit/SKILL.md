---
name: context-radar-audit
description:
  "Audit a project's Claude Code tool stack (MCP servers, plugins, hooks) against the curated recommendations at Context Radar (thoroc.github.io/context-radar): a 90-plus-tool,
  20-layer comparison of context-reduction, memory, and code-nav tools for coding agents. Use when asked to review the Claude Code / MCP tool stack, check for redundant or conflicting tools, decide
  what to install next, or evaluate a new context/memory/code-nav tool against what's already running. DO NOT use for reviewing application dependencies (npm/cargo packages) or general code review -
  scope is Claude Code tooling only. Triggers: 'context radar', 'what tools should we use', 'audit our tool stack', 'is X redundant with Y', 'should I install this MCP server', 'tool stack review',
  'context reduction tools'."
---

# Context Radar Audit

Compare this machine's active Claude Code tooling against Context Radar's curated tool-stack recommendations, surface gaps and redundancies, and report findings: never remediate silently.

## Prerequisites

- Network access to `raw.githubusercontent.com`, where the dataset is published as plain JSON (see Gotcha below)
- `node` on `PATH` (used by both scripts; any recent version with global `fetch` works, tested on v24)
- Read access to the Claude Code config directory (`$CLAUDE_CONFIG_DIR`, default `~/.claude`): `settings.json`, `.mcp.json`, `plugins/installed_plugins.json`, and the target project's `.mcp.json`
- The `create-context-file` skill, to file any findings as a follow-up rather than just stating them in chat

## Gotcha: the comparison table is not in the HTML

A plain `WebFetch` or `curl` of `comparison.html` returns an empty `<table>`, because the page is rendered in the browser. Do not scrape the rendered HTML. The dataset (`{meta, layers, tools}`) is
published as `data/context-reduction-tools.json` in the public Context Radar repository, and the fetch script reads that file as JSON and checks its shape. It never executes fetched content:

```bash
node scripts/fetch-comparison-data.js > /tmp/context-radar-data.json
node -e "const d=require('/tmp/context-radar-data.json'); console.log(d.meta, d.layers.length, d.tools.length)"
```

`d.meta` carries `last_updated`, `stars_verified`, and `tool_count`: always report these alongside any findings so the audit is dated against a known snapshot, since tool verdicts and star counts
drift.

## Data shape

`layers[]` and `tools[]` carry the fields the workflow below reads. `layers[].cardinality` is the
one field every redundancy finding hinges on (`pick-one`/`either-or` means install exactly one for
that layer; `stackable` means several may coexist), and `tools[].verdict.decision` is the other
(`drop` is a removal finding on its own). See the Data shape reference for the full field list,
what each one means, and worked examples.

## Workflow

1. **Fetch the dataset.** Run `fetch-comparison-data.js`, note `meta.last_updated`.
2. **Collect what's actually active.** Run `collect-active-tools.sh <project-dir>`: it prints project `.mcp.json` servers, global `.mcp.json` servers, global user-scope plugins, every hook command
   in global `settings.json` (including tools wired as raw binaries, not MCP servers, e.g. a shell hook calling a path ending in `bin/tokensave` directly), the MCP-server prefixes implied by the
   permissions allowlist, and any global rule file that mandates a specific tool.
3. **Map active tools to dataset `id`s by hand.** This step needs judgement, not string matching: a hook command path like `cargo-tokensave/latest/bin/tokensave` maps to tool id `tokensave`; an MCP
   server named `codebase-memory-mcp` maps to the identically-named id. Not everything active is in the dataset (e.g. `mcp-atlassian`, `playwright`, `aislop` are outside Context Radar's scope -
   ignore them).
4. **Cross-reference against layer rules:**
   - For every `pick-one`/`either-or` layer, count active tools mapped to that layer. More than one → **redundancy finding**.
   - For every active tool, check `verdict.decision`. `drop` → **removal finding**. `watch` with `activityStatus.band: dormant` → **staleness finding**, note but don't recommend action alone.
   - For every layer with a `curatedPick` and zero active coverage → **optional-add note**, not a finding (absence isn't a defect).
   - Check permission allowlists (`mcp__<server>__*` entries) against actual `mcpServers` registrations for that `<server>` name in both project and global `.mcp.json`. Allowlisted-but-unregistered
     → **phantom-tool finding**: permissions exist for a server that isn't wired anywhere, so any rule mandating its use is currently unenforceable.
   - If a rule file (e.g. `~/.claude/rules/*.md`) mandates a tool, verify the tool it names actually resolves to something active per step 2-3. A mandate pointing at a phantom or absent tool is
     worth surfacing even if step 4's other checks don't independently flag it.
5. **Report, don't remediate.** Global config changes affect every project on the machine. Present findings as a list (what's redundant, what's dead config, what's missing) and let the user decide
   what to action. Do not edit the global `.mcp.json` or `settings.json` without explicit confirmation.
6. **File it.** Use the `create-context-file` skill to record findings as a follow-up entry (if deferred) or a finding entry (if it's the whole point of the session), then regenerate the index via
   the `context-index` skill. Don't leave findings only in the chat response, they don't survive a session reset. Typical destinations:

   ```text
   .context/follow-ups/<slug>.md   # deferred finding
   .context/findings/<slug>.md     # finding that is the whole point of the session
   ```

## When to Use

- User asks "what MCP servers / tools should we be using" or references Context Radar directly
- Before adding a new context-reduction, memory, or code-nav tool, to check for cardinality conflicts with what's already installed
- Periodic tool-stack hygiene review (Context Radar's own "Config stack audit" layer note: run this kind of check after assembling a stack, not continuously)

## When Not to Use

- Reviewing application/library dependencies: this skill is scoped to Claude Code's own tooling (MCP servers, plugins, hooks), not the project's `package.json`/`Cargo.toml`
- As a substitute for reading a specific tool's own docs once you've decided to adopt it: the dataset's `decisionRule` field is a summary, not installation instructions

## Mindset

- The dataset is a snapshot with a stated freshness date: treat star counts and "watch"-tier verdicts as time-bound, not permanent
- A tool being active and a tool being *wired in* are different things: hooks, MCP registration, and permission allowlists can drift independently (this is how phantom-tool findings happen)
- Redundancy findings are about layer cardinality (`pick-one` etc.), not personal preference: cite the layer's `cardinality` and `note` field as the reasoning, not "seems duplicative"
- Default to reporting; treat global-config edits as requiring explicit user sign-off every time, not just the first time

## Anti-Patterns

**NEVER** `WebFetch` or `curl` the comparison page directly and treat an empty table as "no data available." **WHY:** The table is client-rendered; the fetch tool never runs the page's JS, so the raw
HTML genuinely has no rows regardless of how the page looks in a browser. **BAD:** Concluding "the comparison page has no tools listed." **GOOD:** Run `fetch-comparison-data.js`, which reads the published
JSON dataset instead of the rendered DOM.

**NEVER** treat "tool X isn't registered as an MCP server" as proof it's inactive. **WHY:** Some tools in the dataset (e.g. `tokensave`, `rtk`) run as hook-invoked CLI binaries, not MCP servers -
absence from `.mcp.json` doesn't mean absence from `settings.json` hooks. **BAD:** "tokensave isn't in any `.mcp.json`, so it's not installed." **GOOD:** Check both `.mcp.json` registrations and
`settings.json` hook commands before concluding a tool is inactive.

**NEVER** edit global Claude Code config (`~/.claude/.mcp.json`, `~/.claude/settings.json`) as part of running this audit. **WHY:** Those files apply to every project on the machine;
an unrequested edit has a blast radius far beyond the project being audited. **BAD:** Silently removing a redundant MCP server entry after finding a `pick-one` violation. **GOOD:** Report the
violation and the two candidate tools, then wait for the user to choose.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Dataset extraction | `scripts/fetch-comparison-data.js` | Pulling the current `{meta, layers, tools}` dataset |
| Active-tool collection | `scripts/collect-active-tools.sh <project-dir>` | Enumerating what's actually registered/hooked on this machine |
| Data shape | `references/data-shape.md` | Looking up the full `layers[]`/`tools[]` field list, meanings, and worked examples |
