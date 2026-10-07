# Follow-up grading: severity and priority

A follow-up can carry two grades, set together with the day they were confirmed. Anything that lists follow-ups to pick the next piece of work, such as the context index's readers, a session-start summary or the `follow-up` skill, can sort by them. Grades apply to follow-ups only.

## The fields

```yaml
severity: HIGH
priority: P2
graded: 2026-10-07
```

- **`severity`** is what is at risk if the follow-up is never done.
- **`priority`** is how soon it should be picked up.
- **`graded`** is the day the user confirmed the two grades. It is never filled in automatically.

The three lines go at the end of the frontmatter, just before the closing `---`, in the order `severity`, `priority`, `graded`. That keeps them away from the `status:` line that other sessions edit when they close a follow-up. They are set together: a follow-up has both grades and a `graded` date, or none. A follow-up with none of them is ungraded. One with only one grade, no `graded` date, or a value not listed below is invalid, and readers show it as written with a `?`. Readers match values case-insensitively.

## Severity: what is at risk if it is never done

| Value      | Decision test                                                                                                                                  |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `CRITICAL` | Never doing it leaves a regulated system, participant data, a security exposure or a broken gate open.                                         |
| `HIGH`     | Never doing it leaves a real defect, a misleading result or a blocked workflow in place. People work around it, but the workaround costs them. |
| `MEDIUM`   | Never doing it leaves friction or drift: slower work, a stale document, a tool that misleads now and then.                                     |
| `LOW`      | Never doing it changes little: a tidy-up, a nice-to-have, a note for later.                                                                    |

Severity is the follow-up's own risk. It is not inherited from its plan's `value`: a follow-up of a `HIGH`-value plan can be `LOW`.

## Priority: how soon it should be picked up

| Value | Decision test                                                                             |
| ----- | ----------------------------------------------------------------------------------------- |
| `P1`  | It should be picked up this week, or it blocks other work now.                            |
| `P2`  | It should be picked up within about a month, or before the work it relates to next moves. |
| `P3`  | It can wait until someone is working in that area anyway.                                 |

A follow-up with an open `blocked-by` is not `P1`, because it cannot be started.

## Who sets a grade

1. The agent proposes `severity` and `priority`, with a one-line reason, when it files a follow-up or during a grading batch.
2. The user confirms or changes them, or leaves the follow-up ungraded.
3. Only then are the three lines written, with `graded` set to that day. With the generator, that means passing `--severity`, `--priority` and `--graded` only after the user has confirmed.

Where the project has a guardrail that asks the user before any grade line is written or changed, it enforces step 2. Where it has none, this rule is the only check. A grade written without the user's confirmation is a defect to revert, not a judgement to keep.

A file that already has a `graded` line is never re-proposed for grading unless the user asks, even if a reader shows no grade for it. Readers that take grades from the context index show none until the index has been regenerated with an indexer that carries them.

## Changing or closing

- **To regrade,** the user says so and the agent rewrites the three lines with a new `graded` date.
- **Closing a follow-up** leaves its grades in place. Readers ignore grades on closed follow-ups.

## Sort order

Graded follow-ups come first, by severity (`CRITICAL` before `HIGH`, `MEDIUM`, `LOW`), then priority (`P1` before `P2`, `P3`), then newest date, then path. The ones needing a grade, ungraded or invalid, follow, newest first, and are counted.
