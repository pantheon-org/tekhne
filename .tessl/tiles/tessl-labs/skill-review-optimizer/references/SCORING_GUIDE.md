# Interpreting tessl Review Output

## Score Breakdown

**Average Score** = (Description Score + Content Score) / 2

### Description Score (out of 100%)

- **Specificity** (0-3 points): Concrete actions listed
- **Trigger Term Quality** (0-3 points): Natural keywords present
- **Completeness** (0-3 points): What AND when specified
- **Distinctiveness** (0-3 points): Clear niche, low conflict risk

### Content Score (out of 100%)

- **Conciseness** (0-3 points): Token efficiency, no verbosity
- **Actionability** (0-3 points): Executable examples present
- **Workflow Clarity** (0-3 points): Clear steps with validation
- **Progressive Disclosure** (0-3 points): Good structure, references used

## Validation Checks

All validation checks must pass (no errors):
- `skill_md_line_count`: Must be ≤ 500 lines
- `frontmatter_valid`: YAML must be valid
- `name_field`: Name must be present and valid
- `description_field`: Description required, proper length
- `description_voice`: Must use third person
- `description_trigger_hint`: Should have "Use when..." clause
- `metadata_version`: metadata.version should exist
- `body_present`: Body content must exist
- `body_examples`: Should include code examples
- `body_output_format`: Should specify outputs
- `body_steps`: Should have numbered workflow

Warnings don't block packaging but suggest improvements.

## Suggestion Categories

**High Priority** (often mentioned first):
- Add missing metadata dictionary
- Add explicit trigger hints to description
- Add workflow structure with numbered steps
- Add output format specification

**Medium Priority** (improve scores):
- Add specific concrete actions to description
- Include executable code examples
- Add validation checkpoints
- Remove verbose explanations

**Low Priority** (polish):
- Improve progressive disclosure
- Condense repetitive sections
- Add more natural trigger terms
