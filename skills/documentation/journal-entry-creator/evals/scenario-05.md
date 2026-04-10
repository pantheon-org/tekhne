# Scenario 05: Edge Case Documentation Task

## User Prompt

Yesterday, your team encountered an unusual situation where the automated deployment pipeline failed in a way that hadn't been seen before. The failure occurred during the database migration step, but the error messages were misleading and pointed to network connectivity issues rather than the actual schema conflict.

The incident required investigation across multiple systems including the CI/CD pipeline logs, database transaction logs, and application monitoring dashboards. The resolution involved rolling back a specific migration script and coordinating with the database team to resolve schema conflicts.

This was particularly tricky because the error occurred on a file with a very specific name that included API references and CloudWatch monitoring components. The team needs to document this complex scenario for future reference, ensuring all the technical details are captured properly.

Document this incident with:

- Detailed problem description and symptoms
- Investigation process across multiple systems
- Root cause analysis findings
- Resolution steps taken
- Preventive measures for the future

Create the documentation for today's date (use current date). The file should be named appropriately to reflect the complex technical nature involving API_Gateway_CloudWatch_Migration issues, and should be placed in the correct directory structure for the current date.

The following files represent the current state. Extract them before beginning.

```yaml
=============== FILE: inputs/existing-journal.md ===============
---
title: "API Gateway CloudWatch Migration Issue"
date: 2025-02-27
tags:
  - troubleshooting
  - api-gateway
  - cloudwatch-migration
---

This file already exists in the target location to test overwrite handling.
```

## Expected Behavior

1. Show evidence of reading the template schema before attempting generation
2. Convert technical terms in the filename to lowercase hyphenated format (e.g., `api-gateway-cloudwatch` not `API_Gateway_CloudWatch`)
3. Use hyphens instead of underscores throughout the filename slug
4. Ask the user's permission before overwriting the existing file, or create an alternative filename
5. Use the current date (`2025-02-27`) for filename, frontmatter, and H1
6. Place the file in the `2025/02/` directory matching the current date
7. Format the H1 as `February 27, 2025` (not `February 27th` or `Feb 27`)
8. Convert technical tags to lowercase-hyphenated format (e.g., `cloudwatch-migration`)
9. Include the required `troubleshooting` tag for this incident type
10. Include language specifiers on all code blocks, no bare backticks
11. Run the validation script and address any compliance issues found

## Success Criteria

- **Schema loading first**: Evidence shows template schema was read before attempting generation
- **Lowercase filename slug**: Converts technical terms to lowercase hyphenated format (api-gateway-cloudwatch not API_Gateway_CloudWatch)
- **No underscores in slug**: Uses hyphens instead of underscores in filename slug
- **Overwrite protection**: Asks user permission before overwriting existing file or creates alternative filename
- **Current date usage**: Uses current date (2025-02-27) for filename, frontmatter, and H1
- **Directory structure**: Places file in 2025/02/ directory matching the current date
- **Date format precision**: H1 uses `February 27, 2025` format (not February 27th or Feb 27)
- **Tag case conversion**: Converts technical tags to lowercase-hyphenated format (cloudwatch-migration)
- **Tag type requirement**: Includes required `troubleshooting` tag for this incident type
- **Code block compliance**: All code blocks include language specifiers, no bare backticks
- **Validation workflow**: Runs validation script and addresses any compliance issues found

## Failure Conditions

- No evidence of reading the template schema before generation
- Filename slug contains uppercase letters or camelCase (e.g., `API_Gateway_CloudWatch`)
- Filename slug uses underscores instead of hyphens
- Overwrites the existing file without asking user permission or providing an alternative
- Uses an incorrect or missing date in filename, frontmatter, or H1
- File placed in wrong directory (not `2025/02/`)
- H1 uses `February 27th` or `Feb 27` instead of `February 27, 2025`
- Tags contain uppercase letters or underscores
- `troubleshooting` tag is missing
- Any code block is missing a language specifier
- No evidence of running the validation script
