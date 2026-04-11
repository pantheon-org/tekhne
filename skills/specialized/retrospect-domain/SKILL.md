---
name: retrospect-domain
description: Analyze domain insights (WHAT/WHY learned) from captured sessions. Use when reviewing learnings, extracting patterns, analyzing decisions. Triggers include "retrospect domain", "domain analysis", "what did I learn", "session insights".
argument-hint: "[domain] [--last Nd|--week|--month|--from DATE --to DATE]"
allowed-tools: bash, read, write, grep
model: opus
context: main
user-invocable: true
cynefin-domain: clear
cynefin-verb: execute
---

# Retrospect Domain — Domain Insights Analysis

Analyze captured sessions to extract domain learnings: what worked, decisions made, patterns emerged.

## Arguments

- **Domain** (optional first arg): `technical` | `research` | generic (default)
- **Timeframe**: `--last Nd` | `--week` | `--month` | `--from DATE --to DATE` | all

## When to Use

Apply this skill when the user asks about:
- What was learned in sessions ("what did I learn?", "session insights")
- Extracting domain patterns or recurring decisions from a time window
- Surfacing anti-patterns or gotchas discovered during work
- Generating a domain knowledge summary across multiple sessions

## When Not to Use

- Use `retrospect-collab` instead when the focus is on HOW the collaboration worked, not WHAT was learned.
- Do not run this skill against zero session files; no sessions means no domain insights to extract.

## Principles

- **Extract, don't invent**: Base every insight on actual session content. NEVER fabricate learnings not present in the session data.
- **Be specific**: Reference actual conversations, tools used, and decisions made. ALWAYS name the session when citing evidence.
- **Detect patterns**: Look for themes across multiple sessions, not isolated observations.
- **Domain-appropriate**: Apply the domain framework matching the session type. Consider falling back to the generic framework only when domain content is ambiguous.
- **Framework flexibility**: Optionally adapt analysis headings when session content does not fit the standard framework questions.

## Anti-Patterns

<!-- BAD: "you learned to avoid N+1 queries" (invented) | GOOD: "session 2026-03-14 revealed an N+1 query in the reporting module — query was rewritten using eager loading" -->

- **NEVER report a learning that is not evidenced in the session** — WHY: fabricated insights give false confidence and may direct future work incorrectly
- **NEVER apply the generic framework to clearly technical sessions** — WHY: the Technical Domain framework surfaces six specific questions that generic analysis misses, reducing insight quality
- **NEVER omit the Patterns Emerged section when reviewing more than two sessions** — WHY: cross-session synthesis is the primary value-add of this skill; single-session summaries are not enough
- **NEVER hardcode the output path** — WHY: the `.retro/insights/` location is project-configurable; use the configured path, not a hardcoded one

## Steps

1. **Load domain framework** (if specified):
   - Check for a `domain-{domain}.md` file at the project-configurable domain frameworks location.
   - If missing: warn, use generic framework

   > **Note**: The domain frameworks location is project-configurable — point it to wherever your project stores domain reference files.

2. **Filter sessions**:
   ```bash
   bash ${CLAUDE_PLUGIN_ROOT}/scripts/retrospect-load-sessions.sh $@
   ```
   First line: `PERIOD: YYYY-MM-DD_to_YYYY-MM-DD`. Remaining: session paths.

   > **Note**: `CLAUDE_PLUGIN_ROOT` points to the script bundle location. The learnings output path (`.retro/insights/`) is project-configurable — set it to wherever your project stores retrospective outputs.

3. **Read session files** — extract conversation turns, tools used, git context

4. **Analyze domain insights** using framework:
   - What new patterns/approaches discovered?
   - What decisions were made and why?
   - What worked well / what failed?
   - What patterns recur across sessions?

5. **Generate Start/Stop/Continue** recommendations

6. **Write insights** to `.retro/insights/domain/{PERIOD}.md`
   ```bash
   # Output path example (project-configurable)
   .retro/insights/domain/2026-03-01_to_2026-03-31.md
   ```

7. **Report**: file path, suggest `/retrospect collab` or `/retrospect report`

## Usage Examples

Run for the last 7 days with the technical domain framework:
```bash
/retrospect domain technical --last 7d
```

Run for the current month with no domain filter (generic):
```bash
/retrospect domain --month
```

Run for a custom date range with the research framework:
```bash
/retrospect domain research --from 2026-03-01 --to 2026-03-31
```

Run and then follow up with collaboration analysis:
```bash
/retrospect domain technical --week
/retrospect collab --week
```

## Report Section Format

The domain report follows this structure:

```markdown
# Domain Retrospective: YYYY-MM-DD to YYYY-MM-DD

## Sessions Reviewed
- [List sessions with timestamps and brief summaries]

## Domain Learnings
### What I Learned
### What Worked Well
### What Didn't Work
### Decisions Made
### Patterns Emerged

## Start/Stop/Continue
## Action Items
## Metrics Summary
```

## Gotchas

- The session filter script outputs a PERIOD header on line one; parse it separately from session file paths.
- Domain framework files may not exist for every domain. ALWAYS warn the user and fall back to generic rather than aborting.
- Consider including a short "Sessions Reviewed" list at the top of the report so the reader knows what data was used.
- Run the session loader directly to test it: `./scripts/retrospect-load-sessions.sh --last 7d`

## References

- [Reference](references/reference.md) — scoring rubrics, metric definitions, and report format
