# Scenario 02: Vague Directive Requires Reformatting

## User Prompt

"Make a rule about being more careful with error handling."

## Expected Behavior

1. Recognize that "being more careful with error handling" is not an actionable directive -- it has no precise ALWAYS/NEVER form and no clear triggering condition.
2. Do not copy the phrase verbatim into a rule entry.
3. Ask the user a clarifying question (or propose a concrete reformatted candidate directive) to pin down what "careful" means in checkable terms -- e.g. "ALWAYS log the original exception before re-raising a wrapped error" or similar, grounded in what the user actually meant.
4. Once a precise directive is settled, append it using the full three-part format, with the schema-validated tooling, exactly as any other rule.

## Failure Conditions

- A rule entry created with "be more careful with error handling" as the literal directive text.
- No attempt made to reformat or clarify the vague instruction before writing.
- The eventual entry still missing a Rationale, even after reformatting.
