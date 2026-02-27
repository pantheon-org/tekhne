# Skills Collection Quality Audit

## Problem/Feature Description

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

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/skills/skill-a/SKILL.md ===============
---
name: skill-a
description: Testing patterns
---
# Basic Testing
Write tests for your code...
```

=============== FILE: inputs/skills/skill-b/SKILL.md ===============
---
name: skill-b
description: Advanced deployment strategies
---
# Deployment Patterns
Deploy applications safely...
```

=============== FILE: inputs/previous-audit.json ===============
{
  "audit_date": "2026-01-15",
  "total_skills": 2,
  "average_score": 76,
  "skills": {
    "skill-a": {"score": 72, "grade": "C"},
    "skill-b": {"score": 80, "grade": "B-"}
  }
}
```