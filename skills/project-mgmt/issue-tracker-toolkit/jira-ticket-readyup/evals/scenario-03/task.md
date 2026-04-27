# Scenario 03: Ready Up an Investigation Ticket (No Linked Incident)

## User Prompt

The following Jira ticket needs to be readied up for refinement.

**Ticket:** PROJ-789
**Summary:** Investigate elevated memory usage in batch-processing service during overnight runs
**Status:** Backlog
**Type:** Investigation
**Linked issues:** none

The ticket description says: "A CloudWatch memory alarm has fired on the
batch-processing service three nights in a row, each time around 02:00 UTC when
the nightly data-export job runs. X-Ray traces show the job accumulating objects
inside `ResultAccumulator.process_batch()` across the full duration of the run,
but no explicit memory leak has been identified yet. We need a thorough
investigation to determine root cause and propose a remediation plan."

Ready up PROJ-789. Store outputs in the journal under today's date.
