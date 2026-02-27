# Common Optimization Strategies

## Strategy 1: Fix Validation Warnings First

Validation warnings (metadata, output format, workflow steps) are quick fixes that often improve scores immediately:

1. Add metadata dictionary if missing
2. Add "Output Format" section specifying deliverables
3. Add numbered workflow steps if missing
4. Fix YAML frontmatter issues

## Strategy 2: Enhance Description for 100% Score

Description score measures specificity, trigger quality, completeness, and distinctiveness:

### Specificity

Add concrete actions (verbs + objects):
- Instead of: "Process documents"
- Better: "Extract text from PDFs, merge documents, rotate pages"

### Trigger Terms

Include natural keywords users say:
- Add: "dashboard", "web app", "interactive UI", ".pdf file"

### Completeness

Add explicit "Use when..." clause:
- Template: "Use when users want to X, Y, or Z"

### Distinctiveness

Clarify scope boundaries:
- Add: "Not for simple X" or "Specifically for Y"

## Strategy 3: Improve Content Score

Content score measures conciseness, actionability, workflow clarity, and progressive disclosure:

### Conciseness

Remove verbose explanations:
- Delete: Explanations of basic concepts
- Delete: Motivational language
- Keep: Domain-specific information only

### Actionability

Add executable examples:
- Include: Copy-paste ready code
- Include: Specific commands with placeholders
- Include: Clear input/output specifications

### Workflow Clarity

Add validation checkpoints:
- After each major step: verify output exists
- Include: "If X fails, do Y" guidance
- Add: Expected outputs at each step

### Progressive Disclosure

Move details to references:
- Keep in SKILL.md: Essential workflow and core examples
- Move to references/: Detailed patterns, advanced topics, long examples
