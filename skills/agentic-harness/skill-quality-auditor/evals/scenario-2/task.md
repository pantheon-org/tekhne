# Task: Skills Collection Quality Audit

Your team maintains a collection of 12 AI agent skills in `skills/`. There has been no quality audit in six months. A new engineer wants to establish a repeatable monthly audit process that tracks quality trends over time.

Run a batch audit of the entire collection, compare results against a previous baseline stored in `.context/audits/`, and produce a summary report identifying which skills need remediation before the next sprint.

## Context

- Repository root: current working directory
- Previous baseline: `.context/audits/*/2025-10-01/audit.json` files exist for 8 of the 12 skills
- Four new skills have no baseline yet
- Grading thresholds: A >=126, B+ >=119, B >=112, C/C+ <112 (blocked from publishing)

## Output Specification

Produce:
1. **audit-execution.sh** — commands used to run the batch audit
2. **audit-results.json** — full JSON output from the batch run
3. **audit-report.md** — human-readable table: skill | score | grade | delta vs baseline
4. **baseline-comparison.md** — trend analysis: improvements, regressions, new skills
