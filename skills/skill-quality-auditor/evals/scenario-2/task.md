# Skill Improvement Planning

## Problem/Feature Description

Your team has completed a quality audit of your AI agent skills and found several that fall below the acceptable quality threshold. The audit identified specific weaknesses in different skills: some lack expert knowledge, others have poor anti-pattern documentation, and a few need better structural organization.

The development lead wants you to create a systematic improvement plan for one of the underperforming skills. The plan needs to be structured, actionable, and include specific steps that developers can follow to address each identified weakness. The plan should also include success criteria so the team can verify when improvements are complete.

The improvement plan needs to be detailed enough that any team member could implement it, with specific examples of what changes to make, which files to modify, and how to measure success. The plan should follow a consistent format that can be reused for other skills in the future.

## Output Specification

Create a comprehensive remediation plan that includes:

1. **remediation-plan.md** - Complete structured improvement plan
2. **success-criteria.md** - Measurable targets for verification
3. **implementation-steps.sh** - Script showing key commands and validation steps
4. **plan-validation.md** - Document validating the plan follows proper format

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/audit-results.json ===============
{
  "skill_name": "api-testing",
  "current_score": 67,
  "target_score": 108,
  "current_grade": "D+",
  "target_grade": "A",
  "dimension_scores": {
    "knowledge_delta": 8,
    "mindset_procedures": 11,
    "anti_pattern_quality": 5,
    "specification": 12,
    "progressive_disclosure": 7,
    "freedom_calibration": 10,
    "pattern_recognition": 6,
    "practical_usability": 8
  },
  "critical_issues": [
    {
      "issue": "Contains basic API concepts that developers already know",
      "dimension": "D1",
      "severity": "Critical"
    },
    {
      "issue": "Missing NEVER statements with WHY explanations", 
      "dimension": "D3",
      "severity": "High"
    },
    {
      "issue": "All content frontloaded in single 400-line file",
      "dimension": "D5", 
      "severity": "High"
    }
  ]
}
```

=============== FILE: inputs/schema-template.json ===============
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["executive_summary", "critical_issues", "phases"],
  "properties": {
    "executive_summary": {
      "type": "object",
      "required": ["current_score", "target_score", "effort"]
    },
    "phases": {
      "type": "array",
      "items": {
        "required": ["phase_name", "target", "steps"]
      }
    }
  }
}
```