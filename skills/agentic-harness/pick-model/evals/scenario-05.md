# Scenario 05: Business Task Classification — Strategic Plan Requires Opus

## User Prompt

"Which model is best for drafting our 3-year market entry strategy for Southeast Asia?"

## Expected Behavior

1. Agent parses task: strategic planning, market entry, multi-region, long-horizon decision
2. Agent classifies against the Business & Strategy section of the decision matrix — strategic planning maps to Opus
3. Agent identifies multiple business escalators: strategic impact (long-term, irreversible), cross-functional (tech + business + legal + logistics), multiple stakeholders (regional teams, leadership)
4. Agent confirms Opus is correct and notes escalators reinforce but do not exceed the cap
5. Agent outputs Opus recommendation using the standard output format template with a business-domain rationale

## Success Criteria

- Agent recommends Opus (not Haiku or Sonnet)
- Agent classifies task under the Business & Strategy section (not Technical Tasks)
- Output follows the template: `[emoji] **[Model]** — [1-line reason]` on the first line
- Output includes `Cost: highest | Speed: slowest` cost/speed line
- Rationale references strategic planning, long-term impact, or multi-stakeholder signals
- Agent does not default to Sonnet as "content creation" without recognizing strategic scope

## Failure Conditions

- Agent recommends Sonnet by misclassifying the task as standard business writing or content creation
- Agent recommends Haiku for any reason
- Agent applies the Technical Tasks matrix instead of the Business & Strategy matrix
- Output omits the cost/speed line
- Agent provides no rationale for choosing Opus over Sonnet
- Agent applies no business escalators despite clear strategic impact and stakeholder signals
