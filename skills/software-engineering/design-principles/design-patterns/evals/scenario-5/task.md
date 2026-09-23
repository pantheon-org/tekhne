# Scenario 5: Evaluate Pattern Fit and Recommend Against Premature Extraction

## User Prompt

A developer has asked you to apply the Strategy pattern to the following `TaxCalculator`. Review the code and determine whether the pattern is warranted.

## Expected Behavior

1. Correctly note in the Problem Statement that there is no current conditional logic, coupling problem, or varying algorithm — the code is a simple calculation
2. Explicitly state in the Pattern Fit Assessment that the Strategy win condition (eliminating if/else selection between algorithms) is not met because only one implementation exists
3. Recommend NOT applying the Strategy pattern now in the Recommendation section
4. Reference YAGNI or an equivalent statement about not applying patterns for imagined future needs
5. Describe a concrete code event in the Trigger Condition that would justify the pattern (e.g. "when a second tax type with a different formula must be added and selected conditionally")

## Failure Conditions

- Problem Statement claims there is a design problem in the current single-method class
- Pattern Fit Assessment states the Strategy win condition is already met with a single implementation
- Recommendation is to apply the pattern now, despite no concrete need
- No reference to YAGNI or anti-preemptive pattern principle
- Trigger Condition is vague (e.g. "when we need it") rather than describing a specific code change
