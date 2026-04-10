# Scenario 01: Skill Quality Assessment

## User Prompt

Your development team has created several agent skills for internal use, but there's no standardized way to evaluate their quality. The team lead wants to establish a consistent evaluation framework that can identify which skills are production-ready and which need improvement.

One particular skill called "database-optimization" has been flagged for review. The team suspects it may contain too much basic documentation that experienced developers already know, but they're not sure how to measure this objectively. They need a comprehensive quality assessment that covers multiple dimensions of skill effectiveness.

The assessment should help determine whether the skill contains expert knowledge that justifies its existence, follows proper structural conventions, provides clear guidance on when and when not to use it, and includes appropriate examples and anti-patterns.

## Output Specification

Create a comprehensive quality assessment that includes:

1. **evaluation-report.md** - A detailed evaluation following a structured framework
2. **scoring-breakdown.txt** - Numerical scores with justification for each evaluation dimension
3. **recommendations.md** - Specific actionable recommendations for improvement
4. **assessment-process.md** - Document the methodology you used for this assessment

## Expected Behavior

1. Apply all 8 evaluation dimensions: Knowledge Delta, Mindset+Procedures, Anti-Pattern Quality, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability
2. Assess Knowledge Delta by identifying redundant content (basic concepts, installation instructions) and calculating redundancy percentage accurately
3. Use the correct 120-point scoring system (Knowledge Delta: 20, other dimensions: 15/10/15) and reference the A-grade target of >= 108 points (>= 90%)
4. Correctly classify basic SQL syntax, installation commands, and generic best practices as redundant content — not expert knowledge
5. Identify missing domain-specific patterns, production gotchas, decision frameworks, and anti-patterns with WHY explanations
6. Provide specific numerical scores per dimension with justification, and actionable improvement recommendations

## Success Criteria

- **8-dimension framework**: Uses all 8 evaluation dimensions: Knowledge Delta, Mindset+Procedures, Anti-Pattern Quality, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability
- **Knowledge Delta assessment**: Identifies redundant content (basic concepts, installation instructions) and calculates redundancy percentage accurately
- **120-point scoring system**: Uses correct point allocation (Knowledge Delta: 20, others: 15/10/15) totaling 120 points maximum
- **A-grade target**: References >= 108 points as A-grade target or >= 90% threshold
- **Expert vs redundant classification**: Correctly identifies that basic SQL syntax, installation commands, and generic best practices are redundant content
- **Missing expert knowledge**: Identifies lack of domain-specific patterns, production gotchas, decision frameworks, or anti-patterns with WHY explanations
- **Specification compliance check**: Evaluates frontmatter quality, description field, and activation keywords
- **Progressive disclosure evaluation**: Assesses content structure, notes lack of references/ directory, and evaluates frontloading issues
- **Numerical scoring breakdown**: Provides specific scores for each dimension with justification
- **Mindset principles**: Treats scores as directional signals and prioritizes deterministic checks over subjective opinions
- **Self-audit awareness**: Mentions that skill-quality-auditor must pass its own evaluator with score >= 100
- **Actionable recommendations**: Provides specific improvement suggestions based on identified weaknesses

## Failure Conditions

- Applies fewer than 8 evaluation dimensions or omits major dimensions like Knowledge Delta
- Fails to distinguish between expert/unique knowledge and content that any developer already knows
- Uses an incorrect or arbitrary scoring system not matching the 120-point framework
- Does not identify the A-grade threshold or provides a wrong threshold
- Classifies basic SQL syntax or installation steps as expert knowledge
- Provides vague recommendations without tying them to specific identified weaknesses
