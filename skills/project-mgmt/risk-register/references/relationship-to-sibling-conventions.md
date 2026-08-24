# Relationship to Sibling Conventions

How the register relates to the two other places deferred-work-shaped information can live in this project: the
tech-debt list, and session-local scratch notes.

## Relationship to `tech-debt`

`docs/TECH_DEBT.md` (the `tech-debt` skill) is this register's sibling for code-level cleanup that carries no real
cost or risk -- lint/style violations, small known messiness, nothing that would belong in this register. It shares
this skill's scaffolding pattern (dedicated skill, template, schema, validation script) but deliberately drops the
append-only guard, since there is no decision to preserve an audit trail of.

If a row here turns out, on inspection, to actually be cleanup rather than a real risk, that is a correction to make
in the register itself (amend or raise it), not a silent move into the tech-debt list. The two lists are not
interchangeable just because both track "things not fixed yet" -- the register is for items with a real cost or risk
if left unaddressed; the tech-debt list is for items with neither.

## Relationship to session-local scratch notes

Session-local, gitignored working notes (created via the `context-file` skill, typically under a `follow-ups/` or
`known-issues/` typology) carry the full investigative detail behind a shortcut or deferred item. This register is
the durable, cross-session, cross-teammate index that survives when those working notes do not (different machine,
cleared cache, a teammate who never had them).

When filing a scratch-note file for something that should survive beyond the current session (most should), also add
a register row in the same session -- do not leave it to a later pass. The two are not mutually exclusive: the row
is the permanent pointer, the scratch note is detail that may itself later be lost without the underlying risk being
lost too.

**When to use this reference:** deciding whether something belongs in the register, the tech-debt list, or neither;
writing a register row that needs to point back to a scratch-note file for more detail.
