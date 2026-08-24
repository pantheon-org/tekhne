# Sub-Agent Spawn Pattern

Full workflow for offloading the reflection to a sub-agent, for deep sessions with significant work. This keeps the
main model focused on delivery and surfaces a fresh perspective. If the environment routes exploration sub-agents to
a cheaper or faster model, this also reduces cost -- see
[Recommended Sub-Agent Models](recommended-subagent-models.md) for current options and pricing caveats.

The reflection task needs instruction following, reasoning, tool-call support, and a large context window, but does
NOT need frontier reasoning -- many cheap or free models suffice.

## Workflow

1. **Main agent** detects session-end signals and composes a **session summary** capturing:
   - What work was done (files touched, commands run, decisions made)
   - What was assumed without verification (dependency versions, code paths, configurations)
   - What was explicitly skipped or deferred
   - What alternatives were considered but not explored
   - Any open questions or unresolved threads from the conversation
2. **Main agent** spawns an exploration sub-agent with this prompt:

   ```text
   Review this session summary and answer two questions with specific, actionable items:

   1. CONFIDENCE AUDIT: What 3-7 things are you least confident about in this work?
      For each: state what was done, what was not verified, and why confidence is low.
      Be precise -- file paths, function names, specific assumptions.

   2. BLIND-SPOT CHECK: What's the biggest thing the user might be missing?
      Consider unexamined assumptions, alternative approaches not explored,
      constraints that may have shifted, or signals that were dropped.

   Session summary:
   <summarize the session here>
   ```

3. **Sub-agent** returns its reflection as structured text.
4. **Main agent** presents the results with a preamble such as: "I asked a second agent to review the session for
   blind spots. Here's what it surfaced:"
5. Optionally let the sub-agent do the investigation too (if the user flags an item), by spawning another task with
   the investigation context.

## When the Sub-Agent Disagrees With the Main Agent

If the sub-agent flags something the main agent is confident about, investigate anyway -- the whole point of the
reflection is catching blind spots; defensive disagreement is a feature, not a bug.

## Choosing Inline vs Sub-Agent Mode

Prefer inline mode for short sessions -- the sub-agent spawn overhead (summary composition plus the extra call) is
only justified when there is substantial work to review. For deep sessions, the act of summarizing the session
itself forces the main agent to be explicit about what was done versus assumed, which is a valuable metacognitive
exercise on its own, independent of what the sub-agent returns.

**When to use this reference:** deciding whether a given session is deep enough to warrant the sub-agent spawn
pattern, or when actually composing the spawn prompt.
