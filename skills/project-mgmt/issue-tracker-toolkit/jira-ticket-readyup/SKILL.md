---
name: jira-ticket-readyup
description: "Ready up Jira tickets for refinement, including brand-new not-yet-keyed backlog items, by gathering context from linked incidents or interviewing the user when no incident exists, populating a YAML template against the project's 'ready for refinement' standard, validating it against a JSON schema plus gap-detection heuristics for testability and vagueness, and generating a structured markdown document. Use when asked to ready up, prepare for refinement, draft a new backlog ticket, or write up a specific ticket key."
---

# jira-ticket-readyup

Turn a sparse Jira backlog ticket — or a brand-new item with no key yet — into a fully-structured "ready for refinement" document by gathering incident context (or interviewing the user when no source material answers a required question), applying the standard template, and validating the output against both the schema and gap-detection heuristics.

## When to Use This Skill

Use jira-ticket-readyup when:
- User asks to "ready up" or "prepare for refinement" one or more Jira tickets
- A ticket exists but lacks Context, Conditions of Satisfaction, or Acceptance Criteria
- User wants to promote a follow-up ticket (spawned from an incident) into a proper backlog item
- User wants to draft a brand-new backlog ticket that has no Jira key yet (set `ticket.new_ticket: true` — see Step 2a)

**Do NOT use for:**
- Tickets that are already in "Ready for Refinement" status with full descriptions

## What a Good Ticket Looks Like

A ticket is "ready for refinement" when it has all four sections:

| Section | Purpose |
|---|---|
| **Context** | Background on the service, current behaviour, and why it matters |
| **Conditions of Satisfaction** | MUST / SHOULD / COULD requirements |
| **Acceptance Criteria** | Specific, testable conditions that define "done" |
| **Supporting Information** | Links to repos, files, and related tickets |

The reference example ticket should be supplied by the user when invoking this skill (a ticket already in "Ready for Refinement" status in their Jira project).

## Step-by-Step Workflow

### Step 1 — Read the YAML template

**MANDATORY before generating any output:**

```bash
# Always read this first — it defines the required fields and comments
skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/assets/templates/ready-for-refinement.yaml
```

Do NOT generate a ticket data file without loading this template.

### Step 2 — Gather context

**For an existing ticket** (has a Jira key):

1. Fetch the ticket itself:
   ```
   jira_get_issue(issue_key="PROJ-NNN", fields="summary,description,status,issuetype,issuelinks")
   ```

2. Fetch every linked incident (look for `issuelinks` with type "Follow-up"):
   ```
   jira_get_issue(issue_key="INC-NNN", fields="summary,description,comment")
   ```
   Comments on the incident ticket contain the investigation findings — read them all.

3. Extract from the incident:
   - Root cause
   - Service/component affected
   - Evidence (CloudWatch alarms, log errors, X-Ray traces)
   - Who investigated and what they concluded
   - Any proposed solution or follow-up actions already noted

**For a brand-new ticket** (no Jira key yet, set `ticket.new_ticket: true`): there is no ticket or incident to fetch. Gather whatever source material the user already gave you (a meeting summary, a chat thread, a prior finding) and go straight to Step 2a — the interview pass is what supplies everything a fetch would otherwise have provided.

### Step 2a — Interview for CoS/AC gaps

Run one pass per section below, **after** gathering context and **before** populating the YAML. A pass only asks the user a question when the gathered source material does not already answer it — never invent an answer, never write a placeholder, and never silently pick one option among several open ones. Ask one section's questions at a time rather than front-loading everything at once.

1. **Context / Problem.** If the source material states that something happened but not why it matters or what specifically breaks: ask what the triggering event was (incident, request, backlog carry-over, meeting decision), and what an engineer or user cannot currently do because of the gap. If the work touches a regulated system (personal data, payments, safety), ask whether a data-handling constraint needs stating in `context.background`.

2. **Conditions of Satisfaction, per MoSCoW tier.** For each candidate MUST, ask what breaks if it ships without it — if nothing breaks, it likely belongs in SHOULD, not MUST. For each candidate SHOULD, ask whether it is expected in this delivery or genuinely deferrable (then it belongs in COULD). An empty COULD tier is not a gap; only ask about it if the user wants nice-to-haves captured. If two MUST/SHOULD items look like they require different, incompatible choices, ask the user to resolve it — do not silently pick one (see "Gap-Detection Heuristics" below).

3. **Acceptance Criteria.** For each candidate AC, ask how someone who has not read the ticket would confirm it is met — what they would run, click, or observe. If the answer cannot be phrased as an observable check, the AC is not ready. If a MUST item has no AC that would fail were the MUST not met, ask what observable outcome proves it.

4. **Supporting Information.** If a CoS/AC/Background item references something (a repo, file, incident, decision) without a link, ask for it. Never invent a plausible-looking URL — ask, or leave it as an explicit open item if the user doesn't have it to hand.

### Step 3 — Populate the YAML data file

Create a data file at the output path using the template. The output path convention is:

```
<journal-dir>/YYYY/MM/YYYY-MM-DD-PROJ-NNN-ready-for-refinement.yaml
```

For a brand-new ticket, use a placeholder slug instead of `PROJ-NNN` in the filename (e.g. the first few words of the summary) since no key exists yet.

Use your judgment to synthesise context from all available sources — the ticket description, linked incident comments, and anything surfaced by the Step 2a interview. The goal is a coherent narrative that lets a developer pick up the ticket cold and understand what needs to be done and why.

Key constraints:
- Never leave a required field empty or as the placeholder `""`
- `conditions_of_satisfaction.must` must have at least one item
- `acceptance_criteria` must have at least two specific, testable items
- If no incident is linked, derive all context from the ticket description alone
- For a brand-new ticket, set `ticket.new_ticket: true` and omit `ticket.key` entirely (the schema rejects the combination of `new_ticket: true` with a `key` present)

### Step 4 — Review for gaps beyond presence/count, then validate

Before running the validator, read all MUST/SHOULD items together and check for two things a schema cannot catch (see "Gap-Detection Heuristics" below for the full set):

- **Conflict:** do any two MUST/SHOULD items require different, incompatible choices? If so, this should already have been resolved in Step 2a — if it wasn't, resolve it now rather than letting both stand.
- **Self-containment:** does any item depend on a fact not stated elsewhere in the ticket (e.g. "the way we discussed" with no record of what was discussed)? A reader with only this ticket must be able to act on every item.

Then run the validator, which also checks three more gap-detection heuristics mechanically (testability, vagueness, MUST-to-AC coverage) and prints them as advisory warnings:

```bash
bun run skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/scripts/validate-ticket.ts \
  <path-to-ticket-data.yaml>
```

Fix every schema/field error before proceeding to markdown generation. Gap-detection warnings are advisory and can false-positive — review each one, but they do not block by default. Pass `--strict-gaps` to fail the run on any gap-detection warning (useful in CI or when you want a hard gate).

**Prerequisite:** `bun` must be available (`which bun`).

### Step 5 — Generate the markdown output

Create the markdown file at:
```
<journal-dir>/YYYY/MM/YYYY-MM-DD-PROJ-NNN-ready-for-refinement.md
```

For a brand-new ticket, use the same placeholder slug from Step 3 in place of `PROJ-NNN`.

Use this structure exactly:

```markdown
# <Summary> — <Ticket Key>

**Type:** <Bug|Feature|Maintenance|Investigation>
**Linked Incident:** [<INC-NNN>](<jira-url>)  *(omit if none)*

## Context

<background paragraph>

<current behaviour paragraph>

**Key implications:**

- <implication 1>
- <implication 2>

## Conditions of Satisfaction

- [ ] **MUST** <requirement>
- [ ] **MUST** <requirement>
- [ ] **SHOULD** <requirement>
- [ ] **COULD** <requirement>

## Acceptance Criteria

- <criterion 1>
- <criterion 2>

## Supporting Information

- [<Repo name>](<url>)
- [<File path>](<url>) — <description>
- [<INC-NNN>](<url>) — <incident summary>
```

For a brand-new ticket (`ticket.new_ticket: true`), replace the `<Ticket Key>` in the H1 and drop the em dash if there is none yet, and add one line directly under the H1 stating this is a draft with no ticket created yet: `*Draft — no ticket key yet; creating the ticket and assigning a key is a separate, human-confirmed step, not performed by this skill.*`

### Step 6 — Validate the markdown

```bash
bun run skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/scripts/validate-ticket.ts \
  <path-to-ticket-data.yaml> \
  --markdown <path-to-ticket-output.md>
```

### Step 7 — Report to user

When complete, report:
- Path to the YAML data file
- Path to the markdown output file
- Validation result (OK or errors)
- A one-paragraph summary of what the ticket is about

## Output File Location

Store outputs in the journal directory:

```
<journal-dir>/YYYY/MM/
  YYYY-MM-DD-PROJ-NNN-ready-for-refinement.yaml   ← data file (validated source)
  YYYY-MM-DD-PROJ-NNN-ready-for-refinement.md     ← markdown output
```

Use today's date for YYYY-MM-DD. The `journal-dir` is the current project root.

## Gap-Detection Heuristics

Beyond the schema's presence/count checks (>=1 MUST, >=2 AC, minimum lengths), five heuristics catch gaps a count cannot:

| # | Heuristic | Where it runs |
|---|---|---|
| 1 | **Testability** — an acceptance criterion has no verb describing a checkable action/state (return, produce, emit, contain, respond, ...) and no measurable token (a number, quoted value, HTTP/error code) | `validate-ticket.ts`, advisory warning |
| 2 | **Aspiration/vagueness** — a CoS or AC item is only a comparative/quality adjective with no concrete threshold ("more robust", "better logging", "improve reliability", "properly handle") | `validate-ticket.ts`, advisory warning |
| 3 | **MUST/SHOULD conflict** — two items require different, incompatible choices | Manual review, Step 2a and Step 4 (needs semantic judgment, not scriptable) |
| 4 | **Self-containment** — an item depends on a fact not stated elsewhere in the ticket | Manual review, Step 4 (needs semantic judgment, not scriptable) |
| 5 | **MUST-to-AC coverage** — a MUST item has no acceptance criterion that would fail if the MUST were not met (checked via keyword overlap, not exact — a heuristic, not proof) | `validate-ticket.ts`, advisory warning |

Heuristics 1, 2, and 5 print as advisory warnings by default (they can false-positive — read each one) and only fail the run when `--strict-gaps` is passed. Heuristics 3 and 4 cannot be scripted reliably because they require understanding what an item means, not just its text, so they are explicit review steps for the agent (Step 2a asks the interview questions that head off most conflicts before they're written down; Step 4 is the last check before validation).

## Template Schema Reference

The canonical template is at:
```
skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/assets/templates/ready-for-refinement.yaml
```

The JSON schema is at:
```
skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/assets/schemas/ready-for-refinement.schema.json
```

## Anti-Patterns

### NEVER generate the ticket data file without reading the YAML template first
WHY: The template defines required fields and their expected format. Skipping it produces output that fails schema validation or omits required sections.
❌ BAD: Writing context.background directly without consulting the template structure.
✅ GOOD: Run Step 1 (read template) every time, even for the second ticket in a session.

### NEVER skip fetching incident comments
WHY: The root cause, evidence, and proposed fix almost always appear in comments, not in the incident description. Skipping comments produces shallow context and vague acceptance criteria.
❌ BAD: Fetching only INC-NNN description, ignoring comments.
✅ GOOD: Fetch all comments via `jira_get_issue(fields="comment")` and read every one.

### NEVER proceed to markdown generation if YAML validation fails
WHY: A broken YAML file silently propagates errors into the markdown and downstream tooling. The validate-ticket.ts script exists precisely to catch these before they spread.
❌ BAD: Continuing to generate markdown after a schema validation error, assuming it "looks fine".
✅ GOOD: Fix every validation error reported by validate-ticket.ts before calling Step 5.

### NEVER write conditions of satisfaction as vague aspirations
WHY: "Improve reliability" is untestable. Refinement sessions get blocked when teams cannot determine whether a requirement is met.
❌ BAD: MUST make the service more robust.
✅ GOOD: MUST return HTTP 400 with error code `INVALID_POSTAL_CODE` when input fails the expected validation pattern.

### NEVER guess or leave a placeholder when source material doesn't answer a required question
WHY: A guessed background, MUST, or acceptance criterion looks complete and passes the schema, but it's fiction — the ticket reader has no way to tell it apart from something actually confirmed. The whole point of Step 2a is that the agent stops and asks instead of filling the gap itself.
❌ BAD: The meeting notes don't say what happens if the change ships without it, so writing "MUST ensure this is handled" anyway to keep the ticket moving.
✅ GOOD: Ask the user what breaks if it ships without it (Step 2a, pass 2); use their answer, or leave it as an explicit open item if they don't know yet.

### NEVER treat a clean validator run as proof the ticket is testable
WHY: `validate-ticket.ts`'s schema check only confirms fields are present and meet a length minimum — it says nothing about whether an item is actually verifiable, non-contradictory, or self-contained. The gap-detection heuristics exist because presence and testability are different properties.
❌ BAD: Schema validation passed, so the ticket is ready — skip Step 4's manual conflict/self-containment review.
✅ GOOD: Always do the Step 4 manual review (conflict, self-containment) even when the automated gap-detection warnings are empty; the two scripted heuristics do not cover everything.

## References

| Topic | Reference | When to Use |
|---|---|---|
| YAML field guide | `references/yaml-field-guide.md` | When unsure what a template field expects |
| YAML template | `assets/templates/ready-for-refinement.yaml` | Step 1 of every workflow run |
| JSON schema | `assets/schemas/ready-for-refinement.schema.json` | Understand validation rules, including the `new_ticket`/`key` cross-field rule |
| Validation script | `scripts/validate-ticket.ts` | Steps 4 and 6 of the workflow; pass `--strict-gaps` to hard-fail on gap-detection warnings |
| Gap-Detection Heuristics (this file) | `## Gap-Detection Heuristics` above | Understand which checks are scripted vs. manual review |
