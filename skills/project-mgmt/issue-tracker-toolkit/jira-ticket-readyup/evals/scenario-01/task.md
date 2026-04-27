# Scenario 01: Ready Up a Bug Ticket with Linked Incident

## User Prompt

The following Jira ticket needs to be readied up for the next refinement session.

**Ticket:** PROJ-501
**Summary:** Payment processor returns 500 on invalid postal code input
**Status:** Backlog
**Type:** Bug
**Linked issue:** INC-2034 (incident, follow-up relationship)

The incident INC-2034 was raised when the payment-service CloudWatch alarm fired
after customers with unusual postal code formats caused unhandled exceptions in the
upstream validator. The incident comments contain a root-cause analysis identifying
that the postal code field passes through to the external payment API without
format validation at the service boundary.

Ready up PROJ-501 for refinement. Store outputs in the journal under today's date.
