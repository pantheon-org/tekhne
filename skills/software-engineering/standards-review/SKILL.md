---
name: standards-review
description: Review a saved standards document via a one-standard-at-a-time interview. Prompts the user for the file path, parses individual standards, then presents each one for evaluation with concrete options (Accept / Revise / Reject / Defer). Produces a summary report at the end.
---

# Standards Review

Review a set of project standards through a structured, one-at-a-time interview. Each standard is presented individually with actionable options; the user picks one and moves on. Ends with a confirmed recap and a written report.

## Mindset

Every standard should be clear, enforceable, and necessary. If it's not all three, it needs work.

The interview is a triage loop — for each standard, present it, get the user's action, and move on. Do not over-analyse a single standard or extend the interview beyond the user's stated scope.

## When to Use

- A team has codified standards (in `RULES.md`, `CONTRIBUTING.md`, or similar) and wants to audit them
- Onboarding to a new project and reviewing whether existing standards still apply
- Before a migration or refactor, checking which standards are still relevant
- After a period of drift, to clean up stale or contradictory standards
- The user explicitly asks for a "standards review" or to "audit our rules"

## Prerequisites

- The standards file must exist at a path the agent can read.
- The file must have parseable structure (Markdown headings, numbered lists, or line-separated entries).

## Workflow

### 1. Get the file path

Ask the user for the path to their standards file. Use a single `question` tool call with the file path as the only question.

Expected file formats, in priority order:

| Format | Parsing rule |
|--------|-------------|
| **Markdown** (`*.md`) | Each `##` / `###` / `####` heading block is one standard. If no headings exist, each bullet list item (`-` / `*`) is one standard. |
| **Plain text** | Each non-empty line is one standard. |

Read the file with the `Read` tool. Parse it into individual standard entries.

If the file has no detectable structure, ask the user how standards are delimited before continuing.

### 2. Confirm scope

Tell the user how many standards were found and ask if they want to review all of them. If the count is 0, report that no standards were detected and offer to re-parse with a user-specified delimiter.

### 3. Review each standard — one at a time

For each standard, use the `question` tool to present **exactly one question**. Never bundle multiple standards or multiple questions into a single call.

Each question must:
- Show the standard's text. If it exceeds ~200 characters, include a summary and note the full length.
- Offer 3-4 concrete, mutually exclusive options.
- Let the free-text/"Other" path remain available (automatic with the `question` tool).

The default option set, which applies to every standard unless the user explicitly requests different criteria:

1. **Accept** — Clear, actionable, necessary. Keep as-is.
2. **Revise** — Has merit but needs clarification, updating, or concrete examples.
3. **Reject** — Outdated, redundant, unnecessary, or counterproductive.
4. *(Other)* — Defer to team discussion, needs more context, or describe another action.

Collect the answer silently. Do not discuss or debate the user's assessment. Store it internally for the summary.

When the user picks "Other", capture what they said as their desired action. If their free-text answer matches one of the curated options (e.g. "I think we should remove this"), map it to the closest curated label for the summary.

### 4. Stop when done

After the last standard has been answered, stop asking questions immediately. Do not ask follow-ups about individual answers, and do not offer to revisit standards unless the user explicitly requests it.

### 5. Recap and confirm

Present a summary table of all standards reviewed:

```
| # | Standard | Action | Notes |
|---|----------|--------|-------|
| 1 | First standard title...     | Accept | |
| 2 | Second standard title...    | Revise | needs examples |
| 3 | Third standard title...     | Reject | duplicates rule X |
```

Use a single `question` call: "Does this recap look right? Any changes before I write the report?" with options:
1. **Looks good — write the report**
2. **Need to make changes** — describe what to fix

If the user wants changes, apply them, then re-recap once. Do not loop indefinitely.

### 6. Write the review report

After confirmation, produce a review output. Default to writing a `.context/findings/` file named `standards-review-{file-stem}-{date}.md`. If `.context/findings/` does not exist, write to `.context/` instead, or ask the user where to put it.

The report must include:

```markdown
# Standards Review: {filename}

**Reviewed:** {date}
**File:** {path}
**Standards found:** {N}
**Actions:** Accept: {X}, Revise: {Y}, Reject: {Z}, Other/Deferred: {W}

## Summary

{One-paragraph overview — key themes, what's healthy, what needs work}

## Detail

### 1. {Standard title or first N chars}
**Text:** {full standard text}
**Action:** {Accept | Revise | Reject | Defer}
**Notes:** {if the user added any}

## Next Steps

- {X} standards to keep — no action needed
- {Y} standards to revise — {list them}
- {Z} standards to reject — {list them, with rationale}
- {W} to discuss further — {list them}
```

## Anti-Patterns

**NEVER** present more than one standard per question turn. This is the core constraint of the skill — one standard, one question, one turn.

**NEVER** skip the recap and confirmation. Without it, the user has no chance to correct misinterpretations before the report is written.

**NEVER** make up assessment criteria that the user didn't agree to. The default option set (Accept/Revise/Reject/Other) applies; if the user wants different criteria, they will use the free-text path.

**NEVER** debate the user's assessment. The review captures their judgement, not the agent's opinion. If a standard is plainly contradictory or impossible, you may note it in the report's summary section, but do not argue during the interview.

**NEVER** ask about the same standard twice. Once assessed, that standard is closed.

**NEVER** bundle an evaluation question with a follow-up (e.g., "Accept or Revise, and also tell me why"). Let the user answer the action question first; if they volunteer notes in their answer, capture them. If not, move on.

## Verification

- Was the file path asked for and successfully read?
- Were standards correctly parsed from the file (count matches user's expectation)?
- Was exactly one standard presented per question tool call?
- Were exactly 3-4 curated options offered per standard (plus automatic free-text)?
- Was a recap presented and confirmed before the report was written?
- Does the output report match the confirmed answers?
- Does the report file exist at the expected path?
