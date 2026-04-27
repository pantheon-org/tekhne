# YAML Field Guide — ready-for-refinement.yaml

A concise reference for every field in the data file template. Use this when unsure what a field expects before populating it.

---

## `ticket`

Top-level metadata identifying the Jira ticket being readied up.

| Field | Purpose | Example value |
|---|---|---|
| `ticket.key` | Jira issue key | `PROJ-NNN` |
| `ticket.summary` | Ticket title as it appears in Jira | `Handle invalid input in the address-lookup service` |
| `ticket.type` | Issue type — controls markdown heading and linked-incident line | `Bug` / `Feature` / `Maintenance` / `Investigation` |
| `ticket.linked_incident` | Key of the incident that triggered this ticket; omit or leave blank if none | `INC-NNN` |

---

## `context`

Narrative sections that explain the situation to a reader arriving cold.

| Field | Purpose | Example value |
|---|---|---|
| `context.background` | What is this service? What does it do? Why did this ticket come about? | `The address-lookup service validates and normalises postal codes before passing them to the fulfilment API. This ticket was raised after a customer-reported error during checkout.` |
| `context.current_behaviour` | What is currently broken or missing — the problem statement in concrete terms | `When an input with an unrecognised format is submitted, the service raises an unhandled exception and returns HTTP 500 instead of a validation error.` |
| `context.implications` | Bullet list of concrete effects — errors observed, alarms triggered, user or business impact | `- Customers cannot complete checkout when their postcode contains a trailing space` |

---

## `conditions_of_satisfaction`

Requirements using MoSCoW priority. Each item is phrased as a full sentence beginning with the keyword.

| Field | Purpose | Example value |
|---|---|---|
| `conditions_of_satisfaction.must` | Non-negotiable requirements — these must be delivered for the ticket to be considered done | `MUST return HTTP 400 with error code INVALID_POSTAL_CODE when the input does not match the expected pattern` |
| `conditions_of_satisfaction.should` | Strongly recommended but not strictly blocking | `SHOULD log the offending value at WARNING level with a correlation ID for traceability` |
| `conditions_of_satisfaction.could` | Optional improvements that would add value if time permits | `COULD add a metrics counter for validation failures to support alerting` |

---

## `acceptance_criteria`

Ordered list of specific, independently testable conditions that define "done". Each criterion must be falsifiable.

| Field | Purpose | Example value |
|---|---|---|
| `acceptance_criteria` (each item) | A single testable condition an engineer can verify by running a test or inspecting behaviour | `Submitting a postcode with an invalid format returns HTTP 400 and no downstream call is made` |

Minimum: two items.

---

## `supporting_information`

Links and references that give reviewers and engineers the context they need to start work.

| Field | Purpose | Example value |
|---|---|---|
| `supporting_information.repositories[].url` | URL of a relevant code repository | `https://github.com/example-org/address-lookup-service` |
| `supporting_information.repositories[].description` | Short description of what the repository contains | `Main service repository` |
| `supporting_information.files[].path` | Relative path to a specific source file relevant to the work | `src/validator.py` |
| `supporting_information.files[].description` | What is notable about this file and why it is referenced | `Entry point for the validation logic (line 42)` |
| `supporting_information.links[].url` | URL to a related ticket, runbook, or external document | `https://example.atlassian.net/browse/INC-NNN` |
| `supporting_information.links[].description` | Short description of what the link points to | `Incident ticket that triggered this work` |
