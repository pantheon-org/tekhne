# Scenario 01: Triage Tool — Session Lifecycle Context Manager

## User Prompt

Triage the following tool into the repo:

- **Repo**: https://github.com/versatly/clawvault
- **Description**: ClawVault is an npm CLI tool providing structured markdown memory vault with observation pipeline, session lifecycle management (wake/sleep/checkpoint), knowledge graph, task/project primitives, and OpenClaw hooks.

The tool is not currently in the repo. Add it to the research collection.

## Expected Behavior

1. Search `REVIEWED.md` and/or `references/REFERENCE_INDEX.md` for the repo URL or a matching slug before writing any new file
2. Explicitly assess whether the tool is in scope, borderline, or out of scope and document the reasoning
3. Create `references/<slug>.md` (e.g. `references/versatly-clawvault.md`) using the `<author-or-org>-<tool-name>` slug convention
4. Write a TL;DR section with 3 to 8 bullets
5. Address injection mechanism, session lifecycle (wake/sleep/checkpoint), and compression or eviction handling in the architecture overview; mark inapplicable sections as N/A
6. Mark any README benchmark numbers or performance claims with `(as reported)` — no metric stated as a verified fact
7. Include a `scope-fit` field in the REVIEWED.md detailed section showing `in scope`, `borderline`, or `out of scope` with a brief reason
8. Append a row to the REVIEWED.md summary table and add a detailed section with repo URL, author, version, language, tags, scope-fit, and disposition (pending)
9. Add a new row to the appropriate category in `references/REFERENCE_INDEX.md`
10. Refrain from running `git submodule add` during triage; offer vendoring as a follow-up option only
11. Conclude by presenting the user with at least the options to promote, vendor, keep, or skip

## Success Criteria

- **Duplicate check performed before any file is created**: The agent searches REVIEWED.md and/or references/REFERENCE_INDEX.md for the repo URL or a matching slug before writing any new file.
- **Scope fit is assessed and documented**: The agent explicitly assesses whether the tool is in scope, borderline, or out of scope and documents the reasoning.
- **references/<slug>.md is created**: A new file is created at references/<slug>.md (e.g. references/versatly-clawvault.md or references/clawvault.md).
- **Slug follows naming conventions**: The slug is either <author-or-org>-<tool-name> or just the tool name for well-known tools.
- **TL;DR contains 3 to 8 bullets**: The TL;DR section in the reference file has between 3 and 8 bullet points.
- **Architecture overview covers context-relevant sections**: The architecture overview addresses injection mechanism, session lifecycle (wake/sleep/checkpoint), and any compression or eviction handling. Inapplicable sections are marked N/A.
- **Self-reported metrics marked as reported**: Any benchmark numbers or performance claims from the README are marked '(as reported)'. No metric is stated as a verified fact.
- **REVIEWED.md scope-fit field is present**: The detailed section in REVIEWED.md includes a scope-fit field showing 'in scope', 'borderline', or 'out of scope' with a brief reason.
- **REVIEWED.md summary table row and detailed section are added**: A new row is added to the summary table and a detailed section is added with repo URL, author, version, language, tags, scope-fit, and disposition (pending).
- **REFERENCE_INDEX.md is updated**: A new row is added to the appropriate category in references/REFERENCE_INDEX.md.
- **No git submodule add is executed automatically**: The agent does not run git submodule add during triage. Vendoring is offered as a follow-up option only.
- **User is offered promotion, vendor, keep, and skip choices at the end**: The agent ends by presenting at least the options to promote, vendor, keep, or skip.

## Failure Conditions

- Agent creates files without first checking for a duplicate
- Agent omits the scope fit assessment or does not document its reasoning
- Agent creates the reference file at the wrong path or with a non-conforming slug
- Agent writes fewer than 3 or more than 8 TL;DR bullets
- Agent omits context-relevant architecture sections without marking them N/A
- Agent states performance claims as verified facts without `(as reported)` qualifiers
- Agent omits the `scope-fit` field from the REVIEWED.md detailed section
- Agent omits the REVIEWED.md summary table row, detailed section, or required fields
- Agent omits the REFERENCE_INDEX.md update
- Agent runs `git submodule add` automatically during triage
- Agent does not offer the user promotion, vendor, keep, or skip choices at the end
