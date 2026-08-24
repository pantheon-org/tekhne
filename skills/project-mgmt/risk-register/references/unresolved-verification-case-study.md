# Case Study: Verify "Unresolved" Before Filing

An illustrative case for the "confirm this is genuinely unresolved" step in the Workflow section of `SKILL.md` --
`docs/RISK_REGISTER.md` doesn't exist in this repo yet, so treat the row below as a hypothetical, not a past incident.

## What could go wrong

A row gets filed as an "Open Decision" about, say, whether a session-local scratch-notes directory should be
committed to the repository -- but the project's own instructions already settle this question (for example,
`planning-flow.md` already states that `.context/` is deliberately gitignored working scratch, not something to
commit). The row would be wrong the moment it was written -- not because the underlying observation was false (there
may genuinely be a discrepancy worth noticing), but because the "is this already decided?" check never happened
before filing.

## The lesson

A discrepancy you have not seen before is a candidate for a register row, not a verdict that it belongs there. Before
filing:

1. Search the project's root-level agent guidance file (`CLAUDE.md`) and `.claude/instructions/` for an existing rule
   or convention that already settles the question.
2. Check any doc the item touches for a stated decision.
3. Only file the row once you have confirmed no existing convention already answers it.

A claim of "this is unresolved" needs the same scrutiny as a claim of "this is a bug" -- verify it against the actual
code/docs before trusting it. Skipping the check produces a register that looks authoritative but contains rows that
were never actually open questions.

**When to use this reference:** before filing a new "Open Decision" row, especially one that feels like a surprising
gap -- confirm it is not already settled somewhere in the project's own documentation first.
