---
name: pin
description: "Pin session decisions, questions, objections, scope constraints, and corrections to a persistent board that survives context compaction. Use PROACTIVELY when: (1) user approves/rejects a recommendation, (2) user asks a clarifying question about a proposal, (3) user states a scope constraint, (4) user corrects a misunderstanding. Also use when user says pin, track this, mark as approved, board, show pins. Auto-invoke without user asking whenever a decision, question, or constraint is detected."
allowed-tools: [Bash, Read, Write, Edit]
model: haiku
context: main
user-invocable: true
---

# Pin — Session Decision Board

Persist decisions, questions, constraints, and corrections to a JSON file that survives context compaction. A companion hook injects the board into every tool call so the model never forgets.

## Auto-Invoke Rules

After responding to any user message where a decision was made, a question was asked about a proposal, or a constraint was stated, IMMEDIATELY invoke /pin with the appropriate category. Do not ask permission — just pin it.

Examples of auto-invoke triggers:
- User: "yes go with bun" → respond normally, then `/pin ✅ use bun`
- User: "what about the latency impact?" → respond normally, then `/pin ❓ split services — latency impact?`
- User: "no skip auth for now" → respond normally, then `/pin ❌ auth layer — skip for MVP`
- User: "MVP only, max 3 files" → respond normally, then `/pin 📌 MVP only, max 3 files`
- User: "no I meant artisans not developers" → respond normally, then `/pin 🔧 target = artisans, not developers`

Do NOT pin:
- Casual conversation, greetings
- Implementation details (code changes, file edits)
- Things already pinned (check board first)

## Commands

| Command | Action |
|---|---|
| `/pin ✅ <text>` | Pin approved item |
| `/pin ❓ <text>` | Pin pending question |
| `/pin ❌ <text>` | Pin killed/rejected item |
| `/pin 📌 <text>` | Pin scope constraint |
| `/pin 🔧 <text>` | Pin correction |
| `/pin show` or `/pin` | Display current board |
| `/pin rm <n>` | Remove pin by number |
| `/pin clear` | Clear all pins |
| `/pin clear triage` | Clear ✅/❓ only, keep 📌/❌/🔧 |

## State File

Path: a session-scoped `.session-logs/<slug>/pins.json` file under a configurable base directory.

Derive slug from CWD:

```bash
GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
REL_PATH="${PWD#$GIT_ROOT/}"
SLUG=$(echo "$REL_PATH" | tr '/' '-')
PINS_DIR="$PRAXIS_DIR/.session-logs/$SLUG"
PINS_FILE="$PINS_DIR/pins.json"
```

Schema:

```json
{
  "items": [
    {
      "id": 1,
      "type": "approved",
      "emoji": "✅",
      "content": "use bun everywhere",
      "detail": "",
      "ts": "2026-04-01T14:30:00Z"
    }
  ],
  "next_id": 2
}
```

Type mapping: ✅=approved, ❓=pending, ❌=killed, 📌=scope, 🔧=correction

## Pin (`/pin <emoji> <text>`)

Parse the emoji prefix to determine type. Text after emoji is content. If text contains ` — `, split into content and detail.

1. Derive `PINS_FILE` path (see State File above)
2. `mkdir -p` the directory
3. Read existing file or init `{"items":[],"next_id":1}`
4. Check if content already pinned (exact match on content field) → if so, respond `⚠️ Already pinned.` and stop
5. Check limits: 5 items per type, 20 total. If category full, drop oldest item of that type.
6. Append new item with `id=next_id`, increment `next_id`
7. Write file
8. Respond: `📌 Pinned #N: <emoji> <content>` — one line only, then resume prior work

## Show (`/pin show` or `/pin`)

1. Read `PINS_FILE`
2. If file missing or items empty: `📋 Pin board is empty.`
3. Display:

```
📋 Pin Board (5 items)
 1. ✅ use bun everywhere (minor: keep fallback for CI)
 2. ✅ split the PR into 2
 3. ❓ split services — latency impact?
 4. ❌ auth rewrite — out of MVP scope
 5. 📌 MVP only, max 3 files
```

## Remove (`/pin rm <n>`)

If no number: `⚠️ Usage: /pin rm <number>`

1. Read `PINS_FILE`, find item with `id === n`
2. If not found: `⚠️ Pin #N not found.`
3. Remove item, write file
4. Respond: `🗑️ Pin #N removed.`

## Clear (`/pin clear`)

1. Reset file to `{"items":[],"next_id":<keep current next_id>}`
2. Respond: `🗑️ Pin board cleared.`

## Clear Triage (`/pin clear triage`)

1. Remove items where type is `approved` or `pending`
2. Keep items where type is `killed`, `scope`, or `correction`
3. Write file
4. Respond: `🗑️ Triage cleared. <N> pins remaining.`

## Limits

- 5 items per type, 20 total
- When a category is full, drop the oldest item of that type (lowest id)

## Philosophy

The pin board is a **capture tool, not a management tool**. Its only job is to hold decisions and constraints in a persistent, queryable form so that context compaction does not erase them. ALWAYS treat the board as a write-once, append-only record during a session — edits and removals are operations of last resort, not routine cleanup. A well-run session generates fewer than 20 pins; if you hit the limit frequently, the signals are too granular.

## Key Behaviors

- **One-line responses only.** Never add commentary about pin content.
- **Resume immediately.** After any pin command, pick up the prior conversation exactly where it left off.
- **Store verbatim.** No cleanup, no categorization, no reformulation of user's words.
- **Stable numbering.** Gaps stay after deletion. `next_id` always increments, never reuses.
- **Gotcha — emoji parsing**: Always split on the first emoji character, not the first space. Content like `📌 MVP only, max 3 files` has the emoji as a 2-byte or 4-byte sequence; naive `cut -d' '` will misparse on some shells.

## When to Use

- The user explicitly approves or rejects a recommendation during a multi-step session ("yes, go with that approach", "no, skip auth for now").
- The user states a scope constraint or boundary that must survive context compaction ("MVP only", "max 3 files", "no external dependencies").
- The user corrects a misunderstanding or reframes a key concept ("I meant artisans, not developers").
- The user asks a clarifying question about a proposal that remains unresolved and will affect future decisions.
- A session involves multiple decision points and you need a shared reference that both parties can query mid-conversation.

## When Not to Use

- The conversation is casual or exploratory with no binding decisions made.
- The item being tracked is an implementation detail (a code change, a file edit) rather than a decision or constraint.
- The content is already pinned — always check the board before adding a duplicate.
- The session is short and single-purpose; context compaction is not a risk.
- The user has explicitly asked to stop auto-pinning for the current session.

## Anti-Patterns

- **NEVER reformulate or paraphrase the user's words when pinning** — store the exact wording as given. **Why:** Rewording introduces your interpretation and erases the original mental model; future agents will read the pin as authoritative, not as a paraphrase.
- **NEVER pin implementation details like code edits or file changes** — pins are for decisions, constraints, and corrections only. **Why:** Flooding the board with low-signal items makes it harder to spot the constraints that actually govern future choices.
- **NEVER add commentary or explanation after a pin response** — the response is one line only, then resume prior work. **Why:** This skill is a capture tool; any extra output derails the primary conversation and signals that the board matters more than the user's task.
- **NEVER reuse or reassign an id after deletion** — `next_id` always increments forward. **Why:** Stable numbering prevents confusion when users reference pins by number across a session; id recycling silently corrupts that contract.
- **NEVER skip the duplicate check before writing** — always compare incoming content against existing items. **Why:** Duplicate pins create a false picture of the decision count and waste the 20-item budget on noise.

## Usage Examples

**Pinning a scope constraint mid-session:**

```bash
# User says: "Keep this MVP only, no more than 3 new files"
/pin 📌 MVP only, max 3 files
# Response: 📌 Pinned #1: 📌 MVP only, max 3 files
# Session resumes immediately from where it left off
```

**Pinning a decision reversal and viewing the board:**

```bash
# User says: "Actually use PostgreSQL, not SQLite"
/pin 🔧 target database = PostgreSQL, not SQLite
# Response: 📌 Pinned #2: 🔧 target database = PostgreSQL, not SQLite

# Later: review all active pins
/pin show
# Response:
# 📋 Pin Board (2 items)
#  1. 📌 MVP only, max 3 files
#  2. 🔧 target database = PostgreSQL, not SQLite
```

**Pinning an unresolved question and clearing triage items after resolution:**

```bash
# Unresolved question surfaces during design
/pin ❓ split services — will latency exceed 200ms SLA?
# Response: 📌 Pinned #3: ❓ split services — will latency exceed 200ms SLA?

# After the question is answered, clear only approved/pending pins
/pin clear triage
# Response: 🗑️ Triage cleared. 1 pin remaining.
```

## References

- [Architectural Decision Records (ADR)](https://adr.github.io/) — lightweight format for capturing and tracking architectural decisions; pin is the session-scoped, in-conversation equivalent
- [Cognitive Offloading and External Memory](https://en.wikipedia.org/wiki/Cognitive_offloading) — theoretical basis for persisting decisions to an external store so working memory is freed for active reasoning
