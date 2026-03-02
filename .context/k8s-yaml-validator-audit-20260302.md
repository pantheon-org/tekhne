# k8s-yaml-validator Skill Quality Audit

**Date**: 2026-03-02  
**Skill Path**: skills/k8s-yaml-validator

## Overall Score: 93%

## Validation Checks

All validation checks PASSED (0 errors, 0 warnings):

- ✔ skill_md_line_count - SKILL.md line count is 380 (<= 500)
- ✔ frontmatter_valid - YAML frontmatter is valid
- ✔ name_field - 'name' field is valid: 'k8s-yaml-validator'
- ✔ description_field - 'description' field is valid (292 chars)
- ✔ compatibility_field - 'compatibility' field not present (optional)
- ✔ allowed_tools_field - 'allowed-tools' field not present (optional)
- ✔ metadata_version - 'metadata' field not present (optional)
- ✔ metadata_field - 'metadata' field not present (optional)
- ✔ license_field - 'license' field not present (optional)
- ✔ frontmatter_unknown_keys - No unknown frontmatter keys found
- ✔ body_present - SKILL.md body is present

## Judge Evaluation

### Description: 100%

- **specificity**: 3/3 - Lists multiple specific concrete actions: 'validating, linting, and testing Kubernetes YAML resources', 'debugging YAML syntax errors', 'performing dry-run tests on clusters', 'working with Custom Resource Definitions (CRDs)'. These are concrete, actionable capabilities.
- **trigger_term_quality**: 3/3 - Excellent coverage of natural terms users would say: 'Kubernetes', 'YAML', 'validating', 'linting', 'manifests', 'syntax errors', 'dry-run', 'CRDs', 'Custom Resource Definitions'. These are terms users naturally use when working with Kubernetes configurations.
- **completeness**: 3/3 - Clearly answers both what ('validating, linting, and testing Kubernetes YAML resources') and when ('Use this skill when validating Kubernetes manifests, debugging YAML syntax errors, performing dry-run tests on clusters, or working with Custom Resource Definitions'). Has explicit 'Use this skill when...' clause with multiple trigger scenarios.
- **distinctiveness_conflict_risk**: 3/3 - Clear niche focused specifically on Kubernetes YAML validation and testing. The combination of 'Kubernetes', 'manifests', 'CRDs', and 'dry-run tests' creates a distinct domain that wouldn't conflict with general YAML or document processing skills.

**Assessment**: This is a well-crafted skill description that excels across all dimensions. It provides specific capabilities, uses natural trigger terms that Kubernetes users would actually say, includes an explicit 'Use this skill when...' clause with multiple scenarios, and carves out a clear niche that distinguishes it from general YAML or document processing skills.

### Content: 85%

- **conciseness**: 2/3 - The skill is comprehensive but includes some redundant explanations (e.g., explaining what dry-run modes do, verbose decision trees). The report structure section is detailed but could be more concise since Claude understands markdown formatting.
- **actionability**: 3/3 - Provides fully executable bash commands, specific tool invocations with exact flags, concrete JSON output examples, and copy-paste ready code blocks throughout. The workflow stages have clear, specific commands.
- **workflow_clarity**: 3/3 - Excellent sequential workflow with 6 clearly numbered stages, explicit validation checkpoints, decision trees for error handling, and feedback loops (e.g., 'fix and re-validate'). The partial parsing behavior and error handling strategies are well-documented.
- **progressive_disclosure**: 3/3 - Well-structured with clear overview, conditional loading of reference files (k8s_best_practices.md, validation_workflow.md), and organized resources section. References are one level deep and clearly signaled with specific load conditions.

**Assessment**: This is a high-quality skill with excellent workflow clarity, strong actionability through concrete commands and examples, and good progressive disclosure. The main weakness is moderate verbosity - some sections could be tightened without losing clarity, particularly the report structure examples and decision tree explanations.

### Suggestions for Improvement

1. **Condense the 'Stage 6: Generate Detailed Validation Report' section** - The example tables and format specifications are overly detailed for Claude's capabilities
2. **Simplify the dry-run decision tree** to a more compact format since Claude can infer the logic from fewer examples

## Average Score: 93%

## Status

✔ Skill evaluation completed successfully!
