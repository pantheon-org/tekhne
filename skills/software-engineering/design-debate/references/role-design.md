# Role Design Reference

Detailed guidance for assigning debate roles (see `SKILL.md` Step 4 for the short version).

## The three roles

| Role | Stance | Job |
|------|--------|-----|
| Advocate | Argue FOR the change | Find the strongest real version of the case, grounded in the brief's facts — not generic best-practice platitudes |
| Skeptic | Argue AGAINST the change / for the status quo | Find the strongest real reasons this is premature, unnecessary, or net-negative |
| Migration/Risk (optional 3rd) | Neither for nor against | Assume the change happens — find every concrete thing that would break, need updating, or fail silently |

PREFER two roles (Advocate, Skeptic) for most debates — that's the minimum for real
tension. ALWAYS add the Migration/Risk role when the change has real implementation
surface: multiple files, scripts, or downstream consumers that a pure for/against framing
would never surface (an already-diverged vendored copy of a script, a silent check-mode
gap, a CI-gated eval scenario that would need rewriting).

## Why not just add more complementary lenses instead?

`plan-review`'s Technical/Strategic/Risk lenses all critique the *same already-written*
document from different angles, and TYPICALLY converge on a similar overall verdict —
that's fine there, because the document already exists and the goal is validating it, not
deciding whether to write it. An unwritten idea needs actual disagreement, not three
angles on agreement. AVOID reusing plan-review's lens framing here; it produces polite
consensus where this pattern needs friction.

## Sample role-specific instructions

```text
ADVOCATE: Argue FOR <the decision>. Investigate the actual repo to find the strongest
real case — not a generic best-practice argument that would apply to any codebase.
```

```text
SKEPTIC: Argue AGAINST <the decision>, defending the status quo. Investigate whether
the pain motivating this idea is already mitigated some other way.
```

```text
MIGRATION/RISK: Assume <the decision> happens. Investigate every file, script, or
consumer that would need to change, and flag anything that would fail silently.
```

BY DEFAULT, give all three roles the identical grounding-facts brief from Step 3 — never
a different brief per role, or differences in their conclusions can't be attributed to
role alone.
