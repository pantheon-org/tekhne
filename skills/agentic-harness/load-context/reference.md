# load-context Reference

## Section Mapping

| CONTEXT Section | Report Block | Fallback names |
|----------------|-------------|----------------|
| Header fields (saved/stream/status/focus/goal) | Stream/Saved/Focus/Goal | — |
| `## Session` | 💬 Session Context | `## Session Progression` |
| `## Next Tasks` | ✅ NextTasks | `## NextTasks` |
| `## Hot Files` | 📁 Hot Files | `## Files` |
| `## Tasks` | 📋 Task Snapshot | — |
| `## Project` | 🎯 Project Context | — |
| `## Refs` | 📎 References | `## References` |

**Graceful degradation**: Missing/malformed sections → skip (don't error). Only Header + `## Next Tasks` required.

## Report Structure

```
# 🔄 Session Resume: [stream-name]

Stream/Saved/Focus/Goal (always show)
📂 Available Streams (if multiple)
🎯 Project Context (if refs exist)
🎯 Active Work (if OpenSpec active)
📋 Task Snapshot (if tasks saved)
✅ NextTasks (always show top 3)
💬 Session Context (always show)
📁 Hot Files (always show)
🧪 Tests (if script found)
🎯 Next Step (always show)
```

## Formatting Principles

- Parse key-value header + markdown sections directly, don't re-synthesize
- Expand inline objects: `{done: 5, active: 2}` → "5 done, 2 active"
- Lists from markdown without re-ordering
- Emoji-rich but concise
- **Next Step**: Single sentence action based on NextTasks[0] + focus field

## Meta-Awareness

**Input**: Key-value header + markdown sections from `/save-context` (token-optimized)
**Audience**: Human user resuming work
**Purpose**: Transform LLM-optimized context → human-friendly resume report

## Error Messages

| Condition | Message |
|-----------|---------|
| No context files | "No context files found. Run `/save-context` to create one." |
| Stream not found | "Stream '{name}' not found. Available: {list}" (also check `.context/session/done/` subfolder) |
| Stream in done/ | Load normally, prefix report with "📦 Loaded from `.context/session/done/` — this context is archived ({status})" |
| File read error | "Could not read {filename}. Check file permissions." |
| Malformed file | Parse what's available, skip unparseable sections |

## Related

- `/save-context [stream] [description]` - Save session to named stream
- `/list-contexts [--sync] [--archive <stream>]` - List/sync all contexts
- `/create-context` - Create baseline from `.context/session/in/` folder
