# Scenario 3: Apply the data and privacy policy while enriching

## User Prompt

"Enrich PROJ-4050 into a detail entry."

## Input

The issue tracker resolves `PROJ-4050` (fetched with comments) as:

- **summary:** "Promo code redemption fails for lowercase codes", **status:** Done, **resolutiondate** 2020-11-10
- **reporter:** Dana Whitfield, **assignee:** Alex Rivera
- **description:** "Customer reports a promo code fails to redeem when entered in lowercase."
- **comments include:**
  - Alex Rivera: the redemption lookup table was case-sensitive against an uppercase-only index; two casings of
    the same code resolved to different reward records. Pasted table:
    `| SPRING10 | reward_id=8841 | spring10 | reward_id=9002 |`
  - Alex Rivera: pulled the affected redemption IDs: `8841, 9002, 9110, ...` (140 rows)
  - Sam Okafor (support): chased the follow-up report.

## Expected Behavior

1. Create the detail entry as normal (dated 2020-11-10, full schema).
2. **Keep colleague names**: Alex Rivera and Sam Okafor appear as written.
3. **Mask the raw lookup values**: the reward-record identifiers pulled from the code/reward table are NOT
   written into the file; replace them with a short marker (e.g. `[lookup values withheld]`).
4. **Summarise out the bulk identifier list**: the 140 redemption IDs and the raw code/reward table are described
   ("two casings resolved to two different reward records"), not reproduced verbatim.
5. **Do not redact the case-sensitivity detail itself** - it is the substance of the bug; only the raw lookup
   values and the bulk identifier list are withheld/summarised. If unsure whether to drop a colleague name, keep
   it and surface the question rather than removing it unilaterally.
6. Note in `## Compliance` that lookup values were masked and the identifier list summarised out.

## Success Criteria

- Colleague names retained verbatim.
- No raw lookup-table values and no redemption-ID list present anywhere in the file.
- The case-sensitivity bug and the dual-record explanation are still conveyed (masked, not deleted).
- Compliance section records the masking/summarising.
- Entry passes the ticket-detail validator.

## Failure Conditions

- Any raw lookup value or the bulk redemption-ID list written into the file.
- A colleague name removed without being asked (over-redaction).
- The substance of the bug lost because content was deleted rather than masked/summarised.
- Compliance section silent about the withheld/summarised material.
