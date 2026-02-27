---
name: acceptance-criteria
description: Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes.
---

# Acceptance Criteria

Navigation hub for writing measurable, test-ready acceptance criteria.

## Use When

- "Write acceptance criteria for this feature."
- "Define done conditions for this story."
- "Create testable requirements before implementation."
- "Apply INVEST quality checks to this story."

## When Not to Use

- Architecture decisions and system design tradeoffs.
- Deep technical specifications about implementation internals.

## Workflow

1. Capture user outcome and business intent.
2. Select criteria format (Given/When/Then or rule-oriented).
3. Draft must-pass criteria with measurable outcomes.
4. Add negative and boundary scenarios.
5. Validate with checklist before sign-off.

## Worked Example

**Starting Point (Vague Requirement):**
> "Users should be able to reset their password easily."

**Step 1: Capture Intent**
- User outcome: User regains account access after forgetting password.
- Business intent: Reduce support tickets; maintain security.

**Step 2: Select Format**
Choose Given/When/Then (Gherkin) for clarity.

**Step 3: Draft Must-Pass Criteria**
```gherkin
Given a user is on the login page
When they click "Forgot Password"
Then a password reset form appears within 1 second

Given a user enters a valid email in the reset form
When they click "Send Reset Link"
Then an email is sent within 30 seconds
And the email contains a clickable reset link valid for 24 hours

Given a user clicks the reset link from the email
When they enter a new password (min 8 chars, 1 uppercase, 1 number)
And click "Update Password"
Then the password is updated
And they are redirected to login within 2 seconds
```

**Step 4: Add Negative & Boundary Scenarios**
```gherkin
Given a user enters an unregistered email
When they click "Send Reset Link"
Then they see "Email not found" message (no account enumeration)

Given a user's reset link has expired (>24 hours old)
When they click the link
Then they see "Link expired, request a new one"

Given a user enters a weak password (e.g., "pass")
When they click "Update Password"
Then validation error appears: "Password must be 8+ chars with uppercase and number"
```

**Step 5: Validation Checklist**
- ✓ All criteria are measurable (pass/fail).
- ✓ Happy path (valid email → reset → new password) covered.
- ✓ Failure paths (invalid email, expired link, weak password) covered.
- ✓ Wording describes outcomes, not implementation ("email is sent within 30s" not "use SendGrid API").
- ✓ Scope clear: password reset flow only, not account recovery via SMS.
- ✓ Product, engineering, and QA can all understand and test.

## Quick Commands

```bash
# Find vague language
grep -nE "\b(should be|user-friendly|fast|easy|good)\b" <file>.md
```

```bash
# Count checklist criteria lines
grep -nE "^- \[ \]" <file>.md | wc -l
```

```bash
# Detect missing negative/error words
grep -niE "invalid|error|timeout|denied|failed" <file>.md
```

## Expected Output

```markdown
User Story:
As a [role], I want [action], so that [benefit].

Acceptance Criteria (Must Have):
- [ ] ...
- [ ] ...

Negative/Edge Scenarios:
- [ ] ...

Out of Scope:
- ...
```

## Anti-Patterns

### NEVER use vague, non-measurable criteria

WHY: vague language cannot be tested objectively.
BAD: "The system should be fast." GOOD: "Search returns in <= 2s for 95% of requests."

### NEVER write implementation instructions as acceptance criteria

WHY: criteria define outcomes, not implementation details.
BAD: "Use JavaScript validation on submit button click." GOOD: "Invalid form input shows error message within 1 second."

### NEVER skip negative and boundary scenarios

WHY: real users hit error and edge conditions.
BAD: only valid-login scenario listed. GOOD: include invalid credentials, timeout, and expired session behavior.

### NEVER couple multiple behaviors into one ambiguous criterion

WHY: broad criteria hide failure source and reduce test clarity.
BAD: "Checkout and email and invoice should work." GOOD: split into separate measurable criteria.

### NEVER bypass audience alignment on definition of done

WHY: misalignment causes rework at QA/review time.
BAD: criteria written without stakeholder validation. GOOD: confirm criteria with product and QA before implementation.

## Verification Checklist

- [ ] Every criterion is measurable (pass/fail).
- [ ] Happy path and failure paths are both covered.
- [ ] Wording describes outcomes, not implementation.
- [ ] Scope boundaries are explicit.
- [ ] Criteria are understandable by product + QA.

## Quick Reference

| Topic | Reference |
| --- | --- |
| INVEST criteria | [references/invest-criteria.md](references/invest-criteria.md) |
| Gherkin examples | [references/gherkin-examples.md](references/gherkin-examples.md) |
| Pattern selection | [references/patterns-by-type.md](references/patterns-by-type.md) |
| Scenario examples | [references/examples.md](references/examples.md) |
| Reusable templates | [references/templates.md](references/templates.md) |

## References

- [Gherkin Reference](https://cucumber.io/docs/gherkin/)