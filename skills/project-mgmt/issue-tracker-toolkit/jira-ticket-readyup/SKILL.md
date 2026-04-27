---
name: jira-ticket-readyup
description: "Ready up Jira tickets for refinement by gathering context from linked incidents, populating a YAML template following the project's 'ready for refinement' standard, validating against a JSON schema, and generating a structured markdown document. Use when asked to 'ready up', 'ready a ticket', 'prepare for refinement', or 'write up PROJ-NNN'. Keywords: jira, ticket, refinement, ready, backlog, YAML template, JSON schema, context, conditions of satisfaction, acceptance criteria."
---

# jira-ticket-readyup

Turn a sparse Jira backlog ticket into a fully-structured "ready for refinement" document by gathering incident context, applying the standard template, and validating the output.

## When to Use This Skill

Use jira-ticket-readyup when:
- User asks to "ready up" or "prepare for refinement" one or more Jira tickets
- A ticket exists but lacks Context, Conditions of Satisfaction, or Acceptance Criteria
- User wants to promote a follow-up ticket (spawned from an incident) into a proper backlog item

**Do NOT use for:**
- Creating new Jira tickets from scratch (use Jira MCP tools directly)
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

### Step 2 — Gather context from Jira

For each ticket to ready up:

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

### Step 3 — Populate the YAML data file

Create a data file at the output path using the template. The output path convention is:

```
<journal-dir>/YYYY/MM/YYYY-MM-DD-PROJ-NNN-ready-for-refinement.yaml
```

Use your judgment to synthesise context from all available sources — the ticket description, linked incident comments, and any other information gathered in Step 2. The goal is a coherent narrative that lets a developer pick up the ticket cold and understand what needs to be done and why.

Key constraints:
- Never leave a required field empty or as the placeholder `""`
- `conditions_of_satisfaction.must` must have at least one item
- `acceptance_criteria` must have at least two specific, testable items
- If no incident is linked, derive all context from the ticket description alone

### Step 4 — Validate the YAML

```bash
bun run skills/project-mgmt/issue-tracker-toolkit/jira-ticket-readyup/scripts/validate-ticket.ts \
  <path-to-ticket-data.yaml>
```

Fix any reported errors before proceeding to markdown generation.

**Prerequisite:** `bun` must be available (`which bun`).

### Step 5 — Generate the markdown output

Create the markdown file at:
```
<journal-dir>/YYYY/MM/YYYY-MM-DD-PROJ-NNN-ready-for-refinement.md
```

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

## References

| Topic | Reference | When to Use |
|---|---|---|
| YAML field guide | `references/yaml-field-guide.md` | When unsure what a template field expects |
| YAML template | `assets/templates/ready-for-refinement.yaml` | Step 1 of every workflow run |
| JSON schema | `assets/schemas/ready-for-refinement.schema.json` | Understand validation rules |
| Validation script | `scripts/validate-ticket.ts` | Steps 4 and 6 of the workflow |
