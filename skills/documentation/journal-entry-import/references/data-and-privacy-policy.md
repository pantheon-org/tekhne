# Data and Privacy Policy (journal-entry-import)

Imported and reconstructed entries surface historical work from external sources (a legacy wiki, an issue
tracker). Those sources routinely contain personal data. Apply this policy on every entry produced by any of the
three modes. It is enforced by review, not by either validator - treat it as a mindset, not a checklist item you
can skip because a script did not flag it.

## Names and source content - never redact on your own judgement

- **Keep colleague and requestor names.** A work journal legitimately names the people involved. Removing a name
  is the author's decision, not yours.
- If something looks like it might warrant redaction, **import it verbatim and surface it in your reply** so the
  author can decide. Do not pre-emptively strip it.

## Credentials and tokens - never write them, no exception

- Access tokens, API keys, passwords, and similar secrets are **never** written into the repo, even when asked.
  Mask them with a short marker, e.g. `` `[access token value withheld]` ``.
- Flag the masking in your reply and offer the masked or described alternative. This is the one item you do not
  ask about first - you withhold, then tell.

## End-user/customer identifiers and raw dumps - summarise out by default

- End-user or customer identifiers (account numbers, customer IDs, transaction references) and raw SQL / XML /
  CSV data dumps are **summarised out** of narrative summaries by default. A summary does not need them, and they
  are personal data that should not accumulate in the repo.
- Note in the `## Compliance` section that they were summarised out, and offer to include them verbatim if the
  author asks.
- If the work touches a regulated domain (health, finance, gambling, or similar), treat this as a hard floor, not
  a suggestion, and say so in your reply rather than deciding silently.

## Framing

These entries are decision-support documentation about past work. They are not a regulatory record, and they
must not become an uncontrolled store of end-user or customer personal data.
