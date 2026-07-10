---
name: cache-audit
description: Audit your Claude Code setup against prompt caching best practices. Checks ordering, tool stability, dynamic content handling, and hook injection patterns. Returns a scored report with specific fixes.
---

# Prompt Cache Audit Skill

**Trigger:** `/cache-audit` or "audit my caching" or "check my cache setup" or "am I breaking the cache?"

**What it does:** Reads your live Claude Code configuration and checks it against the 6 prompt caching rules from Anthropic's engineering team. Returns a scored report with specific, actionable fixes.

**Reference:** Based on Thariq Shihipar's thread "Lessons from Building Claude Code: Prompt Caching Is Everything"

---

## When Invoked

Run all checks automatically. Do not ask for confirmation. Read the relevant files and produce the full report in one pass.

---

## The 6 Checks

### Check 1 — Prompt Ordering (Static Before Dynamic)

**Read:** `~/.claude/settings.json`, all active `CLAUDE.md` files in the project hierarchy

**What to look for:**
- Does the system prompt load in the right order? Rule: static content first, dynamic content last
- Correct order: System prompt → Tools → CLAUDE.md → Session context → Messages
- Flag: Any dynamic content (timestamps, git status, current date, user stats) appearing in the system prompt itself
- Flag: CLAUDE.md files that include session-specific or time-sensitive data
- Pass: CLAUDE.md files that are purely static instructions, conventions, and file references

**Scoring:**
- PASS: System prompt is fully static, dynamic data injected via messages
- WARNING: Some dynamic data in system prompt but low-frequency change
- FAIL: High-churn dynamic content (timestamps, file contents) in system prompt

---

### Check 2 — Dynamic Updates via Messages (not System Prompt Edits)

**Read:** All hook files listed under `SessionStart` and `UserPromptSubmit` in `~/.claude/settings.json`

**What to look for:**
- Hooks should output dynamic data as `additionalContext` in their JSON response (which becomes a `<system-reminder>` message) — not by modifying the system prompt
- Check each hook's output format: does it use `hookSpecificOutput.additionalContext`? ✅
- Flag: Any hook that writes to a system prompt file, modifies CLAUDE.md, or injects into the static prefix
- Pass: Hooks that return JSON with `additionalContext` key

**Also check:**
- Is `currentDate` injected via message (memory.md context) or hardcoded in system prompt?
- Is git status coming from a hook → message, or somewhere static?

**Scoring:**
- PASS: All hooks use additionalContext pattern
- FAIL: Any hook modifies system prompt or CLAUDE.md mid-session

---

### Check 3 — Tool Set Stability (No Add/Remove Mid-Session)

**Read:** `~/.claude/settings.json`, `~/.claude/skills/*.md`, MCP server configurations

**What to look for:**
- Tools should be identical at every turn of the conversation
- Check: Do any skills explicitly add new tools when invoked? (tool definitions that only appear after a skill runs)
- Check: Are MCP tools using `defer_loading: true` stubs rather than full schemas loaded conditionally?
- Flag: Any skill that modifies the available tool set
- Pass: MCP tools present as lightweight stubs in every request, full schemas only loaded on demand via ToolSearch

**Note:** The ToolSearch tool itself is the correct pattern — it lets the model discover tools without adding/removing from the base set.

**Scoring:**
- PASS: Tool set is fixed at session start, all MCP tools deferred
- WARNING: Some conditional tool loading that may cause cache misses
- FAIL: Skills or hooks that add/remove tools mid-conversation

---

### Check 4 — No Mid-Session Model Switches

**Read:** `~/.claude/settings.json`, `~/.claude/skills/*.md`, any agent/team configurations

**What to look for:**
- The `model` field in settings.json should be set and stable
- Check: Do any skills switch models in the same conversation thread? (e.g., running a quick haiku query inline)
- Pass: Model switches are done via subagents with handoff messages — the Explore/Plan agent types running on Haiku are separate conversations, not inline switches
- Flag: Any pattern where the main conversation calls a different model mid-turn

**Scoring:**
- PASS: Single model per conversation, subagents used for model delegation
- FAIL: Inline model switching in same conversation thread

---

### Check 5 — Dynamic Content Size

**Read:** Hook files, git status injection, session-reminder outputs

**What to measure:**
- Estimate the size of dynamic content injected per session/turn
- Check the git status hook output — measure the typical size of the injected git diff/status
- Check streak/quota hook output size
- Check if granola meeting sync injects large content per turn

**Thresholds:**
- < 2k chars injected per turn: ✅ PASS
- 2k–10k chars: ⚠️ WARNING (still correct pattern, just expensive)
- > 10k chars: ❌ FLAG — consider trimming

**Known issue to flag:**
- Git status with hundreds of untracked files (like the personalOS repo) can easily exceed 40k chars per session start. This is injected correctly via messages, but the raw token cost is high. Recommend: trim to branch name + changed file count + modified file list only.

**Suggested fix for large git status:**
```bash
# Instead of full git status, inject only:
git branch --show-current
git diff --stat HEAD | tail -5
git status --short | grep "^[^?]" | head -20  # only tracked changes, no untracked
```

---

### Check 6 — Fork Safety (Compaction & Subagent Calls)

**Read:** Any compaction configuration, skill invocations that fork context

**What to look for:**
- When Claude Code runs compaction (context window fills), does the summary request reuse the same system prompt + tools as the parent?
- When skills fork a subagent, do they pass through the same prefix?
- This is partially an Anthropic infrastructure concern — Claude Code handles it correctly by default

**Scoring:**
- PASS: Using Claude Code's built-in compaction (handled correctly)
- MANUAL CHECK: Custom compaction or summarization flows — verify they use identical system prompt + tool definitions as parent

---

## Output Format

After running all checks, output this report:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  PROMPT CACHE AUDIT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Score: X/6

✅ / ⚠️ / ❌  Rule 1 — Ordering: [PASS/WARNING/FAIL]
   → [Specific finding or "All good"]

✅ / ⚠️ / ❌  Rule 2 — Message injection: [PASS/WARNING/FAIL]
   → [Specific hooks checked and their pattern]

✅ / ⚠️ / ❌  Rule 3 — Tool stability: [PASS/WARNING/FAIL]
   → [MCP tool count, defer status]

✅ / ⚠️ / ❌  Rule 4 — Model switching: [PASS/WARNING/FAIL]
   → [Model in settings, any inline switches found]

✅ / ⚠️ / ❌  Rule 5 — Dynamic content size: [PASS/WARNING/FAIL]
   → [Estimated chars/turn for each injection point]

✅ / ⚠️ / ❌  Rule 6 — Fork safety: [PASS/MANUAL CHECK]
   → [Compaction pattern used]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  TOP FIX
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Single most impactful change with exact code/config to implement]
```

If everything passes: say so clearly and note the estimated cost savings from the current setup vs. a naive implementation.

---

## Reference: The 6 Rules (Quick Cheatsheet)

| Rule | Do | Don't |
|------|----|-------|
| 1. Ordering | Static system prompt → CLAUDE.md → messages | Dynamic data in system prompt |
| 2. Updates | Inject via `<system-reminder>` in messages | Edit system prompt mid-session |
| 3. Tools | Fixed tool set + deferred stubs | Add/remove tools per turn |
| 4. Models | One model per conversation, subagents for switches | Inline model switching |
| 5. Size | Trim dynamic injections to minimum needed | Dump full git status (40k chars) |
| 6. Forks | Same prefix for compaction/subagents | Different system prompt for summary calls |
