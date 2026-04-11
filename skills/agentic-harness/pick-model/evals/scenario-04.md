# Scenario 04: Complexity Escalator — Production Risk Upgrade

## User Prompt

"Should I use Haiku or Sonnet to fix a SQL bug in our production payment processing service?"

## Expected Behavior

1. Agent parses task: SQL bug fix (initially a Haiku/Sonnet candidate), but production payment processing noted
2. Agent classifies base task as Sonnet (single-system bug fix, moderate reasoning)
3. Agent identifies the "stakes" complexity escalator: production system, data-loss risk, financial domain
4. Agent applies the +1 tier escalator: Sonnet → Opus
5. Agent outputs Opus recommendation with explicit reference to the production/stakes escalator
6. Agent may include the optional consideration note about Sonnet if stakes are lower (exploratory/staging)

## Success Criteria

- Agent recommends Opus (not Haiku or Sonnet)
- Output follows the template: `[emoji] **[Model]** — [1-line reason]` on the first line
- Output includes `Cost: highest | Speed: slowest` cost/speed line
- Rationale explicitly names the stakes/production escalator as the reason for upgrade
- Agent does not stay at Sonnet despite the explicit production risk signal
- Agent correctly demonstrates one escalator upgrading from the base classification

## Failure Conditions

- Agent recommends Sonnet or Haiku despite the production payment system signal
- Agent applies no complexity escalator and stays at base Sonnet classification
- Agent recommends Opus but does not explain which escalator triggered the upgrade
- Agent suggests the user's framing (Haiku vs Sonnet) is the full option space without considering Opus
- Output omits the cost/speed line
