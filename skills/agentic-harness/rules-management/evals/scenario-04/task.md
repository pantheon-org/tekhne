# Scenario 04: Overlapping Rule (Edge Case)

## User Prompt

"New rule: always validate SES sender addresses before sending a digest email."

The rules file already contains:

```markdown
### Rule: Always verify recipient identity before sending

**Directive:** ALWAYS confirm the recipient email address is on the SES verified-identity list before sending any digest email.

**Rationale:** Sending to an unverified address in sandbox mode fails silently and the delivery is recorded as sent when it never left SES.
```

## Expected Behavior

1. Read the existing rules file first, as always, and notice this new request substantially overlaps with the existing "Always verify recipient identity before sending" rule (same trigger -- sending a digest email -- and a closely related directive about address validation).
2. Do not create a second, near-duplicate entry. Surface the overlap to the user explicitly.
3. Suggest amending the existing rule (e.g. broadening its directive or rationale to also cover sender-address validation, if that's genuinely distinct) rather than duplicating.
4. Only append a new, separate entry if the user confirms after seeing the overlap that a second rule is genuinely warranted (e.g. because sender and recipient validation are truly different failure modes worth tracking separately).

## Failure Conditions

- A near-duplicate rule appended without the agent noticing or mentioning the overlap with the existing entry.
- The user not given the choice between amending the existing rule and creating a genuinely separate one.
- The existing rule silently deleted or rewritten without the user's input.
