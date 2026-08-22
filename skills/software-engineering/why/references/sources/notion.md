# Long-Form Documents (Notion example — adapt for Confluence via `mcp-atlassian`)

## What this source contains

- PRDs (product requirement documents)
- Technical specs and RFCs
- Architectural decision records (ADRs)
- Meeting notes from design reviews
- Team pages with domain context
- Postmortems from incidents
- Runbooks that may explain defensive code
- Strategy documents that set priorities

This is where "why" often lives in long-form before it becomes code. A significant feature usually has a doc.

## How to search it

Use whichever long-form-docs MCP is connected (Confluence via `mcp-atlassian` for this org; adapt tool names accordingly).

1. **Keyword searches.** Try:
   - The feature name
   - Key symbols / class names from the target code
   - Author handles (design docs are often authored before the code lands)
   - Error strings or user-visible terms
   - Time-bounded queries if you know when the code shipped
2. **Fetch candidate pages in full.** Read the full content, not the preview; rationale is often buried mid-document.
3. **Follow backlinks and child pages.** Design docs often have sub-pages for alternatives considered, appendices, or implementation notes.
4. **Check related spaces/databases.** Meeting-notes spaces or databases can surface discussion that led to the decision.
5. **Search author-specific spaces.** If the PR/MR author has a personal notebook or space (common at some companies), it may hold exploratory thinking that preceded the code.

## What good evidence looks like here

- A PRD with a "Problem statement" or "Motivation" section that matches the target code's purpose
- An "Alternatives considered" or "Rejected approaches" section
- A postmortem that names the target code as the fix for a specific incident
- Meeting notes that record "we decided X because Y" and tie to the same author/date range as the PR/MR
- An ADR template filled out non-trivially (status, context, decision, consequences)

## Common pitfalls

- **Outdated docs.** Specs are often written before implementation and not updated; the doc may describe a plan that changed. Cross-check against the actual PR/MR.
- **Doc vs. reality drift.** A spec may say "we'll do X" but the code actually does Y. Flag the divergence; the synthesizer will surface the contradiction.
- **Boilerplate templates.** Some orgs require a "Why" section that gets filled with fluff. Look for specificity.
- **Unlinked docs.** The most relevant doc may not be linked from anywhere. Broad keyword searches help.
- **Multiple drafts.** If a topic has multiple docs, find the one that was finalized or most recently updated. Check dates.
- **Access-restricted pages.** If you can't access a page, note it as a gap.

## What to return

For each relevant doc:
- Title and URL
- Authors and last-updated date
- The motivation text (verbatim quote), with page/section location
- Relevant linked pages (so the synthesizer can cite them)
- Whether the doc was finalized or draft
