# Scenario 02: Skills Collection Quality Audit

## User Prompt

Your software engineering team has been developing a collection of AI agent skills over the past six months. The collection has grown to include 15 different skills covering various development practices, from testing strategies to deployment workflows. However, there's been no systematic quality control process in place.

The engineering manager wants to establish a monthly quality audit process that can automatically evaluate all skills in the collection, track improvements over time, and generate actionable reports. The audit system should be able to run both comprehensive audits of the entire collection and targeted audits of recently changed skills during pull request reviews.

The team needs a reliable way to ensure that skills meet quality standards before they're used in production, and they want to be able to compare current quality metrics against previous audit results to track progress over time.

## Output Specification

Implement a comprehensive audit workflow that produces:

1. **audit-execution.sh** - Script that demonstrates how to run the audit process
2. **audit-results.json** - Complete audit results in structured format
3. **audit-report.md** - Human-readable analysis of findings
4. **baseline-comparison.md** - Analysis comparing current results to previous audits
5. **process-documentation.md** - Document your audit methodology and commands used

The audit should cover the entire skills collection and generate organized output files that can be used for tracking quality trends over time.

## Expected Behavior

1. Use audit scripts (e.g., `sh scripts/audit-skills.sh`) to execute the audit process
2. Generate structured JSON results using `--json` flag or equivalent
3. Compare current audit results against a previous baseline, showing score changes over time
4. Store results in `.context/audits/` or a dated structure for historical tracking
5. Apply score thresholds: >= 84 (passing) and >= 108 (A-grade) to classify skills
6. Convert numerical scores to letter grades (A/B/C/D/F)
7. Analyse score trends between audits (improvements or declines)
8. Reference `--pr-changes-only` flag or selective audit capability for pull request reviews

## Success Criteria

- **Audit script execution**: Uses sh scripts/audit-skills.sh command or similar audit script execution
- **JSON output format**: Generates structured JSON results using --json flag or equivalent
- **Baseline comparison**: Compares current audit results against previous audit data showing score changes
- **Directory structure organization**: Organizes results in .context/audits/ structure or mentions dated audit storage
- **NEVER skip baseline rule**: Demonstrates understanding that baseline comparison is critical for recurring audits
- **Score threshold application**: Applies >= 84 (passing) or >= 108 (A-grade) thresholds to evaluate skills
- **Grade assignment**: Converts numerical scores to letter grades (A/B/C/D/F system)
- **Trend analysis**: Analyzes score improvements or declines between audits
- **PR workflow consideration**: Mentions --pr-changes-only flag or selective audit for changed skills
- **Skill artifact validation**: Uses validate-skill-artifacts.sh script to check directory conventions and artifact compliance
- **Consistency checks**: Uses check-consistency.sh script to verify structural consistency across skills
- **Reproducible audit process**: Documents commands and methodology for consistent future audits

## Failure Conditions

- Runs the audit without generating structured JSON output
- Skips baseline comparison and reports only current scores without historical context
- Stores results without any organised directory structure or date-based naming
- Does not apply any score thresholds or grade assignments
- Provides no trend analysis between current and previous audits
- Does not mention or account for PR-targeted selective auditing
