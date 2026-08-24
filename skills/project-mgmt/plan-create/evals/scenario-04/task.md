# Scenario 04: Flat 15-Step List Needs Splitting Into Phases

## User Prompt

"Here's my rough list for the observability plan: 1) add X-Ray tracing to the
handler, 2) add X-Ray to collectors, 3) add X-Ray to analysis, 4) add X-Ray to
delivery, 5) create the DLQ, 6) wire DLQ redrive alarm, 7) add CloudWatch log
group, 8) add metric filter for delivery failures, 9) add metric filter for
processing failures, 10) add alarm for delivery failures, 11) add alarm for
processing failures, 12) create SNS topic, 13) subscribe email to SNS, 14)
build the dashboard, 15) add a cost budget. Just turn this into a plan as-is."

## Expected Behavior

1. Do NOT create a single phase containing all 15 tasks -- that violates the
   "no phase with more than 8 tasks" rule and leaves no shippable checkpoint.
2. Group the 15 items into 2-5 phases by what can ship independently, e.g.
   tracing (1-4), DLQ + alarms (5-11), dashboard/notification/budget
   (12-15) -- exact grouping may vary, but each phase must have 2-5 tasks.
3. Identify which tasks within a phase are independent of each other and
   mark them as a parallel wave (e.g. items 1-4, one per subsystem, likely
   have no cross-dependency and can be a single wave).
4. Give each phase an exit criterion (a deliverable or passing check), not
   just "done when all items are ticked."
5. State why the phases are ordered the way they are (e.g. tracing first
   because later phases' alarms may reference traced spans).
6. Still populate the required frontmatter (title, type, status, date,
   effort, value, themes).

## Success Criteria

- No single phase contains more than 8 tasks.
- 2-5 phases total, each with 2-5 tasks.
- At least one wave of parallel tasks is explicitly identified within a
  phase.
- Each phase states an exit criterion.
- The plan states a reason for the phase ordering (a dependency, not just
  the order the user listed them).
- Frontmatter is complete and valid.

## Failure Conditions

- All 15 items placed in a single "Phase 1" or left as an unordered flat
  list.
- Any phase with more than 8 tasks.
- No wave/parallelism annotation anywhere.
- Phases with no stated exit criterion.
- Frontmatter missing or incomplete.
