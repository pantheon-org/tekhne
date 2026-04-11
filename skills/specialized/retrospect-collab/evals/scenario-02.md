# Scenario 02: Identifying a Pattern of Over-Reliance on AI

## User Prompt

"Review my sessions from the last two weeks and tell me if I'm over-relying on AI outputs without verifying them."

## Expected Behavior

1. Agent runs the session filter script with `--last 14d` to load sessions from the last two weeks
2. Reads each session file and extracts tool call sequences and user prompt patterns
3. Scans the Bias dimension specifically for over-trust signals: accepting outputs without verification, automation bias, copy-paste prompts, repetitive simple requests
4. Counts and quantifies instances: "X out of Y sessions showed over-trust patterns"
5. Identifies specific prompt examples where the user accepted AI output without challenge or follow-up verification
6. Classifies sessions showing over-reliance as Automation or Low-impact augmentation
7. Produces a Stop recommendation targeting the specific over-reliance pattern
8. Writes insights to `.retro/insights/collab/{PERIOD}.md`

## Success Criteria

- **Multi-session scope processed**: Agent loads and reads sessions spanning 14 days, not just the most recent one
- **Over-trust pattern quantified**: Report states a ratio like "5 out of 9 sessions showed unchallenged acceptance of first AI output"
- **Specific prompt examples cited**: At least two actual user prompts from the session data are referenced as evidence
- **Impact distribution reflects finding**: Automation and Low-impact percentages are elevated in the metrics summary when over-reliance is prevalent
- **Stop recommendation is concrete**: SSC Stop section names the specific pattern (e.g., "Stop accepting code changes without reviewing the diff") not a generic "verify more"
- **Bias section populated**: Report's Bias and Blindspots section is non-empty and addresses automation bias

## Failure Conditions

- Agent reports over-reliance without citing session evidence
- Agent processes only one session despite the two-week scope request
- Over-trust finding is buried rather than prominently flagged in the analysis
- Agent provides only generic bias warnings without tying them to actual session behavior
- Impact categorization does not reflect the identified over-reliance (e.g., High-impact target is met despite clear automation patterns)
- Stop recommendation is vague ("be more careful") with no actionable specificity
