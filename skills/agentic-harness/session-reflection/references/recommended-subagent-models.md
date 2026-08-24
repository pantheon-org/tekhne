# Recommended Sub-Agent Models for Session Reflection

The reflection task (reviewing a session summary, identifying blind spots, listing confidence gaps) does **not** need frontier reasoning. It benefits from:

| Requirement | Why |
|-------------|-----|
| Strong instruction following | Must produce structured output matching the prompt template |
| Large context window (≥256K) | Session summaries can be long with file paths, code snippets, command outputs |
| Reasoning capability | Must synthesize across the session, not just parrot the summary |
| Tool-call support | Needed for the sub-agent to investigate flagged items (optional but valuable) |

Models available depend on which subscription/plan the agent uses. Below are the options for each.

---

## OpenCode Zen (pay-as-you-go)

Cheapest models with reasoning + tool-call support, sorted by input price (per MTok):

| Model | Input/M | Output/M | Context | Notes |
|-------|--------|---------|---------|-------|
| DeepSeek V4 Flash | $0.14 | $0.28 | 1M | Strong reasoning, large context, open weights |
| GPT 5 Nano | $0.05 | $0.40 | 400K | Very cheap, solid structured output |
| GPT 5.1 Codex Mini | $0.25 | $2.00 | 200K | Code-oriented, lightweight |
| Qwen3.5 Plus | $0.20 | $1.20 | 1M | Good reasoning, huge context |
| GPT 5.4 Nano | $0.20 | $1.25 | 400K | Current-gen ultra-cheap OpenAI |
| MiniMax M3 | $0.30 | $1.20 | 512K | Open weights, solid reasoning |
| DeepSeek V4 Pro | $1.74 | $3.48 | 1M | More capable than Flash, higher cost |
| Claude Haiku 4.5 | $1.00 | $5.00 | 200K | Fastest Anthropic model through Zen |

**Free models** (available on Zen but may have data retention caveats — see Zen privacy policy):

- DeepSeek V4 Flash Free — best free option with reasoning + tool-call + 1M context
- MiMo-V2.5 Free — large context (1M)
- Nemotron 3 Ultra Free — 1M context
- North Mini Code Free — 256K context
- Big Pickle — stealth model

---

## OpenCode Go ($10/month flat subscription)

No per-token cost — models are rate-limited by requests per 5 hours. All Go models are equally "free" within the subscription. Pick the fastest model with adequate reasoning for the reflection task:

| Model | Context | Why for reflection |
|-------|---------|-------------------|
| DeepSeek V4 Flash | 1M | Fast, strong reasoning, lowest rate-limit consumption |
| MiniMax M3 | 512K | Good speed/reasoning balance |
| GLM-5.2 | 1M | Large context for long session summaries |
| Kimi K2.7 Code | 262K | Code-oriented reasoning |

Since Go is flat-rate, the cheapest option is whichever model is fastest and least rate-limited — **DeepSeek V4 Flash** is the best default.

---

## Claude Code (Anthropic subscription / API)

Only Anthropic models are available. Claude Haiku is the cheapest and sufficient for reflection:

| Model | Input/M | Output/M | Context | Why for reflection |
|-------|--------|---------|---------|-------------------|
| **Claude Haiku 4.5** | $1.00 | $5.00 | 200K | Fastest, cheapest Claude. Plenty of reasoning for meta-cognitive tasks |
| Claude Sonnet 5 | $3.00 | $15.00 | 1M | Overkill for reflection (same cost as main model) |
| Claude Opus 4.8 | $5.00 | $25.00 | 1M | Overkill |

**Claude Haiku 4.5** is the clear default for sub-agent reflection in Claude Code.

---

## Summary: default recommendations

| Environment | Cheapest viable model | Why |
|-------------|----------------------|-----|
| OpenCode Zen | GPT 5 Nano ($0.05/$0.40) or DeepSeek V4 Flash ($0.14/$0.28) | Lowest per-token cost with reasoning + tool-call |
| OpenCode Go | DeepSeek V4 Flash | Flat-rate subscription, fastest model with good reasoning |
| Claude Code | Claude Haiku 4.5 ($1/$5) | Only cheap Anthropic option, sufficient for reflection |
| OpenCode + BYOK | DeepSeek V4 Flash (if available via provider) or cheapest available reasoning model | Depends on provider pricing |

## What NOT to use

- **No reasoning** models — the task requires synthesis and judgment
- **No tool-call** models — limits ability to investigate flagged items autonomously
- **Nano-scale models** (sub-10B params without MoE) — typically too weak for reliable metacognitive reasoning

## Caveats

- **Zen free models** may retain data for model improvement. Do not use with confidential session summaries unless you've reviewed the privacy terms.
- **Go subscription** rate limits apply per 5-hour window. Using a slower model for reflection consumes rate limit you might want for main work.
- **Always test your chosen model on 3-5 real session summaries before relying on it.** A model that costs nothing but misses blind spots is more expensive than one that costs $0.20/M tok and catches them.
