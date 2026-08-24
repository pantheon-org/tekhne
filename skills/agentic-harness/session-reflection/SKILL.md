---
name: session-reflection
description: "Conduct a two-question session-end reflection to catch blind spots and under-investigated areas before concluding. The agent surfaces its lowest-confidence work items and identifies what the user might be missing, then offers to investigate. Based on a Reddit-post technique combining an LLM-suggested confidence audit with Sam Altman's blind-spot question. Do NOT trigger for brief single-answer queries, CI contexts, or in the middle of active work -- only when a session appears to be concluding. Triggers: 'wrap up', 'we're done', 'conclude', 'session end', 'final review', 'before we go', 'sign off', 'that's all', 'anything else', 'finished', 'reflection', 'confidence check', 'blind spot', 'what are you missing', 'rate your confidence', 'review the session'."
---

# Session-End Reflection

Catch blind spots and under-investigated areas before concluding a session by asking two questions:

1. **Confidence audit:** What am I least confident about right now?
2. **Blind-spot check (Sam Altman):** What's the biggest thing I'm missing about this situation? What don't I realize?

~1 in 4 sessions, one of the answers reveals a critical gap that would silently invalidate work. This skill catches
those gaps at the cheapest possible moment: before the user has walked away.

## Prerequisites

- A session that appears to be concluding (user signals completion, asks for summary, or starts wrap-up language)
- A project-level behavioural rule for "always conduct session-end reflection" should already be active, if this
  project has one -- check the project's own rules file
- For persisting uncovered findings: the `context-file` skill, and optionally `adr-capture` if a binding decision
  emerges
- For genuine gaps that aren't being fixed in this session: the `context-file` skill's known-issue document type --
  this is the primary, intended source of those entries; see Workflow step 6

## When to Use

- A session appears to be concluding: the user signals completion, asks for a summary, or starts wrap-up language.
- All identified tasks are marked complete and the agent is about to hand back to the user.

## When NOT to Use

- During a brief query that is clearly complete (e.g., "what's the capital of France?") -- the reflection overhead is
  not justified.
- When the user has explicitly said "don't do the reflection this time" or similar -- honour it, don't insist.
- In automated/CI contexts -- this is a human-interactive skill only.
- In the middle of active work -- only at session-end boundaries.

## Workflow

1. **Detect session-end signals**: "we're done", "thanks", "that's all", a request for a summary or next steps, or
   all identified tasks marked complete.
2. **Choose the reflection mode** -- inline (default, for short sessions) or sub-agent spawn (preferred for deep
   sessions with significant work; see [Sub-Agent Spawn Pattern](references/subagent-spawn-pattern.md)).
3. **Initiate the reflection** with a natural opening, e.g. "Before we wrap up, I'd like to do a quick reflection."
   Ask the two questions sequentially, never both at once -- wait for the user's response to each.
4. **Question 1, confidence audit**: "What am I least confident about right now?" Generate 3-7 specific items, each
   naming what was done, what was not verified, and why confidence is low -- not a vague feeling.
5. **Question 2, blind-spot check**: "What's the biggest thing I'm missing about this situation?" Target assumptions
   the user stated but that went unverified, alternatives not explored, and signals dropped mid-conversation.
6. **Follow up.** If the user flags an item, investigate before concluding. If a finding warrants preservation, use
   `context-file`. **If an item is a verified, concrete gap that is NOT being fixed in this session, create a
   known-issue document** (`status: ACTIVE`, `severity: CRITICAL | HIGH | MEDIUM | LOW`) via `context-file` -- this
   is the primary source of those entries; a reflection item that only lives in chat scrollback is the failure mode
   this step exists to prevent. Skip this only when the item is being fixed right now instead.
7. **Conclude** only once the investigation loop is resolved. Note clearly if new work was spawned.
8. **Verify outcome**: confirm with the user that the reflection addressed their concerns, summarize any
   investigation performed, confirm any known-issue file was actually created, and explicitly ask whether anything
   else from the reflection needs addressing.

## Definition of Done

- [ ] Both questions were asked sequentially, with the user's response read before the next question, not skipped.
- [ ] Every confidence item names a concrete file, function, or assumption -- none are vague feelings.
- [ ] Any flagged item was either investigated and resolved, or filed as a known-issue document -- not left as a
      bare chat mention.
- [ ] The user confirmed the reflection addressed their concerns before the session is treated as closed.

## Anti-Patterns

**NEVER** skip the reflection because the session feels complete.
**WHY:** the agent is a poor judge of its own completeness; the ~1-in-4 statistic means confidence is not a reliable
signal for whether to skip it.
**SYMPTOM:** the reply jumps straight to a wrap-up summary with no confidence-audit or blind-spot question asked.
**CONSEQUENCE:** a genuine gap ships unnoticed because nobody ever asked the two questions that would have surfaced
it.
**BAD:** "Everything looks good, no need for reflection."
**GOOD:** Always run the reflection regardless of how confident the session seems.

**NEVER** give a vague confidence item.
**WHY:** "I'm not confident about the overall approach" gives the user nothing to act on.
**SYMPTOM:** a confidence item names a feeling ("not sure about performance") instead of a file, function, or
assumption.
**CONSEQUENCE:** the user cannot decide whether to investigate, so the reflection produces no actionable follow-up.
**BAD:** "I'm least confident about performance."
**GOOD:** "I'm least confident about the query performance in `getUserOrders()` -- I assumed the index exists but
didn't verify it against the production schema."

**NEVER** ask both reflection questions in one breath.
**WHY:** combining them reduces the thoughtfulness of each individual answer.
**SYMPTOM:** a single message asks "what are you least confident about and what am I missing?" back to back.
**CONSEQUENCE:** the user answers the easier question and skips the harder one, and the reflection silently loses
half its value.
**BAD:** "What am I least confident about and what am I missing?"
**GOOD:** Ask, wait for the full answer and discussion, then ask the second question.

**NEVER** deflect or make excuses for a low-confidence item.
**WHY:** the reflection is a safe space for surfacing uncertainty, and excuses undermine the honesty it depends on.
**SYMPTOM:** a confidence item is immediately followed by a justification for why it wasn't checked.
**CONSEQUENCE:** the user learns to treat future reflections as self-justification rather than a genuine audit.
**BAD:** "I'm not confident about X, but that's because you didn't ask for it."
**GOOD:** "I'm not confident about X -- I didn't verify it. Want me to check now?"

**NEVER** spawn a sub-agent reflection without a concrete session summary.
**WHY:** a vague prompt ("review this session") produces vague output; the summary quality determines the
reflection quality.
**SYMPTOM:** the sub-agent prompt says only "review this session" with no list of what was done, assumed, or
skipped.
**CONSEQUENCE:** the sub-agent's blind-spot check has nothing concrete to interrogate and returns generic filler.
**BAD:** Spawning with only "please reflect on this conversation."
**GOOD:** Summarize files touched, commands run, assumptions made, and what was skipped, then spawn with that
summary (full template in [Sub-Agent Spawn Pattern](references/subagent-spawn-pattern.md)).

**NEVER** let a verified-but-deferred gap live only as a chat mention.
**WHY:** a reflection item that gets discussed and then only lives in chat scrollback is exactly the failure mode
Workflow step 6 exists to prevent.
**SYMPTOM:** the reply says "that's worth looking into sometime" with no known-issue document created.
**CONSEQUENCE:** the gap is lost the moment the conversation ends, and a future session rediscovers it from scratch.
**BAD:** Noting a real, unfixed gap in prose and moving on to the next question.
**GOOD:** Create a known-issue document (via `context-file`) with an honest severity and status, in the same turn.

## Mindset

- The agent is a poor judge of its own blind spots -- this is why the reflection is structured and, where a project
  rule mandates it, effectively non-optional.
- Precision over quantity for confidence items. Better 3 specific items than 7 vague ones.
- The blind-spot check is the harder question -- it requires synthesizing across the entire session.
- If a reflection reveals a critical issue, the session was not actually over -- treat it as continuation, not
  wrap-up.
- Persist important findings as context entries so future sessions benefit -- a verified-but-deferred gap becomes a
  known-issue document, not just a paragraph in a chat transcript nobody re-reads.
- Sub-agent spawn is preferred for deep sessions, but is not mandatory for every reflection -- a short session is
  usually better served inline; consider session depth and available model cost before choosing.
- LOW and MEDIUM severity ratings on a known-issue entry are legitimate, honest outcomes, not a failure to act on
  something -- do not inflate severity just to make an item feel actioned.

## Quick Start

```bash
# No commands needed -- this is a behavioural skill, triggered by conversational cues.
# Read the Workflow section above for execution detail.
```

```text
# Expected shape of a completed reflection:
# 1. Confidence audit answered with 3-7 specific, file/function-level items.
# 2. Blind-spot check answered with 1-3 broad patterns.
# 3. Any flagged item is either resolved or filed as a known-issue document.
```

## Reference: Question Design

| Aspect | Confidence Audit | Blind-Spot Check |
|--------|-----------------|------------------|
| Origin | LLM-suggested | Sam Altman |
| Focus | Agent's own work quality | Shared understanding |
| Scope | Under-investigated items | Assumptions & alternatives |
| Depth | 3-7 specific items | 1-3 broad patterns |
| Risk caught | Silent failures in execution | Conceptual blind spots |

## Troubleshooting

| Situation | Response |
|-----------|----------|
| User says "no need" to reflection | Accept gracefully. Do not insist. |
| User asks to skip on a future session | Honour the preference; consider noting it as a project-level finding. |
| Reflection reveals a huge issue | Do not panic. Investigate calmly, present findings, offer remediation options. |
| User has no response to either question | Accept that the reflection ran -- surfacing items is valuable on its own. |

## Integration with Other Skills

| Skill | How it connects |
|-------|----------------|
| `context-file` | Persist reflection findings as context entries; persist verified-but-deferred gaps as known-issue documents (see Workflow step 6) -- this is the standing backlog of "critical, fix soon" items this skill drives |
| `adr-capture` | If a reflection reveals a decision-level blind spot, capture it as an ADR |
| `rules-management` | If reflection reveals a pattern worth codifying as a behavioural rule, create one |

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Technique origins and why the two questions work | [Session-End Reflection Reference](references/session-reflection-reference.md) | Understanding the rationale, not just the mechanics |
| Full sub-agent spawn workflow and prompt template | [Sub-Agent Spawn Pattern](references/subagent-spawn-pattern.md) | Running the reflection as a sub-agent for a deep session -- skip for short sessions |
| Model selection for the sub-agent spawn pattern | [Recommended Sub-Agent Models](references/recommended-subagent-models.md) | Choosing a cheap model once sub-agent mode is already selected |

- See also: [Sub-Agent Spawn Pattern](references/subagent-spawn-pattern.md) for the complete workflow.
