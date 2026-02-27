---
name: skill-review-optimizer
description: Automate iterative skill improvement using tessl skill review. Use when optimizing skills, improving skill quality scores, iterating on skill design based on tessl feedback, or systematically enhancing skill descriptions and content. Runs tessl reviews, parses scores and suggestions, identifies missing metadata fields, rewrites descriptions with concrete actions, restructures content sections, adjusts frontmatter fields, and guides incremental refinement until target scores are achieved.
license: MIT
metadata:
  version: "1.0.0"
  category: "skill-development"
---

# Skill Review Optimizer

Automate the process of iteratively improving skills using `tessl skill review` feedback until they achieve target quality scores.

## Output Format

Displays baseline scores, suggestions, and score progression directly to stdout for immediate review and action.

## Workflow

1. **Setup**: Verify tessl is installed (auto-install if needed via npm or brew)
2. **Baseline**: Run initial `tessl skill review` to get starting scores
3. **Analyze**: Parse review output to extract scores, warnings, and suggestions
4. **Improve**: Apply suggested improvements to skill files based on feedback
5. **Validate**: Re-run review to verify improvements and measure progress
6. **Iterate**: Repeat steps 4-5 until target score reached or max iterations hit
7. **Summarize**: Generate final report with all changes and score progression

## Prerequisites

Tessl CLI (auto-installs via npm or brew if missing)

## Quick Start

### Step 1: Run Baseline Review

Run `scripts/optimize_skill.py` from this skill directory:

```bash
python3 scripts/optimize_skill.py /path/to/skill [--max-iterations N]
```

**Target criteria**: No validation errors, Description score 100%, Content score ≥ 90%

### Step 2: Apply Improvements Based on Suggestions

The script identifies improvement opportunities but does not auto-apply edits. Review suggestions and make targeted changes:

**Metadata fields**: Add missing frontmatter entries
```yaml
metadata:
  version: "1.0.0"
  category: "your-category"
```

**Description improvements**: Add concrete action verbs and trigger terms
```yaml
description: Automate X by doing Y. Use when user needs Z. Performs A, B, and C.
```

**Content actionability**: Replace vague guidance with executable commands
```bash
# Instead of: "Run the build"
# Write: "npm run build"
```

See [STRATEGIES.md](references/STRATEGIES.md) for comprehensive optimization patterns.

### Step 3: Iterate Until Target Reached

After making improvements, re-run the script to measure progress. Continue the improve → review cycle until target criteria are met.

```bash
python3 scripts/optimize_skill.py /path/to/skill
```

**Validation checkpoint**: If score decreased or unchanged after 3 iterations, review [STRATEGIES.md](references/STRATEGIES.md) for alternative approaches. Focus on the first 2-3 suggestions in review output—these typically have highest impact on scores.

## Troubleshooting

**Scores not improving**: Review suggestions in output, focus on highest-impact items first. See [STRATEGIES.md](references/STRATEGIES.md) for proven optimization patterns.

**Understanding scores**: See [SCORING_GUIDE.md](references/SCORING_GUIDE.md) for how tessl evaluates description and content quality.

**Validation errors**: Fix YAML frontmatter, ensure required fields (name, description) exist.
