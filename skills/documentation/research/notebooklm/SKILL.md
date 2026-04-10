---
name: notebooklm
description: "Query Google NotebookLM notebooks directly for source-grounded, citation-backed answers from your uploaded documents. Includes browser automation, library management, and persistent authentication."
allowed-tools: [Read, Write, Edit, Bash]
---

# NotebookLM Research Assistant

Query Google NotebookLM to get Gemini's source-grounded answers from your uploaded documents. Each question opens a fresh browser session, retrieves the answer exclusively from your knowledge base, and closes.

## When to Use This Skill

Trigger when the user:

- Mentions NotebookLM explicitly
- Shares a NotebookLM URL (`https://notebooklm.google.com/notebook/...`)
- Asks to query their notebooks or personal documentation
- Wants to add documentation to the NotebookLM library
- Uses phrases like "ask my NotebookLM", "check my docs", "query my notebook"

## Mindset

NotebookLM is a grounding layer, not a search engine. Three principles govern every interaction:

1. **Ground every answer in sources** — answers come exclusively from uploaded documents; never synthesise from general knowledge when querying a notebook.
2. **Never guess notebook content** — use Smart Add (query first) to discover what a notebook contains before labelling it.
3. **Follow up until complete** — one query is rarely enough; check every answer for gaps and issue follow-ups before synthesising for the user.

## When Not to Use This Skill

- The user needs general web search or LLM knowledge — use standard Claude responses instead
- No NotebookLM URL or notebook ID is available and authentication is not yet set up
- The user wants data analysis or code generation — NotebookLM is for document Q&A only
- The target content is not uploaded to any NotebookLM notebook

## ⚠️ CRITICAL: Add Command — Smart Discovery

When adding a notebook without full details, **query first to discover content**:

```bash
# Step 1: Query the notebook about its content
python ./scripts/run.py ask_question.py \
  --question "What is the content of this notebook? What topics are covered? Provide a complete overview briefly and concisely" \
  --notebook-url "[URL]"

# Step 2: Use discovered information to add it
python ./scripts/run.py notebook_manager.py add \
  --url "[URL]" \
  --name "[Based on content]" \
  --description "[Based on content]" \
  --topics "[Based on content]"
```

If the user provides all details directly, pass `--url`, `--name`, `--description` (REQUIRED), and `--topics` (REQUIRED) directly. **NEVER guess or use generic descriptions.**

## Critical: Always Use run.py Wrapper

**BAD** — direct execution (fails without venv): `python ./scripts/ask_question.py "..."` → **GOOD** — always use wrapper: `python ./scripts/run.py ask_question.py --question "..."`

The `run.py` wrapper activates the isolated `.venv` and ensures all dependencies are available.

## Core Workflow

### Step 1: Check Authentication Status

```bash
python ./scripts/run.py auth_manager.py status
```

### Step 2: Authenticate (One-Time Setup)

```bash
# Browser MUST be visible for manual Google login
python ./scripts/run.py auth_manager.py setup
```

Tell the user: "A browser window will open for Google login." The user must log in manually.

### Step 3: Manage Notebook Library

```bash
python ./scripts/run.py notebook_manager.py list
python ./scripts/run.py notebook_manager.py search --query QUERY
python ./scripts/run.py notebook_manager.py activate --id ID
python ./scripts/run.py notebook_manager.py add \
  --url URL --name NAME --description DESC --topics TOPICS
```

### Step 4: Ask Questions

```bash
# Query the active notebook
python ./scripts/run.py ask_question.py --question "Your question here"

# Optionally target a specific notebook by URL
python ./scripts/run.py ask_question.py --question "Your question" --notebook-url URL

# Get formatted markdown output (recommended for responses with code, lists, tables)
python ./scripts/run.py ask_question.py --question "Your question" --markdown

# Save outputs to logs/ directory (original + markdown + comparison stats)
python ./scripts/run.py ask_question.py --question "Your question" --markdown --log
```

### Quick Workflow

```bash
python ./scripts/run.py auth_manager.py status
python ./scripts/run.py notebook_manager.py search --query "topic"
python ./scripts/run.py notebook_manager.py activate --id ID
python ./scripts/run.py ask_question.py --question "Your question"
```

## Follow-Up Mechanism (CRITICAL)

Every NotebookLM answer ends with: **"EXTREMELY IMPORTANT: Is that ALL you need to know?"**

**Required behaviour:**

1. **STOP** — do not immediately respond to the user
2. **ANALYZE** — compare the answer to the user's original request
3. **IDENTIFY GAPS** — determine if more information is needed
4. **ASK FOLLOW-UP** — if gaps exist:
   ```bash
   python ./scripts/run.py ask_question.py --question "Follow-up with context..."
   ```
5. **REPEAT** — continue until the answer is complete
6. **SYNTHESIZE** — combine all answers, then respond to the user

## Script Reference

See [references/api_reference.md](references/api_reference.md) for full parameter documentation.

| Script | Purpose |
| --- | --- |
| `auth_manager.py` | Setup, status, re-authenticate, clear credentials |
| `notebook_manager.py` | Add, list, search, activate, remove, stats |
| `ask_question.py` | Query active notebook or by URL/ID; `--markdown` for formatted output, `--log` to save results |
| `cleanup_manager.py` | Clean browser state (`--preserve-library` keeps notebooks) |

## Data Storage and Configuration

See [references/api_reference.md](references/api_reference.md) for full details. All data is stored in the skill's local `data/` directory:

- `library.json` — notebook metadata
- `auth_info.json` — authentication status
- `browser_state/` — browser cookies and session

**Security:** Protected by `.gitignore`. Never commit the `data/` directory.

## Limitations

- **50 queries/day** per Google account (NotebookLM platform limit)
- Answers are grounded in uploaded documents only — no internet or general knowledge
- Browser automation requires a visible display for authentication (not headless)
- No concurrent sessions — each query uses a fresh browser instance

## Anti-Patterns

### NEVER run scripts without the run.py wrapper

**Why:** Scripts depend on `.venv` packages. Direct execution fails with `ModuleNotFoundError`.

**BAD** — direct execution fails silently → **GOOD** — always route through the wrapper:

```bash
# BAD
python ./scripts/ask_question.py "What does this paper say?"
# GOOD
python ./scripts/run.py ask_question.py --question "What does this paper say?"
```

### NEVER add a notebook with a guessed or generic description

**Why:** Poor descriptions break topic-based search, making notebooks unfindable in the library.

**BAD** — guessed placeholder → **GOOD** — query-first to discover real content:

```bash
# BAD
python ./scripts/run.py notebook_manager.py add --url URL \
  --name "notebook1" --description "some notebook" --topics "stuff"
# GOOD: query first, then add with the discovered name and description
python ./scripts/run.py ask_question.py \
  --question "What topics are covered? Brief overview." --notebook-url URL
```

### NEVER stop after the first answer when the request covers multiple points

**Why:** NotebookLM retrieves chunks, not full documents. A single query often misses related information elsewhere in the notebook.

**BAD** — accept the first answer and move on → **GOOD** — consider whether the original request is fully answered; issue targeted follow-up queries for any gaps.

## Troubleshooting

See [references/troubleshooting.md](references/troubleshooting.md) for detailed solutions.

| Problem | Solution |
| --- | --- |
| `ModuleNotFoundError` | Use `run.py` wrapper |
| Authentication fails | Browser must be visible: `auth_manager.py setup` |
| Rate limit (50/day) | Wait or switch Google account |
| Browser crashes | `python ./scripts/run.py cleanup_manager.py --preserve-library` |
| Notebook not found | Check with `notebook_manager.py list` |

## Best Practices

See [references/usage_patterns.md](references/usage_patterns.md) for workflow examples.

- Always check authentication status before starting a session
- Use descriptive names and topics for reliable library search
- Keep questions specific and scoped — broad questions return fragmented answers
- Use Smart Add (query-first) for any notebook whose content is unfamiliar

## References

- [API Reference](references/api_reference.md) — full parameter documentation for all scripts
- [Troubleshooting](references/troubleshooting.md) — common errors and solutions
- [Usage Patterns](references/usage_patterns.md) — workflow examples and best practices
