---
name: retrospect-collab
description: Analyze collaboration patterns (HOW) and compute metrics from captured sessions. Use when reviewing collaboration quality, analyzing human-AI interaction, computing session metrics. Triggers include "retrospect collab", "collaboration analysis", "session patterns", "how am I collaborating".
argument-hint: "[--last Nd|--week|--month|--from DATE --to DATE]"
allowed-tools: bash, read, write, grep
model: opus
context: main
user-invocable: true
cynefin-domain: clear
cynefin-verb: execute
---

# Retrospect Collab — Collaboration Analysis

**Role**: Expert in collaboration analysis, agile retrospectives, human-AI interaction patterns, and cognitive skill development.

## When to Use

Apply this skill when the user asks about:
- How they collaborate with AI ("how am I collaborating?", "session patterns")
- Collaboration quality review for a specific time window
- Computing impact metrics (Automation / Low-impact / High-impact breakdown)
- Longitudinal skill progression analysis across multiple sessions

## When Not to Use

- Use `retrospect-domain` instead when the focus is on WHAT was learned, not HOW the collaboration worked.
- Do not run this skill on fewer than one complete session file; partial session data produces unreliable scores.

## Principles

- **Evidence-only**: Reference actual prompts and tool calls from sessions. NEVER invent patterns that are not supported by session data.
- **Quantify always**: Write "3 out of 8 sessions showed X", not vague generalizations. ALWAYS state ratios, not adjectives.
- **Impact targets**: >60% high-impact augmentation, <20% automation. Surface the gap explicitly when targets are missed.
- **Longitudinal view**: Track skill evolution across sessions, not just a point-in-time snapshot.

## Anti-Patterns

<!-- BAD: report claims "user often accepted outputs without challenge" | GOOD: "user accepted AI output without follow-up in 6 of 9 exchanges in session 2026-03-14" -->

- **NEVER fabricate session evidence** — WHY: the entire value of this skill is grounded analysis; invented evidence erodes trust and makes the retrospective useless
- **NEVER report a single session as a trend** — WHY: one instance is an observation, not a pattern; patterns require recurrence across at least two sessions
- **NEVER skip the impact categorization step** — WHY: without Automation/Low/High classification the metrics summary is incomplete and the report template cannot be filled
- **NEVER use a hardcoded output path** — WHY: the `.retro/insights/` location is project-configurable; assume it, don't hardcode it

## Steps

1. **Filter sessions**:
   ```bash
   bash ${CLAUDE_PLUGIN_ROOT}/scripts/retrospect-load-sessions.sh $@
   ```
   First line: `PERIOD: YYYY-MM-DD_to_YYYY-MM-DD`. Remaining: session paths.

   > **Note**: `CLAUDE_PLUGIN_ROOT` points to the script bundle location. The learnings output path (`.retro/insights/`) is project-configurable — set it to wherever your project stores retrospective outputs.

2. **Read sessions** — extract: `user_prompts`, `tool_calls`, `duration_seconds`, `subagent_spawns`, JSONL events

3. **Analyze technical effectiveness**:
   - Context Management: preparation, over/under-dump, progressive feeding
   - Guidance: prompt clarity, exploration vs constraints balance
   - Critical Thinking: user challenges, AI pushback, alternative-seeking
   - Bias: over-trust, dismissal, confirmation bias, automation bias

4. **Analyze cognitive posture**:
   - Intentionality: structured prompts vs trial-and-error, "why before how"
   - Agency: custom construction vs template copy-paste, decision ownership
   - Impact: categorize each session (Automation / Low-impact / High-impact augmentation)
   - Progression: session 1→N sophistication, prompt evolution, strategic vs tactical

5. **Generate Start/Stop/Continue** with specific session examples

6. **Write insights** to `.retro/insights/collab/{PERIOD}.md`
   ```bash
   # Output path example (project-configurable)
   .retro/insights/collab/2026-03-01_to_2026-03-31.md
   ```

7. **Report**: file path, suggest `/retrospect report` for aggregates

## Usage Examples

Run for the last 7 days:
```bash
/retrospect collab --last 7d
```

Run for the current week:
```bash
/retrospect collab --week
```

Run for a custom date range:
```bash
/retrospect collab --from 2026-03-01 --to 2026-03-31
```

Run and then view aggregates:
```bash
/retrospect collab --month
/retrospect report
```

## Metrics Summary Format

The Metrics Summary section uses this format:

```markdown
## Metrics Summary
- **Sessions**: 8 | **Total prompts**: 64 (avg 8) | **Total tool calls**: 112 (avg 14)
- **Total duration**: 3h 20m (avg 25m/session) | **Subagents**: 4
- **Impact**: High 62% (>60% target ✓) | Low 25% | Auto 13% (<20% target ✓)
```

## Gotchas

- The session filter script outputs a PERIOD header on the first line; everything after is a file path. Parse carefully — do not treat the PERIOD line as a session path.
- `duration_seconds` may be absent in older session files. Optionally skip duration metrics rather than erroring.
- Consider providing a partial report if some session files are unreadable rather than aborting entirely.
- Run the session loader directly to test it: `./scripts/retrospect-load-sessions.sh --last 7d`

## References

- [Reference](references/reference.md) — scoring rubrics, metric definitions, and report format
