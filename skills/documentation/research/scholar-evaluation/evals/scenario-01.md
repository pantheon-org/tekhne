# Scenario 01: Scholar Evaluation — Empirical Research Paper

## User Prompt

The user wants to evaluate a machine learning paper titled "Attention Mechanisms for Drug-Target Interaction Prediction" for submission to a peer-reviewed journal.

Identify the work type, load the evaluation framework, evaluate all applicable dimensions, calculate an overall score, and produce a structured evaluation report.

## Expected Behavior

1. Explicitly identify this as an empirical research paper and select the appropriate evaluation dimensions before beginning assessment
2. Reference or load `references/evaluation_framework.md` before scoring any dimension
3. Evaluate at least Problem Formulation, Methodology, Data Quality, Analysis Rigor, and Writing Quality
4. Accompany each dimension score with specific evidence from the work — not generic statements
5. Include at least 2 prioritised, specific recommendations for improvement in the report

## Success Criteria

- **Work type identified before selecting dimensions**: The agent explicitly identifies this as an empirical research paper and selects the appropriate evaluation dimensions before beginning assessment.
- **references/evaluation_framework.md loaded**: The agent references or loads evaluation_framework.md before scoring any dimension.
- **All core dimensions addressed**: The agent evaluates at least Problem Formulation, Methodology, Data Quality, Analysis Rigor, and Writing Quality.
- **Scores provided with supporting evidence**: Each dimension score is accompanied by specific evidence from the work, not generic statements.
- **Actionable recommendations provided**: The report includes at least 2 prioritised, specific recommendations for improvement.

## Failure Conditions

- Agent begins scoring without identifying the work type or selecting appropriate dimensions
- Agent scores dimensions without referencing or loading the evaluation framework
- Agent omits one or more core dimensions (Problem Formulation, Methodology, Data Quality, Analysis Rigor, Writing Quality)
- Agent provides scores with generic statements instead of evidence specific to the paper
- Agent provides fewer than 2 actionable, prioritised recommendations
