# Scenario 05: Resolve a Stakeholder Conflict Using the Failure-Focused Test

## User Prompt

Two stakeholders disagree on the categorization of two features for an e-commerce checkout redesign (8-week release, 4 engineers):

**Feature A — Guest checkout (no account required)**
- Engineering lead says: "Should — users can always register."
- Marketing VP says: "Must — our conversion data shows 40% of users abandon when forced to register."

**Feature B — Animated cart micro-interactions**
- UX lead says: "Must — our brand guidelines require premium feel."
- Engineering lead says: "Could — it's polish, not function."

Apply the MoSCoW skill's failure-focused Must test to both features and produce a written arbitration document saved to `conflict-resolution.md`.

Your output must:

1. State the failure-focused Must test question ("Does the release fail without this?").
2. Apply the test to Feature A with reference to the conversion data evidence.
3. Apply the test to Feature B with explanation of why brand feel is not release failure.
4. Provide a recommended final category for each feature with justification.
5. Provide language the team can use to explain the decision to each stakeholder.

## Expected Behavior

1. Explicitly state the failure-focused test question ("Does the release fail without this?") before applying it
2. Apply the test to Feature A and reference the 40% abandonment rate as the deciding evidence (Must is defensible given conversion impact)
3. Apply the test to Feature B and conclude that brand feel / UX polish does not constitute release failure, placing it in Could or Should
4. Provide specific language or talking points for explaining each decision to the affected stakeholder (Marketing VP and UX lead)
5. Explicitly reference the skill rule that stakeholder preference is not the same as release-critical need when addressing Feature B

## Success Criteria

- **Failure-focused test question stated**: `conflict-resolution.md` explicitly states the failure-focused test question ("Does the release fail without this?") before applying it
- **Feature A correctly resolved as Must or Should with evidence**: `conflict-resolution.md` applies the test to Feature A and references the 40% abandonment rate as the deciding evidence (Must is defensible given conversion impact)
- **Feature B correctly resolved as Could or Should (not Must)**: `conflict-resolution.md` applies the test to Feature B and concludes that brand feel / UX polish does not constitute release failure, placing it in Could or Should
- **Stakeholder communication language provided**: `conflict-resolution.md` includes specific language or talking points for explaining each decision to the affected stakeholder (Marketing VP and UX lead)
- **Anti-pattern rule cited**: `conflict-resolution.md` explicitly references the skill rule that stakeholder preference is not the same as release-critical need when addressing Feature B

## Failure Conditions

- Does not state the failure-focused test question before applying it
- Does not reference the 40% abandonment data when analyzing Feature A
- Categorizes Feature B as Must without applying the failure test
- Provides recommendations without any stakeholder communication language
- Does not cite the anti-pattern rule distinguishing preference from release-critical need
